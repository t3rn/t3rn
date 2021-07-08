#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use codec::{Encode, Decode};

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn get_value)]
    pub type Value<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn flip(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            Value::<T>::put(!Value::<T>::get());
            Ok(().into())
        }
    }
}