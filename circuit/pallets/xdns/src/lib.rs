// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! <!-- markdown-link-check-disable -->
//! # X-DNS Pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_system::{ensure_root, ensure_signed};
use crate::types::{XdnsRecordId, XdnsRecord, AllowedSideEffect};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::prelude::*;
use sp_std::vec::Vec;
use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::{ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use crate::pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
pub mod weights;
use weights::WeightInfo;

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use crate::WeightInfo;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Time;
    use frame_system::pallet_prelude::*;
    use sp_std::convert::TryInto;
    use t3rn_primitives::{ChainId, EscrowTrait, GatewayType, GatewayVendor};

    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config + EscrowTrait {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: weights::WeightInfo;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {
            // Perform necessary data/state clean up here.
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: T::BlockNumber) {
            // We don't do anything here.
            // but we could dispatch extrinsic (transaction/unsigned/inherent) using
            // sp_io::submit_extrinsic.
            // To see example on offchain worker, please refer to example-offchain-worker pallet
            // accompanied in this repository.
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Inserts a xdns_record into the on-chain registry. Root only access.
        #[pallet::weight(<T as Config>::WeightInfo::add_new_xdns_record())]
        pub fn add_new_xdns_record(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: ChainId,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: GatewayVendor,
            gateway_type: GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            allowed_side_effects: Vec<AllowedSideEffect>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // ToDo: Uncomment when switching into a model with open registration. Sudo access for now.
            // xdns_record.assign_registrant(registrant.clone());
            let registrant = Default::default();

            let mut xdns_record = XdnsRecord::<T::AccountId>::new(
                url,
                gateway_id,
                gateway_abi,
                gateway_vendor,
                gateway_type,
                gateway_genesis,
                allowed_side_effects,
            );

            let now = TryInto::<u64>::try_into(<T as EscrowTrait>::Time::now())
                .map_err(|_| "Unable to compute current timestamp")?;

            xdns_record.set_last_finalized(now);

            let xdns_record_id = xdns_record.generate_id::<T>();

            if <XDNSRegistry<T>>::contains_key(&xdns_record_id) {
                Err(Error::<T>::XdnsRecordAlreadyExists.into())
            } else {
                <XDNSRegistry<T>>::insert(&xdns_record_id, xdns_record);
                Self::deposit_event(Event::<T>::XdnsRecordStored(registrant, xdns_record_id));
                Ok(().into())
            }
        }

        /// Updates the last_finalized field for an xdns_record from the onchain registry. Root only access.
        #[pallet::weight(<T as Config>::WeightInfo::update_ttl())]
        pub fn update_ttl(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            last_finalized: u64,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Self::update_gateway_ttl(gateway_id, last_finalized)
        }

        /// Removes a xdns_record from the onchain registry. Root only access.
        #[pallet::weight(<T as Config>::WeightInfo::purge_xdns_record())]
        pub fn purge_xdns_record(
            origin: OriginFor<T>,
            requester: T::AccountId,
            xdns_record_id: XdnsRecordId<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if !<XDNSRegistry<T>>::contains_key(&xdns_record_id) {
                Err(Error::<T>::UnknownXdnsRecord)?
            } else {
                <XDNSRegistry<T>>::remove(&xdns_record_id);
                Self::deposit_event(Event::<T>::XdnsRecordPurged(requester, xdns_record_id));
                Ok(().into())
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[requester, xdns_record_id\]
        XdnsRecordStored(T::AccountId, XdnsRecordId<T>),
        /// \[requester, xdns_record_id\]
        XdnsRecordPurged(T::AccountId, XdnsRecordId<T>),
        /// \[xdns_record_id\]
        XdnsRecordUpdated(XdnsRecordId<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Stored xdns_record has already been added before
        XdnsRecordAlreadyExists,
        /// Access of unknown xdns_record
        UnknownXdnsRecord,
        /// Xdns Record not found
        XdnsRecordNotFound,
    }

    /// The pre-validated composable xdns_records on-chain registry.
    #[pallet::storage]
    #[pallet::getter(fn xdns_registry)]
    pub type XDNSRegistry<T: Config> =
        StorageMap<_, Blake2_128Concat, XdnsRecordId<T>, XdnsRecord<T::AccountId>, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub known_xdns_records: Vec<XdnsRecord<T::AccountId>>,
    }

    /// The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                known_xdns_records: Default::default(),
            }
        }
    }

    /// The build of genesis for the pallet.
    /// Populates storage with the known XDNS Records
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for xdns_record in self.known_xdns_records.clone() {
                <XDNSRegistry<T>>::insert(xdns_record.generate_id::<T>(), xdns_record);
            }
        }
    }

    impl<T: Config> Pallet<T> {
        /// Locates the best available gateway based on the time they were last finalized.
        /// Priority goes Internal > External > TxOnly, followed by the largest last_finalized value
        pub fn best_available(
            gateway_id: ChainId,
        ) -> Result<XdnsRecord<T::AccountId>, &'static str> {
            // Sort each available gateway pointer based on its GatewayType
            let gateway_pointers = t3rn_primitives::retrieve_gateway_pointers(gateway_id);
            ensure!(gateway_pointers.is_ok(), "No available gateway pointers");
            let mut sorted_gateway_pointers = gateway_pointers.unwrap();
            sorted_gateway_pointers.sort_by(|a, b| a.gateway_type.cmp(&b.gateway_type));

            // Fetch each XdnsRecord and re-sort based on its last_finalized descending
            let mut sorted_gateways: Vec<XdnsRecord<T::AccountId>> = sorted_gateway_pointers
                .into_iter()
                .map(|gateway_pointer| {
                    <XDNSRegistry<T>>::get(T::Hashing::hash(
                        Encode::encode(&gateway_pointer.id).as_ref(),
                    ))
                })
                .filter(|xdns_record| xdns_record.is_some())
                .map(|xdns_record| xdns_record.unwrap())
                .collect();
            sorted_gateways
                .sort_by(|xdns_a, xdns_b| xdns_b.last_finalized.cmp(&xdns_a.last_finalized));

            // Return the first result
            if sorted_gateways.is_empty() {
                return Err("Xdns record not found");
            }

            Ok(sorted_gateways[0].clone())
        }

        pub fn update_gateway_ttl(
            gateway_id: ChainId,
            last_finalized: u64,
        ) -> DispatchResultWithPostInfo {
            let xdns_record_id = T::Hashing::hash(Encode::encode(&gateway_id).as_ref());

            if !XDNSRegistry::<T>::contains_key(xdns_record_id) {
                Err(Error::<T>::XdnsRecordNotFound.into())
            } else {
                XDNSRegistry::<T>::mutate(xdns_record_id, |xdns_record| match xdns_record {
                    None => Err(Error::<T>::XdnsRecordNotFound),
                    Some(record) => {
                        record.set_last_finalized(last_finalized);
                        Ok(())
                    }
                })?;

                Self::deposit_event(Event::<T>::XdnsRecordUpdated(xdns_record_id));
                Ok(().into())
            }
        }

        /// Fetches all known XDNS records
        pub fn fetch_records() -> Vec<XdnsRecord<T::AccountId>> {
            pallet::XDNSRegistry::<T>::iter_values().collect()
        }
    }
}
