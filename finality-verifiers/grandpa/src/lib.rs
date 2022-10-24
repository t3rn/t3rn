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
use bp_header_chain::{justification::GrandpaJustification, InitializationData};
use bp_runtime::{BlockNumberOf, Chain, ChainId, HashOf, HasherOf, HeaderOf};
use sp_std::convert::TryInto;

use finality_grandpa::voter_set::VoterSet;
use frame_support::{ensure, pallet_prelude::*, StorageHasher};
use frame_system::{ensure_signed, RawOrigin};
use num_traits::cast::AsPrimitive;
use sp_core::crypto::ByteArray;
use sp_finality_grandpa::{ConsensusLog, GRANDPA_ENGINE_ID};
use sp_runtime::traits::{BadOrigin, Header as HeaderT, Zero};
use sp_std::{vec, vec::Vec};

mod types;

use sp_trie::{read_trie_value, LayoutV1, StorageProof};
use types::GrandpaRegistrationData;

#[cfg(feature = "testing")]
pub mod mock;

pub mod bridges;
mod side_effects;
/// Pallet containing weights for this pallet.
pub mod weights;

use bridges::{
    header_chain as bp_header_chain, header_chain::ProofTriePointer, runtime as bp_runtime,
};

// #[cfg(feature = "runtime-benchmarks")]
// pub mod benchmarking;

// Re-export in crate namespace for `construct_runtime!`
pub use pallet::*;

/// Block number of the bridged chain.
pub type BridgedBlockNumber<T, I> = BlockNumberOf<<T as Config<I>>::BridgedChain>;
/// Block hash of the bridged chain.
pub type BridgedBlockHash<T, I> = HashOf<<T as Config<I>>::BridgedChain>;
/// Hasher of the bridged chain.
pub type BridgedBlockHasher<T, I> = HasherOf<<T as Config<I>>::BridgedChain>;
/// Header of the bridged chain.
pub type BridgedHeader<T, I> = HeaderOf<<T as Config<I>>::BridgedChain>;

