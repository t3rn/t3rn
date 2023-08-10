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
    traits::{fungibles::Inspect, Currency, Get},
};
use sp_runtime::traits::Convert;

use t3rn_primitives::{
    account_manager::{AccountManager, Outcome},
    claimable::{BenefitSource, CircuitRole},
    clock::Clock,
    common::RoundInfo,
    executors::Executors,
    reexport_currency_types,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod manager;
pub mod monetary;
pub mod transaction;
pub mod weights;

reexport_currency_types!();

pub type AssetsBalanceOf<T> =
    <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    use codec::FullCodec;
    use sp_std::fmt::Debug;
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{tokens::fungibles::Unbalanced, Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    use t3rn_primitives::account_manager::{ExecutionId, RequestCharge, Settlement};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        type Currency: ReservableCurrency<Self::AccountId>;

        type Assets: Unbalanced<Self::AccountId>;

        type Clock: Clock<Self>;

        type Executors: Executors<Self, BalanceOf<Self>>;

        /// Type providing some time handler
        type Time: frame_support::traits::Time;

        #[pallet::constant]
        type EscrowAccount: Get<Self::AccountId>;

        type AssetBalanceOf: Convert<BalanceOf<Self>, AssetsBalanceOf<Self>>;

        type AssetId: FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default + Eq + TypeInfo;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    pub type ContractsRegistryExecutionNonce<T: Config> = StorageValue<_, ExecutionId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_charges_per_round)]
    pub type PendingCharges<T: Config> = StorageMap<
        _,
        Identity,
        T::Hash, // sfx_id
        RequestCharge<
            T::AccountId,
            <T::Currency as Currency<T::AccountId>>::Balance,
            <T::Assets as Inspect<T::AccountId>>::AssetId,
        >,
    >;

    #[pallet::storage]
    #[pallet::getter(fn settlements_per_round)]
    pub type SettlementsPerRound<T: Config> = StorageDoubleMap<
        _,
        Blake2_128,
        RoundInfo<BlockNumberFor<T>>,
        Identity,
        T::Hash, // sfx_id
        Settlement<
            T::AccountId,
            <T::Currency as Currency<T::AccountId>>::Balance,
            <T::Assets as Inspect<T::AccountId>>::AssetId,
        >,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::DbWeight::get().reads(2) + T::DbWeight::get().writes(1))]
        pub fn deposit(
            origin: OriginFor<T>,
            charge_id: T::Hash,
            payee: T::AccountId,
            charge_fee: BalanceOf<T>,
            offered_reward: BalanceOf<T>,
            source: BenefitSource,
            role: CircuitRole,
            recipient: Option<T::AccountId>,
            maybe_asset_id: Option<<T::Assets as Inspect<T::AccountId>>::AssetId>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <Self as AccountManager<
                T::AccountId,
                BalanceOf<T>,
                T::Hash,
                frame_system::pallet_prelude::BlockNumberFor<T>,
                <T::Assets as Inspect<T::AccountId>>::AssetId,
            >>::deposit(
                charge_id,
                RequestCharge {
                    payee,
                    offered_reward,
                    charge_fee,
                    source,
                    role,
                    recipient,
                    maybe_asset_id,
                },
            )
            .map(|_| ())
        }

        #[pallet::weight(T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn finalize(
            origin: OriginFor<T>,
            charge_id: T::Hash,
            outcome: Outcome,
            maybe_recipient: Option<T::AccountId>,
            maybe_actual_fees: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <Self as AccountManager<
                T::AccountId,
                BalanceOf<T>,
                T::Hash,
                frame_system::pallet_prelude::BlockNumberFor<T>,
                <T::Assets as Inspect<T::AccountId>>::AssetId,
            >>::finalize(charge_id, outcome, maybe_recipient, maybe_actual_fees)
        }
    }

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) {
            // Perform necessary data/state clean up here.
        }

        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
            // TODO: we may want to retry failed transactions here, ensuring a max weight and max retry list
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            Weight::zero()
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: frame_system::pallet_prelude::BlockNumberFor<T>) {
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
        ContractsRegistryExecutionFinalized {
            execution_id: ExecutionId,
        },
        Issued {
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        DepositReceived {
            charge_id: T::Hash,
            payee: T::AccountId,
            recipient: Option<T::AccountId>,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PendingChargeNotFoundAtCommit,
        PendingChargeNotFoundAtRefund,
        ExecutionNotRegistered,
        ExecutionAlreadyRegistered,
        SkippingEmptyCharges,
        NoChargeOfGivenIdRegistered,
        ChargeAlreadyRegistered,
        ChargeOrSettlementCalculationOverflow,
        ChargeOrSettlementActualFeesOutgrowReserved,
        DecodingExecutionIDFailed,
        TransferDepositFailedOldChargeNotFound,
        TransferDepositFailedToReleasePreviousCharge,
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
    impl<T: Config> BuildGenesisConfig<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}

impl<T: Config> Convert<Weight, BalanceOf<T>> for Pallet<T>
where
    <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance:
        From<u64>,
{
    fn convert(w: Weight) -> BalanceOf<T> {
        BalanceOf::<T>::from(w.ref_time() + w.proof_size())
    }
}
