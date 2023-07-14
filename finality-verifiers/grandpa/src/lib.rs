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

extern crate core;

use crate::weights::WeightInfo;
use bp_header_chain::{justification::GrandpaJustification, InitializationData};
use bp_runtime::{BlockNumberOf, Chain, ChainId, HashOf, HasherOf, HeaderOf};
use sp_std::convert::TryInto;

use finality_grandpa::voter_set::VoterSet;
use frame_support::{ensure, pallet_prelude::*, transactional, StorageHasher};
use frame_system::{ensure_signed, RawOrigin};
use sp_core::crypto::ByteArray;
use sp_finality_grandpa::{ConsensusLog, GRANDPA_ENGINE_ID};
use sp_runtime::traits::{BadOrigin, Header as HeaderT, Zero};
use sp_std::vec::Vec;
use t3rn_primitives::light_client::LightClientAsyncAPI;

pub mod types;

use sp_trie::{read_trie_value, LayoutV1, StorageProof};

#[cfg(feature = "testing")]
pub mod mock;

pub mod bridges;
pub mod light_clients;
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum VMSource {
    EVM([u8; 20]),
    WASM([u8; 32]),
}

pub fn to_local_block_number<T: Config<I>, I: 'static>(
    block_number: BridgedBlockNumber<T, I>,
) -> Result<T::BlockNumber, DispatchError> {
    let local_block_number: T::BlockNumber = Decode::decode(&mut block_number.encode().as_slice())
        .map_err(|_e| {
            DispatchError::Other(
                "LightClient::Grandpa - failed to decode block number from bridged header",
            )
        })?;
    Ok(local_block_number)
}

use crate::types::{
    GrandpaHeaderData, ParachainInclusionProof, ParachainRegistrationData,
    RelaychainInclusionProof, RelaychainRegistrationData,
};
use frame_system::pallet_prelude::*;

use t3rn_primitives::ExecutionSource;

