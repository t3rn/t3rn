use super::*;
use frame_support::{traits::{Get, StorageVersion, GetStorageVersion}, weights::Weight};
use t3rn_abi::SFXAbi;
use sp_runtime::DispatchError;


pub mod migration {
    use super::*;

   // pub mod v2 {...} // only contains V1 storage format

    pub fn migrate<T: Config>() -> Weight {
        // Define the maximum weight of this migration.
        let max_weight = T::DbWeight::get().reads_writes(10, 10);
        // Define the current storage migration version.
        const CURRENT_STORAGE_VERSION: u32 = 2;
        // Migrate the storage entries.
        StorageMigrations::<T>::try_mutate(|current_version| {
            match *current_version {
                0 => {
                    // Storage Migration: StandardSideEffects -> StandardSFXABIs
                    // Storage Migration Details: 16-03-2023; v1.4.0-rc -> v1.5.0-rc
                    // Iterate through the old storage entries and migrate them.
                    for (key, _value) in StandardSideEffects::<T>::drain() {
                        let sfx4b_id = key;
                        match SFXAbi::get_standard_interface(sfx4b_id) {
                            Some(sfx_abi) => {
                                StandardSFXABIs::<T>::insert(sfx4b_id, sfx_abi);
                            }
                            None => {
                                log::error!(
                                "Failed to migrate StandardSideEffects to StandardSFXABIs for sfx4b_id: {:?}",
                                sfx4b_id
                            );
                            }
                        }
                    }

                    // Set migrations_done to true
                    *current_version = CURRENT_STORAGE_VERSION;

                    // Return the weight consumed by the migration.
                    Ok::<Weight, DispatchError>(max_weight)
                }
                // Storage Migration: Raw XDNS storage entry kill
                // Storage Migration Details: 27-07-2023; v1.4.43-rc -> v1.4.44-rc
                //     Many Collators on t0rn hit: frame_support::storage: (key, value) failed to decode at [225, 205, 72, 162, 242, 43, 101, 142, 192, 157, 178, 168, 200, 143, 21, 13, 175, 239, 182, 147, 135, 79, 226, 105, 210, 52, 22, 179, 228, 93, 185, 249, 114, 111, 99, 111]
                1 => {
                    // Manually kill the old XDNS storage entry (XDNSRegistry is now replaced by Gateways)
                    frame_support::storage::unhashed::kill(&[225, 205, 72, 162, 242, 43, 101, 142, 192, 157, 178, 168, 200, 143, 21, 13, 175, 239, 182, 147, 135, 79, 226, 105, 210, 52, 22, 179, 228, 93, 185, 249, 114, 111, 99, 111]);
                    // Set migrations_done to true
                    *current_version = CURRENT_STORAGE_VERSION;
                    // Return the weight consumed by the migration.
                    Ok::<Weight, DispatchError>(T::DbWeight::get().writes(1))
                }
                // Storage Migration: Another Raw XDNS storage entry kill
                // Storage Migration Details: 27-07-2023; v1.4.44-rc -> v1.4.45-rc
                //     Many Collators on t0rn hit: frame_support::storage: (key, value) failed to decode at [84, 10, 79, 135, 84, 170, 82, 152, 163, 214, 233, 170, 9, 233, 63, 151, 78, 11,
                //      18, 119, 80, 58, 19, 112, 111, 133, 165, 20, 116, 96, 124, 88, 24, 172, 250, 191, 195, 140, 91, 41, 106, 32, 177, 28, 37, 248, 177, 35, 27, 230, 169, 204, 8, 192, 121, 163, 226, 24, 100, 166, 207, 36, 66, 173, 219, 150, 184, 250, 101, 171, 135, 85,]
                2 => {
                    // Manually kill the old XDNS storage entry (XDNSRegistry is now replaced by Gateways)
                    frame_support::storage::unhashed::kill(&[84, 10, 79, 135, 84, 170, 82, 152, 163, 214, 233, 170, 9, 233, 63, 151, 78, 11,
                        18, 119, 80, 58, 19, 112, 111, 133, 165, 20, 116, 96, 124, 88, 24, 172, 250,
                        191, 195, 140, 91, 41, 106, 32, 177, 28, 37, 248, 177, 35, 27, 230, 169, 204,
                        8, 192, 121, 163, 226, 24, 100, 166, 207, 36, 66, 173, 219, 150, 184, 250, 101,
                        171, 135, 85,]);
                    // Set migrations_done to true
                    *current_version = CURRENT_STORAGE_VERSION;
                    // Return the weight consumed by the migration.
                   Ok::<Weight, DispatchError>(T::DbWeight::get().writes(1))
                }
                // Add more migration cases here, if needed in the future
                _ => {
                    // No migration needed.
                    Ok::<Weight, DispatchError>(Weight::zero())
                }
            }
        })
        .unwrap_or_default()
        /*
        let onchain_version =  Pallet::<T>::on_chain_storage_version();
        if onchain_version < 3 {
            // TO DO: We transform the storage values from the old into the new format.
            // Update storage version.
            StorageVersion::new(3).put::<Pallet::<T>>();
            // TO DO: TokenRecord
            let count = NameOf::<T>::iter().count();
            T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
        }
        else {
            // We don't do anything here.
            Weight::zero()
        }
        */
    }
}