use crate::{
    side_effects::decode_event,
    types::{InclusionData, Parachain, ParachainHeaderData, RelaychainHeaderData},
};
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_std::convert::TryFrom;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// The chain we are bridging to here.
        type BridgedChain: Chain;

        /// Maximal number of finalized headers to keep in the storage.
        ///
        /// The setting is there to prevent growing the on-chain state indefinitely. Note
        /// the setting does not relate to block numbers - we will simply keep as much items
        /// in the storage, so it doesn't guarantee any fixed timeframe for finality headers.
        #[pallet::constant]
        type HeadersToStore: Get<u32>;

        /// Weights gathered through benchmarking.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

    /// Hash of the header used to bootstrap the pallet.
    #[pallet::storage]
    #[pallet::getter(fn get_initial_hash_map)]
    pub(super) type InitialHashMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, BridgedBlockHash<T, I>>;

    /// Map of hashes of the best finalized header.
    #[pallet::storage]
    #[pallet::getter(fn get_bridged_block_hash)]
    pub type BestFinalizedMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, BridgedBlockHash<T, I>>;

    /// A ring buffer of imported hashes. Ordered by the insertion time.
    #[pallet::storage]
    #[pallet::getter(fn get_multi_imported_hashes)]
    pub(super) type MultiImportedHashes<T: Config<I>, I: 'static = ()> =
        StorageDoubleMap<_, Blake2_256, ChainId, Identity, u32, BridgedBlockHash<T, I>>;

    /// Current ring buffer position.
    #[pallet::storage]
    #[pallet::getter(fn get_multi_imported_hashes_pointer)]
    pub(super) type MultiImportedHashesPointer<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, u32>;

    /// Headers which have been imported into the pallet.
    #[pallet::storage]
    #[pallet::getter(fn get_multi_imported_headers)]
    pub type MultiImportedHeaders<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
        _,
        Blake2_256,
        ChainId,
        Identity,
        BridgedBlockHash<T, I>,
        BridgedHeader<T, I>,
    >;

    /// Roots (ExtrinsicsRoot + StateRoot) which have been imported into the pallet for a given gateway.
    #[pallet::storage]
    #[pallet::getter(fn get_imported_roots)]
    pub(super) type MultiImportedRoots<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
        _,
        Blake2_256,
        ChainId,
        Identity,
        BridgedBlockHash<T, I>,
        (BridgedBlockHash<T, I>, BridgedBlockHash<T, I>),
    >;

    #[pallet::storage]
    pub(super) type RelayChainId<T: Config<I>, I: 'static = ()> =
        StorageValue<_, ChainId, OptionQuery>;

    /// The current GRANDPA Authority set.
    #[pallet::storage]
    pub(super) type CurrentAuthoritySet<T: Config<I>, I: 'static = ()> =
        StorageValue<_, bp_header_chain::AuthoritySet, OptionQuery>;

    /// The current GRANDPA Authority set.
    #[pallet::storage]
    pub(super) type ParachainIdMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, Parachain>;

    /// Optional pallet owner.
    ///
    /// Pallet owner has a right to halt all pallet operations and then resume it. If it is
    /// `None`, then there are no direct ways to halt/resume pallet operations, but other
    /// runtime methods may still be used to do that (i.e. democracy::referendum to update halt
    /// flag directly or call the `halt_operations`).
    #[pallet::storage]
    pub(super) type PalletOwnerMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, T::AccountId>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    pub(super) type IsHalted<T: Config<I>, I: 'static = ()> = StorageValue<_, bool, ValueQuery>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    pub(super) type IsHaltedMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, bool>;

    /// Map of instance ids of gateways which are active
    #[pallet::storage]
    pub(super) type InstantiatedGatewaysMap<T: Config<I>, I: 'static = ()> =
        StorageValue<_, Vec<ChainId>, ValueQuery>;

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

    /// Add a header range for the relaychain
    ///
    /// It will use the underlying storage pallet to fetch information about the current
    /// authorities and best finalized header in order to verify that the header is finalized,
    /// and the corresponding range valid.
    ///
    /// If successful in verification, it will write the target range to the underlying storage
    /// pallet.
    pub(crate) fn submit_relaychain_headers<T: pallet::Config<I>, I>(
        gateway_id: ChainId,
        // seq vector of headers to be added.
        range: Vec<BridgedHeader<T, I>>,
        // The header with the highest height, signed in the justification
        signed_header: BridgedHeader<T, I>,
        // GrandpaJustification for the signed_header
        justification: GrandpaJustification<BridgedHeader<T, I>>,
    ) -> Result<Vec<u8>, DispatchError> {
        ensure!(!range.is_empty(), Error::<T, I>::EmptyRangeSubmitted);

        // °°°°° Implicit Check: °°°°°
        // range.len() < T::HeadersToStore::get() - ensures that we don't mess up our ring buffer
        // Since polkadot updates its authority set every 24h, this is implicitly ensured => Justification check would fail after 1/7th of max len

        // we get the latest header from storage
        let mut best_finalized = <MultiImportedHeaders<T, I>>::get(
            gateway_id,
            // Every time `BestFinalized` is updated `ImportedHeaders` is also updated. Therefore
            // `ImportedHeaders` must contain an entry for `BestFinalized`.
            <BestFinalizedMap<T, I>>::get(gateway_id).ok_or(Error::<T, I>::NoFinalizedHeader)?,
        )
        .ok_or(Error::<T, I>::NoFinalizedHeader)?;

        // °°°°° Explaination °°°°°
        // To be able to submit ranges of headers, we need to ensure a number of things.
        // 1. Ensure correct header linkage. All submitted headers must follow the linkage rule.
        // 2. As this is not PoW, we must ensure there is a valid GrandpaJustification for the last header of what we're submitting. This can be seen as ensuring the correct fork is selected
        // 3. The justification verifies a header that follows the linkage rule of the range

        // For efficiency we check the the justification first. If it's invalid, we can skip the rest
        let (signed_hash, signed_number) = (signed_header.hash(), signed_header.number());
        let authority_set =
            <CurrentAuthoritySet<T, I>>::get().ok_or(Error::<T, I>::InvalidAuthoritySet)?;

        let set_id = authority_set.set_id;
        // °°°°° Begin Check: #2 °°°°°
        verify_justification_single::<T, I>(
            &justification,
            signed_hash,
            *signed_number,
            authority_set,
            gateway_id,
        )?;
        // °°°°° Checked: #2 °°°°°°

        // check for authority set update and enact if available.
        let _enacted =
            try_enact_authority_change_single::<T, I>(&signed_header, set_id, gateway_id)?;

        // We get the latest buffer_index, which maps to the next header we can overwrite, and the index where we insert the verified header
        let mut buffer_index =
            <MultiImportedHashesPointer<T, I>>::get(gateway_id).unwrap_or_default();

        // °°°°° Begin Check: #1 °°°°°
        for header in range {
            if best_finalized.hash() == *header.parent_hash() {
                // write header to storage if correct
                write_and_clean_header_data::<T, I>(
                    gateway_id,
                    &mut buffer_index,
                    &header,
                    header.hash(),
                    false,
                )?;

                best_finalized = header;
            } else {
                // if anything fails here, noop!
                return Err(Error::<T, I>::InvalidRangeLinkage.into())
            }
        }
        // °°°°° Check Success: #1 °°°°°

        // °°°°° Begin Check: #3 °°°°°
        if best_finalized.hash() == *signed_header.parent_hash() {
            // write header to storage if correct
            write_and_clean_header_data::<T, I>(
                gateway_id,
                &mut buffer_index,
                &signed_header,
                signed_hash,
                true,
            )?;
        } else {
            return Err(Error::<T, I>::InvalidJustificationLinkage.into())
        }
        // °°°°° Check Success: #3 °°°°°
        // Proof success! Submitted header range valid

        // Update pointer
        <MultiImportedHashesPointer<T, I>>::insert(gateway_id, buffer_index);

        let height: usize = signed_number.as_();
        match u32::try_from(height) {
            Ok(number) => Ok(number.to_be_bytes().to_vec()),
            _ => Err(Error::<T, I>::BlockHeightConversionError.into()),
        }
    }

    /// Verify a target parachain header can be verified with its relaychains storage_proof
    ///
    /// It will use the underlying storage pallet to fetch the relaychains header, which is then used to verify the storage_proof
    ///
    /// If successful in verification, it will write the target header to the underlying storage
    /// pallet.
    ///
    pub(crate) fn submit_parachain_header<T: Config<I>, I>(
        gateway_id: ChainId,
        relay_block_hash: BridgedBlockHash<T, I>,
        range: Vec<BridgedHeader<T, I>>,
        proof: StorageProof,
    ) -> Result<Vec<u8>, DispatchError> {
        ensure!(!range.is_empty(), Error::<T, I>::EmptyRangeSubmitted);
        ensure!(
            range.len() < T::HeadersToStore::get().try_into().unwrap(),
            Error::<T, I>::RangeToLarge
        ); // this should be safe to do, as u32

        let mut best_finalized = <MultiImportedHeaders<T, I>>::get(
            gateway_id,
            // Every time `BestFinalized` is updated `ImportedHeaders` is also updated. Therefore
            // `ImportedHeaders` must contain an entry for `BestFinalized`.
            <BestFinalizedMap<T, I>>::get(gateway_id).ok_or(Error::<T, I>::NoFinalizedHeader)?,
        )
        .ok_or(Error::<T, I>::NoFinalizedHeader)?;

        // °°°°° Explaination °°°°°
        // To be able to submit ranges of headers, we need to ensure a number of things.
        // 1. Ensure the highest submitted header is verified in a Relaychain block
        // 2. Ensure correct header linkage. All submitted headers must follow the linkage rule.
        // 3. The highest submitted header follows the linkage rule of the range

        let parachain = <ParachainIdMap<T, I>>::try_get(gateway_id)
            .map_err(|_| Error::<T, I>::ParachainEntryNotFound)?;

        // °°°°° Begin Check: #1 °°°°°
        let signed_header: BridgedHeader<T, I> =
            verify_header_storage_proof::<T, I>(relay_block_hash, proof, parachain)?;
        // °°°°° Check Success: #1 °°°°°

        // We get the latest buffer_index, which maps to the next header we can overwrite, and the index where we insert the verified header
        let mut buffer_index =
            <MultiImportedHashesPointer<T, I>>::get(gateway_id).unwrap_or_default();

        // °°°°° Begin Check: #2 °°°°°
        for header in range {
            if best_finalized.hash() == *header.parent_hash() {
                // write header to storage if correct
                write_and_clean_header_data::<T, I>(
                    gateway_id,
                    &mut buffer_index,
                    &header,
                    header.hash(),
                    false,
                )?;

                best_finalized = header;
            } else {
                // if anything fails here, noop!
                return Err(Error::<T, I>::InvalidRangeLinkage.into())
            }
        }
        // °°°°° Check Success: #2 °°°°°

        // °°°°° Begin Check: #3 °°°°°
        if best_finalized.hash() == *signed_header.parent_hash() {
            // write header to storage if correct
            write_and_clean_header_data::<T, I>(
                gateway_id,
                &mut buffer_index,
                &signed_header,
                signed_header.hash(),
                true,
            )?;
        } else {
            return Err(Error::<T, I>::InvalidJustificationLinkage.into())
        }
        // °°°°° Check Success: #3 °°°°°
        // Proof success! Submitted header range valid

        // Update pointer
        <MultiImportedHashesPointer<T, I>>::insert(gateway_id, buffer_index);

        let height: usize = signed_header.number().as_();
        match u32::try_from(height) {
            Ok(number) => Ok(number.to_be_bytes().to_vec()),
            _ => Err(Error::<T, I>::BlockHeightConversionError.into()),
        }
    }

    /// Check the given header for a GRANDPA scheduled authority set change. If a change
    /// is found it will be enacted immediately.
    ///
    /// This function does not support forced changes, or scheduled changes with delays
    /// since these types of changes are indicitive of abnormal behaviour from GRANDPA.
    pub(crate) fn try_enact_authority_change_single<T: Config<I>, I: 'static>(
        header: &BridgedHeader<T, I>,
        current_set_id: sp_finality_grandpa::SetId,
        gateway_id: ChainId,
    ) -> Result<bool, sp_runtime::DispatchError> {
        let mut change_enacted = false;

        // We don't support forced changes - at that point governance intervention is required.
        ensure!(
            super::find_forced_change(header).is_none(),
            <Error<T, I>>::UnsupportedScheduledChange
        );

        if let Some(change) = super::find_scheduled_change(header) {
            // GRANDPA only includes a `delay` for forced changes, so this isn't valid.
            ensure!(
                change.delay == Zero::zero(),
                <Error<T, I>>::UnsupportedScheduledChange
            );

            // TODO [#788]: Stop manually increasing the `set_id` here.
            let next_authorities = bp_header_chain::AuthoritySet {
                authorities: change.next_authorities,
                set_id: current_set_id + 1,
            };

            // Since our header schedules a change and we know the delay is 0, it must also enact
            // the change.
            <CurrentAuthoritySet<T, I>>::put(&next_authorities);
            change_enacted = true;

            log::info!(
				"Transitioned from authority set {} to {}! New authorities are: {:?} for gateway: {:?}",
				current_set_id,
				current_set_id + 1,
				next_authorities,
				gateway_id,
			);
        };

        Ok(change_enacted)
    }

    /// Verify a GRANDPA justification (finality proof) for a given header.
    ///
    /// Will use the GRANDPA current authorities known to the pallet.
    ///
    /// If succesful it returns the decoded GRANDPA justification so we can refund any weight which
    /// was overcharged in the initial call.
    pub(crate) fn verify_justification_single<T: Config<I>, I: 'static>(
        justification: &GrandpaJustification<BridgedHeader<T, I>>,
        hash: BridgedBlockHash<T, I>,
        number: BridgedBlockNumber<T, I>,
        authority_set: bp_header_chain::AuthoritySet,
        _gateway_id: ChainId,
    ) -> Result<(), Error<T, I>> {
        use bp_header_chain::justification::verify_justification;

        let voter_set =
            VoterSet::new(authority_set.authorities).ok_or(Error::<T, I>::InvalidAuthoritySet)?;
        let set_id = authority_set.set_id;

        verify_justification::<BridgedHeader<T, I>>(
            (hash, number),
            set_id,
            &voter_set,
            justification,
        )
        .map_err(|_| {
            log::error!("Received invalid justification for {:?}", hash);
            Error::<T, I>::InvalidGrandpaJustification
        })
    }

    /// Since this writes to storage with no real checks this should only be used in functions that
    /// were called by a trusted origin.
    pub(crate) fn initialize_relay_chain<T: Config<I>, I: 'static>(
        init_params: super::InitializationData<BridgedHeader<T, I>>,
        owner: T::AccountId,
    ) -> Result<(), &'static str> {
        can_init_relay_chain::<T, I>()?;

        let super::InitializationData {
            header,
            authority_list,
            set_id,
            is_halted,
            gateway_id,
        } = init_params;

        let initial_hash = header.hash();
        // Store header stuff
        <InitialHashMap<T, I>>::insert(gateway_id, initial_hash);
        <BestFinalizedMap<T, I>>::insert(gateway_id, initial_hash);
        <MultiImportedHeaders<T, I>>::insert(gateway_id, initial_hash, header);
        <MultiImportedHashesPointer<T, I>>::insert(gateway_id, 0); // one ahead of first value
        <RelayChainId<T, I>>::put(gateway_id);
        let authority_set = bp_header_chain::AuthoritySet::new(authority_list, set_id);
        <CurrentAuthoritySet<T, I>>::put(authority_set);

        // Other configs
        <IsHaltedMap<T, I>>::insert(gateway_id, is_halted);
        <PalletOwnerMap<T, I>>::insert(gateway_id, owner);
        <InstantiatedGatewaysMap<T, I>>::mutate(|gateways| {
            gateways.push(gateway_id);
            gateways.len() + 1
        });

        Ok(())
    }

    /// Since this writes to storage with no real checks this should only be used in functions that
    /// were called by a trusted origin.
    pub(crate) fn initialize_para_chain<T: Config<I>, I: 'static>(
        header: BridgedHeader<T, I>,
        is_halted: bool,
        gateway_id: ChainId,
        owner: T::AccountId,
        parachain: Parachain,
    ) -> Result<(), &'static str> {
        can_init_para_chain::<T, I>(&parachain)?;
        let initial_hash = header.hash();
        <InitialHashMap<T, I>>::insert(gateway_id, initial_hash);
        <BestFinalizedMap<T, I>>::insert(gateway_id, initial_hash);
        <MultiImportedHeaders<T, I>>::insert(gateway_id, initial_hash, header);
        <MultiImportedHashesPointer<T, I>>::insert(gateway_id, 1u32);

        <IsHaltedMap<T, I>>::insert(gateway_id, is_halted);
        <ParachainIdMap<T, I>>::insert(gateway_id, parachain);
        <PalletOwnerMap<T, I>>::insert(gateway_id, owner);
        <InstantiatedGatewaysMap<T, I>>::mutate(|gateways| {
            gateways.push(gateway_id);
            gateways.len() + 1
        });

        Ok(())
    }

    /// removes old header data based on ring buffer logic. Adds new header data, updates the ring buffer entry and increments buffer index
    pub(crate) fn write_and_clean_header_data<T: Config<I>, I: 'static>(
        gateway_id: ChainId,
        buffer_index: &mut u32,
        header: &BridgedHeader<T, I>,
        hash: BridgedBlockHash<T, I>,
        is_signed_header: bool,
    ) -> Result<(), &'static str> {
        // If we find an entry to overwrite, do so.
        if let Ok(hash) = <MultiImportedHashes<T, I>>::try_get(
            gateway_id,
            *buffer_index, // can't overflow because of incrementation logic
        ) {
            <MultiImportedHeaders<T, I>>::remove(gateway_id, hash);
            <MultiImportedRoots<T, I>>::remove(gateway_id, hash);
        }

        // Once deleted, we add the new header
        <MultiImportedHeaders<T, I>>::insert(gateway_id, hash, header.clone());
        <MultiImportedHashes<T, I>>::insert(gateway_id, *buffer_index, hash);
        <MultiImportedRoots<T, I>>::insert(
            gateway_id,
            hash,
            (header.extrinsics_root(), header.state_root()),
        );

        // if this is the signed header, we set best finalized
        if is_signed_header {
            <BestFinalizedMap<T, I>>::insert(gateway_id, hash);
        }

        *buffer_index = (*buffer_index + 1) % T::HeadersToStore::get(); // prevents overflows
        Ok(())
    }

    /// Ensure that the pallet is in operational mode (not halted).
    pub fn ensure_operational_single<T: Config<I>, I: 'static>(
        gateway_id: ChainId,
    ) -> Result<(), Error<T, I>> {
        if <IsHaltedMap<T, I>>::get(gateway_id)
            .expect("Is halted prop is should have been set before during initialize")
        {
            Err(<Error<T, I>>::Halted)
        } else {
            Ok(())
        }
    }
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    // /// Get the best finalized header the pallet knows of.
    // ///
    // /// Returns a dummy header if there is no best header. This can only happen
    // /// if the pallet has not been initialized yet.
    pub fn best_finalized_map(gateway_id: ChainId) -> BridgedHeader<T, I> {
        let hash = <BestFinalizedMap<T, I>>::get(gateway_id).unwrap_or_default();
        <MultiImportedHeaders<T, I>>::get(gateway_id, hash).unwrap_or_else(|| {
            <BridgedHeader<T, I>>::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        })
    }

    /// Check if a particular header is known to the bridge pallet.
    pub fn is_known_header(hash: BridgedBlockHash<T, I>, gateway_id: ChainId) -> bool {
        <MultiImportedHeaders<T, I>>::contains_key(gateway_id, hash)
    }

    /// Verify that the passed storage proof is valid, given it is crafted using
    /// known finalized header. If the proof is valid, then the `parse` callback
    /// is called and the function returns its result.
    pub fn parse_finalized_storage_proof<R>(
        hash: BridgedBlockHash<T, I>,
        storage_proof: sp_trie::StorageProof,
        parse: impl FnOnce(bp_runtime::StorageProofChecker<BridgedBlockHasher<T, I>>) -> R,
        gateway_id: ChainId,
    ) -> Result<R, DispatchError> {
        let header = <MultiImportedHeaders<T, I>>::get(gateway_id, hash)
            .ok_or(Error::<T, I>::UnknownHeader)?;
        let storage_proof_checker =
            bp_runtime::StorageProofChecker::new(*header.state_root(), storage_proof)
                .map_err(|_| Error::<T, I>::StorageRootMismatch)?;

        Ok(parse(storage_proof_checker))
    }

    pub fn initialize(
        origin: T::Origin,
        gateway_id: ChainId,
        encoded_registration_data: Vec<u8>,
    ) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin, gateway_id)?;
        let init_allowed = !<BestFinalizedMap<T, I>>::contains_key(gateway_id);
        ensure!(init_allowed, "Already initialized");
        let registration_data: GrandpaRegistrationData<T::AccountId> =
            Decode::decode(&mut &*encoded_registration_data)
                .map_err(|_| "Registration data decoding error!")?;
        let header: BridgedHeader<T, I> = Decode::decode(&mut &registration_data.first_header[..])
            .map_err(|_| "Decoding error: received GenericPrimitivesHeader -> CurrentHeader<T>")?;
        match registration_data.parachain {
            Some(parachain) => initialize_para_chain::<T, I>(
                header,
                false,
                gateway_id,
                registration_data.owner,
                parachain,
            ),
            None => {
                ensure!(
                    registration_data.authorities.is_some()
                        && registration_data.authority_set_id.is_some(),
                    "Relaychain parameters missing"
                );

                let init_data = bp_header_chain::InitializationData {
                    header,
                    authority_list: registration_data
                        .authorities
                        .unwrap()
                        .iter()
                        .map(|id| {
                            sp_finality_grandpa::AuthorityId::from_slice(&id.encode()).unwrap()
                        }) // not sure why this is still needed
                        .map(|authority| (authority, 1))
                        .collect::<Vec<_>>(),
                    set_id: registration_data.authority_set_id.unwrap(),
                    is_halted: false,
                    gateway_id,
                };
                initialize_relay_chain::<T, I>(init_data, registration_data.owner)
            },
        }
    }

    /// Change `PalletOwner`.
    ///
    /// May only be called either by root, or by `PalletOwner`.
    pub fn set_owner(
        origin: T::Origin,
        gateway_id: ChainId,
        encoded_new_owner: Vec<u8>,
    ) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin, gateway_id)?;
        let new_owner: Option<T::AccountId> =
            Decode::decode(&mut &*encoded_new_owner).map_err(|_| "New Owner decoding error")?;

        match new_owner {
            Some(new_owner) => {
                PalletOwnerMap::<T, I>::insert(gateway_id, &new_owner);
                log::info!("Setting pallet Owner to: {:?}", new_owner);
            },
            None => {
                PalletOwnerMap::<T, I>::remove(gateway_id);
                log::info!("Removed Owner of pallet.");
            },
        }

        Ok(())
    }

    /// Halt or resume all pallet operations.
    ///
    /// May only be called either by root, or by `PalletOwner`.
    pub fn set_operational(
        origin: T::Origin,
        operational: bool,
        gateway_id: ChainId,
    ) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin, gateway_id)?;
        <IsHaltedMap<T, I>>::insert(gateway_id, !operational);

        if operational {
            // If a gateway shall be operational the pallet must be too.
            <IsHalted<T, I>>::put(false);
            log::info!("Resuming pallet operations.");
        } else {
            log::info!("Stopping pallet operations.");
        }

        Ok(())
    }

    pub fn submit_headers(
        origin: OriginFor<T>,
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError> {
        ensure_operational_single::<T, I>(gateway_id)?;
        ensure_signed(origin)?;
        if Some(gateway_id) == <RelayChainId<T, I>>::get() {
            let data: RelaychainHeaderData<BridgedHeader<T, I>> =
                Decode::decode(&mut &*encoded_header_data)
                    .map_err(|_| Error::<T, I>::HeaderDataDecodingError)?;

            submit_relaychain_headers::<T, I>(
                gateway_id,
                data.range,
                data.signed_header,
                data.justification,
            )
        } else {
            let data: ParachainHeaderData<BridgedHeader<T, I>> =
                Decode::decode(&mut &*encoded_header_data)
                    .map_err(|_| Error::<T, I>::HeaderDataDecodingError)?;

            submit_parachain_header::<T, I>(
                gateway_id,
                data.relay_block_hash,
                data.range,
                data.proof,
            )
        }
    }

    pub fn confirm_and_decode_payload_params(
        gateway_id: ChainId,
        encoded_inclusion_data: Vec<u8>,
        submission_target_height: Vec<u8>,
        value_abi_unsigned_type: &[u8],
        side_effect_id: [u8; 4],
    ) -> Result<(Vec<Vec<u8>>, Vec<u8>), DispatchError> {
        let inclusion_data: InclusionData<BridgedHeader<T, I>> =
            Decode::decode(&mut &*encoded_inclusion_data)
                .map_err(|_| Error::<T, I>::InclusionDataDecodeError)?;

        // ensures old equal side_effects can't be replayed
        executed_after_creation::<T, I>(gateway_id, submission_target_height)?;

        match &side_effect_id {
            b"tran" => verify_event_storage_proof::<T, I>(
                gateway_id,
                inclusion_data,
                value_abi_unsigned_type,
                side_effect_id,
            ),
            _ => unimplemented!(),
        }
    }

    pub fn get_latest_finalized_header(gateway_id: ChainId) -> Option<Vec<u8>> {
        if let Some(header_hash) = <BestFinalizedMap<T, I>>::get(gateway_id) {
            return Some(header_hash.encode())
        }
        None
    }

    pub fn get_latest_finalized_height(gateway_id: ChainId) -> Option<Vec<u8>> {
        if let Some(header_hash) = <BestFinalizedMap<T, I>>::get(gateway_id) {
            if let Some(header) = <MultiImportedHeaders<T, I>>::get(gateway_id, header_hash) {
                return Some(header.number().encode())
            } else {
                None
            }
        } else {
            Some(vec![0]) // ToDo this is here more for testing.
        }
    }
}

