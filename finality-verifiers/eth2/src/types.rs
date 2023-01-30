#![cfg_attr(not(feature = "std"), no_std)]

use crate::Config;
use frame_support::pallet_prelude::{PhantomData, *};

use ssz_rs::prelude::Vector;

pub use crate::types_to_ssz::*;

// todo: add Default + TypeInfo traits
// pub use ethereum_types::{Bloom};
use codec::{Decode, Encode};

use sp_core::{H160, H256, U256};

#[derive(Debug, Clone)]
pub struct LightClientSnapshot<T: Config> {
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,
    pub _marker: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct LightClientUpdate<T: Config> {
    pub header: BeaconBlockHeader,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<Bytes32>,
    pub finality_header: BeaconBlockHeader,
    pub finality_branch: Vec<Bytes32>,
    pub sync_committee_bits: Vec<bool>,
    pub sync_committee_signature: BLSSignature,
    pub fork_version: ForkVersion,
    pub _marker: PhantomData<T>,
}

#[derive(Default, Debug, Encode, Clone, Decode, TypeInfo, MaxEncodedLen, Eq, PartialEq)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body_root: Root,
}

#[derive(Default, Debug, Encode, Clone, Decode, MaxEncodedLen, Eq, PartialEq)]
pub struct ExecutionPayloadHeader {
    pub parent_hash: H256,
    pub coinbase: H160,
    pub state_root: H256,
    pub receipt_root: H256,
    pub logs_bloom: H256, // Bloom
    pub random: H256,
    pub block_number: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Bytes32,
    pub base_fee_per_gas: Option<U256>,
    pub block_hash: Option<H256>,
}

#[derive(Default, Debug, Clone, Encode, Decode, MaxEncodedLen, Eq, PartialEq, TypeInfo)]
pub struct Eth2LightClientInitData {
    pub(crate) genesis_validators_root: Root,
    pub(crate) trusted_snapshot_header: BeaconBlockHeader,
}

pub type BLSSignature = [u8; 96];

pub type Slot = u64;

pub type ValidatorIndex = u64;

pub type Root = Bytes32;

pub type Bytes = Vec<u8>;

pub type Bytes32 = [u8; 32];

// todo: Incorrect, just for testing
pub type Bloom = Vector<u8, { crate::constants::MAX_LOGS_BLOOM_SIZE }>;

pub type ForkVersion = [u8; 4];

pub type Domain = Bytes32;

pub type DomainType = [u8; 4];
