use super::*;
use frame_support::{traits::{Get, StorageVersion, GetStorageVersion}, weights::Weight};
use sp_runtime::DispatchError;
use t3rn_types::migrations::v13::FullSideEffectV13;


pub mod migration {
    use super::*;

   // pub mod v2 {...} // only contains V1 storage format

    pub fn migrate<T: Config>() -> Weight {
        // Define the maximum weight of this migration.
        let max_weight = T::DbWeight::get().reads_writes(10, 10);
        // Define the current storage migration version.
        const CURRENT_STORAGE_VERSION: u32 = 1;
        // Migrate the storage entries.
        StorageMigrations::<T>::try_mutate(|current_version| {
            match *current_version {
                0 => {
                    // Storage Migration: FSX::SFX updates field "encoded_action: Vec<u8>" to "action: Action: [u8; 4]"
                    // Storage Migration Details: 16-03-2023; v1.3.0-rc -> v1.4.0-rc
                    // Iterate through the old storage entries and migrate them.
                    FullSideEffects::<T>::translate(
                        |_,
                         value: Vec<
                             Vec<
                                 FullSideEffectV13<
                                     T::AccountId,
                                     frame_system::pallet_prelude::BlockNumberFor<T>,
                                     BalanceOf<T>,
                                 >,
                             >,
                         >| {
                            Some(
                                value
                                    .into_iter()
                                    .map(|v| v.into_iter().map(FullSideEffect::from).collect())
                                    .collect(),
                            )
                        },
                    );

                    // Set migrations_done to true
                    *current_version = CURRENT_STORAGE_VERSION;

                    // Return the weight consumed by the migration.
                    Ok::<Weight, DispatchError>(max_weight)
                },
                // Add more migration cases here, if needed in the future
                _ => {
                    // No migration needed.
                    Ok::<Weight, DispatchError>(Weight::zero())
                },
            }
        })
            .unwrap_or(Weight::zero())
    }
}