/// Verifies a given storage proof. Returns the encoded entry that is proven
pub(crate) fn verify_storage_proof<T: Config<I>, I: 'static>(
    gateway_id: ChainId,
    block_hash: BridgedBlockHash<T, I>,
    key: Vec<u8>,
    proof: StorageProof,
    trie_type: ProofTriePointer,
) -> Result<Vec<u8>, &'static str> {
    let root = get_header_roots::<T, I>(block_hash, gateway_id, trie_type)?;
    let db = proof.into_memory_db::<BridgedBlockHasher<T, I>>();
    match read_trie_value::<LayoutV1<BridgedBlockHasher<T, I>>, _>(&db, &root, key.as_ref()) {
        Ok(Some(value)) => Ok(value),
        _ => Err(Error::<T, I>::InvalidStorageProof.into()),
    }
}

/// returns the specified header root from a specific header
pub(crate) fn get_header_roots<T: pallet::Config<I>, I>(
    block_hash: BridgedBlockHash<T, I>,
    gateway_id: ChainId,
    trie_type: ProofTriePointer,
) -> Result<BridgedBlockHash<T, I>, DispatchError> {
    let (extrinsics_root, storage_root): (BridgedBlockHash<T, I>, BridgedBlockHash<T, I>) =
        <MultiImportedRoots<T, I>>::get(gateway_id, block_hash)
            .ok_or(Error::<T, I>::StorageRootNotFound)?;
    match trie_type {
        ProofTriePointer::State => Ok(storage_root),
        ProofTriePointer::Transaction => Ok(extrinsics_root),
        ProofTriePointer::Receipts => Ok(storage_root),
    }
}

