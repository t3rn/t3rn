// Copyright 2018-2020 Parity Technologies (UK) Ltd.
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

//! # Contract Module
//!
//! The Contract module provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.
//!
//! - [`contract::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//! This module extends accounts based on the `Currency` trait to have smart-contract functionality. It can
//! be used with other modules that implement accounts based on `Currency`. These "smart-contract accounts"
//! have the ability to instantiate smart-contracts and make calls to other contract and non-contract accounts.
//!
//! The smart-contract code is stored once in a `code_cache`, and later retrievable via its `code_hash`.
//! This means that multiple smart-contracts can be instantiated from the same `code_cache`, without replicating
//! the code each time.
//!
//! When a smart-contract is called, its associated code is retrieved via the code hash and gets executed.
//! This call can alter the storage entries of the smart-contract account, instantiate new smart-contracts,
//! or call other smart-contracts.
//!
//! Finally, when an account is reaped, its associated code and storage of the smart-contract account
//! will also be deleted.
//!
//! ### Gas
//!
//! Senders must specify a gas limit with every call, as all instructions invoked by the smart-contract require gas.
//! Unused gas is refunded after the call, regardless of the execution outcome.
//!
//! If the gas limit is reached, then all calls and state changes (including balance transfers) are only
//! reverted at the current call's contract level. For example, if contract A calls B and B runs out of gas mid-call,
//! then all of B's calls are reverted. Assuming correct error handling by contract A, A's other calls and state
//! changes still persist.
//!
//! ### Notable Scenarios
//!
//! Contract call failures are not always cascading. When failures occur in a sub-call, they do not "bubble up",
//! and the call will only revert at the specific contract level. For example, if contract A calls contract B, and B
//! fails, A can decide how to handle that failure, either proceeding or reverting A's changes.
//!
//! ## Interface
//!
//! ### Dispatchable functions
//!
//! * `put_code` - Stores the given binary Wasm code into the chain's storage and returns its `code_hash`.
//! * `instantiate` - Deploys a new contract from the given `code_hash`, optionally transferring some balance.
//! This instantiates a new smart contract account and calls its contract deploy handler to
//! initialize the contract.
//! * `call` - Makes a call to an account, optionally transferring some balance.
//!
//! ## Usage
//!
//! The Contract module is a work in progress. The following examples show how this Contract module
//! can be used to instantiate and call contracts.
//!
//! * [`ink`](https://github.com/paritytech/ink) is
//! an [`eDSL`](https://wiki.haskell.org/Embedded_domain_specific_language) that enables writing
//! WebAssembly based smart contracts in the Rust programming language. This is a work in progress.
//!
//! ## Related Modules
//!
//! * [Balances](../pallet_balances/index.html)

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub mod gas;
mod benchmarking;
pub mod escrow_exec;
pub mod exec;
pub mod storage;
pub mod wasm;

#[cfg(test)]
mod tests;

pub use crate::exec::ExecutionContext;

pub use crate::exec::{ExecResult, ExecReturnValue};
pub use crate::gas::{Gas, GasMeter};

use codec::{Decode, Encode};
use frame_support::storage::child::ChildInfo;
use sp_runtime::traits::Saturating;

use frame_support::traits::{Currency, Get};

pub use pallet_contracts::Config as ContractsConfig;
pub use pallet_contracts::*;

use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

/// Information for managing an account and its sub trie abstraction.
/// This is the required info to cache for an account.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Unique ID for the subtree encoded as a bytes vector.
    pub trie_id: TrieId,
    /// The total number of bytes used by this contract.
    ///
    /// It is a sum of each key-value pair stored by this contract.
    pub storage_size: u32,
    /// The number of key-value pairs that have values of zero length.
    /// The condition `empty_pair_count ≤ total_pair_count` always holds.
    pub empty_pair_count: u32,
    /// The total number of key-value pairs in storage of this contract.
    pub total_pair_count: u32,
    /// The code associated with a given account.
    pub code_hash: CodeHash,
    /// Pay rent at most up to this value.
    pub rent_allowance: Balance,
    /// Last block rent has been payed.
    pub deduct_block: BlockNumber,
    /// Last block child storage has been written.
    pub last_write: Option<BlockNumber>,
}

impl<CodeHash, Balance, BlockNumber> RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Associated child trie unique id is built from the hash part of the trie id.
    pub fn child_trie_info(&self) -> ChildInfo {
        child_trie_info(&self.trie_id[..])
    }
}

/// Associated child trie unique id is built from the hash part of the trie id.
pub(crate) fn child_trie_info(trie_id: &[u8]) -> ChildInfo {
    ChildInfo::new_default(trie_id)
}

pub struct Config<T: Trait> {
    pub schedule: Schedule,
    pub existential_deposit: BalanceOf<T>,
    pub tombstone_deposit: BalanceOf<T>,
    pub max_depth: u32,
    pub max_value_size: u32,
}

impl<T: Trait> Config<T> {
    pub fn preload() -> Config<T> {
        Config {
            schedule: <Module<T>>::current_schedule(),
            existential_deposit: T::Currency::minimum_balance(),
            tombstone_deposit: T::TombstoneDeposit::get(),
            max_depth: T::MaxDepth::get(),
            max_value_size: T::MaxValueSize::get(),
        }
    }

    /// Subsistence threshold is the extension of the minimum balance (aka existential deposit) by the
    /// tombstone deposit, required for leaving a tombstone.
    ///
    /// Rent or any contract initiated balance transfer mechanism cannot make the balance lower
    /// than the subsistence threshold in order to guarantee that a tombstone is created.
    ///
    /// The only way to completely kill a contract without a tombstone is calling `seal_terminate`.
    pub fn subsistence_threshold(&self) -> BalanceOf<T> {
        self.existential_deposit
            .saturating_add(self.tombstone_deposit)
    }

    /// The same as `subsistence_threshold` but without the need for a preloaded instance.
    ///
    /// This is for cases where this value is needed in rent calculation rather than
    /// during contract execution.
    pub fn subsistence_threshold_uncached() -> BalanceOf<T> {
        T::Currency::minimum_balance().saturating_add(T::TombstoneDeposit::get())
    }
}
