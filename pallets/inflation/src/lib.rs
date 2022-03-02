#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod inflation;

#[frame_support::pallet]
pub mod pallet {
    use crate::inflation::Range;
    use crate::pallet;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency};
    use frame_system::ensure_root;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::Perbill;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn inflation_config)]
    // The pallet's inflation config per year
    type InflationConfig<T: Config> = StorageValue<_, Range<T::Balance>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintedTokensForRound(T::AccountId, u32),
        MintedTokensExactly(T::AccountId, u32),
        AllocatedToAccount(T::AccountId, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        MintingFailed,
        NotEnoughFunds,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            // check if era finished in current block
            // if so, update storage reward objects and create a new empty one
            todo!()
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T> {
        pub starting_inflation: Range<T::Balance>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                starting_inflation: Default::default(),
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn mint_for_round(
            origin: OriginFor<T>,
            #[pallet::compact] amount: T::Balance,
            _round: u32,
        ) -> DispatchResult {
            // mint can only be called from a root account
            ensure_root(origin)?;

            let treasury = T::TreasuryAccount::get();
            // ensure treasury has enough reserved funds to continue with minting

            // free up reserved funds in treasury

            // Emit an event.
            Self::deposit_event(Event::MintedTokensForRound(who, amount));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn mint_exact(origin: OriginFor<T>, amount: T::Currency) -> DispatchResult {
            ensure_root(origin)?;
            T::Currency::issue(amount)?;
            Ok(())
        }

        /// Sets the annual inflation rate to derive per-round inflation
        #[pallet::weight(10_000)]
        pub fn set_inflation(
            origin: OriginFor<T>,
            schedule: Range<Perbill>,
        ) -> DispatchResultWithPostInfo {
            T::MonetaryGovernanceOrigin::ensure_origin(origin)?;
            ensure!(schedule.is_valid(), Error::<T>::InvalidSchedule);
            let mut config = <InflationConfig<T>>::get();
            ensure!(config.annual != schedule, Error::<T>::NoWritingSameValue);
            config.annual = schedule;
            config.set_round_from_annual::<T>(schedule);
            Self::deposit_event(Event::InflationSet(
                config.annual.min,
                config.annual.ideal,
                config.annual.max,
                config.round.min,
                config.round.ideal,
                config.round.max,
            ));
            <InflationConfig<T>>::put(config);
            Ok(().into())
        }
    }
}