pub(crate) fn find_scheduled_change<H: HeaderT>(
    header: &H,
) -> Option<sp_finality_grandpa::ScheduledChange<H::Number>> {
    use sp_runtime::generic::OpaqueDigestItemId;

    let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

    let filter_log = |log: ConsensusLog<H::Number>| match log {
        ConsensusLog::ScheduledChange(change) => Some(change),
        _ => None,
    };

    // find the first consensus digest with the right ID which converts to
    // the right kind of consensus log.
    header
        .digest()
        .convert_first(|l| l.try_to(id).and_then(filter_log))
}

/// Ensure that the origin is either root, or `PalletOwner`.
fn ensure_owner_or_root_single<T: Config<I>, I: 'static>(
    origin: T::Origin,
    gateway_id: ChainId,
) -> Result<(), &'static str> {
    match origin.into() {
        Ok(RawOrigin::Root) => Ok(()),
        Ok(RawOrigin::Signed(ref signer))
            if <PalletOwnerMap<T, I>>::contains_key(gateway_id)
                && Some(signer) == <PalletOwnerMap<T, I>>::get(gateway_id).as_ref() =>
            Ok(()),
        _ => Err(BadOrigin.into()),
    }
}

/// Ensure that no relaychain has been set so far. Relaychains are unique
fn can_init_relay_chain<T: Config<I>, I: 'static>() -> Result<(), &'static str> {
    match <RelayChainId<T, I>>::exists() {
        true => Err("Duplicate relaychain"), // we have a relaychain registered already
        false => Ok(()),                     // is first relaychain
    }
}

