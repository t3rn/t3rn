#![cfg_attr(not(feature = "std"), no_std)]
use crate::{pallet::Config, Error, GenesisValidatorsRoot, LatestBeaconBlockHeader};
use frame_support::pallet_prelude::{PhantomData, *};
use sp_core::hashing::sha2_256;

use crate::types::{
    BLSPubkey, BeaconBlockHeader, Bytes32, Domain, DomainType, ForkData, ForkVersion,
    LightClientSnapshot, LightClientUpdate, Root, SigningData,
};

use crate::constants::*;
use sp_std::convert::TryInto;
use ssz_rs::Merkleized;

/// Minimal Light Client for Eth2 Beacon Chain as per https:///github.com/ethereum/annotated-spec/blob/master/altair/sync-protocol.md#minimal-light-client
///     def validate_light_client_update(snapshot: LightClientSnapshot,
///                                  update: LightClientUpdate,
///                                  genesis_validators_root: Root) -> None:
///     # Verify update slot is larger than snapshot slot
///     assert update.header.slot > snapshot.header.slot
///
///     # Verify update does not skip a sync committee period
///     snapshot_period = compute_epoch_at_slot(snapshot.header.slot) /// EPOCHS_PER_SYNC_COMMITTEE_PERIOD
///     update_period = compute_epoch_at_slot(update.header.slot) /// EPOCHS_PER_SYNC_COMMITTEE_PERIOD
///     assert update_period in (snapshot_period, snapshot_period + 1)
///
///     # Verify update header root is the finalized root of the finality header, if specified
///     if update.finality_header == BeaconBlockHeader():
///         signed_header = update.header
///         assert update.finality_branch == [Bytes32() for _ in range(floorlog2(FINALIZED_ROOT_INDEX))]
///     else:
///         signed_header = update.finality_header
///         assert is_valid_merkle_branch(
///             leaf=hash_tree_root(update.header),
///             branch=update.finality_branch,
///             depth=floorlog2(FINALIZED_ROOT_INDEX),
///             index=get_subtree_index(FINALIZED_ROOT_INDEX),
///             root=update.finality_header.state_root,
///         )
///
///     # Verify update next sync committee if the update period incremented
///     if update_period == snapshot_period:
///         sync_committee = snapshot.current_sync_committee
///         assert update.next_sync_committee_branch == [Bytes32() for _ in range(floorlog2(NEXT_SYNC_COMMITTEE_INDEX))]
///     else:
///         sync_committee = snapshot.next_sync_committee
///         assert is_valid_merkle_branch(
///             leaf=hash_tree_root(update.next_sync_committee),
///             branch=update.next_sync_committee_branch,
///             depth=floorlog2(NEXT_SYNC_COMMITTEE_INDEX),
///             index=get_subtree_index(NEXT_SYNC_COMMITTEE_INDEX),
///             root=update.header.state_root,
///         )
///
///     # Verify sync committee has sufficient participants
///     assert sum(update.sync_committee_bits) >= MIN_SYNC_COMMITTEE_PARTICIPANTS
///
///     # Verify sync committee aggregate signature
///     participant_pubkeys = [pubkey for (bit, pubkey) in zip(update.sync_committee_bits, sync_committee.pubkeys) if bit]
///     domain = compute_domain(DOMAIN_SYNC_COMMITTEE, update.fork_version, genesis_validators_root)
///     signing_root = compute_signing_root(signed_header, domain)
///     assert bls.FastAggregateVerify(participant_pubkeys, signing_root, update.sync_committee_signature)
///
///
///

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Fork {
    pub version: [u8; 4],
    pub epoch: u64,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ForkVersions {
    pub genesis: Fork,
    pub altair: Fork,
    pub bellatrix: Fork,
}

pub const FORK_VERSIONS: ForkVersions = ForkVersions {
    genesis: Fork {
        version: [0, 0, 0, 1], // 0x00000001
        epoch: 0,
    },
    altair: Fork {
        version: [1, 0, 0, 1], // 0x01000001
        epoch: 0,
    },
    bellatrix: Fork {
        version: [2, 0, 0, 1], // 0x02000001
        epoch: 0,
    },
};

pub struct MinimalLightClient<T: Config>(PhantomData<T>);

impl<T: Config> MinimalLightClient<T> {
    // Verify sync committee has sufficient participants
    pub fn compute_fork_version(epoch: u64) -> ForkVersion {
        if epoch >= FORK_VERSIONS.bellatrix.epoch {
            return FORK_VERSIONS.bellatrix.version
        }
        if epoch >= FORK_VERSIONS.altair.epoch {
            return FORK_VERSIONS.altair.version
        }
        FORK_VERSIONS.genesis.version
    }

    pub fn sync_committee_count_bits(bits: Vec<bool>) -> usize {
        bits.iter().fold(0, |acc, &x| acc + x as usize)
    }

    pub fn compute_domain(
        domain_type: DomainType,
        fork_version: ForkVersion,
        genesis_validators_root: Root,
    ) -> Result<Domain, Error<T>> {
        let mut domain = [0u8; 32];
        domain[0..4].copy_from_slice(&domain_type);
        domain[4..32].copy_from_slice(
            //  Return the 32-byte fork data root for the ``current_version`` and ``genesis_validators_root``.
            //     This is used primarily in signature domains to avoid collisions across forks/chains.
            &ForkData::new(fork_version, genesis_validators_root).try_hash_tree_root::<T>()?[0..28],
        );
        Ok(domain)
    }

    pub fn compute_signing_root(
        header: BeaconBlockHeader,
        domain: Domain,
    ) -> Result<Root, Error<T>> {
        let mut signing_data = SigningData::new(header.hash_tree_root::<T>()?, domain);
        signing_data.try_hash_tree_root::<T>()
    }

    // Validate merkle path for a given leaf and root hash (uses sha2_256)
    pub fn is_valid_merkle_branch(
        leaf: Bytes32,
        branch: Vec<Bytes32>,
        depth: u64,
        index: u64,
        root: Bytes32,
    ) -> bool {
        let mut value = leaf;
        let mut index = index;
        let mut data = [0u8; 64];

        for i in 0..depth {
            if index % 2 == 0 {
                // left node
                data[0..32].copy_from_slice(&(value));
                data[32..64].copy_from_slice(&(branch[i as usize]));
                value = sha2_256(&data);
            } else {
                data[0..32].copy_from_slice(&(branch[i as usize]));
                data[32..64].copy_from_slice(&(value));
                value = sha2_256(&data);
            }
            index /= 2;
        }
        value == root
    }

    pub fn validate_light_client_update(
        snapshot: LightClientSnapshot<T>,
        update: LightClientUpdate<T>,
        _genesis_validators_root: Root,
    ) -> Result<bool, Error<T>> {
        // Verify update slot is larger than snapshot slot
        ensure!(
            update.header.slot > snapshot.header.slot,
            Error::<T>::InvalidSlot
        );
        // Verify update does not skip a sync committee period
        let snapshot_period = snapshot.header.slot / SLOTS_PER_EPOCH;
        let update_period = update.header.slot / SLOTS_PER_EPOCH;
        ensure!(
            update_period == snapshot_period || update_period == snapshot_period + 1,
            Error::<T>::InvalidPeriod
        );
        // Verify update header root is the finalized root of the finality header, if specified
        let _signed_header = if update.finality_header == LatestBeaconBlockHeader::<T>::get() {
            // Equivalent of assert update.finality_branch == [Bytes32() for _ in range(floorlog2(FINALIZED_ROOT_INDEX))]
            let next_sync_committee_branch: Vec<Bytes32> = (0..FLOOR_LOG_2_OF_FINALIZED_ROOT_INDEX)
                .map(|_| Bytes32::default())
                .collect();
            ensure!(
                update.finality_branch == next_sync_committee_branch,
                Error::<T>::InvalidFinalityBranch
            );
            update.header.clone()
        } else {
            ensure!(
                Self::is_valid_merkle_branch(
                    // todo: verify hash_tree_root(update.finality_header) and body_root are the same
                    update.finality_header.hash_tree_root::<T>()?,
                    update.finality_branch,
                    FLOOR_LOG_2_OF_FINALIZED_ROOT_INDEX,
                    // todo: should be get_subtree_index(FINALIZED_ROOT_INDEX)?
                    FLOOR_LOG_2_OF_FINALIZED_ROOT_INDEX,
                    update.finality_header.state_root,
                ),
                Error::<T>::InvalidFinalityBranch
            );
            update.finality_header.clone()
        };

        // Verify sync committee has sufficient participants
        ensure!(
            Self::sync_committee_count_bits(update.sync_committee_bits.clone()) * 3
                >= SYNC_COMMITTEE_SIZE * 2,
            Error::<T>::SyncCommitteeParticipantsNotSupermajority
        );

        // Verify update next sync committee if the update period incremented
        let sync_committee = if update_period == snapshot_period {
            ensure!(
                // This should NEXT_SYNC_COMMITTEE_DEPTH = floorlog2(NEXT_SYNC_COMMITTEE_INDEX)
                update.next_sync_committee_branch
                    == vec![Bytes32::default(); NEXT_SYNC_COMMITTEE_DEPTH as usize],
                Error::<T>::InvalidLightClientUpdate
            );
            snapshot.current_sync_committee
        } else {
            ensure!(
                Self::is_valid_merkle_branch(
                    update
                        .next_sync_committee
                        .clone()
                        .hash_tree_root()
                        .unwrap()
                        .as_bytes()
                        .try_into()
                        .unwrap(),
                    update.next_sync_committee_branch,
                    NEXT_SYNC_COMMITTEE_DEPTH,
                    // todo: should be get_subtree_index(NEXT_SYNC_COMMITTEE_INDEX)
                    NEXT_SYNC_COMMITTEE_INDEX,
                    update.header.state_root,
                ),
                Error::<T>::InvalidLightClientUpdate
            );
            snapshot.next_sync_committee
        };

        // Verify sync committee aggregate signature
        // Gathers all the pubkeys of the sync committee members that participated in siging the
        // header.
        let mut participant_pubkeys: Vec<BLSPubkey> = Vec::new();
        for (bit, pubkey) in update
            .sync_committee_bits
            .iter()
            .zip(sync_committee.pubkeys.iter())
        {
            if *bit {
                participant_pubkeys.push(pubkey.clone());
            }
        }

        let domain = Self::compute_domain(
            DOMAIN_SYNC_COMMITTEE,
            update.fork_version,
            GenesisValidatorsRoot::<T>::get().0,
        )?;

        let signing_root = Self::compute_signing_root(update.header.clone(), domain)?;

        crate::bls::fast_aggregate_verify(
            participant_pubkeys,
            signing_root.to_vec(),
            update.sync_committee_signature,
        )
    }
}

#[cfg(feature = "testing")]
#[cfg(test)]
pub mod mlc_test {
    use super::*;
    use crate::mock::{run_test, TestRuntime};
    use codec::Encode;
    use frame_support::assert_ok;
    use hex_literal::hex;

    #[test]
    pub fn mlc_test_compute_domain() {
        run_test(|| {
            let domain: Domain = MinimalLightClient::<TestRuntime>::compute_domain(
                hex!("07000000"),
                hex!("00000001"),
                hex!("5dec7ae03261fde20d5b024dfabce8bac3276c9a4908e23d50ba8c9b50b0adff"),
            )
            .expect("compute_domain should not fail with correct inputs");

            // assert_ok!(&domain);
            assert_eq!(
                domain.encode(),
                hex!("0700000046324489ceb6ada6d118eacdbe94f49b1fcb49d5481a685979670c7c")
            );
        });
    }

    #[test]
    pub fn mlc_test_compute_signing_root_bls() {
        run_test(|| {
            let signing_root = MinimalLightClient::<TestRuntime>::compute_signing_root(
                BeaconBlockHeader {
                    slot: 3529537,
                    proposer_index: 192549,
                    parent_root: hex!(
                        "1f8dc05ea427f78e84e2e2666e13c3befb7106fd1d40ef8a3f67cf615f3f2a4c"
                    ),
                    state_root: hex!(
                        "0dfb492a83da711996d2d76b64604f9bca9dc08b6c13cf63b3be91742afe724b"
                    ),
                    body_root: hex!(
                        "66fba38f7c8c2526f7ddfe09c1a54dd12ff93bdd4d0df6a0950e88e802228bfa"
                    ),
                },
                hex!("07000000afcaaba0efab1ca832a15152469bb09bb84641c405171dfa2d3fb45f"),
            );
            assert_ok!(&signing_root);
            assert_eq!(
                signing_root.unwrap().encode(),
                hex!("3ff6e9807da70b2f65cdd58ea1b25ed441a1d589025d2c4091182026d7af08fb")
            );
        });
    }
}
