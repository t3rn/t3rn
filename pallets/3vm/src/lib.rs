#![feature(associated_type_defaults)]
#![feature(slice_take)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
    dispatch::DispatchResult, ensure, sp_runtime::DispatchError, traits::Currency,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::StaticLookup;
use sp_std::vec::Vec;
use t3rn_primitives::{
    account_manager::Outcome,
    circuit::{LocalStateExecutionView, OnLocalTrigger, VacuumEVMOrder},
    contract_metadata::ContractType,
    contracts_registry::{AuthorInfo, ContractsRegistry, KindValidator, RegistryContract},
    threevm::{
        LocalStateAccess, ModuleOperations, Precompile, PrecompileArgs, PrecompileInvocation,
        Remunerated, Remuneration, SignalOpcode, ThreeVm, VacuumAccess,
    },
};
use t3rn_sdk_primitives::signal::{ExecutionSignal, Signaller};

const LOG_TARGET: &str = "3vm";

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod precompile;
pub mod remuneration;
pub mod signal;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type CurrencyOf<T> = <T as pallet::Config>::Currency;
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {

    use crate::BalanceOf;
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::BlockNumberFor;
    use sp_std::vec::Vec;
    use t3rn_primitives::{
        account_manager::AccountManager,
        circuit::OnLocalTrigger,
        contract_metadata::ContractType,
        contracts_registry::ContractsRegistry,
        portal::Portal,
        threevm::{AddressMapping, VacuumAccess},
        ChainId,
    };

    use t3rn_sdk_primitives::signal::SignalKind;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The ID of the circuit
        type CircuitTargetId: Get<ChainId>;

        /// Determines the tolerance of debouncing signal requests that have already been sent.
        #[pallet::constant]
        type SignalBounceThreshold: Get<u32>;

        /// The pallet that handles the contracts registry, used to fetch contracts
        type ContractsRegistry: ContractsRegistry<Self, Self::Currency>;

        type Currency: Currency<Self::AccountId>;

        /// The address of the escrow account
        #[pallet::constant]
        type EscrowAccount: Get<Self::AccountId>;

        /// Asset Id for the account manager
        type AssetId;

        /// The account manager that handles the escrow pool
        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            BlockNumberFor<Self>,
            Self::AssetId,
        >;
        type AddressMapping: AddressMapping<Self::AccountId>;

        type VacuumEVMApi: VacuumAccess<Self>;

        /// A provider that will give us access to on_local_trigger
        type OnLocalTrigger: OnLocalTrigger<Self, BalanceOf<Self>>;

        /// Inject access to portal so contracts can use light clients
        type Portal: Portal<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Holds the amount of times the signal was posted or attempted to be posted
    #[pallet::storage]
    pub(crate) type Signals<T: Config> = StorageDoubleMap<_, Identity, T::Hash, Identity, u32, u32>;

    /// A mapping of precompile pointers
    #[pallet::storage]
    pub(crate) type PrecompileIndex<T: Config> = StorageMap<_, Identity, T::Hash, u8>;

    /// A mapping of a contract's address to its author.
    #[pallet::storage]
    #[pallet::getter(fn author_of)]
    pub(crate) type AuthorOf<T: Config> = StorageMap<_, Identity, T::AccountId, T::AccountId>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub precompiles: Vec<(T::Hash, u8)>,
        #[serde(skip)]
        pub _marker: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (hash, ptr) in &self.precompiles {
                <PrecompileIndex<T>>::insert(hash, ptr);
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A signal event was bounced back, because a signal was already sent for the current step. [step, kind, xtx_id]
        SignalBounced((u32, SignalKind, T::Hash)),
        /// A signal event was bounced beyond the threshold. [step, kind, xtx_id]
        ExceededBounceThrehold((u32, SignalKind, T::Hash)),
        /// A module was instantiated from the registry [id, module_author, module_type, module_len]
        ModuleInstantiated((T::Hash, T::AccountId, ContractType, u32)),
        /// An author of a module was stored [contract, author]
        AuthorStored((T::AccountId, T::AccountId)),
        /// An author of a module was removed [contract]
        AuthorRemoved(T::AccountId),
    }

    #[derive(PartialEq)]
    #[pallet::error]
    pub enum Error<T> {
        /// A user exceeded the bounce threshold for submitting signals
        ExceededSignalBounceThreshold,
        /// You can't submit side effects without any side effects
        CannotTriggerWithoutSideEffects,
        /// The contract could not be found in the registry
        ContractNotFound,
        /// An origin could not be extracted from the buffer
        InvalidOrigin,
        /// The contract cannot be instantiated due to its type
        CannotInstantiateContract,
        /// The contract cannot remunerate due to its type
        ContractCannotRemunerate,
        // TODO: this is not implemented yet?
        /// The contract cannot have storage due to its type
        ContractCannotHaveStorage,
        /// The contract cannot generate side effects due to its type
        ContractCannotGenerateSideEffects,
        /// The precompile pointer was invalid
        InvalidPrecompilePointer,
        /// Invalid precompile arguments
        InvalidPrecompileArgs,
        /// Invalid arithmetic computation causes overflow
        InvalidArithmeticOverflow,
        DownstreamCircuit,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {
    pub fn get_author(contract: &<T::Lookup as StaticLookup>::Source) -> Option<T::AccountId> {
        let contract = T::Lookup::lookup(contract.clone()).ok()?;

        log::debug!(target: LOG_TARGET, "Reading author {:?}", contract);
        Self::author_of(contract)
    }
}

impl<T: Config> Precompile<T, BalanceOf<T>> for Pallet<T> {
    fn lookup(dest: &T::Hash) -> Option<u8> {
        precompile::lookup::<T>(dest)
    }

    fn invoke_raw(precompile: &u8, args: &[u8], output: &mut Vec<u8>) {
        log::debug!(
            target: LOG_TARGET,
            "Invoking raw precompile {:?} with arguments: {:?}",
            precompile,
            args
        );
        precompile::invoke_raw::<T>(precompile, &mut &args.to_vec()[..], output)
    }

    fn invoke(
        args: PrecompileArgs<T, BalanceOf<T>>,
    ) -> Result<PrecompileInvocation<T, BalanceOf<T>>, DispatchError> {
        precompile::invoke(args)
    }
}

impl<T: Config> LocalStateAccess<T, BalanceOf<T>> for Pallet<T> {
    fn load_local_state(
        origin: &T::RuntimeOrigin,
        xtx_id: Option<&T::Hash>,
    ) -> Result<LocalStateExecutionView<T, BalanceOf<T>>, DispatchError> {
        <T as Config>::OnLocalTrigger::load_local_state(origin, xtx_id.cloned())
    }
}

impl<T: Config> VacuumAccess<T> for Pallet<T> {
    fn evm_order(
        origin: &T::RuntimeOrigin,
        vacuum_evm_order: VacuumEVMOrder,
    ) -> Result<bool, DispatchError> {
        <T as Config>::VacuumEVMApi::evm_order(origin, vacuum_evm_order)
    }

    fn evm_bid(
        origin: &T::RuntimeOrigin,
        vacuum_evm_order: VacuumEVMOrder,
    ) -> Result<bool, DispatchError> {
        <T as Config>::VacuumEVMApi::evm_bid(origin, vacuum_evm_order)
    }

    fn evm_confirm(
        origin: &T::RuntimeOrigin,
        vacuum_evm_order: VacuumEVMOrder,
    ) -> Result<bool, DispatchError> {
        <T as Config>::VacuumEVMApi::evm_confirm(origin, vacuum_evm_order)
    }

    fn evm_3d_order(
        origin: &T::RuntimeOrigin,
        vacuum_evm_order: VacuumEVMOrder,
    ) -> Result<bool, DispatchError> {
        <T as Config>::VacuumEVMApi::evm_3d_order(origin, vacuum_evm_order)
    }
}

impl<T: Config> Signaller<<T as frame_system::Config>::Hash> for Pallet<T> {
    type Result = Result<SignalOpcode, DispatchError>;

    fn signal(signal: &ExecutionSignal<<T as frame_system::Config>::Hash>) -> Self::Result {
        signal::signal::<T>(signal).map_err(|e| {
            // TODO: Decide what we want to do to users who try to bounce too many signals
            e.into()
        })
    }
}

impl<T: Config> Remuneration<T, BalanceOf<T>> for Pallet<T> {
    fn try_remunerate<Module: ModuleOperations<T, BalanceOf<T>>>(
        payee: &T::AccountId,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, DispatchError> {
        remuneration::try_remunerate::<T, Module>(payee, module)
    }

    fn try_remunerate_exact<Module: ModuleOperations<T, BalanceOf<T>>>(
        payee: &T::AccountId,
        amount: BalanceOf<T>,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, DispatchError> {
        remuneration::try_remunerate_exact::<T, Module>(payee, amount, module)
    }

    fn try_finalize(ledger_id: T::Hash, outcome: Outcome) -> DispatchResult {
        remuneration::try_finalize::<T>(ledger_id, outcome)
    }
}

impl<T: Config> ThreeVm<T, BalanceOf<T>> for Pallet<T> {
    fn peek_registry(
        id: &T::Hash,
    ) -> Result<
        RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
        DispatchError,
    > {
        let contract = T::ContractsRegistry::fetch_contract_by_id(*id)
            .map_err(|_| Error::<T>::ContractNotFound)?;
        Ok(contract)
    }

    fn from_registry<Module, ModuleGen>(
        id: &T::Hash,
        module_generator: ModuleGen,
    ) -> Result<Module, DispatchError>
    where
        Module: ModuleOperations<T, BalanceOf<T>>,
        ModuleGen: Fn(Vec<u8>) -> Module,
    {
        let registry_contract = T::ContractsRegistry::fetch_contract_by_id(*id)
            .map_err(|_| Error::<T>::ContractNotFound)?;
        let contract_len = registry_contract.bytes.len();
        let mut module = module_generator(registry_contract.bytes);
        module.set_author(registry_contract.author.clone());
        module.set_type(*registry_contract.meta.get_contract_type());
        Self::deposit_event(Event::<T>::ModuleInstantiated((
            *id,
            registry_contract.author.account,
            *registry_contract.meta.get_contract_type(),
            contract_len as u32,
        )));
        Ok(module)
    }

    fn instantiate_check(kind: &ContractType) -> Result<(), DispatchError> {
        ensure!(
            kind.can_instantiate(),
            <Error<T>>::CannotInstantiateContract
        );
        Ok(())
    }

    fn storage_check(kind: &ContractType) -> Result<(), DispatchError> {
        ensure!(kind.has_storage(), <Error<T>>::ContractCannotHaveStorage);
        Ok(())
    }

    fn volatile_check(kind: &ContractType) -> Result<(), DispatchError> {
        ensure!(
            kind.can_generate_side_effects(),
            <Error<T>>::ContractCannotGenerateSideEffects
        );
        Ok(())
    }

    fn remunerable_check(kind: &ContractType) -> Result<(), DispatchError> {
        ensure!(kind.can_remunerate(), <Error<T>>::ContractCannotRemunerate);
        Ok(())
    }

    fn try_persist_author(
        contract: &T::AccountId,
        author: Option<&AuthorInfo<T::AccountId, BalanceOf<T>>>,
    ) -> Result<(), DispatchError> {
        if let Some(author) = author {
            if !AuthorOf::<T>::contains_key(contract) {
                AuthorOf::<T>::insert(contract, author.account.clone());
                Self::deposit_event(Event::<T>::AuthorStored((
                    contract.clone(),
                    author.account.clone(),
                )))
            }
        }
        Ok(())
    }

    fn try_remove_author(contract: &T::AccountId) -> Result<(), DispatchError> {
        if AuthorOf::<T>::contains_key(contract) {
            AuthorOf::<T>::remove(contract);
            Self::deposit_event(Event::<T>::AuthorRemoved(contract.clone()))
        }

        Ok(())
    }
}
