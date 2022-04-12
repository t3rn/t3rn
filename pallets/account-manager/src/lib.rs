//! <!-- markdown-link-check-disable -->
//! # Account Manager pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use codec::Encode;
use sp_runtime::traits::Hash;
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

pub mod currency;
pub mod types;
pub mod weights;

use weights::WeightInfo;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use crate::WeightInfo;
    use frame_support::{
        pallet_prelude::*,
        traits::{
            fungible::{Inspect, Mutate},
            Time,
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
    pub trait Config<I: 'static = ()>: frame_system::Config + pallet_balances::Config {
        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        /// Type providing some time handler
        type Time: frame_support::traits::Time;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
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

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        Example,
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T, I = ()> {
        Example,
    }

    // #[pallet::storage]
    // pub type StandardSideEffects<T: Config<I>, I: 'static> = StorageMap<_, Identity, [u8; 4], SideEffectInterface>;

    // #[pallet::storage]
    // #[pallet::getter(fn side_effect_registry)]
    // pub type CustomSideEffects<T> = StorageMap<_, Identity, SideEffectId<T>, SideEffectInterface>;

    // /// The pre-validated composable xdns_records on-chain registry.
    // #[pallet::storage]
    // #[pallet::getter(fn xdns_registry)]
    // pub type XDNSRegistry<T: Config<I>, I: 'static> =
    //     StorageMap<_, Identity, [u8; 4], XdnsRecord<T::AccountId>, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
        phantom: PhantomData<(T, I)>,
    }

    /// The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
            }
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
        fn build(&self) {}
    }
}

impl<T: Config + pallet_balances::Config> EscrowTrait<T> for Pallet<T> {
    type Currency = Self;
    type Time = T::Time;
}
