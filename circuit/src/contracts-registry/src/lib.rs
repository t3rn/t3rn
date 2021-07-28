// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

//! <!-- markdown-link-check-disable -->
//! # X-DNS Pallet
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_system::ensure_signed;
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::prelude::*;
use t3rn_primitives::{abi::ContractActionDesc, transfers::BalanceOf, Compose};
use volatile_vm::storage::RawAliveContractInfo;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod tests;

mod weights;

pub use weights::*;

pub type RegistryContractId<T> = <T as frame_system::Config>::Hash;
pub type ChainId = [u8; 4];

/// A preliminary representation of a contract in the onchain registry.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct RegistryContract<Hash, AccountId, BalanceOf, BlockNumber> {
    /// Original code text
    pub code_txt: Vec<u8>,
    /// Bytecode
    pub bytes: Vec<u8>,
    /// Original code author
    pub author: AccountId,
    /// Optional renumeration fee for the author
    pub author_fees_per_single_use: Option<BalanceOf>,
    /// Optional ABI
    pub abi: Option<Vec<u8>>,
    /// Action descriptions (calls for now)
    pub action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
    /// Contracts Info after Contracts Pallet
    pub info: RawAliveContractInfo<Hash, BalanceOf, BlockNumber>,
}

impl<Hash: Encode, AccountId: Encode, BalanceOf: Encode, BlockNumber: Encode>
    RegistryContract<Hash, AccountId, BalanceOf, BlockNumber>
{
    pub fn new(
        code_txt: Vec<u8>,
        bytes: Vec<u8>,
        author: AccountId,
        author_fees_per_single_use: Option<BalanceOf>,
        abi: Option<Vec<u8>>,
        action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
        info: RawAliveContractInfo<Hash, BalanceOf, BlockNumber>,
    ) -> Self {
        RegistryContract {
            code_txt,
            bytes,
            author,
            author_fees_per_single_use,
            abi,
            action_descriptions,
            info,
        }
    }

    pub fn generate_id<T: Config>(&self) -> RegistryContractId<T> {
        let mut protocol_part_of_contract = self.code_txt.clone();
        protocol_part_of_contract.extend(self.bytes.clone());
        T::Hashing::hash(Encode::encode(&protocol_part_of_contract).as_ref())
    }

    pub fn from_compose(
        compose: Compose<AccountId, BalanceOf>,
        action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
        author: AccountId,
        author_fees_per_single_use: Option<BalanceOf>,
        abi: Option<Vec<u8>>,
        info: RawAliveContractInfo<Hash, BalanceOf, BlockNumber>,
    ) -> RegistryContract<Hash, AccountId, BalanceOf, BlockNumber> {
        RegistryContract::new(
            compose.code_txt,
            compose.bytes,
            author,
            author_fees_per_single_use,
            abi,
            action_descriptions,
            info,
        )
    }
}

// Definition of the pallet logic, to be aggregated at runtime definition through
// `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        pallet_balances::Config + frame_system::Config + t3rn_primitives::EscrowTrait
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;
    }

    // Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
    // method.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Pallet implements [`Hooks`] trait to define some logic to execute in some context.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            // Anything that needs to be done at the start of the block.
            // We don't do anything here.
            0
        }

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {
            // Perform necessary data/state clean up here.
        }

        // A runtime code run after every block and have access to extended set of APIs.
        //
        // For instance you can generate extrinsics for the upcoming produced block.
        fn offchain_worker(_n: T::BlockNumber) {
            // We don't do anything here.
            // but we could dispatch extrinsic (transaction/unsigned/inherent) using
            // sp_io::submit_extrinsic.
            // To see example on offchain worker, please refer to example-offchain-worker pallet
            // accompanied in this repository.
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Inserts a contract into the on-chain registry. Root only access.
        #[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn add_new_contract(
            origin: OriginFor<T>,
            requester: T::AccountId,
            contract: RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let contract_id = contract.generate_id::<T>();

            assert!(
                requester == contract.author,
                "only the first submitter of contract to registry can become the author",
            );

            if <ContractsRegistry<T>>::contains_key(&contract_id) {
                Err(Error::<T>::ContractAlreadyExists)?
            } else {
                <ContractsRegistry<T>>::insert(&contract_id, contract);
                Self::deposit_event(Event::<T>::ContractStored(requester, contract_id));
                Ok(().into())
            }
        }

        /// Removes a contract from the onchain registry. Root only access.
        #[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn purge(
            origin: OriginFor<T>,
            requester: T::AccountId,
            contract_id: RegistryContractId<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if !<ContractsRegistry<T>>::contains_key(&contract_id) {
                Err(Error::<T>::UnknownContract)?
            } else {
                <ContractsRegistry<T>>::remove(&contract_id);
                Self::deposit_event(Event::<T>::ContractPurged(requester, contract_id));
                Ok(().into())
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[requester, contract_id\]
        ContractStored(T::AccountId, RegistryContractId<T>),
        /// \[requester, contract_id\]
        ContractPurged(T::AccountId, RegistryContractId<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Stored contract has already been added before
        ContractAlreadyExists,
        /// Access of unknown contract
        UnknownContract,
    }

    /// The pre-validated composable contracts on-chain registry.
    #[pallet::storage]
    #[pallet::getter(fn contracts_registry)]
    pub type ContractsRegistry<T> = StorageMap<
        _,
        Blake2_128Concat,
        RegistryContractId<T>,
        RegistryContract<
            <T as frame_system::Config>::Hash,
            <T as frame_system::Config>::AccountId,
            BalanceOf<T>,
            <T as frame_system::Config>::BlockNumber,
        >,
        OptionQuery,
    >;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub known_contracts: Vec<(T::AccountId, T::Balance)>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                known_contracts: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }
}

impl<T: Config> Pallet<T> {
    // Add public immutables and private mutables.
    #[allow(dead_code)]
    fn placeholder(origin: T::Origin) -> DispatchResult {
        let _sender = ensure_signed(origin)?;

        Ok(())
    }
}
