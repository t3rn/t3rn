// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
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

#![cfg_attr(not(feature = "std"), no_std)]

mod precompile;

use codec::{Decode, Encode};
pub use evm::ExitReason;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{H160, U256};
use sp_std::vec::Vec;

pub use evm::backend::{Basic as Account, Log};

pub use self::precompile::{
    Context, ExitError, ExitRevert, ExitSucceed, LinearCostPrecompile, Precompile,
    PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult, PrecompileSet,
};

pub mod traits;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// External input from the transaction.
pub struct Vicinity {
    /// Current transaction gas price.
    pub gas_price: U256,
    /// Origin of the transaction.
    pub origin: H160,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct ExecutionInfo<T> {
    pub exit_reason: ExitReason,
    pub value: T,
    pub used_gas: U256,
    pub logs: Vec<Log>,
}

pub type CallInfo = ExecutionInfo<Vec<u8>>;
pub type CreateInfo = ExecutionInfo<H160>;

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum CallOrCreateInfo {
    Call(CallInfo),
    Create(CreateInfo),
}

/// Account definition used for genesis block construction.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct GenesisAccount {
    /// Account nonce.
    pub nonce: U256,
    /// Account balance.
    pub balance: U256,
    /// Full account storage.
    pub storage: std::collections::BTreeMap<sp_core::H256, sp_core::H256>,
    /// Account code.
    pub code: Vec<u8>,
}

/// Trait that outputs the current transaction gas price.
pub trait FeeCalculator {
    /// Return the minimal required gas price.
    fn min_gas_price() -> U256;
}

impl FeeCalculator for () {
    fn min_gas_price() -> U256 {
        U256::zero()
    }
}
