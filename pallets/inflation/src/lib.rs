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

#[pallet]
pub mod pallet {
    use crate::inflation::{InflationInfo, Range, RoundInfo};
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
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize;

        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn inflation_config)]
    // The pallet's inflation config per year
    pub type InflationConfig<T: Config> = StorageValue<_, InflationInfo, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_round)]
    // Information on the current epoch
    pub type CurrentRound<T: Config> = StorageValue<_, RoundInfo<T::BlockNumber>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintedTokensForRound(T::AccountId, T::Balance),
        MintedTokensExactly(T::AccountId, T::Balance),
        AllocatedToAccount(T::AccountId, T::Balance),
        InflationSet(Perbill, Perbill, Perbill, Perbill, Perbill, Perbill),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInflationSchedule,
        MintingFailed,
        NotEnoughFunds,
        NoWritingSameValue, // when trying to update the inflation schedule
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            todo!()
        }
        fn on_finalize(_n: BlockNumberFor<T>) {
            // check if era finished in current block
            // if so, update storage reward objects and create a new empty one
            todo!()
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub annual_inflation: Range<T::Balance>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                annual_inflation: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
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
            Self::deposit_event(Event::MintedTokensForRound(treasury, amount));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
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
            Self::deposit_event(Event::InflationSet(
                config.annual.min,
                config.annual.ideal,
                config.annual.max,
                config.per_round.min,
                config.per_round.ideal,
                config.per_round.max,
            ));
            <InflationConfig<T>>::put(config);
            Ok(().into())
        }
    }
}
