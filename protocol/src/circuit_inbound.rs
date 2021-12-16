#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::U256;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;
use sp_std::vec::Vec;

use crate::gateway_outbound_protocol::GatewayOutboundEvent;
use t3rn_primitives::ProofTriePointer;

pub type Bytes = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProofType {
    FullValue,
    MerklePath,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proof {
    /// Original value to prove
    pub value: Bytes,
    /// Hashed value using adequate for given gateway hashing algorithm
    pub value_hash: Bytes,
    /// Pointer to block on that gateway that includes value
    pub block_hash: Bytes,
    /// Proof type
    pub proof_type: ProofType,
    /// Selector of trie root in that block
    pub proof_trie_pointer: ProofTriePointer,
    /// Proof as bytes
    pub proof_data: Vec<Vec<u8>>,
    /// Value Index in Proof
    pub in_proof_index: Option<U256>,
    /// Value Index in Block
    pub in_block_index: Option<U256>,
    /// Value Index in Transaction
    pub in_tx_index: Option<U256>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct StepConfirmation {
    pub step_index: u8,
    pub value: Bytes,
    pub proof: Proof,
    pub outbound_event: GatewayOutboundEvent,
}
