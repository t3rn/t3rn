#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::U256;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

use t3rn_primitives::abi::Bytes;
use crate::message_assembly::circuit_outbound::ProofTriePointer;
use crate::message_assembly::gateway_outbound_protocol::GatewayOutboundEvent;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum ProofType {
    FullValue,
    MerklePath,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
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
    pub proof: Bytes,
    /// Value Index in Proof
    pub in_proof_index: Option<U256>,
    /// Value Index in Block
    pub in_block_index: Option<U256>,
    /// Value Index in Transaction
    pub in_tx_index: Option<U256>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub struct StepConfirmation {
    pub step_index: u8,
    pub value: Bytes,
    pub proof: Bytes,
    pub proof_type: ProofType,
    pub proof_trie_pointer: ProofTriePointer,
    pub pointer_to_value_in_proof: Option<Bytes>,
    pub block_pointer: Bytes,
    pub outbound_event: GatewayOutboundEvent,
}
