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

use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::prelude::*;

pub mod abi;
pub mod transfers;

pub type InstanceId = [u8; 4];

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub enum GatewayType {
    ProgrammableInternal,
    ProgrammableExternal,
    TxOnly,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub enum GatewayVendor {
    Substrate,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
/// Structure used at gateway registration as a starting point for multi-finality-verifier
pub struct GenericPrimitivesHeader {
    pub parent_hash: Option<sp_core::hash::H256>,
    pub number: u64,
    pub state_root: Option<sp_core::hash::H256>,
    pub extrinsics_root: Option<sp_core::hash::H256>,
    pub digest: Option<sp_runtime::generic::Digest<sp_core::hash::H256>>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
pub struct GatewayPointer {
    pub id: InstanceId,
    pub vendor: GatewayVendor,
    pub gateway_type: GatewayType,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Debug)]
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
#[derive(Eq, PartialEq, Encode, Decode, Serialize, Deserialize, Debug, Clone, Default)]
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
#[derive(Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize, Clone)]
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
#[derive(Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize, Clone)]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecPhase<Account, Balance> {
    pub steps: Vec<ExecStep<Account, Balance>>,
}

#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecStep<Account, Balance> {
    pub compose: Compose<Account, Balance>,
}
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterExecSchedule<Account, Balance> {
    pub phases: Vec<ExecPhase<Account, Balance>>,
}

pub trait EscrowTrait: frame_system::Config + pallet_sudo::Config {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}
