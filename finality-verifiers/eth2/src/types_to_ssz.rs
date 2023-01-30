#![cfg_attr(not(feature = "std"), no_std)]

use crate::{Config, Error};

use sp_std::{convert::TryInto, iter::FromIterator};
use ssz_rs::{prelude::Vector, Deserialize, Merkleized, Sized};

use crate::types::{BeaconBlockHeader, ForkVersion, Root};

use codec::Encode;

use ssz_rs_derive::SimpleSerialize;

// SSZ types
#[derive(Debug, Default, Clone, Eq, PartialEq, SimpleSerialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubkey, { crate::constants::SYNC_COMMITTEE_SIZE }>,
    pub pubkey_aggregates: BLSPubkey,
}

pub type BLSPubkey = Vector<u8, { crate::constants::BLS_PUBKEY_SIZE }>;

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
pub struct ForkData {
    pub current_version: Vector<u8, 4>,
    pub genesis_validators_root: Vector<u8, 32>,
}

impl ForkData {
    pub fn new(current_version: ForkVersion, genesis_validators_root: Root) -> Self {
        Self {
            current_version: Vector::from_iter(current_version.to_vec()),
            genesis_validators_root: Vector::from_iter(genesis_validators_root.to_vec()),
        }
    }

    pub fn try_hash_tree_root<T: Config>(&mut self) -> Result<Root, Error<T>> {
        match self.hash_tree_root() {
            Ok(hash_root) => Ok(hash_root
                .as_bytes()
                .try_into()
                .expect("ssz_rs stores Nodes as [u8; 32] and should decode to 32b root")),
            Err(_) => Err(Error::<T>::ForkDataHashTreeRootFailed),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Vector<u8, 32>,
    pub domain: Vector<u8, 32>,
}

impl SigningData {
    pub fn new(object_root: Root, domain: Root) -> Self {
        Self {
            object_root: Vector::from_iter(object_root.to_vec()),
            domain: Vector::from_iter(domain.to_vec()),
        }
    }

    pub fn try_hash_tree_root<T: Config>(&mut self) -> Result<Root, Error<T>> {
        match self.hash_tree_root() {
            Ok(hash_root) => Ok(hash_root
                .as_bytes()
                .try_into()
                .expect("ssz_rs stores Nodes as [u8; 32] and should decode to 32b root")),
            Err(_) => Err(Error::<T>::SigningDataHashTreeRootFailed),
        }
    }
}

impl BeaconBlockHeader {
    pub fn hash_tree_root<T: Config>(&self) -> Result<Root, Error<T>> {
        #[derive(Default, Debug, SimpleSerialize, Clone)]
        struct SSZBeaconBlockHeader {
            slot: Vector<u8, 8>,
            proposer_index: Vector<u8, 8>,
            parent_root: Vector<u8, 32>,
            state_root: Vector<u8, 32>,
            body_root: Vector<u8, 32>,
        }
        let mut ssz_header = SSZBeaconBlockHeader {
            slot: Vector::from_iter(self.slot.encode()),
            proposer_index: Vector::from_iter(self.proposer_index.encode()),
            parent_root: Vector::from_iter(self.parent_root.to_vec()),
            state_root: Vector::from_iter(self.state_root.to_vec()),
            body_root: Vector::from_iter(self.body_root.to_vec()),
        };

        match ssz_header.hash_tree_root() {
            Ok(hash_root) => Ok(hash_root
                .as_bytes()
                .try_into()
                .expect("ssz_rs stores Nodes as [u8; 32] and should decode to 32b root")),
            Err(_) => Err(Error::<T>::BeaconHeaderHashTreeRootFailed),
        }
    }
}
