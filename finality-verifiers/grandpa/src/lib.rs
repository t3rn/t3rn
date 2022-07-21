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

// this doesnt work
// pub mod bridges;
// use bridges::{header_chain as bp_header_chain, runtime as bp_runtime};

// this does work
use t3rn_primitives::bridges::{header_chain as bp_header_chain, runtime as bp_runtime};
use bp_header_chain::{justification::GrandpaJustification, InitializationData};
use bp_runtime::{BlockNumberOf, Chain, ChainId, HashOf, HasherOf, HeaderOf};

use finality_grandpa::voter_set::VoterSet;
use frame_support::{ensure, pallet_prelude::*, StorageHasher};
use frame_system::{ensure_signed, RawOrigin};
use scale_info::prelude::string::String;
use sp_finality_grandpa::{ConsensusLog, GRANDPA_ENGINE_ID};
use sp_runtime::traits::{BadOrigin, Header as HeaderT, Zero};
use sp_std::vec::Vec;
use sp_core::crypto::ByteArray;
mod types;
use types::{GrandpaRegistrationData};
use sp_trie::{read_trie_value, LayoutV1, StorageProof};

#[cfg(test)]
mod mock;

/// Pallet containing weights for this pallet.
pub mod weights;

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

const LOG_TARGET: &str = "multi-finality-verifier";
use frame_system::pallet_prelude::*;
use crate::types::{Parachain, RelaychainHeaderData, ParachainHeaderData};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// The chain we are bridging to here.
        type BridgedChain: Chain;

        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;
        /// The upper bound on the number of requests allowed by the pallet.
        ///
        /// A request refers to an action which writes a header to storage.
        ///
        /// Once this bound is reached the pallet will not allow any dispatchables to be called
        /// until the request count has decreased.
        #[pallet::constant]
        type MaxRequests: Get<u32>;

        /// Maximal number of finalized headers to keep in the storage.
        ///
        /// The setting is there to prevent growing the on-chain state indefinitely. Note
        /// the setting does not relate to block numbers - we will simply keep as much items
        /// in the storage, so it doesn't guarantee any fixed timeframe for finality headers.
        #[pallet::constant]
        type HeadersToKeep: Get<u32>;

        /// Weights gathered through benchmarking.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
        fn on_initialize(_n: T::BlockNumber) -> frame_support::weights::Weight {
            let mut acc_weight = 0_u64;

            for gateway_id in <InstantiatedGatewaysMap<T, I>>::get() {
                <RequestCountMap<T, I>>::mutate(gateway_id, |count| match count {
                    Some(count) => *count = count.saturating_sub(1),
                    _ => *count = Some(0),
                });

                acc_weight = acc_weight
                    .saturating_add(T::DbWeight::get().reads(1))
                    .saturating_add(T::DbWeight::get().writes(1));
            }
            acc_weight
        }
    }

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::weight(<T as pallet::Config<I>>::WeightInfo::submit_finality_proof_on_single_fork(
            headers_reversed.len() as u32,
		))]
        pub fn submit_header_range(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            headers_reversed: Vec<BridgedHeader<T, I>>,
            anchor_header_hash: BridgedBlockHash<T, I>,
        ) -> DispatchResultWithPostInfo {
            ensure_operational_single::<T, I>(gateway_id)?;
            ensure_signed(origin)?;
            ensure!(
                Self::request_count_map(gateway_id).unwrap_or(0) < T::MaxRequests::get(),
                <Error<T, I>>::TooManyRequests
            );
            // not ideal because we're doing 2 reads
            ensure!(
                <MultiImportedHeaders<T, I>>::contains_key(gateway_id, anchor_header_hash),
                <Error<T, I>>::InvalidAnchorHeader
            );

            // fetch the 'anchor' (block we're basing the proof on), knowing its been verified
            let mut anchor_header =
                <MultiImportedHeaders<T, I>>::try_get(gateway_id, anchor_header_hash).unwrap();

            let height = anchor_header.number().clone();
            // this is safe, u32 gives us enough space for 300 days worth of blocks.
            let range: u32 = headers_reversed.len().clone().try_into().unwrap();
            let mut index = <MultiImportedHashesPointer<T, I>>::get(gateway_id).unwrap_or_default();

            for header in headers_reversed {
                if *anchor_header.parent_hash() == header.hash() {
                    // currently this allows overwrites. Block 1 is already proven via GRANDPA, but we overwrite it.
                    // We could add additional checks, but not sure if thats worth it
                    <MultiImportedHeaders<T, I>>::insert(gateway_id, header.hash(), header.clone());
                    <MultiImportedHashes<T, I>>::insert(gateway_id, index, header.hash());
                    <MultiImportedRoots<T, I>>::insert(
                        gateway_id,
                        header.hash(),
                        (header.extrinsics_root(), header.state_root()),
                    );

                    // select next header to prune and remove
                    index += 1;
                    let pruning = <MultiImportedHashes<T, I>>::try_get(gateway_id, index);

                    if let Ok(hash) = pruning {
                        <MultiImportedHeaders<T, I>>::remove(gateway_id, hash);
                        <MultiImportedRoots<T, I>>::remove(gateway_id, hash);
                    }

                    anchor_header = header;
                } else {
                    log::info!(
                        "Invalid header detected: {:?}, skipping the remaining imports for gateway {:?}!",
                        header,
                        gateway_id
                    );
                }
            }

            // update ring buffer pointer
            <MultiImportedHashesPointer<T, I>>::insert(gateway_id, index % T::HeadersToKeep::get());

            // im guessing this should count as one?
            <RequestCountMap<T, I>>::mutate(gateway_id, |count| {
                match count {
                    Some(count) => *count += 1,
                    None => *count = Some(1),
                }
                *count
            });

            Self::deposit_event(Event::NewHeaderRangeAvailable(gateway_id, height, range));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        NewHeaderRangeAvailable(ChainId, BridgedBlockNumber<T, I>, u32),
    }

    /// The current number of requests which have written to storage.
    ///
    /// If the `RequestCount` hits `MaxRequests`, no more calls will be allowed to the pallet until
    /// the request capacity is increased.
    ///
    /// The `RequestCount` is decreased by one at the beginning of every block. This is to ensure
    /// that the pallet can always make progress.
    #[pallet::storage]
    #[pallet::getter(fn request_count_map)]
    pub(super) type RequestCountMap<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_256, ChainId, u32>;

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
    pub(super) type PalletOwner<T: Config<I>, I: 'static = ()> =
        StorageValue<_, T::AccountId, OptionQuery>;

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
        /// The given justification is invalid for the given header.
        InvalidJustification,
        /// The authority set from the underlying header chain is invalid.
        InvalidAuthoritySet,
        /// There are too many requests for the current window to handle.
        TooManyRequests,
        /// The header being imported is older than the best finalized header known to the pallet.
        OldHeader,
        /// The header is unknown to the pallet.
        UnknownHeader,
        /// The scheduled authority set change found in the header is unsupported by the pallet.
        ///
        /// This is the case for non-standard (e.g forced) authority set changes.
        UnsupportedScheduledChange,
        /// The pallet has already been initialized.
        AlreadyInitialized,
        /// All pallet operations are halted.
        Halted,
        /// The storage proof doesn't contains storage root. So it is invalid for given header.
        StorageRootMismatch,
        // Submitted anchor header(verified header stored on circuit) was not found
        InvalidAnchorHeader,
        // No finalized header known for the corresponding gateway.
        NoFinalizedHeader,
        // submitted gateway_id does not have the parachain field set
        NoParachainEntryFound,
    }

    /// Verify a target header is finalized according to the given finality proof.
    ///
    /// It will use the underlying storage pallet to fetch information about the current
    /// authorities and best finalized header in order to verify that the header is finalized.
    ///
    /// If successful in verification, it will write the target header to the underlying storage
    /// pallet.
    pub(crate) fn submit_relaychain_header<T: pallet::Config<I>, I>(
        gateway_id: ChainId,
        header: BridgedHeader<T, I>,
        justification: GrandpaJustification::<BridgedHeader<T, I>>
    ) -> Result<(), &'static str> {
        ensure!(
            <RequestCountMap<T,I>>::get(gateway_id).unwrap_or(0) < T::MaxRequests::get(),
            "Too many Requests"
        );

        log::debug!(
            target: LOG_TARGET,
            "Going to try and finalize header {:?} gateway {:?}",
            header,
            String::from_utf8_lossy(gateway_id.as_ref()).into_owned()
        );

        let (hash, number) = (header.hash(), header.number());

        // In order to reach this point the bridge must have been initialized for given gateway.
        let best_finalized = <MultiImportedHeaders<T, I>>::get(
            gateway_id,
            // Every time `BestFinalized` is updated `ImportedHeaders` is also updated. Therefore
            // `ImportedHeaders` must contain an entry for `BestFinalized`.
            <BestFinalizedMap<T, I>>::get(gateway_id)
                .ok_or_else(|| "NoFinalizedHeader")?,
        )
        .ok_or_else(|| "NoFinalizedHeader")?;
        // We do a quick check here to ensure that our header chain is making progress and isn't
        // "travelling back in time" (which could be indicative of something bad, e.g a hard-fork).
        ensure!(best_finalized.number() < number, <Error<T, I>>::OldHeader);
        let authority_set = <CurrentAuthoritySet<T, I>>::get()
            // Expects authorities to be set before verify_justification
            .ok_or_else(|| "InvalidAuthoritySet")?;

        let set_id = authority_set.set_id;
        verify_justification_single::<T, I>(
            &justification,
            hash,
            *number,
            authority_set,
            gateway_id,
        )?;

        // We have to incentivise authority_set update submissions in some way. Important to receive proofs of changing set, even when no transaction is included
        let _enacted =
            try_enact_authority_change_single::<T, I>(&header, set_id, gateway_id)?;

        let index = <MultiImportedHashesPointer<T, I>>::get(gateway_id).unwrap_or_default();

        let pruning = <MultiImportedHashes<T, I>>::try_get(gateway_id, index);

        <BestFinalizedMap<T, I>>::insert(gateway_id, hash);
        <MultiImportedHeaders<T, I>>::insert(gateway_id, hash, header.clone());
        <MultiImportedHashes<T, I>>::insert(gateway_id, index, hash);
        <MultiImportedRoots<T, I>>::insert(
            gateway_id,
            hash,
            (
                header.extrinsics_root(),
                header.state_root(),
            ),
        );
        <RequestCountMap<T, I>>::mutate(gateway_id, |count| {
            match count {
                Some(count) => *count += 1,
                None => *count = Some(1),
            }
            *count
        });

        // Update ring buffer pointer and remove old header.
        <MultiImportedHashesPointer<T, I>>::insert(
            gateway_id,
            (index + 1) % T::HeadersToKeep::get(),
        );

        if let Ok(hash) = pruning {
            log::debug!(
                target: LOG_TARGET,
                "Pruning old header: {:?} for gateway {:?}.",
                hash,
                gateway_id
            );
            <MultiImportedHeaders<T, I>>::remove(gateway_id, hash);
            <MultiImportedRoots<T, I>>::remove(gateway_id, hash);
        }
        log::debug!(
            target: LOG_TARGET,
            "Successfully imported finalized header with hash {:?} for gateway {:?}!",
            hash,
            gateway_id
        );

        log::debug!(
            target: LOG_TARGET,
            "Successfully updated gateway {:?}!",
            gateway_id,
        );
        Ok(().into())
    }

    /// Verify a target parachain header can be verified with its relaychains storage_proof
    ///
    /// It will use the underlying storage pallet to fetch the relaychains header, which is then used to verify the storage_proof
    ///
    /// If successful in verification, it will write the target header to the underlying storage
    /// pallet.
    ///
    pub(crate) fn submit_parachain_header<T: pallet::Config<I>, I>(
        gateway_id: ChainId,
        proof: StorageProof,
        relay_block_hash: BridgedBlockHash<T, I>
    ) -> Result<(), &'static str> {
        let parachain = <ParachainIdMap<T, I>>::try_get(gateway_id)
            .map_err(|_| "Parachain not registered")?;

        let header: BridgedHeader<T, I> = verify_storage_proof::<T, I>(
            relay_block_hash,
            proof,
            parachain
        )?;

        let hash = header.hash();
        let index = <MultiImportedHashesPointer<T, I>>::get(gateway_id).unwrap_or_default();
        let pruning = <MultiImportedHashes<T, I>>::try_get(gateway_id, index);

        <BestFinalizedMap<T, I>>::insert(gateway_id, hash);

        <MultiImportedHeaders<T, I>>::insert(gateway_id, hash, header.clone());
        <MultiImportedHashes<T, I>>::insert(gateway_id, index, hash);
        <MultiImportedRoots<T, I>>::insert(
            gateway_id,
            hash,
            (header.extrinsics_root(), header.state_root()),
        );

        <RequestCountMap<T, I>>::mutate(gateway_id, |count| {
            match count {
                Some(count) => *count += 1,
                None => *count = Some(1),
            }
            *count
        });

        // Update ring buffer pointer and remove old header.
        <MultiImportedHashesPointer<T, I>>::insert(
            gateway_id,
            (index + 1) % T::HeadersToKeep::get(),
        );

        if let Ok(hash) = pruning {
            log::debug!(
                target: LOG_TARGET,
                "Pruning old header: {:?} for gateway {:?}.",
                hash,
                gateway_id
            );
            <MultiImportedHeaders<T, I>>::remove(gateway_id, hash);
            <MultiImportedRoots<T, I>>::remove(gateway_id, hash);
        }

        Ok(().into())
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
    ) -> Result<(), &'static str> {
        use bp_header_chain::justification::verify_justification;

        let voter_set =
            VoterSet::new(authority_set.authorities).ok_or("InvalidAuthoritySet")?;
        let set_id = authority_set.set_id;

        Ok(verify_justification::<BridgedHeader<T, I>>(
            (hash, number),
            set_id,
            &voter_set,
            justification,
        )
        .map_err(|e| {
            log::error!("Received invalid justification for {:?}: {:?}", hash, e);
            "InvalidJustification"
        })?)
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
        <InitialHashMap<T, I>>::insert(gateway_id, initial_hash);
        <BestFinalizedMap<T, I>>::insert(gateway_id, initial_hash);
        <MultiImportedHeaders<T, I>>::insert(gateway_id, initial_hash, header);
        // might get problematic
        let authority_set = bp_header_chain::AuthoritySet::new(authority_list, set_id);
        <CurrentAuthoritySet<T, I>>::put(authority_set);
        <RelayChainId<T, I>>::put(gateway_id);
        <IsHaltedMap<T, I>>::insert(gateway_id, is_halted);
        <PalletOwnerMap::<T, I>>::insert(gateway_id, owner);
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
        parachain: Parachain
    ) -> Result<(), &'static str> {
        can_init_para_chain::<T, I>(&parachain)?;
        let initial_hash = header.hash();
        <InitialHashMap<T, I>>::insert(gateway_id, initial_hash);
        <BestFinalizedMap<T, I>>::insert(gateway_id, initial_hash);
        <MultiImportedHeaders<T, I>>::insert(gateway_id, initial_hash, header);
        <IsHaltedMap<T, I>>::insert(gateway_id, is_halted);
        <ParachainIdMap<T, I>>::insert(gateway_id, parachain);
        <PalletOwnerMap::<T, I>>::insert(gateway_id, owner);
        <InstantiatedGatewaysMap<T, I>>::mutate(|gateways| {
            gateways.push(gateway_id);
            gateways.len() + 1
        });

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
    ) -> Result<R, sp_runtime::DispatchError> {
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
        ensure_owner_or_root_single::<T, I>(origin.clone(), gateway_id)?;
        let init_allowed = !<BestFinalizedMap<T, I>>::contains_key(gateway_id);
        ensure!(init_allowed, "Already initialized");
        let registration_data: GrandpaRegistrationData<T::AccountId> = Decode::decode(&mut &*encoded_registration_data)
            .map_err(|_| "Registration data decoding error!")?;
        let header: BridgedHeader<T, I> = Decode::decode(&mut &registration_data.first_header[..])
            .map_err(|_| "Decoding error: received GenericPrimitivesHeader -> CurrentHeader<T>")?;
        match registration_data.parachain {
            Some(parachain) => initialize_para_chain::<T, I>(header, false, gateway_id, registration_data.owner, parachain),
            None => {
                ensure!(
                    registration_data.authorities != None &&
                    registration_data.authority_set_id != None,
                    "Relaychain parameters missing"
                );

                let init_data = bp_header_chain::InitializationData {
                    header,
                    authority_list: registration_data.authorities.unwrap()
                        .iter()
                        .map(|id| sp_finality_grandpa::AuthorityId::from_slice(&id.encode()).unwrap())// not sure why this is still needed
                        .map(|authority| (authority, 1))
                        .collect::<Vec<_>>(),
                    set_id: registration_data.authority_set_id.unwrap(),
                    is_halted: false,
                    gateway_id
                };
                initialize_relay_chain::<T, I>(init_data.clone(), registration_data.owner)
            }
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
        let new_owner: Option<T::AccountId> = Decode::decode(&mut &*encoded_new_owner)
            .map_err(|_| "New Owner decoding error")?;

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

    pub fn submit_header(
        origin: OriginFor<T>,
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>
    ) -> Result<(), &'static str> {
        ensure_operational_single::<T, I>(gateway_id)?;
        ensure_signed(origin)?;
        if Some(gateway_id) == <RelayChainId<T, I>>::get() {
            let data: RelaychainHeaderData<BridgedHeader<T, I>> = Decode::decode(&mut &*encoded_header_data)
                .map_err(|_| "Error decoding relaychain header data")?;

            submit_relaychain_header::<T, I>(
                gateway_id,
                data.header,
                data.justification
            )
        } else {
            let data: ParachainHeaderData<BridgedBlockHash<T, I>> = Decode::decode(&mut &*encoded_header_data)
                .map_err(|_| "Error decoding parachain header data")?;

            submit_parachain_header::<T, I>(
                gateway_id,
                data.proof,
                data.relay_block_hash
            )
        }
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
    return match <RelayChainId<T, I>>::exists() {
        true => Err("Duplicate relaychain"), // we have a relaychain registered already
        false => Ok(()) // is first relaychain
    }
}

/// Ensure that no relaychain has been set so far. Relaychains are unique
fn can_init_para_chain<T: Config<I>, I: 'static>(
    parachain: &Parachain,
) -> Result<(), &'static str> {
    return match <RelayChainId<T, I>>::get() {
        Some(relay_chain_id) => { // we have a relaychain setup
            match relay_chain_id == parachain.relay_chain_id {
                true => Ok(()), // registering relachchain_id matches the stored one
                false => Err("Invalid relaychainId") // wrong relaychain_id was passed
            }
        }
        None => Err("No relaychain") // we have no relaychain setup
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

pub(crate) fn verify_storage_proof<T: Config<I>, I: 'static>(
    relay_block_hash: BridgedBlockHash<T, I>,
    proof: StorageProof,
    parachain: Parachain
) -> Result<BridgedHeader<T, I>, &'static str> {
    // partial StorageKey for Paras_Heads. We now need to append the parachain_id as LE-u32 to generate the parachains StorageKey
    // This is a bit unclean, but it makes no sense to hash the StorageKey for each exec
    let mut key: Vec<u8> = [205,113,11,48,189,46,171,3,82,221,204,38,65,122,161,148,27,60,37,47,203,41,216,142,255,79,61,229,222,68,118,195].to_vec();
    let mut arg = Twox64Concat::hash(parachain.id.encode().as_ref());
    key.append(&mut arg); // complete storage key

    let (_, storage_root) = <MultiImportedRoots<T, I>>::get(parachain.relay_chain_id, relay_block_hash)
        .ok_or("Relaychain storage root not available")?;

    let db = proof.into_memory_db::<BridgedBlockHasher<T, I>>();
    let res = read_trie_value::<LayoutV1<BridgedBlockHasher<T, I>>, _>(&db, &storage_root, key.as_ref());

    return match res {
        Ok(Some(data)) => {
            // Not ideal, but can't come up with something more concise
            let encoded_header: Vec<u8> = Decode::decode(&mut &data[..]).map_err(|_| "Header decoding error")?;
            let header: BridgedHeader<T, I> = Decode::decode(&mut &*encoded_header).map_err(|_| "Header decoding error")?;
            Ok(header)
        },
        _ => {
            Err("Parachain header verification error")
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{
        run_test, test_header, test_header_range, Origin, TestHash, TestHeader, TestNumber,
        TestRuntime, AccountId
    };
    use bp_test_utils::{
        authority_list, make_default_justification, make_justification_for_header,
        JustificationGeneratorParams, ALICE, BOB,
    };
    use codec::Encode;
    use frame_support::{assert_err, assert_noop, assert_ok};
    use sp_finality_grandpa::AuthorityId;
    use sp_runtime::{Digest, DigestItem, DispatchError};

    use t3rn_primitives::{
        bridges::test_utils as bp_test_utils,
    };
    use t3rn_primitives::bridges::test_utils::{DAVE};

    use crate::types::{RelaychainHeaderData, ParachainHeaderData};
    // fn teardown_substrate_bridge() {
    //     let default_gateway: ChainId = *b"pdot";
    //     // Reset storage so we can initialize the pallet again
    //     BestFinalizedMap::<TestRuntime>::remove(default_gateway);
    //     PalletOwnerMap::<TestRuntime>::remove(default_gateway);
    //     MultiImportedRoots::<TestRuntime>::remove_prefix(default_gateway, None);
    //     BestFinalizedMap::<TestRuntime>::remove(default_gateway);
    //     MultiImportedHashesPointer::<TestRuntime>::remove(default_gateway);
    //     MultiImportedHashes::<TestRuntime>::remove_prefix(default_gateway, None);
    //     MultiImportedHeaders::<TestRuntime>::remove_prefix(default_gateway, None);
    //     MultiImportedRoots::<TestRuntime>::remove_prefix(default_gateway, None);
    //     RequestCountMap::<TestRuntime>::remove(default_gateway);
    //     InstantiatedGatewaysMap::<TestRuntime>::kill();
    // }

    fn initialize_relaychain(
        origin: Origin,
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: Some(t3rn_primitives::bridges::test_utils::authorities()),
            first_header: genesis.encode(),
            authority_set_id: Some(1),
            owner: 1,
            parachain: None
        };

        initialize_custom_relaychain(origin,  *b"pdot", init_data)
    }

    fn initialize_named_relaychain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: Some(t3rn_primitives::bridges::test_utils::authorities()),
            first_header: genesis.encode(),
            authority_set_id: Some(1),
            owner: 1,
            parachain: None
        };

        initialize_custom_relaychain(origin,  gateway_id, init_data)
    }

    fn initialize_custom_relaychain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: GrandpaRegistrationData::<AccountId>
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        Pallet::<TestRuntime>::initialize(origin,  gateway_id, init_data.encode()).map(|_| init_data)
    }

    fn initialize_parachain(
        origin: Origin,
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: None,
            first_header: genesis.encode(),
            authority_set_id: None,
            owner: 1,
            parachain: Some(Parachain{
                relay_chain_id: *b"pdot",
                id: 0
            })
        };

         initialize_custom_parachain(origin,  *b"moon", init_data)
    }

    fn initialize_named_parachain(
        origin: Origin,
        gateway_id: ChainId,
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        let genesis = test_header(0);
        let init_data = GrandpaRegistrationData::<AccountId> {
            authorities: None,
            first_header: genesis.encode(),
            authority_set_id: None,
            owner: 1,
            parachain: Some(Parachain {
                relay_chain_id: *b"pdot",
                id: 0
            })
        };

        initialize_custom_parachain(origin,  gateway_id, init_data)
    }

    fn initialize_custom_parachain(
        origin: Origin,
        gateway_id: ChainId,
        init_data: GrandpaRegistrationData::<AccountId>
    ) -> Result<GrandpaRegistrationData::<AccountId>, &'static str> {
        Pallet::<TestRuntime>::initialize(origin,  gateway_id, init_data.encode()).map(|_| init_data)
    }

    fn submit_finality_proof(header: u8) -> Result<(), &'static str> {
        let header = test_header(header.into());

        let justification = make_default_justification(&header);

        let default_gateway: ChainId = *b"pdot";
        let data = RelaychainHeaderData::<TestHeader> {
            header,
            justification
        };

        Pallet::<TestRuntime>::submit_header(
            Origin::signed(1),
            default_gateway,
            data.encode()
        )
    }

    fn submit_finality_proof_with_header(
        header: TestHeader,
    ) -> Result<(), &'static str> {
        let justification = make_default_justification(&header);
        let default_gateway: ChainId = *b"pdot";

        let data = RelaychainHeaderData::<TestHeader> {
            header,
            justification
        };

        Pallet::<TestRuntime>::submit_header(
            Origin::signed(1),
            default_gateway,
            data.encode()
        )
    }

    pub fn submit_header_range(
        headers_reversed: Vec<TestHeader>,
        anchor_header_hash: TestHash,
    ) -> frame_support::dispatch::DispatchResultWithPostInfo {
        let default_gateway: ChainId = *b"pdot";

        Pallet::<TestRuntime>::submit_header_range(
            Origin::signed(1),
            default_gateway,
            headers_reversed,
            anchor_header_hash,
        )
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
            assert_noop!(
                initialize_relaychain(Origin::signed(1)),
                "Bad origin"
            );
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
            assert_err!(initialize_named_parachain(Origin::root(), *b"pdot"), "Already initialized");
         })
    }

    #[test]
    fn cant_register_duplicate_parachain() {
         run_test(|| {
            assert_ok!(initialize_relaychain(Origin::root()));
            assert_noop!(initialize_named_relaychain(Origin::root(), *b"roco"), "Duplicate relaychain");
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
                parachain: Some(Parachain{
                    relay_chain_id: *b"roco",
                    id: 0
                })
            };
            assert_noop!(initialize_custom_parachain(Origin::root(), *b"moon", init_data), "Invalid relaychainId");
         })
    }

    #[test]
    fn cant_register_relaychain_as_non_root() {
         run_test(|| {
            assert_err!(
                initialize_relaychain(Origin::signed(1)),
                "Bad origin"
            );
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
                parachain: None
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
                authorities: Some(t3rn_primitives::bridges::test_utils::authorities()),
                first_header: genesis.encode(),
                authority_set_id: None,
                owner: 1,
                parachain: None
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
            let _ =initialize_relaychain(Origin::root());
            assert_noop!(
                initialize_relaychain(Origin::root()),
                "Already initialized"
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
            assert_ok!(Pallet::<TestRuntime>::set_operational(
                Origin::root(),
                false,
                default_gateway
            ));
            // assert_noop!(
            //     submit_finality_proof(2),
            //     "Halted"
            // ); ToDo
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
            let _ =initialize_relaychain(Origin::root());
            <IsHaltedMap<TestRuntime>>::insert(gateway_a, true);

            let header = test_header(1);
            let justification = make_default_justification(&header);

            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };

            assert_noop!(
                Pallet::<TestRuntime>::submit_header(
                    Origin::signed(1),
                    gateway_a,
                    data.encode(),
                ),
                Error::<TestRuntime>::Halted,
            );
        })
    }

    #[test]
    fn succesfully_imports_header_with_valid_finality() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            assert_ok!(submit_finality_proof(1));

            let header = test_header(1);
            assert_eq!(
                <BestFinalizedMap<TestRuntime>>::get(default_gateway),
                Some(header.hash())
            );
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                header.hash()
            ));
        })
    }

    #[test]
    fn succesfully_imports_header_ranges() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            // generate valid headers
            let mut headers = test_header_range(0, 10, None);

            assert_ok!(submit_finality_proof_with_header(headers[1].clone()));
            assert_ok!(submit_finality_proof_with_header(headers[10].clone()));
            next_block();

            // verified header stored in circuit we're basing the proof on
            let anchor_header = headers.pop().unwrap();

            // we want to submit the headers in reverse, as we have to iterate backwards
            headers.reverse();

            assert_ok!(submit_header_range(headers.clone(), anchor_header.hash()));

            headers.reverse(); // reversing for tests because I struggle to think backwards

            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                headers[9].hash()
            ));

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::try_get(default_gateway, headers[9].hash()),
                Ok(headers[9].clone())
            );

            assert_eq!(
                <MultiImportedHashes<TestRuntime>>::try_get(default_gateway, 2),
                Ok(headers[9].hash())
            );

            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                headers[8].hash()
            ));

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::try_get(default_gateway, headers[8].hash()),
                Ok(headers[8].clone())
            );

            assert_eq!(
                <MultiImportedHashes<TestRuntime>>::try_get(default_gateway, 3),
                Ok(headers[8].hash())
            );

            assert_eq!(
                <MultiImportedHashesPointer<TestRuntime>>::try_get(default_gateway),
                Ok(2) // ring buffer size is 5 -> 12 % 5 = 2
            );
        })
    }

    #[test]
    fn succesfully_imports_partial_header_ranges() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            // generate valid headers
            let mut headers = test_header_range(0, 10, None);
            assert_ok!(submit_finality_proof_with_header(headers[1].clone()));
            assert_ok!(submit_finality_proof_with_header(headers[10].clone()));
            next_block();
            // verified header stored in circuit we're basing the proof on
            let anchor_header = headers.pop().unwrap();

            // we want to submit the headers in reverse, as we have to iterate backwards
            headers.reverse();

            headers[1] = headers[2].clone(); // create an invalid chain after block 9 -> block 9 should be added, block 8 not

            assert_ok!(submit_header_range(headers.clone(), anchor_header.hash()));

            headers.reverse(); // reversing for tests because I struggle to think backwards

            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                headers[9].hash()
            ));

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::try_get(default_gateway, headers[9].hash()),
                Ok(headers[9].clone())
            );

            assert_eq!(
                <MultiImportedHashes<TestRuntime>>::try_get(default_gateway, 2),
                Ok(headers[9].hash())
            );

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[8].hash()
                ),
                false
            );

            assert_eq!(
                <MultiImportedHashesPointer<TestRuntime>>::try_get(default_gateway),
                Ok(3)
            );
        })
    }

    #[test]
    fn reject_invalid_header_ranges() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            // generate valid headers
            let mut headers = test_header_range(0, 10, None);
            assert_ok!(submit_finality_proof_with_header(headers[1].clone()));
            assert_ok!(submit_finality_proof_with_header(headers[10].clone()));
            next_block();
            // verified header stored in circuit we're basing the proof on
            let anchor_header = headers.pop().unwrap();

            // we want to submit the headers in reverse, as we have to iterate backwards
            headers.reverse();

            headers[0] = headers[1].clone(); // range is now invalid, nothing should be added

            assert_ok!(submit_header_range(headers.clone(), anchor_header.hash()));

            headers.reverse(); // reversing for tests because I struggle to think backwards

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[9].hash()
                ),
                false
            );

            assert_eq!(
                <MultiImportedHashesPointer<TestRuntime>>::try_get(default_gateway),
                Ok(2)
            );
        })
    }

    #[test]
    fn reject_invalid_anchor() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            // generate valid headers
            let mut headers = test_header_range(0, 10, None);
            assert_ok!(submit_finality_proof_with_header(headers[1].clone()));
            assert_ok!(submit_finality_proof_with_header(headers[10].clone()));
            next_block();

            // verified header stored in circuit we're basing the proof on
            headers.pop().unwrap();

            // we want to submit the headers in reverse, as we have to iterate backwards
            headers.reverse();

            headers[0] = headers[1].clone(); // range is now invalid, nothing should be added

            assert_err!(
                submit_header_range(headers.clone(), headers[2].clone().hash()),
                <Error<TestRuntime>>::InvalidAnchorHeader
            );

            headers.reverse(); // reversing for tests because I struggle to think backwards

            assert_eq!(
                <MultiImportedHeaders<TestRuntime>>::contains_key(
                    default_gateway,
                    headers[9].hash()
                ),
                false
            );

            assert_eq!(
                <MultiImportedHashesPointer<TestRuntime>>::try_get(default_gateway),
                Ok(2)
            );
        })
    }

    #[test]
    fn rejects_justification_that_skips_authority_set_transition() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());

            let header = test_header(1);

            let params = JustificationGeneratorParams::<TestHeader> {
                set_id: 2,
                ..Default::default()
            };
            let justification = make_justification_for_header(params);

            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };

            let default_gateway: ChainId = *b"pdot";

            assert_err!(
                Pallet::<TestRuntime>::submit_header(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                "InvalidJustification"
            );
        })
    }

    #[test]
    fn does_not_import_header_with_invalid_finality_proof() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());

            let header = test_header(1);
            let mut justification = make_default_justification(&header);
            justification.round = 42;
            let justification = justification;

            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };
            let default_gateway: ChainId = *b"pdot";

            assert_err!(
                Pallet::<TestRuntime>::submit_header(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                "InvalidJustification"
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
                parachain: None
            };

            assert_ok!(initialize_custom_relaychain(
                Origin::root(),
                default_gateway,
                init_data,
            ));

            let header = test_header(2);
            // this justification contains ALICE, BOB and CHARLIE, to it will be invalid
            let justification = make_default_justification(&header);

            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };

            assert_err!(
                Pallet::<TestRuntime>::submit_header(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                ),
                "InvalidJustification"
            );
        })
    }

    #[test]
    fn importing_header_ensures_that_chain_is_extended() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());

            assert_ok!(submit_finality_proof(4));
            assert_err!(submit_finality_proof(3), Error::<TestRuntime>::OldHeader);
            assert_ok!(submit_finality_proof(5));
        })
    }

    #[test]
    fn importing_header_enacts_new_authority_set() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());

            let next_set_id = 2;
            let next_authorities = vec![(ALICE.into(), 1), (BOB.into(), 1)];

            // Need to update the header digest to indicate that our header signals an authority set
            // change. The change will be enacted when we import our header.
            let mut header = test_header(2);
            header.digest = change_log(0);

            let justification = make_default_justification(&header);
            let data = RelaychainHeaderData::<TestHeader> {
                header: header.clone(),
                justification
            };

            let default_gateway: ChainId = *b"pdot";

            // Let's import our test header
            assert_ok!(Pallet::<TestRuntime>::submit_header(
                Origin::signed(1),
                default_gateway,
                data.encode()
            ));

            // Make sure that our header is the best finalized
            assert_eq!(
                <BestFinalizedMap<TestRuntime>>::get(default_gateway),
                Some(header.hash())
            );
            assert!(<MultiImportedHeaders<TestRuntime>>::contains_key(
                default_gateway,
                header.hash()
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
            let _ =initialize_relaychain(Origin::root());

            // Need to update the header digest to indicate that our header signals an authority set
            // change. However, the change doesn't happen until the next block.
            let mut header = test_header(2);
            header.digest = change_log(1);

            let justification = make_default_justification(&header);
            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };

            let default_gateway: ChainId = *b"pdot";

            // Should not be allowed to import this header
            assert_err!(
                Pallet::<TestRuntime>::submit_header(
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
            let _ =initialize_relaychain(Origin::root());

            // Need to update the header digest to indicate that it signals a forced authority set
            // change.
            let mut header = test_header(2);
            header.digest = forced_change_log(0);

            let justification = make_default_justification(&header);

            let data = RelaychainHeaderData::<TestHeader> {
                header,
                justification
            };

            let default_gateway: ChainId = *b"pdot";

            // Should not be allowed to import this header
            assert_err!(
                Pallet::<TestRuntime>::submit_header(
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
    fn rate_limiter_disallows_imports_once_limit_is_hit_in_single_block() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());

            assert_ok!(submit_finality_proof(1));
            assert_ok!(submit_finality_proof(2));
            assert_err!(
                submit_finality_proof(3),
                "Too many Requests"
            );
        })
    }

    #[test]
    fn rate_limiter_invalid_requests_do_not_count_towards_request_count() {
        let default_gateway: ChainId = *b"pdot";
        run_test(|| {
            let submit_invalid_request = || {
                let header = test_header(1);
                let mut invalid_justification = make_default_justification(&header);
                invalid_justification.round = 42;
                let justification = invalid_justification;
                let data = RelaychainHeaderData::<TestHeader> {
                    header,
                    justification
                };

                Pallet::<TestRuntime>::submit_header(
                    Origin::signed(1),
                    default_gateway,
                    data.encode()
                )
            };

            let _ =initialize_relaychain(Origin::root());

            for _ in 0..<TestRuntime as Config>::MaxRequests::get() + 1 {
                // Notice that the error here *isn't* `TooManyRequests`
                assert_err!(
                    submit_invalid_request(),
                    "InvalidJustification"
                );
            }

            // Can still submit `MaxRequests` requests afterwards
            assert_ok!(submit_finality_proof(1));
            assert_ok!(submit_finality_proof(2));
            assert_err!(
                submit_finality_proof(3),
                "Too many Requests"
            );
        })
    }

    #[test]
    fn rate_limiter_allows_request_after_new_block_has_started() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            assert_ok!(submit_finality_proof(1));
            assert_ok!(submit_finality_proof(2));

            next_block();
            assert_ok!(submit_finality_proof(3));
        })
    }

    #[test]
    fn rate_limiter_disallows_imports_once_limit_is_hit_across_different_blocks() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            assert_ok!(submit_finality_proof(1));
            assert_ok!(submit_finality_proof(2));

            next_block();
            assert_ok!(submit_finality_proof(3));
            assert_err!(
                submit_finality_proof(4),
                "Too many Requests"
            );
        })
    }

    #[test]
    fn rate_limiter_allows_max_requests_after_long_time_with_no_activity() {
        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            assert_ok!(submit_finality_proof(1));
            assert_ok!(submit_finality_proof(2));

            next_block();
            next_block();

            next_block();
            assert_ok!(submit_finality_proof(5));
            assert_ok!(submit_finality_proof(7));
        })
    }

    #[test]
    fn should_prune_headers_over_headers_to_keep_parameter() {
        let default_gateway: ChainId = *b"pdot";

        run_test(|| {
            let _ =initialize_relaychain(Origin::root());
            assert_ok!(submit_finality_proof(1));
            let first_header = Pallet::<TestRuntime>::best_finalized_map(default_gateway);
            next_block();

            assert_ok!(submit_finality_proof(2));
            next_block();
            assert_ok!(submit_finality_proof(3));
            next_block();
            assert_ok!(submit_finality_proof(4));
            next_block();
            assert_ok!(submit_finality_proof(5));
            next_block();

            assert_ok!(submit_finality_proof(6));

            assert!(
                !Pallet::<TestRuntime>::is_known_header(first_header.hash(), default_gateway),
                "First header should be pruned."
            );
        })
    }
}