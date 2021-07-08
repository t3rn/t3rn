#![cfg_attr(not(feature = "std"), no_std)]

//! Transaction Weight Examples

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
    pub type StoredValue<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn store_value(_origin: OriginFor<T>, entry: u32) -> DispatchResultWithPostInfo {
            StoredValue::<T>::put(entry);
            Ok(().into())
        }

        #[pallet::weight(20_000)]
        pub fn add_n(_origin: OriginFor<T>, n: u32) -> DispatchResultWithPostInfo {
            let mut old: u32;
            for _i in 1..=n {
                old = StoredValue::<T>::get();
                StoredValue::<T>::put(old + 1);
            }
            Ok(().into())
        }

        // Here the first parameter, a boolean has a significant effect on the computational
        // intensity of the call.
        // #[weight = Conditional(200)]
        #[pallet::weight(200)]
        pub fn add_or_set(_origin: OriginFor<T>, add_flag: bool, val: u32) -> DispatchResultWithPostInfo {
            if add_flag {
                StoredValue::<T>::put(&val);
            }
            else {
                for _i in 1..=val {
                    StoredValue::<T>::put(StoredValue::<T>::get());
                }
            }

           Ok(().into())
        }

        // This one is quadratic in the first argument plus linear in the second plus a constant.
        // This calculation is not meant to do something really useful or common other than
        // demonstrate that weights should grow by the same order as the compute required by the
        // transaction.
        // #[weight = Quadratic(200, 30, 100)]
        #[pallet::weight(20030)]
        pub fn complex_calculations(_origin: OriginFor<T>, x: u32, y: u32) -> DispatchResultWithPostInfo {
            // This first part performs a relatively cheap (hence 30)
            // in-memory calculations.
            let mut part1 = 0;
            for _i in 1..=y {
                part1 += 2
            }

            // The second part performs x^2 storage read-writes (hence 200)
            for _j in 1..=x {
                for _k in 1..=x {
                    StoredValue::<T>::put(StoredValue::<T>::get() + 1);
                }
            }

            // One final storage write (hence 100)
            StoredValue::<T>::put(StoredValue::<T>::get() + part1);

            Ok(().into())
        }

        // The actual expense of `double` is proportional to a storage value. Dispatch
        // weightings can't use storage values directly, because the weight should be computable
        // ahead of time. Instead we have the caller pass in the expected storage value and we
        // ensure it is correct.
        // #[weight = Linear(200)]
        #[pallet::weight(200)]
        pub fn double(_origin: OriginFor<T>, initial_value: u32) -> DispatchResultWithPostInfo {

            // Ensure the value passed by the caller actually matches storage If this condition
            // were not true, the caller would be able to avoid paying appropriate fees.
            let initial = StoredValue::<T>::get();
            ensure!(initial == initial_value, "Storage value did not match parameter");

            for _i in 1..=initial {
                let old = StoredValue::<T>::get();
                StoredValue::<T>::put(old + 1);
            }
            Ok(().into())
        }
    }
}
