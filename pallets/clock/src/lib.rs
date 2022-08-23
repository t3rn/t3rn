#![feature(associated_type_defaults)]
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
    traits::{Get},
};

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    account_manager::AccountManager,
    claimable::ClaimableArtifacts,
    clock::Clock,
    common::RoundInfo,
    executors::Executors,
    protocol::SideEffectProtocol,
    transfers::EscrowedBalanceOf,
    treasury::Treasury,
    ChainId, EscrowTrait, GatewayGenesisConfig, GatewayType, GatewayVendor,
};

use pallet_account_manager::BalanceOf;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::{pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_account_manager::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type RoundDuration: Get<Self::BlockNumber>;

        type Treasury: Treasury<Self>;

        type Executors: Executors<Self, BalanceOf<Self>>;

        /// A type that provides access to AccountManager
        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            Self::BlockNumber,
        >;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    pub type LastClaims<T: Config> =
        StorageMap<_, Identity, T::AccountId, RoundInfo<T::BlockNumber>>;

    #[pallet::storage]
    pub type ClaimableArtifactsPerRound<T: Config> = StorageMap<
        _,
        Identity,
        RoundInfo<T::BlockNumber>,
        Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>,
    >;

    impl<T: Config> Pallet<T> {
        fn calculate_claimable_for_round(n: T::BlockNumber) -> DispatchResult {
            // fixme: move current_round from treasury to circuit-clock
            let r = T::Treasury::current_round();
            let mut claimable_artifacts = vec![];
            claimable_artifacts.extend(T::AccountManager::on_collect_claimable(n, r)?);
            // claimable_artifacts.push(T::Treasury::on_collect_claimable(n, r)?);
            // claimable_artifacts.push(T::Contracts::on_collect_claimable(n, r)?);
            // claimable_artifacts.push(T::Ambassadors::on_collect_claimable(n, r)?);
            // claimable_artifacts.push(T::LiquidityPools::on_collect_claimable(n, r)?);

            ClaimableArtifactsPerRound::<T>::insert(r, claimable_artifacts.clone());
            // todo: aggregated claimable_artifacts to TotalClaimablePerRound
            Ok(())
        }
    }

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(n: T::BlockNumber) {
            // Perform necessary data/state clean up here.

            if n % T::RoundDuration::get() == T::BlockNumber::zero() {
                Self::calculate_claimable_for_round(n);
                // After the rewards has been recalculate it's safe to shuffle the executors orded and stakes
                <T as Config>::Executors::recalculate_executors_stakes();
            }
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
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

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

    impl<T: Config> Clock<T> for Pallet<T> {
        fn current_round() -> RoundInfo<T::BlockNumber> {
            // todo: move current round from treasury to circui-clock
            T::Treasury::current_round()
        }
    }
}