use t3rn_primitives::light_client::InclusionReceipt;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use num_traits::One;
    use sp_runtime::traits::Saturating;
    use t3rn_primitives::{light_client::LightClient, GatewayVendor};

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

        type FastConfirmationOffset: Get<Self::BlockNumber>;

        type RationalConfirmationOffset: Get<Self::BlockNumber>;

        type FinalizedConfirmationOffset: Get<Self::BlockNumber>;

        type EpochOffset: Get<Self::BlockNumber>;

        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        type LightClientAsyncAPI: LightClientAsyncAPI<Self>;

        type MyVendor: Get<GatewayVendor>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(pub PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

    /// Hash of the header used to bootstrap the pallet.
    #[pallet::storage]
    #[pallet::getter(fn get_initial_hash)]
    pub(super) type InitialHash<T: Config<I>, I: 'static = ()> =
        StorageValue<_, BridgedBlockHash<T, I>, OptionQuery>;

    /// Hash of the best finalized header.
    #[pallet::storage]
    #[pallet::getter(fn get_best_block_hash)]
    pub(super) type BestFinalizedHash<T: Config<I>, I: 'static = ()> =
        StorageValue<_, BridgedBlockHash<T, I>, OptionQuery>;

    /// A ring buffer of imported hashes. Ordered by the insertion time.
    #[pallet::storage]
    #[pallet::getter(fn get_imported_hashes)]
    pub(super) type ImportedHashes<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, u32, BridgedBlockHash<T, I>>;

    /// Count successful submissions.
    #[pallet::storage]
    #[pallet::getter(fn get_submissions_counter)]
    pub(super) type SubmissionsCounter<T: Config<I>, I: 'static = ()> =
        StorageValue<_, T::BlockNumber, ValueQuery>;

    /// Current ring buffer position.
    #[pallet::storage]
    #[pallet::getter(fn get_imported_hashes_pointer)]
    pub(super) type ImportedHashesPointer<T: Config<I>, I: 'static = ()> =
        StorageValue<_, u32, OptionQuery>;

    /// Headers which have been imported into the pallet.
    #[pallet::storage]
    #[pallet::getter(fn get_imported_headers)]
    pub(super) type ImportedHeaders<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, BridgedBlockHash<T, I>, BridgedHeader<T, I>>;

    #[pallet::storage]
    pub(super) type RelayChainId<T: Config<I>, I: 'static = ()> =
        StorageValue<_, ChainId, OptionQuery>;

    /// The current GRANDPA Authority set.
    #[pallet::storage]
    pub(super) type CurrentAuthoritySet<T: Config<I>, I: 'static = ()> =
        StorageValue<_, bp_header_chain::AuthoritySet, OptionQuery>;

    /// Maps a parachain chain_id to the corresponding chain ID.
    #[pallet::storage]
    pub(super) type ParachainIdMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, ParachainRegistrationData>;

    /// Optional pallet owner.
    ///
    /// Pallet owner has a right to halt all pallet operations and then resume it. If it is
    /// `None`, then there are no direct ways to halt/resume pallet operations, but other
    /// runtime methods may still be used to do that (i.e. democracy::referendum to update halt
    /// flag directly or call the `halt_operations`).
    #[pallet::storage]
    pub(super) type PalletOwner<T: Config<I>, I: 'static = ()> =
        StorageValue<_, T::AccountId, OptionQuery>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    #[pallet::getter(fn is_halted)]
    pub(super) type IsHalted<T: Config<I>, I: 'static = ()> = StorageValue<_, bool, ValueQuery>;

    /// If true, all pallet transactions are failed immediately.
    #[pallet::storage]
    #[pallet::getter(fn ever_initialized)]
    pub(super) type EverInitialized<T: Config<I>, I: 'static = ()> =
        StorageValue<_, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        HeadersAdded(BridgedBlockNumber<T, I>),
    }

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
        /// Failed to decode the source of the event
        UnexpectedEventLength,
        /// The source of event is not valid
        UnexpectedSource,
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
        /// The payload source is invalid
        InvalidPayloadSource,
        /// The payload source format is invalid
        InvalidSourceFormat,
    }

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// Add a header range for the relaychain
        ///
        /// It will use the underlying storage pallet to fetch information about the current
        /// authorities and best finalized header in order to verify that the header is finalized,
        /// and the corresponding range valid.
        ///
        /// If successful in verification, it will write the target range to the underlying storage
        /// pallet.
        ///
        /// If the new range was accepted, pays no fee.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_headers(
            origin: OriginFor<T>,
            // seq vector of headers to be added.
            range: Vec<BridgedHeader<T, I>>,
            // The header with the highest height, signed in the justification
            signed_header: BridgedHeader<T, I>,
            // GrandpaJustification for the signed_header
            justification: GrandpaJustification<BridgedHeader<T, I>>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let pointer_prior = <ImportedHashesPointer<T, I>>::get().unwrap_or_default();
            Pallet::<T, I>::verify_and_store_headers(range, signed_header, justification)?;
            let pointer_post = <ImportedHashesPointer<T, I>>::get().unwrap_or_default();
            if pointer_prior != pointer_post {
                let counter = <SubmissionsCounter<T, I>>::get();

                match Pallet::<T, I>(PhantomData).get_latest_heartbeat() {
                    Ok(heartbeat) => {
                        let verifier = T::MyVendor::get();
                        T::LightClientAsyncAPI::on_new_epoch(verifier, counter, heartbeat);
                    },
                    Err(e) => {
                        log::error!(
                            "Failed to get latest heartbeat after submit_headers: {:?}",
                            e
                        );
                    },
                }

                <SubmissionsCounter<T, I>>::put(counter.saturating_add(T::BlockNumber::one()));

                Ok(Pays::No.into())
            } else {
                Ok(Pays::Yes.into())
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn reset(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <EverInitialized<T, I>>::kill();
            <BestFinalizedHash<T, I>>::kill();
            <InitialHash<T, I>>::kill();
            <ImportedHashesPointer<T, I>>::kill(); // one ahead of first value
            <RelayChainId<T, I>>::kill();
            <CurrentAuthoritySet<T, I>>::kill();
            <IsHalted<T, I>>::kill();
            <PalletOwner<T, I>>::kill();
            Ok(().into())
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
    ) -> Result<bool, sp_runtime::DispatchError> {
        let mut change_enacted = false;

        // We don't support forced changes - at that point governance intervention is required.
        ensure!(
            super::find_forced_change(header).is_none(),
            <Error<T, I>>::UnsupportedScheduledChange
        );

        if let Some(change) = find_scheduled_change(header) {
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
                "Transitioned from authority set {} to {}! New authorities are: {:?}",
                current_set_id,
                current_set_id + 1,
                next_authorities,
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
        init_params: InitializationData<BridgedHeader<T, I>>,
        owner: T::AccountId,
    ) -> Result<(), &'static str> {
        can_init_relay_chain::<T, I>()?;

        let InitializationData {
            header,
            authority_list,
            set_id,
            is_halted,
            gateway_id, // ToDo: can be removed
        } = init_params;

        let initial_hash = header.hash();
        // Store header stuff
        <InitialHash<T, I>>::put(initial_hash);
        <BestFinalizedHash<T, I>>::put(initial_hash);
        <ImportedHeaders<T, I>>::insert(initial_hash, header);
        <ImportedHashesPointer<T, I>>::put(0); // one ahead of first value
        <RelayChainId<T, I>>::put(gateway_id);
        let authority_set = bp_header_chain::AuthoritySet::new(authority_list, set_id);
        <CurrentAuthoritySet<T, I>>::put(authority_set);

        // Other configs
        <IsHalted<T, I>>::put(is_halted);
        <EverInitialized<T, I>>::put(true);
        <PalletOwner<T, I>>::put(owner);

        Ok(())
    }

    /// removes old header data based on ring buffer logic. Adds new header data, updates the ring buffer entry and increments buffer index
    pub(crate) fn write_and_clean_header_data<T: Config<I>, I: 'static>(
        buffer_index: &mut u32,
        header: &BridgedHeader<T, I>,
        hash: BridgedBlockHash<T, I>,
        is_signed_header: bool,
    ) -> Result<(), &'static str> {
        // If we find an entry to overwrite, do so.
        if let Ok(hash) = <ImportedHashes<T, I>>::try_get(
            *buffer_index, // can't overflow because of incrementation logic
        ) {
            <ImportedHeaders<T, I>>::remove(hash);
        }

        // Once deleted, we add the new header
        <ImportedHeaders<T, I>>::insert(hash, header.clone());
        <ImportedHashes<T, I>>::insert(*buffer_index, hash);

        // if this is the signed header, we set best finalized
        if is_signed_header {
            <BestFinalizedHash<T, I>>::put(hash);
        }

        *buffer_index = (*buffer_index + 1) % T::HeadersToStore::get(); // prevents overflows
        Ok(())
    }

    /// Ensure that the pallet is in operational mode (not halted).
    pub fn ensure_operational_single<T: Config<I>, I: 'static>() -> Result<(), Error<T, I>> {
        if <IsHalted<T, I>>::get() {
            Err(<Error<T, I>>::Halted)
        } else {
            Ok(())
        }
    }
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    #[transactional]
    pub(crate) fn verify_and_store_headers(
        // seq vector of headers to be added.
        range: Vec<BridgedHeader<T, I>>,
        // The header with the highest height, signed in the justification
        signed_header: BridgedHeader<T, I>,
        // GrandpaJustification for the signed_header
        justification: GrandpaJustification<BridgedHeader<T, I>>,
    ) -> DispatchResult {
        // °°°°° Implicit Check: °°°°°
        // range.len() < T::HeadersToStore::get() - ensures that we don't mess up our ring buffer
        // Since polkadot updates its authority set every 24h, this is implicitly ensured => Justification check would fail after 1/7th of max len

        // we get the latest header from storage
        let mut best_finalized_hash =
            <BestFinalizedHash<T, I>>::get().ok_or(Error::<T, I>::NoFinalizedHeader)?;

        // °°°°° Explanation °°°°°
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
        )?;
        // °°°°° Checked: #2 °°°°°°

        // check for authority set update and enact if available.
        let _enacted = try_enact_authority_change_single::<T, I>(&signed_header, set_id)?;

        // We get the latest buffer_index, which maps to the next header we can overwrite, and the index where we insert the verified header
        let mut buffer_index = <ImportedHashesPointer<T, I>>::get().unwrap_or_default();

        // °°°°° Begin Check: #1 °°°°°
        for header in range {
            if best_finalized_hash == *header.parent_hash() {
                // write header to storage if correct
                write_and_clean_header_data::<T, I>(
                    &mut buffer_index,
                    &header,
                    header.hash(),
                    false,
                )?;

                best_finalized_hash = header.hash();
            } else {
                // if anything fails here, noop!
                return Err(Error::<T, I>::InvalidRangeLinkage.into())
            }
        }
        // °°°°° Check Success: #1 °°°°°

        // °°°°° Begin Check: #3 °°°°°
        if best_finalized_hash == *signed_header.parent_hash() {
            // write header to storage if correct
            write_and_clean_header_data::<T, I>(
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
        <ImportedHashesPointer<T, I>>::set(Some(buffer_index));

        Self::deposit_event(Event::HeadersAdded(*signed_number));
        Ok(())
    }

    // /// Get the best finalized header the pallet knows of.
    // ///
    // /// Returns a dummy header if there is no best header. This can only happen
    // /// if the pallet has not been initialized yet.
    pub fn best_finalized_map() -> BridgedHeader<T, I> {
        let hash = <BestFinalizedHash<T, I>>::get().unwrap_or_default();
        <ImportedHeaders<T, I>>::get(hash).unwrap_or_else(|| {
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
    pub fn is_known_header(hash: BridgedBlockHash<T, I>) -> bool {
        <ImportedHeaders<T, I>>::contains_key(hash)
    }

    /// Verify that the passed storage proof is valid, given it is crafted using
    /// known finalized header. If the proof is valid, then the `parse` callback
    /// is called and the function returns its result.
    pub fn parse_finalized_storage_proof<R>(
        hash: BridgedBlockHash<T, I>,
        storage_proof: sp_trie::StorageProof,
        parse: impl FnOnce(bp_runtime::StorageProofChecker<BridgedBlockHasher<T, I>>) -> R,
    ) -> Result<R, DispatchError> {
        let header = <ImportedHeaders<T, I>>::get(hash).ok_or(Error::<T, I>::UnknownHeader)?;
        let storage_proof_checker =
            bp_runtime::StorageProofChecker::new(*header.state_root(), storage_proof)
                .map_err(|_| Error::<T, I>::StorageRootMismatch)?;

        Ok(parse(storage_proof_checker))
    }

    pub fn initialize(
        origin: OriginFor<T>,
        gateway_id: ChainId,
        encoded_registration_data: Vec<u8>,
    ) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin)?;

        match <RelayChainId<T, I>>::get() {
            Some(relay_chain_id) => {
                // Register parachain
                ensure!(relay_chain_id != gateway_id, "chain_id already initialized");
                let parachain_registration_data: ParachainRegistrationData =
                    Decode::decode(&mut &*encoded_registration_data)
                        .map_err(|_| "Parachain registration decoding error")?;

                ensure!(
                    parachain_registration_data.relay_gateway_id == relay_chain_id,
                    "Invalid relay chain id"
                );

                <ParachainIdMap<T, I>>::insert(gateway_id, parachain_registration_data);

                Ok(())
            },
            None => {
                // register relaychain
                let registration_data: RelaychainRegistrationData<T::AccountId> =
                    Decode::decode(&mut &*encoded_registration_data)
                        .map_err(|_| "Decoding Error")?;

                let header: BridgedHeader<T, I> =
                    Decode::decode(&mut &registration_data.first_header[..])
                        .map_err(|_| "header decoding error")?;

                let init_data = InitializationData {
                    header,
                    authority_list: registration_data
                        .authorities
                        .iter()
                        .map(|id| {
                            sp_finality_grandpa::AuthorityId::from_slice(&id.encode()).unwrap()
                        }) // not sure why this is still needed
                        .map(|authority| (authority, 1))
                        .collect::<Vec<_>>(),
                    set_id: registration_data.authority_set_id,
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
        origin: OriginFor<T>,
        _gateway_id: ChainId,
        encoded_new_owner: Vec<u8>,
    ) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin)?;
        let new_owner: Option<T::AccountId> =
            Decode::decode(&mut &*encoded_new_owner).map_err(|_| "New Owner decoding error")?;

        match new_owner {
            Some(new_owner) => {
                PalletOwner::<T, I>::put(&new_owner);
                log::info!("Setting pallet Owner to: {:?}", new_owner);
            },
            None => {
                PalletOwner::<T, I>::set(None);
                log::info!("Removed Owner of pallet.");
            },
        }

        Ok(())
    }

    /// Halt or resume all pallet operations.
    ///
    /// May only be called either by root, or by `PalletOwner`.
    pub fn set_operational(origin: OriginFor<T>, operational: bool) -> Result<(), &'static str> {
        ensure_owner_or_root_single::<T, I>(origin)?;
        <IsHalted<T, I>>::put(!operational); // inverted because operational vs halted are opposite
        Ok(())
    }

    pub fn submit_encoded_headers(encoded_header_data: Vec<u8>) -> Result<(), DispatchError> {
        ensure_operational_single::<T, I>()?;
        let data: GrandpaHeaderData<BridgedHeader<T, I>> =
            Decode::decode(&mut &*encoded_header_data)
                .map_err(|_| Error::<T, I>::HeaderDataDecodingError)?;

        Pallet::<T, I>::verify_and_store_headers(
            data.range,
            data.signed_header,
            data.justification,
        )?;
        Ok(())
    }

    // ACHTUNG: experimental - if the check_source is set, try to establish whether the source was emitted by EVM or WASM VM by assuming following:
    // - source preceeded with 12 bytes of 0x00 is EVM, otherwise WASM
    // - events order on both EVM and WASM are known and fixed
    //  - EVM emits Log event as the first event (index=0), with the first field being the source on 20 bytes
    //  - WASM emits the source as the fourth (index=3) event, with the first field being the source on 32 bytes
    pub fn check_vm_source(
        source: ExecutionSource,
        message: Vec<u8>,
    ) -> Result<VMSource, DispatchError> {
        // Check if the source is EVM
        if source[0..12] == [0u8; 12] {
            // Skip the first 2 bytes of the message:
            // 1. 0x?? - pallet evm index on target
            // 2. 0x00 - Log event index of pallet evm
            ensure!(message.len() >= 22, Error::<T, I>::UnexpectedEventLength);
            // Ensure we're looking at the EVM::Log event (index=0)
            ensure!(message[1] == 0u8, Error::<T, I>::UnexpectedSource);

            let assumed_evm_source_bytes = &message[2..22];
            if &source[12..] != assumed_evm_source_bytes {
                return Err(Error::<T, I>::UnexpectedSource.into())
            }
            let source = sp_core::H160::from_slice(&source[12..]);
            Ok(VMSource::EVM(source.0))
        } else {
            // Skip the first 2 bytes of the message:
            // 1. 0x?? - pallet contracts index on target
            // 2. 0x03 - ContractEmitted event index of pallet contracts
            // Check if the source is WASM
            ensure!(message.len() >= 34, Error::<T, I>::UnexpectedEventLength);
            // Ensure we're looking at the Contracts::ContractEmitted event (index=3)
            ensure!(message[1] == 3u8, Error::<T, I>::UnexpectedSource);

            // Assume following 32 bytes are the source
            let assumed_wasm_source_bytes = &message[2..34];
            if &source[..] != assumed_wasm_source_bytes {
                return Err(Error::<T, I>::UnexpectedSource.into())
            }
            let source = sp_core::H256::from_slice(&source[..]);
            Ok(VMSource::WASM(source.0))
        }
    }

    pub fn confirm_event_inclusion(
        gateway_id: ChainId,
        encoded_inclusion_proof: Vec<u8>,
        maybe_source: Option<ExecutionSource>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        let is_relaychain = Some(gateway_id) == <RelayChainId<T, I>>::get();

        let (payload_proof, encoded_payload, header, header_hash) = if is_relaychain {
            let proof: RelaychainInclusionProof<BridgedHeader<T, I>> =
                Decode::decode(&mut &*encoded_inclusion_proof)
                    .map_err(|_| Error::<T, I>::HeaderDataDecodingError)?;

            let header = <ImportedHeaders<T, I>>::get(proof.block_hash)
                .ok_or(Error::<T, I>::UnknownHeader)?;

            (
                proof.payload_proof,
                proof.encoded_payload,
                header,
                proof.block_hash,
            )
        } else {
            let proof: ParachainInclusionProof<BridgedHeader<T, I>> =
                Decode::decode(&mut &*encoded_inclusion_proof)
                    .map_err(|_| Error::<T, I>::HeaderDataDecodingError)?;
            let header = verify_header_storage_proof::<T, I>(
                proof.relay_block_hash,
                proof.header_proof,
                <ParachainIdMap<T, I>>::get(gateway_id)
                    .ok_or(Error::<T, I>::ParachainEntryNotFound)?,
            )?;
            (
                proof.payload_proof,
                proof.encoded_payload,
                header,
                proof.relay_block_hash,
            )
        };

        let message =
            verify_event_storage_proof::<T, I>(payload_proof, header.clone(), encoded_payload)?;

        if let Some(source) = maybe_source {
            Self::check_vm_source(source, message.clone())?;
        }

        Ok(InclusionReceipt::<T::BlockNumber> {
            height: to_local_block_number::<T, I>(*header.number())?,
            including_header: header_hash.encode(),
            message,
        })
    }

    pub fn get_latest_finalized_header() -> Option<Vec<u8>> {
        if let Some(header_hash) = <BestFinalizedHash<T, I>>::get() {
            return Some(header_hash.encode())
        }
        None
    }
}

/// Verifies a given storage proof. Returns the encoded entry that is proven
pub(crate) fn verify_storage_proof<T: Config<I>, I: 'static>(
    header: BridgedHeader<T, I>,
    key: Vec<u8>,
    proof: StorageProof,
    trie_type: ProofTriePointer,
) -> Result<Vec<u8>, &'static str> {
    let root = get_header_roots::<T, I>(header, trie_type)?;
    let db = proof.into_memory_db::<BridgedBlockHasher<T, I>>();
    match read_trie_value::<LayoutV1<BridgedBlockHasher<T, I>>, _>(&db, &root, key.as_ref()) {
        Ok(Some(value)) => Ok(value),
        _ => Err(Error::<T, I>::InvalidStorageProof.into()),
    }
}

