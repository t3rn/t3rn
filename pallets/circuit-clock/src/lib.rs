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
    traits::{Currency, Get},
};

pub use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    common::RoundInfo,
    protocol::SideEffectProtocol,
    ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor,
};
use t3rn_primitives::{
    account_manager::AccountManager, executors::Executors, treasury::Treasury, EscrowTrait,
};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::{pallet_prelude::*, traits::ReservableCurrency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use t3rn_primitives::{
        circuit_clock::{CircuitClock, ClaimableArtifacts},
        transfers::EscrowedBalanceOf,
    };

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: ReservableCurrency<Self::AccountId>;

        type WeightInfo: weights::WeightInfo;

        /// Type providing some time handler
        type Time: frame_support::traits::Time;

        #[pallet::constant]
        type EscrowAccount: Get<Self::AccountId>;

        /// A type that manages escrow, and therefore balances
        type Escrowed: EscrowTrait<Self>;

        #[pallet::constant]
        type RoundDuration: Get<Self::BlockNumber>;

        type Treasury: Treasury<Self>;

        type Executors: Executors<
            Self,
            <<Self::Escrowed as EscrowTrait<Self>>::Currency as frame_support::traits::Currency<
                Self::AccountId,
            >>::Balance,
        >;

        /// A type that provides access to AccountManager
        type AccountManager: AccountManager<
            Self::AccountId,
            <<Self::Escrowed as EscrowTrait<Self>>::Currency as frame_support::traits::Currency<
                Self::AccountId,
            >>::Balance,
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
        Vec<ClaimableArtifacts<T::AccountId, EscrowedBalanceOf<T, <T as Config>::Escrowed>>>,
    >;

    #[pallet::storage]
    pub type TotalRewardCountPerRound<T: Config> = StorageMap<
        _,
        Blake2_128,
        RoundInfo<T::BlockNumber>,
        EscrowedBalanceOf<T, <T as Config>::Escrowed>,
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
            Ok(().into())
        }

        // #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        // pub fn claim(origin: OriginFor<T>, role: BeneficiaryRole) -> DispatchResult {
        //     let payee = ensure_signed(origin)?;
        //     // todo: check if who is a legit beneficiary with pallet-executors
        //
        //     let start_claim_from_round = LastClaim::<T>::get(payee.clone());
        //
        //     match role {
        //         BeneficiaryRole::Developer => {
        //             todo!()
        //         },
        //         BeneficiaryRole::Executor => {
        //             for settling_round in
        //                 start_claim_from_round.index..T::Treasury::current_round().index
        //             {
        //                 // ToDo: to safe reward per round add now inflation rate based on tokenomics:
        //                 // To further prevent drastically high payouts to Executors during times when network is stale,
        //                 // there is a threshold for rewards Executors  per cross-chain a single transaction.
        //                 // We put the constant threshold of a maximum of 5 times of the reward amount given by inflation.
        //                 // This is on top of the base reward the executor got from the original fee set be a user.
        //                 //
        //                 // The calculated reward per each cross-chain transaction can be approximated with the following formula:
        //                 // total reward per cross-chain tx = initial reward + MIN ( 5 * initial reward, inflation shares in a given time period)
        //         BeneficiaryRole::Staker => {},
        //         BeneficiaryRole::Collator => {
        //             todo!()
        //         },
        //     }
        //
        //     todo!()
        // }
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
                T::Executors::recalculate_executors_stakes();
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

    impl<T: Config> CircuitClock<T, EscrowedBalanceOf<T, T::Escrowed>> for Pallet<T> {
        fn current_round() -> RoundInfo<T::BlockNumber> {
            // todo: move current round from treasury to circui-clock
            T::Treasury::current_round()
        }
    }
}
