#![feature(associated_type_defaults)]
//! <!-- markdown-link-check-disable -->
//! # Account Manager pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use crate::pallet::*;
use frame_support::{pallet_prelude::Weight, traits::Get};

pub use t3rn_primitives::{
    account_manager::AccountManager, claimable::ClaimableArtifacts, clock::Clock,
    common::RoundInfo, executors::Executors, gateway::GatewayABIConfig, ChainId, EscrowTrait,
    GatewayGenesisConfig, GatewayType, GatewayVendor,
};

use pallet_account_manager::BalanceOf;

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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use t3rn_primitives::clock::OnHookQueues;

    const FIVE: u64 = 5;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_account_manager::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type RoundDuration: Get<Self::BlockNumber>;

        type Executors: Executors<Self, BalanceOf<Self>>;

        /// A type that provides access to AccountManager
        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            Self::BlockNumber,
            u32,
        >;

        /// Description of on_initialize queues and their max. consumption of % of total on_init weight.
        /// The first element of the tuple is the queue name, the second is the max. % of total on_init weight.}
        type OnInitializeQueues: OnHookQueues<Self>;

        type OnFinalizeQueues: OnHookQueues<Self>;
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

    #[pallet::storage]
    #[pallet::getter(fn current_round)]
    /// Information on the current round.
    pub type CurrentRound<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub fn check_bump_round(n: T::BlockNumber) -> Weight {
            let past_round = <CurrentRound<T>>::get();
            let term = T::RoundDuration::get();
            let new_round = RoundInfo {
                index: past_round.index.saturating_add(1),
                head: n,
                term,
            };
            log::debug!(
                "check_bump_round: past_round: {:?}, new_round: {:?}",
                past_round,
                new_round
            );
            if past_round.should_update(n) {
                <CurrentRound<T>>::put(new_round);
                Self::deposit_event(Event::NewRound {
                    index: new_round.index,
                    head: new_round.head,
                    term: new_round.term,
                });
                T::DbWeight::get().reads_writes(2, 1)
            } else {
                T::DbWeight::get().reads(2)
            }
        }
    }

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(n: T::BlockNumber) {
            let max_on_finalize_weight = T::BlockWeights::get().max_block.saturating_div(FIVE);
            log::debug!(
                "Clock::on_finalize process hooks with max_on_finalize_weight: {:?}",
                max_on_finalize_weight
            );
            T::OnFinalizeQueues::process(n, max_on_finalize_weight);
        }

        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let max_on_initialize_weight = T::BlockWeights::get().max_block.saturating_div(FIVE);
            log::debug!(
                "Clock::on_initialize process hooks with max_on_initialize_weight: {:?} and block number: {:?}",
                max_on_initialize_weight,
                n
            );
            T::OnInitializeQueues::process(n, max_on_initialize_weight)
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
        NewRound {
            index: u32,
            head: T::BlockNumber,
            term: T::BlockNumber,
        },
    }

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
            Self::current_round()
        }

        fn round_duration() -> T::BlockNumber {
            T::RoundDuration::get()
        }
    }
}
