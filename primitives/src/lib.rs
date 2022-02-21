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
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
use sp_runtime::MultiSignature;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

use sp_std::convert::TryFrom;
use sp_std::prelude::*;
use sp_std::{convert::TryFrom, vec};

pub mod abi;
pub mod bridges;
pub mod contract_metadata;
pub mod gateway_inbound_protocol;
pub mod match_format;
pub mod side_effect;
pub mod signature_caster;
pub mod transfers;
pub mod volatile;
pub mod xtx;

pub use gateway_inbound_protocol::GatewayInboundProtocol;

pub type ChainId = [u8; 4];

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayType {
    ProgrammableInternal(u32),
    ProgrammableExternal(u32),
    TxOnly(u32),
}

impl GatewayType {
    pub fn fetch_nonce(self) -> u32 {
        match self {
            Self::ProgrammableInternal(nonce) => nonce,
            Self::ProgrammableExternal(nonce) => nonce,
            Self::TxOnly(nonce) => nonce,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayVendor {
    Ethereum,
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
    pub digest: Option<sp_runtime::generic::Digest>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayPointer {
    pub id: ChainId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// SCALE-encoded signed extension - see more at frame_metadata::ExtrinsicMetadata
    // pub signed_extensions: Option<Vec<u8>>,
    /// Runtime version
    pub runtime_version: sp_version::RuntimeVersion,
    /// Extrinsics version
    pub extrinsics_version: u8,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}

impl Default for GatewayGenesisConfig {
    fn default() -> Self {
        Self {
            extrinsics_version: 0,
            runtime_version: Default::default(),
            genesis_hash: vec![],
            modules_encoded: None,
            // signed_extensions: None,
        }
    }
}

/// Represents assorted gateway system properties.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewaySysProps {
    pub ss58_format: u16,
    pub token_symbol: Vec<u8>,
    pub token_decimals: u8,
}

impl TryFrom<&ChainId> for GatewaySysProps {
    type Error = &'static str;

    /// Maps a chain id to its system properties.
    ///
    /// Based on https://wiki.polkadot.network/docs/build-ss58-registry.
    fn try_from(chain_id: &ChainId) -> Result<Self, Self::Error> {
        match chain_id {
            b"circ" => Ok(GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            }),
            b"gate" => Ok(GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            }),
            b"pdot" => Ok(GatewaySysProps {
                ss58_format: 0,
                token_symbol: Encode::encode("DOT"),
                token_decimals: 10,
            }),
            b"ksma" => Ok(GatewaySysProps {
                ss58_format: 2,
                token_symbol: Encode::encode("KSM"),
                token_decimals: 12,
            }),
            _ => Err("unknown chain id"),
        }
    }
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Compose<Account, Balance> {
    pub name: Vec<u8>,
    pub code_txt: Vec<u8>,
    pub exec_type: Vec<u8>,
    pub dest: Account,
    pub value: Balance,
    pub bytes: Vec<u8>,
    pub input_data: Vec<u8>,
}

/// A result type of a get storage call.
pub type FetchContractsResult = Result<Vec<u8>, ContractAccessError>;

pub type RegistryContractId<T> = <T as frame_system::Config>::Hash;

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

/// Exec phase consists out of many parallel steps
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
/// Request message format that derivative of could be compatible with JSON-RPC API
/// with either signed or unsigned payload or custom transmission medium like XCMP protocol
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitOutboundMessage {
    /// Message name/identifier
    pub name: Bytes,
    /// Module/pallet name
    pub module_name: Bytes,
    /// Method name
    pub method_name: Bytes,
    /// Encoded sender's public key
    pub sender: Option<Bytes>,
    /// Encoded target's public key
    pub target: Option<Bytes>,
    /// Array of next arguments: encoded bytes of arguments that that JSON-RPC API expects
    pub arguments: Vec<Bytes>,
    /// Expected results
    pub expected_output: Vec<GatewayExpectedOutput>,
    /// Extra payload in case the message is signed or uses custom delivery protocols like XCMP
    pub extra_payload: Option<ExtraMessagePayload>,
    /// type of gateway chain this message is intended for
    pub gateway_vendor: GatewayVendor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RpcPayloadUnsigned<'a> {
    pub method_name: &'a str,
    pub params: Vec<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RpcPayloadSigned<'a> {
    pub method_name: &'a str,
    pub signed_extrinsic: Bytes,
}

impl CircuitOutboundMessage {
    pub fn to_jsonrpc_unsigned(&self) -> Result<RpcPayloadUnsigned, &'static str> {
        let method_name: &str = sp_std::str::from_utf8(&self.name[..])
            .map_err(|_| "`Can't decode method name to &str")?;

        Ok(RpcPayloadUnsigned {
            method_name,
            params: self.arguments.clone(),
        })
    }

    pub fn to_jsonrpc_signed(&self) -> Result<RpcPayloadSigned, &'static str> {
        let method_name: &str = sp_std::str::from_utf8(&self.name[..])
            .map_err(|_| "`Can't decode method name to &str")?;

        let signed_ext = self
            .extra_payload
            .as_ref()
            .map(|payload| payload.tx_signed.clone())
            .ok_or("no signed extrinsic provided")?;

        Ok(RpcPayloadSigned {
            method_name,
            signed_extrinsic: signed_ext,
        })
    }
}

/// Inclusion proofs of different tries
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
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
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CircuitInboundResult {
    pub result_format: Bytes,
    pub proof_type: ProofTriePointer,
}

/// Inbound Steps that specifie expected data deposited by relayers back to the Circuit after each step
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
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
/// Extra payload in case the message is signed ro has other custom parameters required by linking protocol.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExtraMessagePayload {
    pub signer: Bytes,
    /// Encoded utf-8 string of module name that implements requested entrypoint
    pub module_name: Bytes,
    /// Encoded utf-8 string of method name that implements requested entrypoint
    pub method_name: Bytes,
    /// Encoded call bytes
    pub call_bytes: Bytes,
    /// Encoded tx signature
    pub signature: Bytes,
    /// Encoded extras to that transctions, like versions and gas price /tips for miners. Check GenericExtra for more info.
    pub extra: Bytes,
    /// Encoded and signed transaction ready to send
    pub tx_signed: Bytes,
    /// Custom message bytes, that would have to be decoded by the receiving end.
    /// Could be utilized by custom transmission medium (like Substrate's XCMP)
    pub custom_payload: Option<Bytes>,
}

/// Retrieves all available gateways for a given ChainId.
/// Currently returns a vector with a single hardcoded result.
/// Eventually this will search all known gateways on pallet-xdns.
pub fn retrieve_gateway_pointers(gateway_id: ChainId) -> Result<Vec<GatewayPointer>, &'static str> {
    Ok(vec![GatewayPointer {
        id: gateway_id,
        gateway_type: GatewayType::ProgrammableExternal(0),
        vendor: GatewayVendor::Substrate,
    }])
}

pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <MultiSignature as Verify>::Signer;

/// Common types across all runtimes
pub type BlockNumber = u32;

pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;

/// Index of a transaction in the chain. 32-bit should be plenty.
pub type Nonce = u32;

/// Balance of an account.
pub type Balance = u128;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
