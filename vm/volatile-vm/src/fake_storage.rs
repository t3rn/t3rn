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

//! This module contains routines for accessing and altering a contract related state.

use crate::{CodeHash, Config, Error, StorageKey, TrieId};

use t3rn_primitives::transfers::BalanceOf;

use codec::{Codec, Decode, Encode};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    storage::child::{self, ChildInfo},
    weights::Weight,
};

use sp_io::hashing::blake2_256;
use sp_runtime::{
    traits::{Bounded, Hash, MaybeSerializeDeserialize, Member, Saturating, Zero},
    RuntimeDebug,
};
use sp_std::prelude::*;
use sp_std::{fmt::Debug, marker::PhantomData};

pub type AliveContractInfo<T> =
    RawAliveContractInfo<CodeHash<T>, BalanceOf<T>, <T as system::Config>::BlockNumber>;
pub type TombstoneContractInfo<T> =
    RawTombstoneContractInfo<<T as system::Config>::Hash, <T as system::Config>::Hashing>;

/// Information for managing an account and its sub trie abstraction.
/// This is the required info to cache for an account
#[derive(Encode, Decode, RuntimeDebug)]
pub enum ContractInfo<T: Config> {
    Alive(AliveContractInfo<T>),
    Tombstone(TombstoneContractInfo<T>),
}

impl<T: Config> ContractInfo<T> {
    /// If contract is alive then return some alive info
    pub fn get_alive(self) -> Option<AliveContractInfo<T>> {
        if let ContractInfo::Alive(alive) = self {
            Some(alive)
        } else {
            None
        }
    }

    /// If contract is alive then return some reference to alive info
    #[cfg(test)]
    pub fn as_alive(&self) -> Option<&AliveContractInfo<T>> {
        if let ContractInfo::Alive(ref alive) = self {
            Some(alive)
        } else {
            None
        }
    }

    /// If contract is tombstone then return some tombstone info
    pub fn get_tombstone(self) -> Option<TombstoneContractInfo<T>> {
        if let ContractInfo::Tombstone(tombstone) = self {
            Some(tombstone)
        } else {
            None
        }
    }
}

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
    /// The total number of key-value pairs in storage of this contract.
    pub pair_count: u32,
    /// The code associated with a given account.
    pub code_hash: CodeHash,
    /// Pay rent at most up to this value.
    pub rent_allowance: Balance,
    /// The amount of rent that was payed by the contract over its whole lifetime.
    ///
    /// A restored contract starts with a value of zero just like a new contract.
    pub rent_payed: Balance,
    /// Last block rent has been payed.
    pub deduct_block: BlockNumber,
    /// Last block child storage has been written.
    pub last_write: Option<BlockNumber>,
    /// This field is reserved for future evolution of format.
    pub _reserved: Option<()>,
}

impl<CodeHash, Balance, BlockNumber> RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Associated child trie unique id is built from the hash part of the trie id.
    pub fn child_trie_info(&self) -> ChildInfo {
        child_trie_info(&self.trie_id[..])
    }
}

/// Associated child trie unique id is built from the hash part of the trie id.
fn child_trie_info(trie_id: &[u8]) -> ChildInfo {
    ChildInfo::new_default(trie_id)
}

#[derive(Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct RawTombstoneContractInfo<H, Hasher>(H, PhantomData<Hasher>);

impl<H, Hasher> RawTombstoneContractInfo<H, Hasher>
where
    H: Member
        + MaybeSerializeDeserialize
        + Debug
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Default
        + sp_std::hash::Hash
        + Codec,
    Hasher: Hash<Output = H>,
{
    pub fn new(storage_root: &[u8], code_hash: H) -> Self {
        let mut buf = Vec::new();
        storage_root.using_encoded(|encoded| buf.extend_from_slice(encoded));
        buf.extend_from_slice(code_hash.as_ref());
        RawTombstoneContractInfo(<Hasher as Hash>::hash(&buf[..]), PhantomData)
    }
}

impl<T: Config> From<AliveContractInfo<T>> for ContractInfo<T> {
    fn from(alive_info: AliveContractInfo<T>) -> Self {
        Self::Alive(alive_info)
    }
}

#[derive(Encode, Decode)]
pub struct DeletedContract {
    pair_count: u32,
    trie_id: TrieId,
}

pub struct Storage<T>(PhantomData<T>);

