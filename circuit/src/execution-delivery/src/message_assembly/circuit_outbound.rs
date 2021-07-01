#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use crate::pallet::Config;

use bp_messages::LaneId;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_std::vec;
use sp_std::vec::*;

/// CircuitOutbound covers the path of message assembly and adds it to the queue dispatchable by
pub enum CircuitOutbound<T: Config> {
    Programmable {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
    },
    TxOnlyExternal {
        escrow_account: T::AccountId,
        target_account: T::AccountId,
        message: Vec<u8>,
        gateway_id: bp_runtime::ChainId,
    },
}

type Bytes = Vec<u8>;

/// Inclusion proofs of different tries
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProofTriePointer {
    /// Proof is a merkle path in the state trie
    State,
    /// Proof is a merkle path in the transaction trie (extrisics in Substrate)
    Transaction,
    /// Proof is a merkle path in the receipts trie (in Substrate logs are entries in state trie, this doesn't apply)
    Receipts,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitInboundResult {
    pub result_format: Bytes,
    pub proof_type: ProofTriePointer,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayExpectedOutput {
    /// Effect would be the modified storage key
    Storage {
        key: Vec<[u8; 32]>,
        // key: Vec<sp_core::storage::StorageKey>,
        // value: Vec<Option<sp_core::storage::StorageData>>,
        value: Vec<Option<Bytes>>,
    },

    /// Expect events as a result of that call - will be described with signature
    /// and check against the corresponding types upon receiving
    Events { signatures: Vec<Bytes> },

    /// Yet another event or Storage output
    Extrinsic {
        /// Optionally expect dispatch of extrinsic only at the certain block height
        block_height: Option<u64>,
    },

    /// Yet another event or Storage output. If expecting output u can define its type format.
    Output { output: Bytes },
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MessagePayload {
    Signed {
        signer: Bytes,
        /// Encoded utf-8 string of module name that implements requested entrypoint
        module_name: Bytes,
        /// Encoded utf-8 string of method name that implements requested entrypoint
        method_name: Bytes,
        /// Encoded call bytes
        call_bytes: Bytes,
        /// Encoded tx signature
        signature: Bytes,
        /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
        extra: Bytes,
    },
    /// Request compatible with JSON-RPC API of receiving node
    Rpc {
        /// Encoded utf-8 string of module name that implements requested entrypoint
        module_name: Bytes,
        /// Encoded utf-8 string of method name that implements requested entrypoint
        method_name: Bytes,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    TransactionDispatch {
        /// Encoded call bytes
        call_bytes: Bytes,
        /// Encoded tx signature
        signature: Bytes,
        /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
        extra: Bytes,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Custom {
        /// Custom message bytes, that would have to be decoded by the receiving end.
        payload: Bytes,
    },
}

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CircuitOutboundMessage {
    /// Request compatible with JSON-RPC API of receiving node
    Read {
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results that will be decoded and checked against the format
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    Write {
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Escrowed {
        /// Encoded sender's public key
        sender: Bytes,
        /// Encoded target's public key
        target: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
}

pub trait CircuitOutboundProtocol {}

impl<T: Config> CircuitOutbound<T> {
    pub fn send_message(&self, message: T::OutboundPayload, submitter: T::AccountId) -> Vec<u8> {
        let origin = frame_system::RawOrigin::Signed(submitter).into();
        let lane_id: LaneId = [0, 0, 0, 1];
        let delivery_and_dispatch_fee: T::OutboundMessageFee = 0.into();

        let _res = <pallet_bridge_messages::Module<T>>::send_message(
            origin,
            lane_id,
            message,
            delivery_and_dispatch_fee,
        );

        vec![]
    }
}