/// returns the specified header root from a specific header
pub(crate) fn get_header_roots<T: pallet::Config<I>, I>(
    header: BridgedHeader<T, I>,
    trie_type: ProofTriePointer,
) -> Result<BridgedBlockHash<T, I>, DispatchError> {
    match trie_type {
        ProofTriePointer::State => Ok(*header.state_root()),
        ProofTriePointer::Transaction => Ok(*header.extrinsics_root()),
        ProofTriePointer::Receipts => Ok(*header.state_root()),
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
    origin: OriginFor<T>,
) -> Result<(), &'static str> {
    match origin.into() {
        Ok(RawOrigin::Root) => Ok(()),
        Ok(RawOrigin::Signed(ref signer))
            if <PalletOwner<T, I>>::exists()
                && Some(signer) == <PalletOwner<T, I>>::get().as_ref() =>
            Ok(()),
        _ => Err(BadOrigin.into()),
    }
}

/// Ensure that no relaychain has been set so far. Relaychains are unique
fn can_init_relay_chain<T: Config<I>, I: 'static>() -> Result<(), &'static str> {
    match <BestFinalizedHash<T, I>>::exists() {
        true => Err("Duplicate relaychain"), // we have a relaychain registered already
        false => Ok(()),                     // is first relaychain
    }
}

