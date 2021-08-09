// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Runtime API definition required by Contracts RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts access methods.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use bitflags::bitflags;
use codec::{Codec, Decode, Encode};
use sp_core::Bytes;
use sp_runtime::{DispatchError, RuntimeDebug};

use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait GatewayApi<AccountId, Balance, BlockNumber, Hash> where
        AccountId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
        Hash: Codec,
    {
        /// Perform a call from a specified account to a given contract.
        ///
        /// See [`pallet_contracts::Pallet::call`].
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> ContractExecResult;

        fn call_module(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> ContractExecResult;

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> GetStorageResult;

        fn set_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> SetStorageResult;

        fn transfer(
            address: AccountId,
            key: [u8; 32],
        ) -> TransferResult;

        fn custom(
            address: AccountId,
            key: [u8; 32],
        ) -> CustomResult;

    }
}

/// Result type of a `bare_call` or `bare_instantiate` call.
///
/// It contains the execution result together with some auxiliary information.
#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ContractResult<T> {
    /// How much gas was consumed during execution.
    pub gas_consumed: u64,
    /// An optional debug message. This message is only filled when explicitly requested
    /// by the code that calls into the contract. Otherwise it is empty.
    ///
    /// The contained bytes are valid UTF-8. This is not declared as `String` because
    /// this type is not allowed within the runtime.
    ///
    /// Clients should not make any assumptions about the format of the buffer.
    /// They should just display it as-is. It is **not** only a collection of log lines
    /// provided by a contract but a formatted buffer with different sections.
    ///
    /// # Note
    ///
    /// The debug message is never generated during on-chain execution. It is reserved for
    /// RPC calls.
    #[cfg_attr(feature = "std", serde(with = "as_string"))]
    pub debug_message: Vec<u8>,
    /// The execution result of the wasm code.
    pub result: T,
}

/// Output of a contract call or instantiation which ran to completion.
#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ExecReturnValue {
    /// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
    pub flags: ReturnFlags,
    /// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
    pub data: Bytes,
}

/// Result type of a `bare_call` call.
pub type ContractExecResult = ContractResult<Result<ExecReturnValue, DispatchError>>;

/// Result type of a `get_storage` call.
pub type GetStorageResult = Result<Option<Vec<u8>>, AccessError>;

/// Result type of a `set_storage` call.
pub type SetStorageResult = Result<Option<Vec<u8>>, AccessError>;

/// Result type of a `transfer` call.
pub type TransferResult = Result<Option<Vec<u8>>, AccessError>;

/// Result type of a `transfer` call.
pub type EmitEventResult = Result<Option<Vec<u8>>, AccessError>;

/// Result type of a `custom` call.
pub type CustomResult = Result<Option<Vec<u8>>, AccessError>;

/// The possible errors that can happen querying the storage of a contract.
#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum AccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

bitflags! {
    /// Flags used by a contract to customize exit behaviour.
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase", transparent))]
    pub struct ReturnFlags: u32 {
        /// If this bit is set all changes made by the contract execution are rolled back.
        const REVERT = 0x0000_0001;
    }
}

#[cfg(feature = "std")]
mod as_string {
    use super::*;
    use serde::{ser::Error, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| S::Error::custom(format!("Debug buffer contains invalid UTF8: {}", e)))?
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        Ok(String::deserialize(deserializer)?.into_bytes())
    }
}
