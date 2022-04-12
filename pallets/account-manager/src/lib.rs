//! <!-- markdown-link-check-disable -->
//! # X-DNS Pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
pub use crate::types::{EventSignature, SideEffectId, SideEffectName};
use codec::Encode;
use sp_runtime::traits::Hash;
use sp_std::{collections::btree_map::BTreeMap, prelude::*};
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
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;

        /// A type that provides inspection and mutation to some fungible assets
        type Balances: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {
            // Perform necessary data/state clean up here.
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
    impl<T: Config> Pallet<T> {}

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

    #[pallet::storage]
    pub type StandardSideEffects<T: Config> = StorageMap<_, Identity, [u8; 4], SideEffectInterface>;

    #[pallet::storage]
    #[pallet::getter(fn side_effect_registry)]
    pub type CustomSideEffects<T> = StorageMap<_, Identity, SideEffectId<T>, SideEffectInterface>;

    /// The pre-validated composable xdns_records on-chain registry.
    #[pallet::storage]
    #[pallet::getter(fn xdns_registry)]
    pub type XDNSRegistry<T: Config> =
        StorageMap<_, Identity, [u8; 4], XdnsRecord<T::AccountId>, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {}

    /// The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {}
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}