/// Ensure that the SideEffect was executed after it was created.
fn executed_after_creation<T: Config<I>, I: 'static>(
    submission_target_height: T::BlockNumber,
    header: &BridgedHeader<T, I>,
) -> Result<(), &'static str> {
    let submission_target: BridgedBlockNumber<T, I> =
        Decode::decode(&mut &*submission_target_height.encode())
            .map_err(|_| "Invalid block number")?;

    ensure!(
        submission_target < *header.number(),
        "Transaction executed before SideEffect creation"
    );
    Ok(())
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
    storage_proof: StorageProof,
    header: BridgedHeader<T, I>,
    encoded_payload: Vec<u8>,
) -> Result<Vec<u8>, DispatchError> {
    // storage key for System_Events
    let key: Vec<u8> = [
        38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 128, 212, 30, 94,
        22, 5, 103, 101, 188, 132, 97, 133, 16, 114, 201, 215,
    ]
    .to_vec();
    let verified_block_events =
        verify_storage_proof::<T, I>(header, key, storage_proof, ProofTriePointer::Receipts)?;

    // the problem here is that in substrates current design its not possible to prove the inclusion of a single event, only all events of a block
    // https://github.com/paritytech/substrate/issues/11216
    ensure!(
        is_sub(verified_block_events.as_slice(), encoded_payload.as_slice()),
        Error::<T, I>::EventNotIncluded
    );

    Ok(encoded_payload)
}

