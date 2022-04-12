//! <!-- markdown-link-check-disable -->
//! # Account Manager pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use codec::Encode;
use frame_support::traits::{Currency, ExistenceRequirement, Get, ReservableCurrency};
use sp_runtime::traits::{CheckedDiv, CheckedMul, Hash, Zero};
use sp_std::{collections::btree_map::BTreeMap, prelude::*};
use t3rn_primitives::EscrowTrait;
pub use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    protocol::SideEffectProtocol,
    ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor,
};
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use crate::pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
pub mod weights;

use crate::types::{AccountManager, ExecutionRegistryItem, Reason};
use weights::WeightInfo;

pub type ExecutionId = u64;
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use crate::{types::ExecutionRegistryItem, WeightInfo};
    use frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, Mutate},
            Currency, ReservableCurrency, Time,
        },
    };
    use frame_system::pallet_prelude::*;
    use sp_std::convert::TryInto;
    use t3rn_primitives::{
        side_effect::interface::SideEffectInterface,
        xdns::{AllowedSideEffect, Xdns, XdnsRecord},
        ChainId, EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor,
    };

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
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<(T)>);

    #[pallet::storage]
    #[pallet::getter(fn execution_registry)]
    pub type ExecutionRegistry<T: Config> = StorageMap<
        _,
        Blake2_128,
        ExecutionId,
        ExecutionRegistryItem<T::AccountId, <T::Currency as Currency<T::AccountId>>::Balance>,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

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
        Example,
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        Example,
    }

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        phantom: PhantomData<(T)>,
    }

    /// The default value for the genesis config type.
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

// TODO: remove unwraps from this
impl<T: Config> AccountManager<T::AccountId, BalanceOf<T>> for Pallet<T> {
    fn deposit(
        &self,
        execution_id: ExecutionId,
        payee: T::AccountId,
        recipient: T::AccountId,
        amount: BalanceOf<T>,
    ) {
        /// Reserve the funds from the payee account
        T::Currency::transfer(
            &payee,
            &T::EscrowAccount::get(),
            amount,
            ExistenceRequirement::KeepAlive,
        )
        .unwrap();

        // TODO: Check if registry already has an entry for this execution id

        ExecutionRegistry::<T>::insert(
            execution_id,
            ExecutionRegistryItem::new(payee, recipient, amount),
        );
    }

    fn finalize(&self, execution_id: ExecutionId, reason: Option<Reason>) {
        let item = Pallet::<T>::execution_registry(execution_id).unwrap();
        self.split(item, reason);
    }

    fn issue(&self, recipient: &T::AccountId, amount: BalanceOf<T>) {
        if !amount.is_zero() {
            T::Currency::transfer(
                &T::EscrowAccount::get(),
                recipient,
                amount,
                ExistenceRequirement::KeepAlive,
            )
            .unwrap();
        }
    }

    fn split(
        &self,
        item: ExecutionRegistryItem<T::AccountId, BalanceOf<T>>,
        reason: Option<Reason>,
    ) {
        // Simple rules for splitting, for now
        let (payee_split, recipient_split): (u32, u32) = match reason {
            None => (0, 100),
            Some(Reason::ContractReverted) => (90, 10),
            Some(Reason::UnexpectedFailure) => (50, 50),
        };
        // TODO: check babies first maths
        self.issue(
            item.payee(),
            (item.balance().checked_div(&100_u32.into()))
                .unwrap()
                .checked_mul(&payee_split.into())
                .unwrap(),
        );
        self.issue(
            item.recipient(),
            (item.balance().checked_div(&100_u32.into()))
                .unwrap()
                .checked_mul(&recipient_split.into())
                .unwrap(),
        );
    }
}