/// Ensure that no relaychain has been set so far. Relaychains are unique
fn can_init_para_chain<T: Config<I>, I: 'static>(
    parachain: &Parachain,
) -> Result<(), &'static str> {
    match <RelayChainId<T, I>>::get() {
        Some(relay_chain_id) => {
            // we have a relaychain setup
            match relay_chain_id == parachain.relay_chain_id {
                true => Ok(()), // registering relachchain_id matches the stored one
                false => Err("Invalid relaychainId"), // wrong relaychain_id was passed
            }
        },
        None => Err("No relaychain"), // we have no relaychain setup
    }
}

/// Ensure that the SideEffect was executed after it was created.
fn executed_after_creation<T: Config<I>, I: 'static>(
    gateway_id: ChainId,
    submission_target_height: Vec<u8>,
) -> Result<(), &'static str> {
    let submission_target: BridgedBlockNumber<T, I> =
        Decode::decode(&mut &*submission_target_height).unwrap();
    if let Some(header_hash) = <BestFinalizedMap<T, I>>::get(gateway_id) {
        if let Some(header) = <MultiImportedHeaders<T, I>>::get(gateway_id, header_hash) {
            if submission_target < *header.number() {
                return Ok(())
            }
            Err("Transaction executed before SideEffect creation")
        } else {
            Err("No gateway header found") // this shouldn't be possible tom happen
        }
    } else {
        Err("No gateway header found") // this shouldn't be possible tom happen
    }
}

/// Checks the given header for a consensus digest signalling a **forced** scheduled change and
/// extracts it.
pub(crate) fn find_forced_change<H: HeaderT>(
    header: &H,
) -> Option<(H::Number, sp_finality_grandpa::ScheduledChange<H::Number>)> {
    use sp_runtime::generic::OpaqueDigestItemId;

    let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

    let filter_log = |log: ConsensusLog<H::Number>| match log {
        ConsensusLog::ForcedChange(delay, change) => Some((delay, change)),
        _ => None,
    };

    // find the first consensus digest with the right ID which converts to
    // the right kind of consensus log.
    header
        .digest()
        .convert_first(|l| l.try_to(id).and_then(filter_log))
}

pub(crate) fn verify_event_storage_proof<T: Config<I>, I: 'static>(
    gateway_id: ChainId,
    inclusion_data: InclusionData<BridgedHeader<T, I>>,
    value_abi_unsigned_type: &[u8],
    side_effect_id: [u8; 4],
) -> Result<(Vec<Vec<u8>>, Vec<u8>), DispatchError> {
    let InclusionData {
        encoded_payload,
        proof,
        block_hash,
    } = inclusion_data;

    // storage key for System_Events
    let key: Vec<u8> = [
        38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 128, 212, 30, 94,
        22, 5, 103, 101, 188, 132, 97, 133, 16, 114, 201, 215,
    ]
    .to_vec();
    let verified_block_events = verify_storage_proof::<T, I>(
        gateway_id,
        block_hash,
        key,
        proof,
        ProofTriePointer::Receipts,
    )?;

    // the problem here is that in substrates current design its not possible to prove the inclusion of a single event, only all events of a block
    // https://github.com/paritytech/substrate/issues/11216
    ensure!(
        is_sub(verified_block_events.as_slice(), encoded_payload.as_slice()),
        Error::<T, I>::EventNotIncluded
    );

    decode_event::<T, I>(&side_effect_id, encoded_payload, value_abi_unsigned_type)
}

