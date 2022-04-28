#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod inflation;
pub mod weights;

#[pallet]
pub mod pallet {
    use crate::{
        inflation::{
            CandidateRole, InflationInfo, Range, RewardsAllocationConfig, RoundIndex, RoundInfo,
        },
        weights::WeightInfo,
    };
    use codec::Codec;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    use sp_runtime::{
        traits::{AtLeast32BitUnsigned, CheckedDiv, CheckedMul, Saturating, Zero},
        Perbill,
    };
    use sp_std::{fmt::Debug, ops::Mul};

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + MaxEncodedLen
            + TypeInfo;

        #[pallet::constant]
        type DefaultBlocksPerRound: Get<u32>;

        #[pallet::constant]
        type TokenCirculationAtGenesis: Get<u32>;

        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn inflation_config)]
    // The pallet's inflation config per year
    pub type InflationConfig<T: Config> = StorageValue<_, InflationInfo, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_round)]
    // Information on the current epoch
    pub type CurrentRound<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn available_to_mint)]
    // Remaining tokens to be minted
    pub type AvailableTokensToBeMinted<T: Config> = StorageValue<_, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn candidates)]
    pub type CandidatesForRewards<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        CandidateRole,
        BalanceOf<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn rewards_per_round)]
    pub type RewardsPerCandidatePerRound<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        RoundIndex,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintedTokensForRound(T::AccountId, BalanceOf<T>),
        MintedTokensExactly(T::AccountId, BalanceOf<T>),
        MintedTokensForCandidate(T::AccountId, BalanceOf<T>),
        InflationSet {
            annual_min: Perbill,
            annual_ideal: Perbill,
            annual_max: Perbill,
            round_min: Perbill,
            round_ideal: Perbill,
            round_max: Perbill,
        },
        RewardsConfigSet {
            dev_reward: Perbill,
            exec_reward: Perbill,
        },
        RoundStarted {
            starting_block: T::BlockNumber,
            round: u32,
        },
        ClaimedRewards(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInflationSchedule,
        InvalidInflationRewardsConfig,
        MintingFailed,
        NotEnoughFunds,
        NotCandidate, // when someone tries to claim rewards when not being a candidate
        MisalignedCandidates, // when theres an error when calculating candidate rewards
        NoWritingSameValue, // when trying to update the inflation schedule
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            // check if round finished in current block
            // if so, update storage reward objects and create a new empty one
        }

        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut round = <CurrentRound<T>>::get();
            let _inflation_info = <InflationConfig<T>>::get();

            if round.should_update(n) {
                // mutate round
                round.update(n);

                // update round in storage
                <CurrentRound<T>>::put(round);

                // calculate amount to be rewarded per candidate
                // inflation_info.per_round.ideal
                // let funds = T::Currency::issue(round)
                // <Pallet<T>>::mint_for_round();
                // let candidates = <CandidatesForRewards<T>>::get();
                // candidates.into_iter().for_each(|candidate| {
                //     T::Currency::deposit_into_existing(candidate, amount)
                //         .expect("Should deposit balance to account");
                // });

                // emit event
                Self::deposit_event(Event::RoundStarted {
                    round: round.current,
                    starting_block: round.first_block,
                });
            }

            T::WeightInfo::update_round_on_initialize()
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub candidates: Vec<T::AccountId>,
        pub annual_inflation: Range<Perbill>,
        pub allocation_percentage: RewardsAllocationConfig,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                candidates: Default::default(),
                annual_inflation: Range {
                    min: Perbill::from_percent(1),   // TODO placeholder
                    ideal: Perbill::from_percent(2), // TODO placeholder
                    max: Perbill::from_percent(3),   // TODO placeholder
                },
                allocation_percentage: RewardsAllocationConfig {
                    executor: Perbill::from_percent(40),
                    developer: Perbill::from_percent(60),
                },
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // set first round
            let round: RoundInfo<T::BlockNumber> = RoundInfo::new(
                1_u32,
                T::BlockNumber::zero(),
                T::DefaultBlocksPerRound::get(),
            );
            <CurrentRound<T>>::put(round);
            <Pallet<T>>::deposit_event(Event::RoundStarted {
                round: 1_u32,
                starting_block: T::BlockNumber::zero(),
            })
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn mint_for_round(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            // mint can only be called from a root account
            ensure_root(origin)?;

            let treasury = T::TreasuryAccount::get();

            let round = <CurrentRound<T>>::get();

            let inflation_info = <InflationConfig<T>>::get();

            let candidates = <CandidatesForRewards<T>>::iter_keys();

            let count_devs = candidates
                .filter(|c| c.1 == CandidateRole::Developer)
                .count();
            let count_execs = candidates.count() - count_devs;

            ensure!(
                count_devs + count_execs == candidates.count(),
                Error::<T>::MisalignedCandidates
            );

            let per_developer = inflation_info
                .allocation_percentage
                .developer
                .saturating_mul(amount)
                .checked_div(&count_devs)?;
            let per_executor = inflation_info
                .allocation_percentage
                .executor
                .saturating_mul(amount)
                .checked_div(&count_execs)?;

            // for each candidate in the round, calculate their reward
            for (candidate, role) in candidates {
                let issued = match role {
                    CandidateRole::Developer => T::Currency::issue(amount * per_developer.clone()),
                    CandidateRole::Executor => T::Currency::issue(amount * per_executor.clone()),
                };

                <RewardsPerCandidatePerRound<T>>::insert(candidate.clone(), round.current, issued);

                Self::deposit_event(Event::MintedTokensForCandidate(candidate, &issued));
            }

            // Emit an event.
            Self::deposit_event(Event::MintedTokensForRound(treasury, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Pallet<T>>::ensure_candidate(&who)?;

            // accumulate rewards
            let total_rewards = <RewardsPerCandidatePerRound<T>>::iter_prefix(&who)
                .drain()
                .map(|key2_value| key2_value.1)
                .fold(BalanceOf::<T>::zero(), |acc, item| acc.saturating_add(item));

            // allocate to candidate
            T::Currency::deposit_into_existing(&who, BalanceOf::<T>::from(total_rewards))
                .expect("Should deposit balance to account");

            // emit event
            Self::deposit_event(Event::ClaimedRewards(who, total_rewards));

            Ok(().into())
        }

        /// Sets the annual inflation rate to derive per-round inflation
        #[pallet::weight(10_000)]
        pub fn set_inflation(
            origin: OriginFor<T>,
            annual_inflation_schedule: Range<Perbill>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                annual_inflation_schedule.is_valid(),
                Error::<T>::InvalidInflationSchedule
            );
            let mut config = <InflationConfig<T>>::get();
            ensure!(
                config.annual != annual_inflation_schedule,
                Error::<T>::NoWritingSameValue
            );
            config.annual = annual_inflation_schedule;
            config.set_round_from_annual::<T>(annual_inflation_schedule);
            Self::deposit_event(Event::InflationSet {
                annual_min: config.annual.min,
                annual_ideal: config.annual.ideal,
                annual_max: config.annual.max,
                round_min: config.per_round.min,
                round_ideal: config.per_round.ideal,
                round_max: config.per_round.max,
            });
            <InflationConfig<T>>::put(config);
            Ok(().into())
        }

        /// Sets the reward percentage to be allocated between developers and executors
        #[pallet::weight(10_000)]
        pub fn set_rewards_config(
            origin: OriginFor<T>,
            rewards_config: RewardsAllocationConfig,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                rewards_config.validate(),
                Error::<T>::InvalidInflationRewardsConfig
            );
            let mut config = <InflationConfig<T>>::get();
            ensure!(
                config.allocation_percentage != rewards_config,
                Error::<T>::NoWritingSameValue
            );
            config.allocation_percentage = rewards_config.clone();
            Self::deposit_event(Event::RewardsConfigSet {
                dev_reward: rewards_config.developer,
                exec_reward: rewards_config.executor,
            });
            <InflationConfig<T>>::put(config);
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Helper function to check if the origin belongs to the candidate list
        pub fn ensure_candidate(who: &T::AccountId) -> Result<(), DispatchError> {
            ensure!(
                <CandidatesForRewards<T>>::iter_prefix(who).count() == 1,
                Error::<T>::NotCandidate
            );
            Ok(())
        }
    }
}
