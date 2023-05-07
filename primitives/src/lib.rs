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
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{ReservableCurrency, Time};
use scale_info::TypeInfo;

pub use t3rn_abi::recode::Codec as T3rnCodec;
pub use t3rn_types::{gateway, types::Bytes};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
use sp_runtime::{
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

pub use gateway_inbound_protocol::GatewayInboundProtocol;
// pub use orml_traits;

use sp_std::{prelude::*, vec};
#[cfg(feature = "std")]
use std::fmt::Debug;

pub mod account_manager;
pub mod attesters;
pub mod circuit;
pub mod claimable;
pub mod clock;
pub mod common;
pub mod contract_metadata;
pub mod contracts_registry;
pub mod executors;
pub mod gateway_inbound_protocol;
pub mod light_client;
pub mod match_format;
pub mod monetary;
pub mod portal;
pub mod signature_caster;
pub mod storage;
pub mod threevm;
pub mod transfers;
pub mod volatile;
pub mod xdns;
pub mod xtx;

pub type ChainId = [u8; 4];

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum GatewayType {
    ProgrammableInternal(u32),
    ProgrammableExternal(u32),
    TxOnly(u32),
    OnCircuit(u32),
}

impl Default for GatewayType {
    fn default() -> GatewayType {
        GatewayType::ProgrammableExternal(0)
    }
}

impl GatewayType {
    pub fn fetch_nonce(self) -> u32 {
        match self {
            Self::ProgrammableInternal(nonce) => nonce,
            Self::ProgrammableExternal(nonce) => nonce,
            Self::OnCircuit(nonce) => nonce,
            Self::TxOnly(nonce) => nonce,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub enum GatewayVendor {
    Polkadot,
    Kusama,
    #[default]
    Rococo,
    Ethereum,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub enum ExecutionVendor {
    #[default]
    Substrate,
    EVM,
}

impl TokenInfo {
    pub fn match_execution_vendor(&self) -> ExecutionVendor {
        match self {
            TokenInfo::Substrate(_) => ExecutionVendor::Substrate,
            TokenInfo::Ethereum(_) => ExecutionVendor::EVM,
        }
    }
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

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayGenesisConfig {
    /// SCALE-encoded modules following the format of selected frame_metadata::RuntimeMetadataVXX
    pub modules_encoded: Option<Vec<u8>>,
    /// Extrinsics version
    pub extrinsics_version: u8,
    /// Genesis hash - block id of the genesis block use to distinct the network and sign messages
    /// Length depending on parameter passed in abi::GatewayABIConfig
    pub genesis_hash: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenInfo {
    Substrate(SubstrateToken),
    Ethereum(EthereumToken),
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Default, Decode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SpeedMode {
    Fast,
    Rational,
    #[default]
    Finalized,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SubstrateToken {
    pub id: u32,
    pub symbol: Vec<u8>,
    pub decimals: u8,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EthereumToken {
    pub symbol: Vec<u8>,
    pub decimals: u8,
    pub address: Option<[u8; 20]>,
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self::Substrate(SubstrateToken {
            symbol: Encode::encode("TKN"),
            decimals: 9,
            id: 42,
        })
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

/// Read latest height of gateway known to a light client
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ReadLatestGatewayHeight {
    Success { encoded_height: Vec<u8> },
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

pub type GenericAddress = sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>;

pub trait EscrowTrait<T: frame_system::Config> {
    type Currency: ReservableCurrency<T::AccountId>;
    type Time: Time;
}

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
        vendor: GatewayVendor::Rococo,
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
