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
        inflation::{InflationInfo, Range, RoundIndex, RoundInfo},
        weights::WeightInfo,
    };
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Imbalance, ReservableCurrency},
    };
    use frame_system::{ensure_root, pallet_prelude::*};
    use sp_runtime::{
        traits::{AtLeast32BitUnsigned, Saturating, Zero},
        Perbill,
    };

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaxEncodedLen
            + MaybeSerializeDeserialize;

        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;

        #[pallet::constant]
        type DefaultBlocksPerRound: Get<u32>;

        #[pallet::constant]
        type TokenCirculationAtGenesis: Get<u32>;

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
    pub type CandidatesForRewards<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, OptionQuery>;

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
        InflationSet {
            annual_min: Perbill,
            annual_ideal: Perbill,
            annual_max: Perbill,
            round_min: Perbill,
            round_ideal: Perbill,
            round_max: Perbill,
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
        MintingFailed,
        NotEnoughFunds,
        NotCandidate, // when someone tries to claim rewards when not being a candidate
        NoWritingSameValue, // when trying to update the inflation schedule
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let mut round = <CurrentRound<T>>::get();
            let inflation_info = <InflationConfig<T>>::get();

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

        fn on_finalize(_n: BlockNumberFor<T>) {
            // check if round finished in current block
            // if so, update storage reward objects and create a new empty one
            todo!()
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub candidates: Vec<T::AccountId>,
        pub annual_inflation: Range<BalanceOf<T>>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                candidates: Default::default(),
                annual_inflation: Default::default(),
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

            // let mut remaining_minted_balance = T::Currency::issue(amount);
            // let mut candidates = <CandidatesForRewards<T>>::get()?;
            // distribute to candidates equally
            // candidates.into_iter().for_each(|candidate| {
            // calculate rewards per candidate
            // let amount_per_candidate = amount.div(T::Balance::from(candidates.len() as u8));
            // let (remaining_minted_balance, reward_amount) =
            //     remaining_minted_balance.split(amount_per_candidate);
            // T::Currency::deposit_into_existing(candidate, reward_amount);
            // });

            // Emit an event.
            Self::deposit_event(Event::MintedTokensForRound(treasury, amount));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Pallet<T>>::ensure_candidate(&who)?;

            // accumulate rewards
            let total_rewards = <RewardsPerCandidatePerRound<T>>::iter_prefix(&who)
                .drain()
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
    }

    impl<T: Config> Pallet<T> {
        /// Helper function to check if the origin belongs to the candidate list
        pub fn ensure_candidate(who: &T::AccountId) -> Result<(), DispatchError> {
            ensure!(
                <CandidatesForRewards<T>>::contains_key(who),
                Error::<T>::NotCandidate
            );
            Ok(())
        }
    }
}
