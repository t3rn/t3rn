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

use sp_std::{
	prelude::*
};
use frame_support::{
	dispatch::DispatchResult,
};
use frame_system::{ensure_signed};
use codec::{Encode, Decode};
use sp_runtime::{
	traits::{
		Hash
	},
	RuntimeDebug
};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod tests;

mod weights;

pub use weights::*;

pub type XdnsRecordId<T> = <T as frame_system::Config>::Hash;

/// A preliminary representation of a xdns_record in the onchain registry.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub struct XdnsRecord <AccountId> {

	/// SCALE-encoded url string on where given Consensus System can be accessed
	url: Vec<u8>,

	/// SCALE-encoded metadata specifying available RPC and Extrinsics method implemented by that Consensus System
	metadata: Vec<u8>,

	/// Current runtime version of connected Consensus System
	runtime_version: sp_version::RuntimeVersion,

	/// Genesis hash (first block's id)
	genesis_hash: substrate_api_client::Hash,

	/// ID of gateway. Short and readeable
	gateway_id: t3rn_primitives::InstanceId,

	/// Gateway Vendor
	gateway_vendor: t3rn_primitives::GatewayVendor,

	/// Gateway Type
	gateway_type: t3rn_primitives::GatewayType,

	/// Gateway Type
	maintaned_by: Option<AccountId>,
}

impl<
	AccountId: Encode,
> XdnsRecord <AccountId> {
	pub fn new(
		url: Vec<u8>,
		metadata: Vec<u8>,
		runtime_version: sp_version::RuntimeVersion,
		genesis_hash: substrate_api_client::Hash,
		gateway_id: [u8; 4],
		gateway_vendor: t3rn_primitives::GatewayVendor,
		gateway_type: t3rn_primitives::GatewayType,
		maintaned_by: Option<AccountId>,
	) -> Self {
		XdnsRecord {
			url,
			metadata,
			runtime_version,
			genesis_hash,
			gateway_id,
			gateway_vendor,
			gateway_type,
			maintaned_by,
		}
	}

	pub fn generate_id<T: Config>(
		&self,
	) -> XdnsRecordId<T> {
		T::Hashing::hash(Encode::encode(self).as_ref())
	}
}

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	// Import various types used to declare pallet in scope.
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;
	
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config + t3rn_primitives::EscrowTrait {

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
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
		#[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_new_xdns_record(
			origin: OriginFor<T>,
			requester: T::AccountId,
			xdns_record: XdnsRecord<T::AccountId>
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let xdns_record_id = xdns_record.generate_id::<T>();

			if <XDNSRegistry<T>>::contains_key(&xdns_record_id) {
				Err(Error::<T>::XdnsRecordAlreadyExists)?
			} else {
				<XDNSRegistry<T>>::insert(
					&xdns_record_id,
					xdns_record
				);
				Self::deposit_event(
					Event::<T>::XdnsRecordStored(requester, xdns_record_id)
				);
				Ok(().into())
			}
		}

		/// Removes a xdns_record from the onchain registry. Root only access.
		#[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn update_ttl(
			origin: OriginFor<T>,
			xdns_record_id: XdnsRecordId<T>
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			if !<XDNSRegistry<T>>::contains_key(&xdns_record_id) {
				Err(Error::<T>::UnknownXdnsRecord)?
			} else {
				Self::deposit_event(
					Event::<T>::XdnsRecordUpdated(xdns_record_id)
				);
				Ok(().into())
			}
		}

		/// Removes a xdns_record from the onchain registry. Root only access.
		#[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn purge_xdns_record(
			origin: OriginFor<T>,
			requester: T::AccountId,
			xdns_record_id: XdnsRecordId<T>
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			if !<XDNSRegistry<T>>::contains_key(&xdns_record_id) {
				Err(Error::<T>::UnknownXdnsRecord)?
			} else {
				<XDNSRegistry<T>>::remove(&xdns_record_id);
				Self::deposit_event(
					Event::<T>::XdnsRecordPurged(requester, xdns_record_id)
				);
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
	}

    /// The pre-validated composable xdns_records on-chain registry.
	#[pallet::storage]
	pub type XDNSRegistry<T> = StorageMap<
		_,
		Blake2_128Concat,
		XdnsRecordId<T>,
		XdnsRecord<<T as frame_system::Config>::AccountId>,
		OptionQuery,
	>;

	// The genesis config type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub known_xdns_records: Vec<(T::AccountId, T::Balance)>,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				known_xdns_records: Default::default(),
			}
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {

		}
	}
}

impl<T: Config> Pallet<T> {
	// Add public immutables and private mutables.
	#[allow(dead_code)]
	fn placeholder(origin: T::Origin) -> DispatchResult {
		let _sender = ensure_signed(origin)?;

		Ok(())
	}
}
