// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate Finality Verifier Pallet
//!
//! This pallet is an on-chain GRANDPA light client for Substrate based chains.
//!
//! This pallet achieves this by trustlessly verifying GRANDPA finality proofs on-chain. Once
//! verified, finalized headers are stored in the pallet, thereby creating a sparse header chain.
//! This sparse header chain can be used as a source of truth for other higher-level applications.
//!
//! The pallet is responsible for tracking GRANDPA validator set hand-offs. We only import headers
//! with justifications signed by the current validator set we know of. The header is inspected for
//! a `ScheduledChanges` digest item, which is then used to update to next validator set.
//!
//! Since this pallet only tracks finalized headers it does not deal with forks. Forks can only
//! occur if the GRANDPA validator set on the bridged chain is either colluding or there is a severe
//! bug causing resulting in an equivocation. Such events are outside of the scope of this pallet.
//! Shall the fork occur on the bridged chain governance intervention will be required to
//! re-initialize the bridge and track the right fork.

#![cfg_attr(not(feature = "std"), no_std)]
// Runtime-generated enums
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::weights::WeightInfo;
use sp_std::convert::TryInto;

use frame_support::pallet_prelude::*;
use frame_system::ensure_signed;

use sp_runtime::traits::Header as HeaderT;

pub use light_client_commons::{
    traits::LightClient,
    types::{Bytes, ShardId},
};

#[cfg(feature = "testing")]
pub mod mock;

/// Pallet containing weights for this pallet.
pub mod weights;

// #[cfg(feature = "runtime-benchmarks")]
// pub mod benchmarking;

// Re-export in crate namespace for `construct_runtime!`
pub use pallet::*;

use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// Weights gathered through benchmarking.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<T::BlockNumber> for Pallet<T, I> {}

    /// Optional pallet owner.
    ///
    /// Pallet owner has a right to halt all pallet operations and then resume it. If it is
    /// `None`, then there are no direct ways to halt/resume pallet operations, but other
    /// runtime methods may still be used to do that (i.e. democracy::referendum to update halt
    /// flag directly or call the `halt_operations`).
    #[pallet::storage]
    pub(super) type PalletOwnerMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ShardId, T::AccountId>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    pub(super) type IsHalted<T: Config<I>, I: 'static = ()> = StorageValue<_, bool, ValueQuery>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    pub(super) type IsHaltedMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ShardId, bool>;

    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// The submitted range is empty
        EmptyRangeSubmitted,
        /// The submitted range is larger the HeadersToStore, which is not permitted
        RangeToLarge,
        /// No finalized header was found in storage
        NoFinalizedHeader,
        /// The authority set in invalid
        InvalidAuthoritySet,
        /// The submitted GrandpaJustification is not valid
        InvalidGrandpaJustification,
        /// The header range linkage is not valid
        InvalidRangeLinkage,
        /// The linkage with the justified header is not valid
        InvalidJustificationLinkage,
        /// The parachain entry was not found in storage
        ParachainEntryNotFound,
        /// The relaychains storge root was not found. This implies the header is not available
        StorageRootNotFound,
        /// The inclusion data couldn't be decoded
        InclusionDataDecodeError,
        /// The submitted storage proof is invalid
        InvalidStorageProof,
        /// The event was not found in the specified block
        EventNotIncluded,
        /// The given bytes couldn't be decoded as a header
        HeaderDecodingError,
        /// The given bytes couldn't be decoded as header data
        HeaderDataDecodingError,
        /// The headers storage root doesn't map the supplied once
        StorageRootMismatch,
        /// The header couldn't be found in storage
        UnknownHeader,
        /// The events paramaters couldn't be decoded
        EventDecodingFailed,
        /// The side effect is not known for this vendor
        UnkownSideEffect,
        /// A forced change was detected, which is not supported
        UnsupportedScheduledChange,
        /// The pallet is currently halted
        Halted,
        /// The block height couldn't be converted
        BlockHeightConversionError,
    }
    // #[pallet::event]
    // #[pallet::generate_deposit(pub (super) fn deposit_event)]
    // pub enum Event<T: Config> {
    //     /// \[owner\]
    //     Eth2LightClientInitializedBy(T::AccountId),
    // }

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// Initialize the pallet with the given data.
        ///
        /// This function can only be called by the pallet owner.
        #[pallet::weight(<T as Config<I>>::WeightInfo::initialize())]
        pub fn initialize(origin: OriginFor<T>, _data: Bytes) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            Ok(().into())
        }
    }
}

// Catches missing feature flag
#[cfg(all(not(feature = "testing"), test))]
pub mod tests {

    #[test]
    fn panic_without_testing_feature() {
        panic!("Please use the feature testing when running tests.\n\nUse: cargo test --features testing\n\n");
    }
}

#[cfg(all(feature = "testing", test))]
mod tests {
    use super::*;
    use crate::mock::{
        run_test, test_header, test_header_range, test_header_with_correct_parent, AccountId,
        Origin, TestHeader, TestNumber, TestRuntime,
    };

    use codec::Encode;
    use frame_support::{assert_err, assert_noop, assert_ok};
    use sp_finality_grandpa::AuthorityId;
    use sp_runtime::{Digest, DigestItem, DispatchError};

    const MAIN_SHARD_ID: ShardId = 0;

    #[test]
    fn init_root_or_owner_origin_can_initialize_pallet() {
        assert_eq!(PalletOwnerMap::<TestRuntime>::get(MAIN_SHARD_ID), None);

        assert!(false);
    }

    #[test]
    fn can_register_with_valid_data_and_signer() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn cant_register_duplicate_shard_ids() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn cant_register_relaychain_as_non_root() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn init_can_only_initialize_pallet_once() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn pallet_owner_may_change_owner() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn pallet_may_be_halted_by_root() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn pallet_may_be_halted_by_owner() {
        run_test(|| {
            assert!(false);
        })
    }

    #[test]
    fn pallet_rejects_transactions_if_halted() {
        run_test(|| {
            assert!(false);
        })
    }
}
