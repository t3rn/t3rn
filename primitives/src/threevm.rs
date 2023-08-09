use crate::{
    account_manager::Outcome,
    circuit::LocalStateExecutionView,
    contract_metadata::ContractType,
    contracts_registry::{AuthorInfo, RegistryContract},
    portal::{PortalExecution, PrecompileArgs as PortalPrecompileArgs},
    SpeedMode,
};
use codec::{Decode, Encode};
use frame_system::{pallet_prelude::BlockNumberFor, Config as ConfigSystem};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::{fmt::Debug, result::Result, vec::Vec};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::SideEffects,
};

// Precompile pointers baked into the binary.
// Genesis exists only to map hashes to pointers.
pub const GET_STATE: u8 = 55;
pub const SUBMIT: u8 = 56;
pub const POST_SIGNAL: u8 = 57;
pub const PORTAL: u8 = 70;

#[derive(Encode, Decode)]
pub struct GetState<T: ConfigSystem> {
    pub xtx_id: Option<T::Hash>,
}

// FIXME: none of these work at the moment due to large updates to SFX ABI.
#[derive(Encode, Decode)]
pub enum PrecompileArgs<T, Balance>
where
    T: ConfigSystem,
    Balance: Encode + Decode,
{
    GetState(T::RuntimeOrigin, GetState<T>),
    SubmitSideEffects(
        T::RuntimeOrigin,
        SideEffects<T::AccountId, Balance, T::Hash>,
        SpeedMode,
    ),
    Signal(T::RuntimeOrigin, ExecutionSignal<T::Hash>),
    Portal(PortalPrecompileArgs),
}

/// The happy return type of an invocation
pub enum PrecompileInvocation<T: ConfigSystem, Balance> {
    GetState(LocalStateExecutionView<T, Balance>),
    Submit(LocalStateExecutionView<T, Balance>),
    Signal,
    Portal(PortalExecution<T>),
}

impl<T: ConfigSystem, Balance> PrecompileInvocation<T, Balance> {
    pub fn get_state(&self) -> Option<&LocalStateExecutionView<T, Balance>> {
        match self {
            PrecompileInvocation::GetState(state) => Some(state),
            _ => None,
        }
    }

    pub fn get_submit(&self) -> Option<&LocalStateExecutionView<T, Balance>> {
        match self {
            PrecompileInvocation::Submit(state) => Some(state),
            _ => None,
        }
    }
}

pub trait Precompile<T, Balance>
where
    T: ConfigSystem,
    Balance: Encode + Decode,
{
    /// Looks up a precompile function pointer
    fn lookup(dest: &T::Hash) -> Option<u8>;

    /// Invoke a precompile, providing raw bytes and a pointer
    fn invoke_raw(precompile: &u8, args: &[u8], output: &mut Vec<u8>);

    /// Invoke a precompile
    fn invoke(
        args: PrecompileArgs<T, Balance>,
    ) -> Result<PrecompileInvocation<T, Balance>, DispatchError>;
}

pub trait LocalStateAccess<T, Balance>
where
    T: ConfigSystem,
{
    fn load_local_state(
        origin: &T::RuntimeOrigin,
        xtx_id: Option<&T::Hash>,
    ) -> Result<LocalStateExecutionView<T, Balance>, DispatchError>;
}

pub struct Remunerated<Hash> {
    pub remuneration_id: Option<Hash>,
}

impl<Hash> Default for Remunerated<Hash> {
    fn default() -> Self {
        Remunerated {
            remuneration_id: None,
        }
    }
}

impl<Hash> Remunerated<Hash> {
    pub fn new(id: Option<Hash>) -> Self {
        Remunerated {
            remuneration_id: id,
        }
    }
}

pub trait Remuneration<T: ConfigSystem, Balance> {
    /// Try to remunerate the fees from the given module
    fn try_remunerate<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError>;

    /// Try to remunerate the fees from the given module with a custom balance
    fn try_remunerate_exact<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        amount: Balance,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError>;

    /// Try to finalize a ledger item with an reason
    fn try_finalize(ledger_id: T::Hash, outcome: Outcome) -> DispatchResult;
}
pub enum Characteristic {
    Storage,
    Instantiate,
    Remuneration,
    Volatile,
}

/// Passthrough to validator
pub trait CharacteristicValidator {
    fn validate(characteristic: &Characteristic) -> Result<(), ()>; // TODO: handle error
}

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
pub enum SignalOpcode {
    Initiated,
    Bounced,
}

pub trait ThreeVm<T, Balance>:
    Precompile<T, Balance>
    + Signaller<T::Hash, Result = Result<SignalOpcode, DispatchError>>
    + Remuneration<T, Balance>
