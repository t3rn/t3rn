//! <!-- markdown-link-check-disable -->
//! # Account Manager pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use crate::pallet::*;
use frame_support::{
    pallet_prelude::Weight,
    traits::{Currency, Get},
};
use sp_runtime::traits::Convert;
pub use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    common::RoundInfo,
    protocol::SideEffectProtocol,
    ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor,
};
use t3rn_primitives::{
    account_manager::{AccountManager, ExecutionRegistryItem, Reason, SfxSettlement},
    executors::Executors,
    transfers::EscrowedBalanceOf,
    treasury::Treasury,
    EscrowTrait,
};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod manager;
pub mod transaction;
pub mod weights;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use t3rn_primitives::{account_manager::ExecutionId, monetary::BeneficiaryRole};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        type Currency: ReservableCurrency<Self::AccountId>;

        /// Type providing some time handler
        type Time: frame_support::traits::Time;

        #[pallet::constant]
        type EscrowAccount: Get<Self::AccountId>;

        /// A type that manages escrow, and therefore balances
        type Escrowed: EscrowTrait<Self>;

        type Treasury: Treasury<Self>;

        type Executors: Executors<
            Self,
            <<Self::Escrowed as EscrowTrait<Self>>::Currency as frame_support::traits::Currency<
                Self::AccountId,
            >>::Balance,
        >;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn execution_registry)]
    pub type ExecutionRegistry<T: Config> = StorageMap<
        _,
        Blake2_128,
        ExecutionId,
        ExecutionRegistryItem<T::AccountId, <T::Currency as Currency<T::AccountId>>::Balance>,
    >;

    #[pallet::storage]
    pub type ExecutionNonce<T: Config> = StorageValue<_, ExecutionId, ValueQuery>;

    #[pallet::storage]
    pub type LastClaim<T: Config> =
        StorageMap<_, Identity, T::AccountId, RoundInfo<T::BlockNumber>>;

    #[pallet::storage]
    pub type SfxSettlementsPerRound<T: Config> = StorageDoubleMap<
        _,
        Blake2_128,
        RoundInfo<T::BlockNumber>,
        Identity,
        T::Hash, // sfx_id
        SfxSettlement<<T::Currency as Currency<T::AccountId>>::Balance, T::AccountId>,
    >;

    #[pallet::storage]
    pub type TotalSfxRewardCountPerRoundPerExecutor<T: Config> = StorageDoubleMap<
        _,
        Blake2_128,
        RoundInfo<T::BlockNumber>,
        Identity,
        T::AccountId, // executor id
        <T::Currency as Currency<T::AccountId>>::Balance,
    >;

    #[pallet::storage]
    pub type TotalSfxRewardCountPerRound<T: Config> = StorageMap<
        _,
        Blake2_128,
        RoundInfo<T::BlockNumber>,
        <T::Currency as Currency<T::AccountId>>::Balance,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().reads(2) + T::DbWeight::get().writes(1))]
        pub fn deposit(
            origin: OriginFor<T>,
            payee: T::AccountId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <Self as AccountManager<T::AccountId, BalanceOf<T>, T::Hash>>::deposit(
                &payee, &recipient, amount,
            )
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn finalize(
            origin: OriginFor<T>,
            execution_id: ExecutionId,
            reason: Option<Reason>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <Self as AccountManager<T::AccountId, BalanceOf<T>, T::Hash>>::finalize(
                execution_id,
                reason,
            )
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn claim(origin: OriginFor<T>, role: BeneficiaryRole) -> DispatchResult {
            let payee = ensure_signed(origin)?;
            // todo: check if who is a legit beneficiary with pallet-executors

            let start_claim_from_round = LastClaim::<T>::get(payee.clone());

            match role {
                BeneficiaryRole::Developer => {
                    todo!()
                },
                BeneficiaryRole::Executor => {
                    for settling_round in
                        start_claim_from_round.index..T::Treasury::current_round().index
                    {
                        let reward_per_round = TotalSfxRewardCountPerRoundPerExecutor::<T>::get(
                            settling_round,
                            payee.clone(),
                        );
                        let total_per_round = TotalSfxRewardCountPerRound::get(settling_round);
                        // ToDo: to safe reward per round add now inflation rate based on tokenomics:
                        // To further prevent drastically high payouts to Executors during times when network is stale,
                        // there is a threshold for rewards Executors  per cross-chain a single transaction.
                        // We put the constant threshold of a maximum of 5 times of the reward amount given by inflation.
                        // This is on top of the base reward the executor got from the original fee set be a user.
                        //
                        // The calculated reward per each cross-chain transaction can be approximated with the following formula:
                        // total reward per cross-chain tx = initial reward + MIN ( 5 * initial reward, inflation shares in a given time period)

                        // let inflation_rewards = T::Treasury::calc_executor_inflation_rewards(settling_round, who, total_per_round)?;
                        let inflation_rewards = 0;
                        let mut claimable = reward_per_round + inflation_rewards;
                        claimable *= (T::Executors::collateral_bond(payee)
                            / T::Executors::collateral_bond(payee)
                            - T::Executors::total_nominated_stake(payee));
                        T::Currency::transfer(
                            &T::EscrowAccount::get(),
                            payee,
                            claimable,
                            ExistenceRequirement::KeepAlive,
                        )?;
                    }
                },
                BeneficiaryRole::Staker => {},
                BeneficiaryRole::Collator => {
                    todo!()
                },
            }

            todo!()
        }
    }

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {
            // Perform necessary data/state clean up here.
        }

        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            // TODO: we may want to retry failed transactions here, ensuring a max weight and max retry list
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: T::BlockNumber) {
            // We don't do anything here.
            // but we could dispatch extrinsic (transaction/unsigned/inherent) using
            // sp_io::submit_extrinsic.
            // To see example on offchain worker, please refer to example-offchain-worker pallet
            // accompanied in this repository.
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        DepositReceived {
            execution_id: ExecutionId,
            payee: T::AccountId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        ExecutionFinalized {
            execution_id: ExecutionId,
        },
        Issued {
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ExecutionNotRegistered,
        ExecutionAlreadyRegistered,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        phantom: PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
            }
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}

impl<T: Config> EscrowTrait<T> for Pallet<T> {
    type Currency = T::Currency;
    type Time = T::Time;
}

impl<T: Config> Convert<Weight, BalanceOf<T>> for Pallet<T>
where
    <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance:
        From<u64>,
{
    fn convert(w: Weight) -> EscrowedBalanceOf<T, Self> {
        EscrowedBalanceOf::<T, Self>::from(w)
    }
}