pub(crate) fn verify_header_storage_proof<T: Config<I>, I: 'static>(
    relay_block_hash: BridgedBlockHash<T, I>,
    proof: StorageProof,
    parachain: Parachain,
) -> Result<BridgedHeader<T, I>, DispatchError> {
    // partial StorageKey for Paras_Heads. We now need to append the parachain_id as LE-u32 to generate the parachains StorageKey
    // This is a bit unclean, but it makes no sense to hash the StorageKey for each exec
    let mut key: Vec<u8> = [
        205, 113, 11, 48, 189, 46, 171, 3, 82, 221, 204, 38, 65, 122, 161, 148, 27, 60, 37, 47,
        203, 41, 216, 142, 255, 79, 61, 229, 222, 68, 118, 195,
    ]
    .to_vec();
    let mut arg = Twox64Concat::hash(parachain.id.encode().as_ref());
    key.append(&mut arg); // complete storage key

    // ToDo not very concise
    let encoded_header_vec = verify_storage_proof::<T, I>(
        parachain.relay_chain_id,
        relay_block_hash,
        key,
        proof,
        ProofTriePointer::State,
    )?;
    let encoded_header: Vec<u8> = Decode::decode(&mut &encoded_header_vec[..])
        .map_err(|_| Error::<T, I>::HeaderDecodingError)?;
    let header: BridgedHeader<T, I> =
        Decode::decode(&mut &*encoded_header).map_err(|_| Error::<T, I>::HeaderDecodingError)?;
    Ok(header)
}

pub(crate) fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true
        }
        haystack = &haystack[1..];
    }
    false
}