pub(crate) fn verify_header_storage_proof<T: Config<I>, I: 'static>(
    relay_block_hash: BridgedBlockHash<T, I>,
    proof: StorageProof,
    parachain: ParachainRegistrationData,
) -> Result<BridgedHeader<T, I>, DispatchError> {
    let relay_header =
        <ImportedHeaders<T, I>>::get(relay_block_hash).ok_or(Error::<T, I>::UnknownHeader)?;

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
    let encoded_header_vec =
        verify_storage_proof::<T, I>(relay_header, key, proof, ProofTriePointer::State)?;

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
pub mod tests {
    use super::*;
    use crate::mock::{
        produce_mock_headers_range, run_test, test_header, test_header_range,
        test_header_with_correct_parent, AccountId, Origin, TestHeader, TestNumber, TestRuntime,
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
    use frame_support::{assert_noop, assert_ok};
    use sp_core::{crypto::AccountId32, H160, H256};
    use sp_finality_grandpa::AuthorityId;
    use sp_runtime::{Digest, DigestItem, DispatchError};

    use crate::types::GrandpaHeaderData;

    fn initialize_relaychain(
        origin: Origin,
    ) -> Result<RelaychainRegistrationData<AccountId>, &'static str> {
        let genesis = test_header_with_correct_parent(0, None);
        let init_data = RelaychainRegistrationData::<AccountId> {
            authorities: authorities(),
            first_header: genesis.encode(),
            authority_set_id: 1,
            owner: 1u64,
        };

        initialize_custom_relaychain(origin, *b"pdot", init_data)
    }

    fn initialize_named_relaychain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<RelaychainRegistrationData<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = RelaychainRegistrationData::<AccountId> {
            authorities: authorities(),
            first_header: genesis.encode(),
            authority_set_id: 1,
            owner: 1u64,
        };

        initialize_custom_relaychain(origin, gateway_id, init_data)
    }

    fn initialize_custom_relaychain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: RelaychainRegistrationData<AccountId>,
    ) -> Result<RelaychainRegistrationData<AccountId>, &'static str> {
        Pallet::<TestRuntime>::initialize(origin, gateway_id, init_data.encode()).map(|_| init_data)
    }

    fn initialize_parachain(origin: Origin) -> Result<ParachainRegistrationData, &'static str> {
        let _genesis = test_header(0);
        let init_data = ParachainRegistrationData {
            relay_gateway_id: *b"pdot",
            id: 0,
        };

        initialize_custom_parachain(origin, *b"moon", init_data)
    }

    fn initialize_named_parachain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<ParachainRegistrationData, &'static str> {
        let init_data = ParachainRegistrationData {
            relay_gateway_id: *b"pdot",
            id: 0,
        };

        initialize_custom_parachain(origin, gateway_id, init_data)
    }

    fn initialize_custom_parachain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: ParachainRegistrationData,
    ) -> Result<ParachainRegistrationData, &'static str> {
        Pallet::<TestRuntime>::initialize(origin, gateway_id, init_data.encode()).map(|_| init_data)
    }

    pub fn submit_headers(from: u8, to: u8) -> Result<GrandpaHeaderData<TestHeader>, &'static str> {
        let data = produce_mock_headers_range(from, to);
        Pallet::<TestRuntime>::submit_encoded_headers(data.encode())?;
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

    fn change_log(delay: u32) -> Digest {
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

    fn forced_change_log(delay: u32) -> Digest {
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
        run_test(|| {
            assert_noop!(initialize_relaychain(Origin::signed(1)), "Bad origin");
            assert_ok!(initialize_relaychain(Origin::root()));

            // Reset storage so we can initialize the pallet again
            BestFinalizedHash::<TestRuntime>::set(None);
            PalletOwner::<TestRuntime>::put(2);
            RelayChainId::<TestRuntime>::set(None);
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
            assert_noop!(
                initialize_relaychain(Origin::root()),
                "chain_id already initialized"
            );
        })
    }

    #[test]
    fn cant_register_parachain_without_relaychain() {
        run_test(|| {
            assert_noop!(initialize_parachain(Origin::root()), "Decoding Error");
        })
    }

    #[test]
    fn cant_register_parachain_with_wrong_relaychain_id() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));

            let _genesis = test_header(0);
            let init_data = ParachainRegistrationData {
                relay_gateway_id: *b"roco",
                id: 0,
            };

            assert_noop!(
                initialize_custom_parachain(Origin::root(), *b"moon", init_data),
                "Invalid relay chain id"
            );
        })
    }

    #[test]
    fn cant_register_relaychain_as_non_root() {
        run_test(|| {
            assert_noop!(initialize_relaychain(Origin::signed(1)), "Bad origin");
        })
    }

    #[test]
    fn cant_register_parachain_as_non_root() {
        run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_noop!(initialize_parachain(Origin::signed(0)), "Bad origin");
        })
    }

    #[test]
    fn init_storage_entries_are_correctly_initialized() {
        let header = test_header(0);
        run_test(|| {
            assert_eq!(BestFinalizedHash::<TestRuntime>::get(), None);
            assert_eq!(BestFinalizedHash::<TestRuntime>::get(), None);

            let _init_data = initialize_relaychain(Origin::root()).unwrap();

            assert!(<ImportedHeaders<TestRuntime>>::contains_key(header.hash()));
            assert_eq!(BestFinalizedHash::<TestRuntime>::get(), Some(header.hash()));
            assert_eq!(
                CurrentAuthoritySet::<TestRuntime>::get()
                    .unwrap()
                    .authorities,
                authority_list()
            );
            assert_eq!(IsHalted::<TestRuntime>::get(), false);
        })
    }

    #[test]
    fn init_can_only_initialize_pallet_once() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            assert_noop!(
                initialize_relaychain(Origin::root()),
                "chain_id already initialized"
            );
        })
    }

    #[test]
    fn root_can_reset_pallet() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let _ = Pallet::<TestRuntime>::reset(Origin::root());
            assert_eq!(EverInitialized::<TestRuntime>::get(), false);
            assert_eq!(BestFinalizedHash::<TestRuntime>::get(), None);
            assert_eq!(InitialHash::<TestRuntime>::get(), None);
            assert_eq!(ImportedHashesPointer::<TestRuntime>::get(), None);
            assert_eq!(RelayChainId::<TestRuntime>::get(), None);
            assert_eq!(CurrentAuthoritySet::<TestRuntime>::get(), None);
            assert_eq!(IsHalted::<TestRuntime>::get(), false);
            assert_eq!(PalletOwner::<TestRuntime>::get(), None);
            //can re-register
            assert_ok!(initialize_relaychain(Origin::root()));

            assert_noop!(
                Pallet::<TestRuntime>::reset(Origin::signed(1)),
                DispatchError::BadOrigin
            );
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
                Pallet::<TestRuntime>::set_operational(Origin::signed(2), false),
                DispatchError::BadOrigin,
            );
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                false,
            ));

            let owner: Option<AccountId> = None;
            assert_ok!(Pallet::<TestRuntime>::set_owner(
                Origin::signed(1),
                default_gateway,
                owner.encode(),
            ));
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true),
                DispatchError::BadOrigin,
            );
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(2), true),
                DispatchError::BadOrigin,
            );
            assert_ok!(Pallet::<TestRuntime>::set_operational(Origin::root(), true,));
        });
    }

    #[test]
    fn pallet_may_be_halted_by_root() {
        let _default_gateway: ChainId = *b"pdot";

        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                false,
            ));
            assert_noop!(submit_headers(1, 3), "Halted");
            assert_ok!(Pallet::<TestRuntime>::set_operational(Origin::root(), true,));
        });
    }

    #[test]
    fn pallet_may_be_halted_by_owner() {
        let _default_gateway: ChainId = *b"pdot";

        run_test(|| {
            PalletOwner::<TestRuntime>::put(2);

            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                false,
            ));
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                true,
            ));

            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), false),
                DispatchError::BadOrigin,
            );
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true),
                DispatchError::BadOrigin,
            );

            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::signed(2),
                false,
            ));
            assert_noop!(
                Pallet::<TestRuntime>::set_operational(Origin::signed(1), true),
                DispatchError::BadOrigin,
            );
        });
    }

    #[test]
    fn pallet_rejects_transactions_if_halted() {
        run_test(|| {
            let _gateway_a: ChainId = *b"pdot";
            let _ = initialize_relaychain(Origin::root());
            <IsHalted<TestRuntime>>::put(true);

            assert_noop!(submit_headers(1, 3), "Halted",);
        })
    }

    #[test]
    fn succesfully_imports_headers_with_valid_finality() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let data = submit_headers(1, 3).unwrap();

            assert_eq!(
                <BestFinalizedHash<TestRuntime>>::get(),
                Some(data.signed_header.hash())
            );
            assert!(<ImportedHeaders<TestRuntime>>::contains_key(
                data.signed_header.hash()
            ));
            assert!(<ImportedHeaders<TestRuntime>>::contains_key(
                data.range[0].hash()
            ));
            assert!(<ImportedHeaders<TestRuntime>>::contains_key(
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
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            assert_ok!(submit_headers(1, 5));

            let headers: Vec<TestHeader> = test_header_range(10);
            let signed_header: &TestHeader = headers.last().unwrap();
            let justification = make_default_justification(&signed_header.clone());
            let mut range: Vec<TestHeader> = headers[6..10].to_vec();
            range[1] = range[2].clone();

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
                Error::<TestRuntime>::InvalidRangeLinkage
            );
        })
    }

    #[test]
    fn reject_range_with_invalid_grandpa_linkage() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());

            assert_ok!(submit_headers(1, 5));

            let headers: Vec<TestHeader> = test_header_range(10);
            let signed_header: &TestHeader = headers.last().unwrap();

            let justification = make_default_justification(&signed_header.clone());
            // one header is missing -> Grandpa linkage invalid
            let range: Vec<TestHeader> = headers[6..9].to_vec();

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
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

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
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

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
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
            let init_data = RelaychainRegistrationData::<AccountId> {
                authorities: different_authorities,
                first_header: genesis.encode(),
                authority_set_id: 1,
                owner: 1,
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

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
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
            let data = GrandpaHeaderData::<TestHeader> {
                signed_header: signed_header.clone(),
                range,
                justification,
            };

            // Let's import our test header
            assert_ok!(Pallet::<TestRuntime>::submit_encoded_headers(data.encode()));

            // Make sure that our header is the best finalized
            assert_eq!(
                <BestFinalizedHash<TestRuntime>>::get(),
                Some(signed_header.hash())
            );
            assert!(<ImportedHeaders<TestRuntime>>::contains_key(
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
            let data = GrandpaHeaderData::<TestHeader> {
                signed_header,
                range,
                justification,
            };

            // Should not be allowed to import this header
            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
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

            let data = GrandpaHeaderData::<TestHeader> {
                signed_header,
                range,
                justification,
            };

            // Should not be allowed to import this header
            assert_noop!(
                Pallet::<TestRuntime>::submit_encoded_headers(data.encode()),
                <Error<TestRuntime>>::UnsupportedScheduledChange
            );
        })
    }

    #[test]
    fn confirm_valid_evm_source_from_encoded_evm_event() {
        run_test(|| {
            #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
            struct Log {
                address: H160,
                topics: Vec<H256>,
                data: Vec<u8>,
            }
            #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
            enum EvmEventsMock {
                Log(Log),
            }

            let evm_source: ExecutionSource = hex_literal::hex!(
                "0000000000000000000000003333333333333333333333333333333333333333"
            );
            let evm_address = H160::from_slice(
                hex_literal::hex!("3333333333333333333333333333333333333333").as_slice(),
            );
            let mut message = EvmEventsMock::Log(Log {
                address: evm_address,
                topics: vec![],
                data: vec![],
            })
            .encode();
            // insert pallet index prefix to the message
            message.insert(0, 120u8);
            assert_eq!(
                Pallet::<TestRuntime>::check_vm_source(evm_source, message),
                Ok(VMSource::EVM(evm_address.0))
            );
        });
    }

    #[test]
    fn confirm_valid_account_id_source_from_encoded_wasm_event() {
        run_test(|| {
            #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
            enum WasmEventsMock {
                Instantiated {
                    deployer: AccountId32,
                    contract: AccountId32,
                },
                Terminated {
                    contract: AccountId32,
                    beneficiary: AccountId32,
                },
                CodeStored {
                    code_hash: H256,
                },
                ContractEmitted {
                    contract: AccountId32,
                    data: Vec<u8>,
                },
            }

            let wasm_source: ExecutionSource = hex_literal::hex!(
                "4444444400000000000000003333333333333333333333333333333333333333"
            );

            let wasm_address: AccountId32 = wasm_source.into();
            let mut message = WasmEventsMock::ContractEmitted {
                contract: wasm_address,
                data: vec![1, 2, 3],
            }
            .encode();
            // insert pallet index prefix to the message
            message.insert(0, 121u8);
            assert_eq!(
                Pallet::<TestRuntime>::check_vm_source(wasm_source, message),
                Ok(VMSource::WASM(wasm_source))
            );
        });
    }

    #[test]
    fn parse_finalized_storage_proof_rejects_proof_on_unknown_header() {
        run_test(|| {
            assert_noop!(
                Pallet::<TestRuntime>::parse_finalized_storage_proof(
                    Default::default(),
                    sp_trie::StorageProof::new(vec![]),
                    |_| (),
                ),
                Error::<TestRuntime>::UnknownHeader,
            );
        });
    }

    #[test]
    fn parse_finalized_storage_accepts_valid_proof() {
        run_test(|| {
            let (state_root, storage_proof) = bp_runtime::craft_valid_storage_proof();

            let mut header = test_header(2);
            header.set_state_root(state_root);

            let hash = header.hash();
            <BestFinalizedHash<TestRuntime>>::put(hash);
            <ImportedHeaders<TestRuntime>>::insert(hash, header);

            assert_ok!(
                Pallet::<TestRuntime>::parse_finalized_storage_proof(hash, storage_proof, |_| (),),
                (),
            );
        });
    }

    #[test]
    fn should_prune_headers_over_headers_to_keep_parameter() {
        run_test(|| {
            let _ = initialize_relaychain(Origin::root());
            let headers = test_header_range(111);

            //°°°°°°°°°°°ACHTUNG!!!°°°°°°°°°°°°
            assert_ok!(submit_headers(1, 5));
            // MultiImportedHashes: [1, 2, 3, 4, 5] in MultiImportedHashes
            // Pointer               ^
            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[1].hash(),),
                true
            );
            // contains added header
            assert_ok!(submit_headers(6, 7));
            // [6, 7, 3, 4, 5]
            //        ^

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[3].hash(),),
                true
            ); // still available

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[2].hash(),),
                false
            ); // overwritten by buffer

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[1].hash(),),
                false
            ); // overwritten by buffer

            assert_ok!(submit_headers(8, 10));
            // [6, 7, 8, 9, 10]
            //  ^

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[5].hash(),),
                false
            );

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[4].hash(),),
                false
            );

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[3].hash(),),
                false
            );

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[6].hash(),),
                true
            );

            assert_ok!(submit_headers(11, 15));
            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[10].hash(),),
                false
            );

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[11].hash(),),
                true
            );

            assert_eq!(
                <ImportedHeaders<TestRuntime>>::contains_key(headers[12].hash(),),
                true
            );
        })
    }
}
