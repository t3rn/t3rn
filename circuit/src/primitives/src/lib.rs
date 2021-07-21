// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! A crate that hosts a common definitions that are relevant for the pallet-contracts.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::traits::{Currency, Time};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::prelude::*;

pub mod abi;
pub mod gateway_inbound_protocol;
pub mod transfers;

pub use gateway_inbound_protocol::GatewayInboundProtocol;

pub type InstanceId = [u8; 4];

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayType {
    ProgrammableInternal,
    ProgrammableExternal,
    TxOnly,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayVendor {
    Substrate,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure used at gateway registration as a starting point for multi-finality-verifier
pub struct GenericPrimitivesHeader {
    pub parent_hash: Option<sp_core::hash::H256>,
    pub number: u64,
    pub state_root: Option<sp_core::hash::H256>,
    pub extrinsics_root: Option<sp_core::hash::H256>,
    pub digest: Option<sp_runtime::generic::Digest<sp_core::hash::H256>>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayPointer {
    pub id: InstanceId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// SCALE-encoded signed extension - see more at frame_metadata::ExtrinsicMetadata
    pub signed_extension: Option<Vec<u8>>,
    /// Runtime version
    pub runtime_version: sp_version::RuntimeVersion,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Compose<Account, Balance> {
    pub name: Vec<u8>,
    pub code_txt: Vec<u8>,
    pub gateway_id: [u8; 4],
    pub exec_type: Vec<u8>,
    pub dest: Account,
    pub value: Balance,
    pub bytes: Vec<u8>,
    pub input_data: Vec<u8>,
}
/// A result type of a get storage call.
pub type FetchContractsResult = Result<Option<Vec<u8>>, ContractAccessError>;

/// A result of execution of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ComposableExecResult {
    /// The contract returned successfully.
    ///
    /// There is a status code and, optionally, some data returned by the contract.
    Success {
        /// Flags that the contract passed along on returning to alter its exit behaviour.
        /// Described in `pallet_contracts::exec::ReturnFlags`.
        flags: u32,
        /// Output data returned by the contract.
        ///
        /// Can be empty.
        data: Vec<u8>,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// The contract execution either trapped or returned an error.
    Error,
}

/// The possible errors that can happen querying the storage of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExecPhase<Account, Balance> {
    pub steps: Vec<ExecStep<Account, Balance>>,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExecStep<Account, Balance> {
    pub compose: Compose<Account, Balance>,
}

pub type GenericAddress = sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct InterExecSchedule<Account, Balance> {
    pub phases: Vec<ExecPhase<Account, Balance>>,
}

pub trait EscrowTrait: frame_system::Config + pallet_sudo::Config {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}

type Bytes = Vec<u8>;

/// Outbound Step that specifies expected transmission medium for relayers connecting with that gateway.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CircuitOutboundMessage {
    /// Request compatible with JSON-RPC API of receiving node
    Read {
        /// Method name on the VM
        name: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results that will be decoded and checked against the format
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Transaction (in substrate extrinics), signed offline and including dispatch call(s)
    Write {
        /// Method name on the VM
        name: Bytes,
        /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
        arguments: Vec<Bytes>,
        /// Expected results
        expected_output: Vec<GatewayExpectedOutput>,
        /// Expected results
        payload: MessagePayload,
    },
    /// Custom transmission medium (like Substrate's XCMP)
    Escrowed {
        /// Method name on the VM
        name: Bytes,
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
        key: Vec<Vec<u8>>,
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