/// (Re)initialize bridge with given header for using it in `pallet-bridge-messages` benchmarks.
#[cfg(feature = "runtime-benchmarks")]
pub fn initialize_for_benchmarks<T: Config<I>, I: 'static>(header: BridgedHeader<T, I>) {
    initialize_single_bridge::<T, I>(InitializationData {
        header,
        authority_list: sp_std::vec::Vec::new(), // we don't verify any proofs in external benchmarks
        set_id: 0,
        is_halted: false,
        gateway_id: *b"gate",
    });
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
    use bp_runtime::ChainId;
    use bridges::{
        header_chain as bp_header_chain, runtime as bp_runtime,
        test_utils::{
            authorities, authority_list, make_default_justification, make_justification_for_header,
            JustificationGeneratorParams, ALICE, BOB, DAVE,
        },
    };

    use codec::Encode;
    use frame_support::{assert_err, assert_noop, assert_ok};
    use sp_finality_grandpa::AuthorityId;
    use sp_runtime::{Digest, DigestItem, DispatchError};

    use crate::types::RelaychainHeaderData;

    fn initialize_relaychain(
        origin: Origin,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        let genesis = test_header_with_correct_parent(0, None);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: Some(authorities()),
            first_header: genesis.encode(),
            authority_set_id: Some(1),
            owner: 1u64,
            parachain: None,
        };

        initialize_custom_relaychain(origin, *b"pdot", init_data)
    }

    fn initialize_named_relaychain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: Some(authorities()),
            first_header: genesis.encode(),
            authority_set_id: Some(1),
            owner: 1u64,
            parachain: None,
        };

        initialize_custom_relaychain(origin, gateway_id, init_data)
    }

    fn initialize_custom_relaychain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: GrandpaRegistrationData<AccountId>,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        Pallet::<TestRuntime>::initialize(origin, gateway_id, init_data.encode()).map(|_| init_data)
    }

    fn initialize_parachain(
        origin: Origin,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: None,
            first_header: genesis.encode(),
            authority_set_id: None,
            owner: 1u64,
            parachain: Some(Parachain {
                relay_chain_id: *b"pdot",
                id: 0,
            }),
        };

        initialize_custom_parachain(origin, *b"moon", init_data)
    }

    fn initialize_named_parachain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: None,
            first_header: genesis.encode(),
            authority_set_id: None,
            owner: 1u64,
            parachain: Some(Parachain {
                relay_chain_id: *b"pdot",
                id: 0,
            }),
        };

        initialize_custom_parachain(origin, gateway_id, init_data)
    }

    fn initialize_custom_parachain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: GrandpaRegistrationData<AccountId>,
    ) -> Result<GrandpaRegistrationData<AccountId>, &'static str> {
        Pallet::<TestRuntime>::initialize(origin, gateway_id, init_data.encode()).map(|_| init_data)
    }

    fn submit_headers(from: u8, to: u8) -> Result<RelaychainHeaderData<TestHeader>, &'static str> {
        let headers: Vec<TestHeader> = test_header_range(to.into());
        let signed_header: &TestHeader = headers.last().unwrap();
        let justification = make_default_justification(&signed_header.clone());
        let range: Vec<TestHeader> = headers[from.into()..to.into()].to_vec();

        let default_gateway: ChainId = *b"pdot";
        let data = RelaychainHeaderData::<TestHeader> {
            signed_header: signed_header.clone(),
            range,
            justification,
        };

        Pallet::<TestRuntime>::submit_headers(Origin::signed(1), default_gateway, data.encode())?;

        Ok(data)
    }

    fn next_block() {
        use frame_support::traits::OnInitialize;

        let current_number = frame_system::Pallet::<TestRuntime>::block_number();
        frame_system::Pallet::<TestRuntime>::set_block_number(current_number + 1);
        let _ = <Pallet<TestRuntime> as OnInitialize<
            <TestRuntime as frame_system::Config>::BlockNumber,
        >>::on_initialize(current_number);
    }

    fn change_log(delay: u64) -> Digest {
        let consensus_log =
            ConsensusLog::<TestNumber>::ScheduledChange(sp_finality_grandpa::ScheduledChange {
                next_authorities: vec![(ALICE.into(), 1), (BOB.into(), 1)],
                delay,
            });

        Digest {
            logs: vec![DigestItem::Consensus(
                GRANDPA_ENGINE_ID,
                consensus_log.encode(),
            )],
        }
    }

    fn forced_change_log(delay: u64) -> Digest {
        let consensus_log = ConsensusLog::<TestNumber>::ForcedChange(
            delay,
            sp_finality_grandpa::ScheduledChange {
                next_authorities: vec![(ALICE.into(), 1), (BOB.into(), 1)],
                delay,
            },
        );

        Digest {
            logs: vec![DigestItem::Consensus(
                GRANDPA_ENGINE_ID,
                consensus_log.encode(),
            )],
        }
    }

    #[test]
    fn init_root_or_owner_origin_can_initialize_pallet() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            assert_noop!(initialize_relaychain(Origin::signed(1)), "Bad origin");
            assert_ok!(initialize_relaychain(Origin::root()));

            // Reset storage so we can initialize the pallet again
            BestFinalizedMap::<TestRuntime>::remove(default_gateway);
            RelayChainId::<TestRuntime>::kill();
            PalletOwnerMap::<TestRuntime>::insert(default_gateway, 2);
            assert_ok!(initialize_relaychain(Origin::signed(2)));
        })
    }

    #[test]
    fn can_register_with_valid_data_and_signer() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_ok!(initialize_parachain(Origin::root()));
        })
    }

    #[test]
    fn cant_register_duplicate_gateway_ids() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_err!(initialize_relaychain(Origin::root()), "Already initialized");
            assert_err!(
                initialize_named_parachain(Origin::root(), *b"pdot"),
                "Already initialized"
            );
        })
    }

    #[test]
    fn cant_register_duplicate_parachain() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_noop!(
                initialize_named_relaychain(Origin::root(), *b"roco"),
                "Duplicate relaychain"
            );
        })
    }

    #[test]
    fn cant_register_parachain_without_relaychain() {
        run_test(|| {
            assert_noop!(initialize_parachain(Origin::root()), "No relaychain");
        })
    }

    #[test]
    fn cant_register_parachain_without_wrong_relaychain_id() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));

            let genesis = test_header(0);
            let init_data = GrandpaRegistrationData::<AccountId> {
                authorities: None,
                first_header: genesis.encode(),
                authority_set_id: None,
                owner: 1,
                parachain: Some(Parachain {
                    relay_chain_id: *b"roco",
                    id: 0,
                }),
            };
            assert_noop!(
                initialize_custom_parachain(Origin::root(), *b"moon", init_data),
                "Invalid relaychainId"
            );
        })
    }

    #[test]
    fn cant_register_relaychain_as_non_root() {
        run_test(|| {
            assert_err!(initialize_relaychain(Origin::signed(1)), "Bad origin");
        })
    }

    #[test]
    fn cant_register_relaychain_with_missing_authorities() {
        run_test(|| {
            let genesis = test_header(0);
            let init_data = GrandpaRegistrationData::<AccountId> {
                authorities: None,
                first_header: genesis.encode(),
                authority_set_id: Some(1),
                owner: 1,
                parachain: None,
            };
            assert_noop!(
                initialize_custom_relaychain(Origin::root(), *b"pdot", init_data),
                "Relaychain parameters missing"
            );
        })
    }

    #[test]
    fn cant_register_relaychain_with_missing_authority_set_id() {
        run_test(|| {
            let genesis = test_header(0);
            let init_data = GrandpaRegistrationData::<AccountId> {
                authorities: Some(authorities()),
                first_header: genesis.encode(),
                authority_set_id: None,
                owner: 1,
                parachain: None,
            };
            assert_noop!(
                initialize_custom_relaychain(Origin::root(), *b"pdot", init_data),
                "Relaychain parameters missing"
            );
        })
    }

    #[test]
    fn cant_register_parachain_as_non_root() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_noop!(initialize_parachain(Origin::signed(1)), "Bad origin");
        })
    }

    #[test]
    fn init_storage_entries_are_correctly_initialized() {
        let default_gateway: ChainId = *b"pdot";
        let header = test_header(0);
        run_test(|| {
            assert_eq!(BestFinalizedMap::<TestRuntime>::get(default_gateway), None,);
            assert_eq!(
                Pallet::<TestRuntime>::best_finalized_map(default_gateway),
                test_header(0)
            );

            let _init_data = initialize_relaychain(Origin::root()).unwrap();

            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                header.hash()
            ));
            assert_eq!(
                BestFinalizedMap::<TestRuntime>::get(default_gateway),
                Some(header.hash())
            );
            assert_eq!(
                CurrentAuthoritySet::<TestRuntime>::get()
                    .unwrap()
                    .authorities,
                authority_list()
            );
            assert_eq!(
                IsHaltedMap::<TestRuntime>::get(default_gateway),
                Some(false)
            );
        })
    }

    #[test]
    fn init_can_only_initialize_pallet_once() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            assert_noop!(initialize_relaychain(Origin::root()), "Already initialized");
        })
    }

    #[test]
    fn pallet_owner_may_change_owner() {
        run_test(|| {
            let default_gateway: ChainId = *b"pdot";

            assert_ok!(Pallet::<TestRuntime>::set_owner(
                Origin::root(),
                default_gateway,
                Some(1u64).encode(),
            ));
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(2), false, default_gateway),
                DispatchError::BadOrigin,
            );
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                false,
                default_gateway
            ));

            let owner: Option<AccountId> = None;
            assert_ok!(Pallet::<TestRuntime>::set_owner(
                Origin::signed(1),
                default_gateway,
                owner.encode(),
            ));
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true, default_gateway),
                DispatchError::BadOrigin,
            );
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(2), true, default_gateway),
                DispatchError::BadOrigin,
            );
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                true,
                default_gateway
            ));
        });
    }

    #[test]
    fn pallet_may_be_halted_by_root() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                false,
                default_gateway
            ));
            assert_noop!(submit_headers(1, 3), "Halted");
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                true,
                default_gateway
            ));
        });
    }

    #[test]
    fn pallet_may_be_halted_by_owner() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            PalletOwnerMap::<TestRuntime>::insert(default_gateway, 2);

            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                false,
                default_gateway
            ));
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                true,
                default_gateway
            ));

            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), false, default_gateway),
                DispatchError::BadOrigin,
            );
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true, default_gateway),
                DispatchError::BadOrigin,
            );

            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                false,
                default_gateway
            ));
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true, default_gateway),
                DispatchError::BadOrigin,
            );
        });
    }

    #[test]
    fn pallet_rejects_transactions_if_halted() {
        run_test(|| {
            let gateway_a: ChainId = *b"pdot";
            let _ = initialize_relaychain(Origin::root());
            <IsHaltedMap<TestRuntime>>::insert(gateway_a, true);

            assert_noop!(submit_headers(1, 3), "Halted",);
        })
    }

    #[test]
    fn succesfully_imports_headers_with_valid_finality() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let data = submit_headers(1, 3).unwrap();

            assert_eq!(
                <BestFinalizedMap<TestRuntime>>::get(default_gateway),
                Some(data.signed_header.hash())
            );
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                data.signed_header.hash()
            ));
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                data.range[0].hash()
            ));
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                data.range[1].hash()
            ));
        })
    }

    #[test]
    fn reject_header_range_gap() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            assert_noop!(
                submit_headers(2, 5),
                Error::<TestRuntime>::InvalidRangeLinkage
            );
            assert_ok!(submit_headers(1, 5));
            assert_noop!(
                submit_headers(5, 10),
                Error::<TestRuntime>::InvalidRangeLinkage
            );
            assert_ok!(submit_headers(6, 10));
        })
    }

    #[test]
    fn reject_range_with_invalid_range_linkage() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            assert_ok!(submit_headers(1, 5));

            let headers: Vec<TestHeader> = test_header_range(10);
            let signed_header: &TestHeader = headers.last().unwrap();
            let justification = make_default_justification(&signed_header.clone());
            let mut range: Vec<TestHeader> = headers[6..10].to_vec();
            range[1] = range[2].clone();

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                Error::<TestRuntime>::InvalidRangeLinkage
            );
        })
    }

    #[test]
    fn reject_range_with_invalid_grandpa_linkage() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            assert_ok!(submit_headers(1, 5));

            let headers: Vec<TestHeader> = test_header_range(10);
            let signed_header: &TestHeader = headers.last().unwrap();

            let justification = make_default_justification(&signed_header.clone());
            // one header is missing -> Grandpa linkage invalid
            let range: Vec<TestHeader> = headers[6..9].to_vec();

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                Error::<TestRuntime>::InvalidJustificationLinkage
            );
        })
    }

    #[test]
    fn rejects_justification_that_skips_authority_set_transition() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            let headers: Vec<TestHeader> = test_header_range(5);
            let signed_header: &TestHeader = headers.last().unwrap();
            let mut range: Vec<TestHeader> = headers[1..5].to_vec();
            range.reverse();

            let params = JustificationGeneratorParams::<TestHeader> {
                set_id: 2,
                header: signed_header.clone(),
                ..Default::default()
            };
            let justification = make_justification_for_header(params);

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            let default_gateway: ChainId = *b"pdot";

            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                Error::<TestRuntime>::InvalidGrandpaJustification
            );
        })
    }

    #[test]
    fn does_not_import_header_with_invalid_finality_proof() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let headers: Vec<TestHeader> = test_header_range(5);
            let signed_header: &TestHeader = headers.last().unwrap();
            let mut range: Vec<TestHeader> = headers[1..5].to_vec();
            range.reverse();

            let mut justification = make_default_justification(&signed_header.clone());
            justification.round = 42;
            let justification = justification;

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };
            let default_gateway: ChainId = *b"pdot";

            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                Error::<TestRuntime>::InvalidGrandpaJustification
            );
        })
    }

    #[test]
    fn disallows_invalid_justification() {
        run_test(|| {
            let genesis = test_header(0);
            let default_gateway: ChainId = *b"pdot";
            let different_authorities: Vec<AuthorityId> = vec![ALICE.into(), DAVE.into()];
            let init_data = GrandpaRegistrationData::<AccountId> {
                authorities: Some(different_authorities),
                first_header: genesis.encode(),
                authority_set_id: Some(1),
                owner: 1,
                parachain: None,
            };

            assert_ok!(initialize_custom_relaychain(
                Origin::root(),
                default_gateway,
                init_data,
            ));

            let headers: Vec<TestHeader> = test_header_range(5);
            let signed_header: &TestHeader = headers.last().unwrap();
            let mut range: Vec<TestHeader> = headers[1..5].to_vec();
            range.reverse();

            // this justification contains ALICE, BOB and CHARLIE, to it will be invalid
            let justification = make_default_justification(&signed_header.clone());

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                Error::<TestRuntime>::InvalidGrandpaJustification
            );
        })
    }

    #[test]
    fn importing_header_ensures_that_chain_is_extended() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            assert_ok!(submit_headers(1, 5));
            assert_noop!(
                submit_headers(4, 8),
                Error::<TestRuntime>::InvalidRangeLinkage
            );
            assert_ok!(submit_headers(6, 10));
        })
    }

    #[test]
    fn importing_header_enacts_new_authority_set() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            let next_set_id = 2;
            let next_authorities = vec![(ALICE.into(), 1), (BOB.into(), 1)];

            let headers: Vec<TestHeader> = test_header_range(2);
            let mut signed_header = headers[2].clone();
            let range: Vec<TestHeader> = headers[1..2].to_vec();

            // Need to update the header digest to indicate that our header signals an authority set
            // change. The change will be enacted when we import our header.
            signed_header.digest = change_log(0);

            let justification = make_default_justification(&signed_header);
            let data = RelaychainHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            let default_gateway: ChainId = *b"pdot";

            // Let's import our test header
            assert_ok!(Pallet::<TestRuntime>::submit_headers(
                Origin::signed(1),
                default_gateway,
                data.encode()
            ));

            // Make sure that our header is the best finalized
            assert_eq!(
                <BestFinalizedMap<TestRuntime>>::get(default_gateway),
                Some(signed_header.hash())
            );
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                signed_header.hash()
            ));

            // Make sure that the authority set actually changed upon importing our header
            assert_eq!(
                <CurrentAuthoritySet<TestRuntime>>::get(),
                Some(bp_header_chain::AuthoritySet::new(
                    next_authorities,
                    next_set_id
                )),
            );
        })
    }

    #[test]
    fn importing_header_rejects_header_with_scheduled_change_delay() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            // Need to update the header digest to indicate that our header signals an authority set
            // change. However, the change doesn't happen until the next block.
            let headers: Vec<TestHeader> = test_header_range(2);
            let mut signed_header = headers[2].clone();
            let mut range: Vec<TestHeader> = headers[1..2].to_vec();
            range.reverse();

            signed_header.digest = change_log(1);

            let justification = make_default_justification(&signed_header);
            let data = RelaychainHeaderData::<TestHeader> {
                signed_header,
                range,
                justification,
            };

            let default_gateway: ChainId = *b"pdot";

            // Should not be allowed to import this header
            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                <Error<TestRuntime>>::UnsupportedScheduledChange
            );
        })
    }

    #[test]
    fn importing_header_rejects_header_with_forced_changes() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            // Need to update the header digest to indicate that it signals a forced authority set
            // change.
            let headers: Vec<TestHeader> = test_header_range(2);
            let mut signed_header = headers[2].clone();
            let mut range: Vec<TestHeader> = headers[1..2].to_vec();
            range.reverse();

            signed_header.digest = change_log(1);

            let justification = make_default_justification(&signed_header);

            let data = RelaychainHeaderData::<TestHeader> {
                signed_header,
                range,
                justification,
            };

            let default_gateway: ChainId = *b"pdot";

            // Should not be allowed to import this header
            assert_err!(
                Pallet::<TestRuntime>::submit_headers(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                <Error<TestRuntime>>::UnsupportedScheduledChange
            );
        })
    }

    #[test]
    fn parse_finalized_storage_proof_rejects_proof_on_unknown_header() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            assert_noop!(
                Pallet::<TestRuntime>::parse_finalized_storage_proof(
                    Default::default(),
                    sp_trie::StorageProof::new(vec![]),
                    |_| (),
                    default_gateway,
                ),
                Error::<TestRuntime>::UnknownHeader,
            );
        });
    }

    #[test]
    fn parse_finalized_storage_accepts_valid_proof() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            let (state_root, storage_proof) = bp_runtime::craft_valid_storage_proof();

            let mut header = test_header(2);
            header.set_state_root(state_root);

            let hash = header.hash();
            <BestFinalizedMap<TestRuntime>>::insert(default_gateway, hash);
            <MultiImportedHeaders<TestRuntime>>::insert(default_gateway, hash, header);

            assert_ok!(
                Pallet::<TestRuntime>::parse_finalized_storage_proof(
                    hash,
                    storage_proof,
                    |_| (),
                    default_gateway
                ),
                (),
            );
        });
    }

    #[test]
    fn should_prune_headers_over_headers_to_keep_parameter() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let headers = test_header_range(111u64);

            //°°°°°°°°°°°ACHTUNG!!!°°°°°°°°°°°°
            assert_ok!(submit_headers(1, 5));
            // MultiImportedHashes: [1, 2, 3, 4, 5] in MultiImportedHashes
            // Pointer               ^
            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[1].hash(),
                ),
                true
            );
            // contains added header
            assert_ok!(submit_headers(6, 7));
            // [6, 7, 3, 4, 5]
            //        ^

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[3].hash(),
                ),
                true
            ); // still available

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[2].hash(),
                ),
                false
            ); // overwritten by buffer

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[1].hash(),
                ),
                false
            ); // overwritten by buffer

            assert_ok!(submit_headers(8, 10));
            // [6, 7, 8, 9, 10]
            //  ^

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[5].hash(),
                ),
                false
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[4].hash(),
                ),
                false
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[3].hash(),
                ),
                false
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[6].hash(),
                ),
                true
            );

            assert_ok!(submit_headers(11, 15));
            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[10].hash(),
                ),
                false
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[11].hash(),
                ),
                true
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[12].hash(),
                ),
                true
            );
        })
    }
}