where
    T: ConfigSystem,
    Balance: Encode + Decode,
{
    fn peek_registry(
        id: &T::Hash,
    ) -> Result<RegistryContract<T::Hash, T::AccountId, Balance, BlockNumberFor<T>>, DispatchError>;

    /// Allows creating a `Module` from a binary blob from the contracts registry
    fn from_registry<Module, ModuleGen>(
        id: &T::Hash,
        module_generator: ModuleGen,
    ) -> Result<Module, DispatchError>
    where
        Module: ModuleOperations<T, Balance>,
        ModuleGen: Fn(Vec<u8>) -> Module;

    fn instantiate_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn storage_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn volatile_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn remunerable_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn try_persist_author(
        contract: &T::AccountId,
        author: Option<&AuthorInfo<T::AccountId, Balance>>,
    ) -> Result<(), DispatchError>;

    fn try_remove_author(contract: &T::AccountId) -> Result<(), DispatchError>;
}

pub struct NoopThreeVm;

impl<T, Balance> LocalStateAccess<T, Balance> for NoopThreeVm
where
    T: ConfigSystem,
{
    fn load_local_state(
        _origin: &T::RuntimeOrigin,
        _xtx_id: Option<&T::Hash>,
    ) -> Result<LocalStateExecutionView<T, Balance>, DispatchError> {
        Err("Local State Not implemented").map_err(|e| e.into())
    }
}

impl<T: ConfigSystem, Balance: Encode + Decode> Remuneration<T, Balance> for NoopThreeVm {
    fn try_remunerate<Module: ModuleOperations<T, Balance>>(
        _payee: &T::AccountId,
        _module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError> {
        Ok(Remunerated {
            remuneration_id: None,
        })
    }

    fn try_remunerate_exact<Module: ModuleOperations<T, Balance>>(
        _payee: &T::AccountId,
        _amount: Balance,
        _module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError> {
        Ok(Remunerated {
            remuneration_id: None,
        })
    }

    fn try_finalize(_ledger_id: T::Hash, _outcome: Outcome) -> DispatchResult {
        Ok(())
    }
}

impl<Hash: Encode + Decode + Debug + Clone> Signaller<Hash> for NoopThreeVm {
    type Result = Result<SignalOpcode, DispatchError>;

    fn signal(_signal: &ExecutionSignal<Hash>) -> Self::Result {
        Err("Signalling is not enabled".into())
    }
}

impl<T, Balance> Precompile<T, Balance> for NoopThreeVm
where
    T: ConfigSystem,
    Balance: Encode + Decode,
{
    fn lookup(_dest: &T::Hash) -> Option<u8> {
        None
    }

    fn invoke_raw(_precompile: &u8, _args: &[u8], _output: &mut Vec<u8>) {}

    fn invoke(
        _args: PrecompileArgs<T, Balance>,
    ) -> Result<PrecompileInvocation<T, Balance>, DispatchError> {
        Err("Precompile Invocation Not implemented").map_err(|e| e.into())
    }
}

// Default impl
impl<T: ConfigSystem, Balance: Encode + Decode> ThreeVm<T, Balance> for NoopThreeVm {
    fn peek_registry(
        _id: &<T as ConfigSystem>::Hash,
    ) -> Result<
        RegistryContract<
            <T as ConfigSystem>::Hash,
            <T as ConfigSystem>::AccountId,
            Balance,
            BlockNumberFor<T>,
        >,
        DispatchError,
    > {
        Err("Registry Peek Not implemented").map_err(|e| e.into())
    }

    fn from_registry<Module, ModuleGen>(
        _id: &<T as ConfigSystem>::Hash,
        _module_generator: ModuleGen,
    ) -> Result<Module, DispatchError>
    where
        Module: ModuleOperations<T, Balance>,
        ModuleGen: Fn(Vec<u8>) -> Module,
    {
        Err("From Registry Not implemented").map_err(|e| e.into())
    }

    fn instantiate_check(_kind: &ContractType) -> Result<(), DispatchError> {
        Ok(())
    }

    fn storage_check(_kind: &ContractType) -> Result<(), DispatchError> {
        Ok(())
    }

    fn volatile_check(_kind: &ContractType) -> Result<(), DispatchError> {
        Ok(())
    }

    fn remunerable_check(_kind: &ContractType) -> Result<(), DispatchError> {
        Ok(())
    }

    fn try_persist_author(
        _contract: &<T as ConfigSystem>::AccountId,
        _author: Option<&AuthorInfo<<T as ConfigSystem>::AccountId, Balance>>,
    ) -> Result<(), DispatchError> {
        Ok(())
    }

    fn try_remove_author(_conztract: &<T as ConfigSystem>::AccountId) -> Result<(), DispatchError> {
        Ok(())
    }
}

pub trait ModuleOperations<T: ConfigSystem, Balance> {
    fn get_bytecode(&self) -> &Vec<u8>;
    fn get_author(&self) -> Option<&AuthorInfo<T::AccountId, Balance>>;
    fn set_author(&mut self, author: AuthorInfo<T::AccountId, Balance>);
    fn get_type(&self) -> &ContractType;
    fn set_type(&mut self, kind: ContractType);
}
