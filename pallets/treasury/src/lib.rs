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
            BeneficiaryRole, InflationInfo, InflationAllocation, Range, RoundIndex,
            RoundInfo, perbill_annual_to_perbill_round, BLOCKS_PER_YEAR
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

        /// Total amount to be issued at genesis.
        #[pallet::constant]
        type GenesisIssuance: Get<u32>;

        /// Minimum number of blocks per round.
        /// Serves as the default round term being applied in pallet genesis.
        /// NOTE: Must be at least the size of the active collator set.
        #[pallet::constant]
        type MinBlocksPerRound: Get<u32>;

        /// The parachain treasury account. 5%.
        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;

        /// The vault reserve account. 9%.
        #[pallet::constant]
        type ReserveAccount: Get<Self::AccountId>;

        /// The parachain auction fund account. 30%.
        #[pallet::constant]
        type AuctionFund: Get<Self::AccountId>;

        /// The contracts fund account for additional builder rewards. 3%.
        #[pallet::constant]
        type ContractFund: Get<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn inflation_alloc)]
    /// The pallet's rewards allocation config.
    pub type InflationAlloc<T: Config> =
        StorageValue<_, InflationAllocation, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn inflation_config)]
    /// The pallet's inflation mechanism configuration.
    pub type InflationConfig<T: Config> = StorageValue<_, InflationInfo, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_round)]
    /// Information on the current treasury round.
    pub type CurrentRound<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_stake_expectation)]
    /// Expected total stake sum of active executors and collators.
    /// Active means member of the respective active set.
    pub type TotalStakeExpectation<T: Config> = StorageValue<_, Range<BalanceOf<T>>, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn available_to_mint)]
    // // Remaining tokens to be minted | cap?
    // pub type AvailableTokensToBeMinted<T: Config> = StorageValue<_, BalanceOf<T>>;
    // TODO: eventual cap, circulating, remaining

    #[pallet::storage]
    #[pallet::getter(fn beneficiaries)]
    pub type Beneficiaries<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        BeneficiaryRole,
        BalanceOf<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn rewards_per_round)]
    pub type BeneficiaryRoundRewards<T: Config> = StorageDoubleMap<
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
        NewRound {
            round: RoundIndex,
            head: T::BlockNumber,
        },
        RoundTermChanged {
            old: u32,
            new: u32,
            round_min: Perbill,
            round_ideal: Perbill,
            round_max: Perbill,
        },
        InflationConfigChanged {
            annual_min: Perbill,
            annual_ideal: Perbill,
            annual_max: Perbill,
            round_min: Perbill,
            round_ideal: Perbill,
            round_max: Perbill,
        },
        InflationAllocationChanged {
            developer: Perbill,
            executor: Perbill,
        },
        RoundTokensIssued(RoundIndex, BalanceOf<T>),
        BeneficiaryTokensIssued(T::AccountId, BalanceOf<T>),
        RewardsClaimed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInflationConfig,
        InvalidInflationAllocation,
        ValueNotChanged,
        RoundTermTooShort,
        NotBeneficiary,
        NoRewardsAvailable,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut round = <CurrentRound<T>>::get();
            log::info!(
                "SHOULD ROUND UPDATE AT BLOCK NUMBER {:?}? {:?}",
                n,
                round.should_update(n)
            );
            if round.should_update(n) {
                // update round
                round.update(n);
                <CurrentRound<T>>::put(round);

                Self::deposit_event(Event::NewRound {
                    round: round.index,
                    head: round.head,
                });

                // issue tokens for the past round
                // TODO impl delay
                // TODO sum  active executors + collators stake
                let total_stake: BalanceOf<T> = Self::u32_to_balance(50_u32);
                log::info!(
                    "CIRCULATING CIRCULATING CIRCULATING {:?}",
                    T::Currency::total_issuance()
                );
                let round_issuance = Self::compute_round_issuance(total_stake);
                Self::mint_for_round(T::Origin::root(), round.index - 1, round_issuance)
                    .expect("mint for round");
            }

            T::WeightInfo::on_initialize()
        }

        fn on_finalize(_n: BlockNumberFor<T>) {}
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub candidates: Vec<T::AccountId>,
        pub annual_inflation: Range<Perbill>,
        pub inflation_alloc: InflationAllocation,
        pub round_term: u32,
        pub total_stake_expectation: Range<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                candidates: Default::default(), // TODO blacklist
                annual_inflation: Range {
                    min: Perbill::from_parts(3),   // TODO
                    ideal: Perbill::from_parts(4), // TODO
                    max: Perbill::from_parts(5),   // TODO
                },
                inflation_alloc: InflationAllocation {
                    executor: Perbill::from_parts(500_000_000),// TODO
                    developer: Perbill::from_parts(500_000_000),// TODO
                },
                round_term: T::MinBlocksPerRound::get(),
                total_stake_expectation: Range {
                    min: <Pallet<T>>::u32_to_balance(0_u32),         // TODO
                    ideal: <Pallet<T>>::u32_to_balance(1000_u32),    //TODO
                    max: <Pallet<T>>::u32_to_balance(1_000_000_u32), //TODO
                },
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

            // set annual and round inflation rate
            let rounds_per_year = BLOCKS_PER_YEAR / self.round_term;
            let mut inflation_info: InflationInfo = Default::default();
                    inflation_info.annual = self.annual_inflation;
                    inflation_info.round = perbill_annual_to_perbill_round(inflation_info.annual, rounds_per_year);
                     <InflationConfig<T>>::put(inflation_info);

            // set rewards allocation amongst t3rn actors
            <InflationAlloc<T>>::put(self.inflation_alloc.clone());

            // set total stake expectation
            <TotalStakeExpectation<T>>::put(self.total_stake_expectation);

            // issue genesis tokens
            T::Currency::issue(<Pallet<T>>::u32_to_balance(T::GenesisIssuance::get()));

            <Pallet<T>>::deposit_event(Event::NewRound {
                round: round.index,
                head: round.head,
            })
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mints tokens for given round.
        /// TODO: maybe ensure can only be called once per round
        /// TODO: exec, infl
        #[pallet::weight(10_000)] // TODO
        pub fn mint_for_round(
            origin: OriginFor<T>,
            round_index: RoundIndex,
            #[pallet::compact] amount: BalanceOf<T>, // TODO: revisit
        ) -> DispatchResult {
            ensure_root(origin)?;

            let inflation_alloc = <InflationAlloc<T>>::get();

            // count actors
            let (count_devs, count_execs) =
                <Beneficiaries<T>>::iter_keys().fold((0, 0), |mut acc, c| {
                    match c.1 {
                        BeneficiaryRole::Developer => {
                            acc.0 = acc.0 + 1;
                        },
                        BeneficiaryRole::Executor => {
                            acc.1 = acc.1 + 1;
                        },
                    }
                    acc
                });

            // calculate relative rewards per actor
            let relative_per_dev =
                Perbill::from_rational(1, count_devs as u32) * inflation_alloc.developer;
            let relative_per_exec =
                Perbill::from_rational(1, count_execs as u32) * inflation_alloc.executor;

            // calculate absoute rewards per actor
            let absolute_per_dev = relative_per_dev * amount;
            let absolute_per_exec = relative_per_exec * amount;

            // for each candidate in the round issue rewards
            for (candidate, role) in <Beneficiaries<T>>::iter_keys() {
                let issued = match role {
                    BeneficiaryRole::Developer => {
                        T::Currency::issue(absolute_per_dev);
                        absolute_per_dev
                    },
                    BeneficiaryRole::Executor => {
                        T::Currency::issue(absolute_per_exec);
                        absolute_per_exec
                    },
                };

                <BeneficiaryRoundRewards<T>>::insert(candidate.clone(), round_index, issued);

                Self::deposit_event(Event::BeneficiaryTokensIssued(candidate, issued));
            }

            Self::deposit_event(Event::RoundTokensIssued(round_index, amount));

            Ok(())
        }

        #[pallet::weight(10_000)] // TODO
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Pallet<T>>::ensure_beneficiary(&who)?;

            // accumulate claimer's round rewards up till now
            let total_rewards = <BeneficiaryRoundRewards<T>>::iter_prefix(&who)
                .drain()
                .map(|kv| kv.1)
                .fold(BalanceOf::<T>::zero(), |acc, round_rewards| {
                    acc.saturating_add(round_rewards)
                });

            ensure!(
                total_rewards > BalanceOf::<T>::zero(),
                Error::<T>::NoRewardsAvailable
            );

            // allocate to beneficiary
            T::Currency::deposit_into_existing(&who, BalanceOf::<T>::from(total_rewards))
                .expect("Should deposit balance to account"); // TODO

            Self::deposit_event(Event::RewardsClaimed(who, total_rewards));

            Ok(().into())
        }

        /// Sets the annual inflation rate to derive per-round inflation
        #[pallet::weight(10_000)] // TODO
        pub fn set_inflation(
            origin: OriginFor<T>,
            annual_inflation_config: Range<Perbill>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                annual_inflation_config.is_valid(),
                Error::<T>::InvalidInflationConfig
            );

            let mut inflation_info = <InflationConfig<T>>::get();
            ensure!(
                inflation_info.annual != annual_inflation_config,
                Error::<T>::ValueNotChanged
            );

            // update annual and round inflation config
            inflation_info.update_from_annual::<T>(annual_inflation_config)?;
            let round_inflation_info = inflation_info.round;
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

        /// Sets the reward percentage to be allocated amongst t3rn actors
        #[pallet::weight(10_000)] // TODO
        pub fn set_inflation_alloc(
            origin: OriginFor<T>,
            inflation_alloc: InflationAllocation,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                inflation_alloc.is_valid(),
                Error::<T>::InvalidInflationAllocation
            );
            ensure!(
                inflation_alloc != <InflationAlloc<T>>::get(),
                Error::<T>::ValueNotChanged
            );

            // update rewards config
            let (developer, executor) = (
                inflation_alloc.developer,
                inflation_alloc.executor,
            );
            <InflationAlloc<T>>::put(inflation_alloc);

            Self::deposit_event(Event::InflationAllocationChanged {
                developer,
                executor,
            });

            Ok(().into())
        }

        /// Set blocks per round
        /// - if called with `new` less than term of current round, will transition immediately
        /// in the next block
        /// - also updates per-round inflation config
        #[pallet::weight(10_000)] // TODO
        pub fn set_round_term(origin: OriginFor<T>, new: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                new >= T::MinBlocksPerRound::get(),
                Error::<T>::RoundTermTooShort
            );

            let mut round = <CurrentRound<T>>::get();
            ensure!(round.term != new, Error::<T>::ValueNotChanged);

            // update global round term
            let old = round.term;
            round.term = new;
            <CurrentRound<T>>::put(round);

            // update per-round inflation given the new number of rounds per year
            let mut inflation_info = <InflationConfig<T>>::get();
            inflation_info.update_from_round_term::<T>(new)?;
            let round_inflation_info = inflation_info.round;
            <InflationConfig<T>>::put(inflation_info);

            Self::deposit_event(Event::RoundTermChanged {
                old,
                new,
                round_min: round_inflation_info.min,
                round_ideal: round_inflation_info.ideal,
                round_max: round_inflation_info.max,
            });

            Ok(().into())
        }

        #[pallet::weight(10_000)] // TODO
        pub fn add_beneficiary(
            origin: OriginFor<T>,
            _beneficiary: T::AccountId,
            _role: BeneficiaryRole,
        ) -> DispatchResult {
            ensure_root(origin)?;
            todo!();
        }

        #[pallet::weight(10_000)] // TODO
        pub fn remove_beneficiary(
            origin: OriginFor<T>,
            _beneficiary: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            todo!();
        }

        /// Set the expectations for total staked. These expectations determine the issuance for
        /// the round according to logic in `fn compute_round_issuance`.
        #[pallet::weight(10_000)] // TODO
        pub fn set_total_stake_expectation(
            _origin: OriginFor<T>,
            _expectations: Range<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            todo!();
            // T::MonetaryGovernanceOrigin::ensure_origin(origin)?;
            // ensure!(expectations.is_valid(), Error::<T>::InvalidSchedule);
            // let mut config = <InflationConfig<T>>::get();
            // ensure!(
            // 	config.expect != expectations,
            // 	Error::<T>::NoWritingSameValue
            // );
            // TODO:   // config.set_expectations(expectations);
            // Self::deposit_event(Event::StakeExpectationsSet {
            // 	expect_min: config.expect.min,
            // 	expect_ideal: config.expect.ideal,
            // 	expect_max: config.expect.max,
            // });
            // <InflationConfig<T>>::put(config);
            // Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // Casts a u32 to our runtime's balance.
        fn u32_to_balance(
            input: u32,
        ) -> <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance
        {
            input.into()
        }

        /// Helper function to check if the origin belongs to the candidate list
        pub fn ensure_beneficiary(who: &T::AccountId) -> Result<(), DispatchError> {
            ensure!(
                <Beneficiaries<T>>::iter_prefix(who).count() == 1,
                Error::<T>::NotBeneficiary
            );
            Ok(())
        }

        /// Computes round issuance based on total staked for the given round
        /// Total stake consists of executors' and collators' total stake.
        fn compute_round_issuance(total_stake: BalanceOf<T>) -> BalanceOf<T> {
            let expect = <TotalStakeExpectation<T>>::get();
            let round_inflation = <InflationConfig<T>>::get().round;
            let round_issuance = crate::inflation::round_issuance_range::<T>(round_inflation);

            if total_stake < expect.min {
                round_issuance.min
            } else if total_stake > expect.max {
                round_issuance.max
            } else {
                round_issuance.ideal
            }
        }
    }
}
