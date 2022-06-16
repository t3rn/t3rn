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
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, OriginTrait, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    use sp_runtime::{
        traits::{Saturating, Zero},
        Perbill,
    };

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Minimum number of blocks per round.
        /// Serves as the default round term being applied in pallet genesis.
        /// NOTE: Must be at least the size of the active collator set.
        #[pallet::constant]
        type MinBlocksPerRound: Get<u32>;

        #[pallet::constant]
        type TokenCirculationAtGenesis: Get<u32>;

        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn inflation_config)]
    // The pallet's inflation mechanism configuration.
    pub type InflationConfig<T: Config> = StorageValue<_, InflationInfo, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_round)]
    // Information on the current epoch.
    pub type CurrentRound<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn available_to_mint)]
    // // Remaining tokens to be minted
    // pub type AvailableTokensToBeMinted<T: Config> = StorageValue<_, BalanceOf<T>>;

    // TODO: total supply, circulating, remaining

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
        InflationConfigChanged {
            annual_min: Perbill,
            annual_ideal: Perbill,
            annual_max: Perbill,
            round_min: Perbill,
            round_ideal: Perbill,
            round_max: Perbill,
        },
        RewardsConfigChanged {
            dev_reward: Perbill,
            exec_reward: Perbill,
        },
        NewRound {
            round: RoundIndex,
            head: T::BlockNumber,
        },
        RewardsClaimed(T::AccountId, BalanceOf<T>),
        RoundTermChanged {
            round: RoundIndex,
            head: T::BlockNumber,
            old: u32,
            new: u32,
            new_per_round_inflation_min: Perbill,
            new_per_round_inflation_ideal: Perbill,
            new_per_round_inflation_max: Perbill,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInflationRange,
        InvalidInflationRewardsConfig,
        MintingFailed,
        NotEnoughFunds,
        NotCandidate, // when someone tries to claim rewards when not being a candidate
        MisalignedCandidates, // when theres an error when calculating candidate rewards
        ValueNotChanged, // when trying to set sth without actually updating the value
        TooFewBlocksPerRound,
        NoRewardsAvailable,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut round = <CurrentRound<T>>::get();
            // let inflation_info = <InflationConfig<T>>::get();

            if round.should_update(n) {
                // mutate round
                round.update(n);

                // update round in storage
                <CurrentRound<T>>::put(round);

                // calculate amount to be rewarded per candidate
                // inflation_info.round.ideal
                // let funds = T::Currency::issue(round)
                // <Pallet<T>>::mint_for_round();
                // let candidates = <CandidatesForRewards<T>>::get();
                // candidates.into_iter().for_each(|candidate| {
                //     T::Currency::deposit_into_existing(candidate, amount)
                //         .expect("Should deposit balance to account");
                // });

                // emit event
                Self::deposit_event(Event::NewRound {
                    round: round.index,
                    head: round.head,
                });
            }

            T::WeightInfo::on_initialize()
        }

        fn on_finalize(_n: BlockNumberFor<T>) {
            // check if round finished in current block
            // if so, update storage reward objects and create a new empty one
            todo!();
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub candidates: Vec<T::AccountId>,
        pub annual_inflation: Range<Perbill>,
        pub rewards_alloc: RewardsAllocationConfig,
        pub round_term: u32,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                candidates: Default::default(),
                annual_inflation: Range {
                    min: Perbill::from_parts(3),   //TODO
                    ideal: Perbill::from_parts(4), // TODO
                    max: Perbill::from_parts(5),   // TODO
                },
                rewards_alloc: RewardsAllocationConfig {
                    executor: Perbill::from_parts(500_000_000),
                    developer: Perbill::from_parts(500_000_000),
                },
                round_term: 5,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // set first round
            let round: RoundInfo<T::BlockNumber> =
                RoundInfo::new(1_u32, T::BlockNumber::zero(), self.round_term);
            <CurrentRound<T>>::put(round);

            // set inflation config
            <Pallet<T>>::set_inflation(T::Origin::root(), self.annual_inflation);

            // set rewards allocation amongst t3rn actors
            <Pallet<T>>::set_rewards_alloc(T::Origin::root(), self.rewards_alloc.clone());

            <Pallet<T>>::deposit_event(Event::NewRound {
                round: round.index,
                head: round.head,
            })
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mints a dynamic (FIXME) amount of tokens for given round.
        /// TODO: maybe ensure can only be called once per round
        #[pallet::weight(10_000)]
        pub fn mint_for_round(
            origin: OriginFor<T>,
            round_index: RoundIndex,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let treasury = T::TreasuryAccount::get();
            // let round = <CurrentRound<T>>::get();
            let inflation_info = <InflationConfig<T>>::get();

            // count actors
            let (count_devs, count_execs) =
                <CandidatesForRewards<T>>::iter_keys().fold((0, 0), |mut acc, c| {
                    match c.1 {
                        CandidateRole::Developer => {
                            acc.0 = acc.0 + 1;
                        },
                        CandidateRole::Executor => {
                            acc.1 = acc.1 + 1;
                        },
                    }
                    acc
                });

            // calculate relative rewards per actor
            let relative_per_dev = Perbill::from_rational(1, count_devs as u32)
                * inflation_info.rewards_alloc.developer;
            let relative_per_exec = Perbill::from_rational(1, count_execs as u32)
                * inflation_info.rewards_alloc.executor;

            // calculate absoute rewards per actor
            let absolute_per_dev = relative_per_dev * amount;
            let absolute_per_exec = relative_per_exec * amount;

            // for each candidate in the round issue rewards
            for (candidate, role) in <CandidatesForRewards<T>>::iter_keys() {
                let issued = match role {
                    CandidateRole::Developer => {
                        T::Currency::issue(absolute_per_dev);
                        absolute_per_dev
                    },
                    CandidateRole::Executor => {
                        T::Currency::issue(absolute_per_exec);
                        absolute_per_exec
                    },
                };

                <RewardsPerCandidatePerRound<T>>::insert(candidate.clone(), round_index, issued);

                Self::deposit_event(Event::MintedTokensForCandidate(candidate, issued));
            }

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

            ensure!(
                total_rewards > BalanceOf::<T>::zero(),
                Error::<T>::NoRewardsAvailable
            );

            // allocate to candidate
            T::Currency::deposit_into_existing(&who, BalanceOf::<T>::from(total_rewards))
                .expect("Should deposit balance to account");

            Self::deposit_event(Event::RewardsClaimed(who, total_rewards));

            Ok(().into())
        }

        /// Sets the annual inflation rate to derive per-round inflation
        #[pallet::weight(10_000)]
        pub fn set_inflation(
            origin: OriginFor<T>,
            annual_inflation_config: Range<Perbill>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                annual_inflation_config.is_valid(),
                Error::<T>::InvalidInflationRange
            );

            let mut inflation_info = <InflationConfig<T>>::get();
            ensure!(
                inflation_info.annual != annual_inflation_config,
                Error::<T>::ValueNotChanged
            );

            // update annual and round inflation config
            inflation_info.update_from_annual::<T>(annual_inflation_config);

            // keep the updated round inflation info in scope
            let round_inflation_info = inflation_info.round;

            // update the stored inflation config
            <InflationConfig<T>>::put(inflation_info);

            Self::deposit_event(Event::InflationConfigChanged {
                annual_min: annual_inflation_config.min,
                annual_ideal: annual_inflation_config.ideal,
                annual_max: annual_inflation_config.max,
                round_min: round_inflation_info.min,
                round_ideal: round_inflation_info.ideal,
                round_max: round_inflation_info.max,
            });

            Ok(().into())
        }

        /// Sets the reward percentage to be allocated between developers and executors
        #[pallet::weight(10_000)]
        pub fn set_rewards_alloc(
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
                config.rewards_alloc != rewards_config,
                Error::<T>::ValueNotChanged
            );

            // config.rewards_alloc = rewards_config.clone();
            let (dev_reward, exec_reward) = (rewards_config.developer, rewards_config.executor);
            config.rewards_alloc = rewards_config;

            <InflationConfig<T>>::put(config);

            Self::deposit_event(Event::RewardsConfigChanged {
                dev_reward,
                exec_reward,
            });

            Ok(().into())
        }

        /// Set blocks per round
        /// - if called with `new` less than term of current round, will transition immediately
        /// in the next block
        /// - also updates per-round inflation config
        #[pallet::weight(10_000)]
        pub fn set_blocks_per_round(origin: OriginFor<T>, new: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                new >= T::MinBlocksPerRound::get(),
                Error::<T>::TooFewBlocksPerRound
            );

            let mut round = <CurrentRound<T>>::get();
            ensure!(round.term != new, Error::<T>::ValueNotChanged);

            // update global round term
            round.term = new;
            <CurrentRound<T>>::put(round);

            // update per-round inflation given the new number of rounds per year
            let mut inflation_info = <InflationConfig<T>>::get();
            inflation_info.update_round_term(new);

            // keep the updated round inflation info in scope
            let round_inflation_info = inflation_info.round;

            // update the stored inflation config
            <InflationConfig<T>>::put(inflation_info);

            Self::deposit_event(Event::RoundTermChanged {
                round: round.index,
                head: round.head,
                old: round.term,
                new,
                new_per_round_inflation_min: round_inflation_info.min,
                new_per_round_inflation_ideal: round_inflation_info.ideal,
                new_per_round_inflation_max: round_inflation_info.max,
            });

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