impl<T> Storage<T>
where
    T: Config,
    // T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>
{
    /// Reads a storage kv pair of a contract.
    ///
    /// The read is performed from the `trie_id` only. The `address` is not necessary. If the contract
    /// doesn't store under the given `key` `None` is returned.
    pub fn read(trie_id: &TrieId, key: &StorageKey) -> Option<Vec<u8>> {
        child::get_raw(&child_trie_info(&trie_id), &blake2_256(key))
    }

    /// Update a storage entry into a contract's kv storage.
    ///
    /// If the `opt_new_value` is `None` then the kv pair is removed.
    ///
    /// This function also updates the bookkeeping info such as: number of total non-empty pairs a
    /// contract owns, the last block the storage was written to, etc. That's why, in contrast to
    /// `read`, this function also requires the `account` ID.
    pub fn write(
        block_number: T::BlockNumber,
        new_info: &mut AliveContractInfo<T>,
        key: &StorageKey,
        opt_new_value: Option<Vec<u8>>,
    ) -> DispatchResult {
        let hashed_key = blake2_256(key);
        let child_trie_info = &child_trie_info(&new_info.trie_id);

        let opt_prev_len = child::len(&child_trie_info, &hashed_key);

        // Update the total number of KV pairs and the number of empty pairs.
        match (&opt_prev_len, &opt_new_value) {
            (Some(_), None) => {
                new_info.pair_count = new_info
                    .pair_count
                    .checked_sub(1)
                    .ok_or_else(|| Error::<T>::StorageExhausted)?;
            }
            (None, Some(_)) => {
                new_info.pair_count = new_info
                    .pair_count
                    .checked_add(1)
                    .ok_or_else(|| Error::<T>::StorageExhausted)?;
            }
            (Some(_), Some(_)) => {}
            (None, None) => {}
        }

        // Update the total storage size.
        let prev_value_len = opt_prev_len.unwrap_or(0);
        let new_value_len = opt_new_value
            .as_ref()
            .map(|new_value| new_value.len() as u32)
            .unwrap_or(0);
        new_info.storage_size = new_info
            .storage_size
            .checked_sub(prev_value_len)
            .and_then(|val| val.checked_add(new_value_len))
            .ok_or_else(|| Error::<T>::StorageExhausted)?;

        new_info.last_write = Some(block_number);

        // Finally, perform the change on the storage.
        match opt_new_value {
            Some(new_value) => child::put_raw(&child_trie_info, &hashed_key, &new_value[..]),
            None => child::kill(&child_trie_info, &hashed_key),
        }

        Ok(())
    }

    /// Creates a new contract descriptor in the storage with the given code hash at the given address.
    ///
    /// Returns `Err` if there is already a contract (or a tombstone) exists at the given address.
    pub fn new_contract(
        _account: &T::AccountId,
        trie_id: TrieId,
        ch: CodeHash<T>,
    ) -> Result<AliveContractInfo<T>, DispatchError> {
        let contract = AliveContractInfo::<T> {
            code_hash: ch,
            storage_size: 0,
            trie_id,
            deduct_block:
            // We want to charge rent for the first block in advance. Therefore we
            // treat the contract as if it was created in the last block and then
            // charge rent for it during instantiation.
            <system::Pallet<T>>::block_number().saturating_sub(1u32.into()),
            rent_allowance: <BalanceOf<T>>::max_value(),
            rent_payed: <BalanceOf<T>>::zero(),
            pair_count: 0,
            last_write: None,
            _reserved: None,
        };

        Ok(contract)
    }

    /// Push a contract's trie to the deletion queue for lazy removal.
    ///
    /// You must make sure that the contract is also removed or converted into a tombstone
    /// when queuing the trie for deletion.
    pub fn queue_trie_for_deletion(_contract: &AliveContractInfo<T>) -> DispatchResult {
        unimplemented!()
    }

    /// Calculates the weight that is necessary to remove one key from the trie and how many
    /// of those keys can be deleted from the deletion queue given the supplied queue length
    /// and weight limit.
    pub fn deletion_budget(_queue_len: usize, _weight_limit: Weight) -> (u64, u32) {
        unimplemented!("contracts are not supposed to be stored using VVM")
    }

    /// Delete as many items from the deletion queue possible within the supplied weight limit.
    ///
    /// It returns the amount of weight used for that task or `None` when no weight was used
    /// apart from the base weight.
    pub fn process_deletion_queue_batch(_weight_limit: Weight) -> Weight {
        unimplemented!()
    }

    /// This generator uses inner counter for account id and applies the hash over `AccountId +
    /// accountid_counter`.
    pub fn generate_trie_id(account_id: &T::AccountId, seed: u64) -> TrieId {
        let buf: Vec<_> = account_id
            // .as_ref()
            .encode()
            .iter()
            .chain(&seed.to_le_bytes())
            .cloned()
            .collect();
        T::Hashing::hash(&buf).as_ref().into()
    }

    /// Returns the code hash of the contract specified by `account` ID.
    #[cfg(test)]
    pub fn code_hash(_account: &T::AccountId) -> Option<CodeHash<T>> {
        unimplemented!()
        // <ContractInfoOf<T>>::get(account).and_then(|i| i.as_alive().map(|i| i.code_hash))
    }

    /// Fill up the queue in order to exercise the limits during testing.
    #[cfg(test)]
    pub fn fill_queue_with_dummies() {
        unimplemented!()
    }
}
