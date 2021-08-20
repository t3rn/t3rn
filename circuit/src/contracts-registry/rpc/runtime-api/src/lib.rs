#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_contracts_registry::{FetchContractsResult, RegistryContractId};
use sp_core::Bytes;
use sp_runtime::traits::MaybeDisplay;

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait ContractsRegistryApi<AccountId, Hash> where
        AccountId: Codec,
        Hash: Codec,
    {
        /// Returns the contracts searchable by name, author or metadata
        fn fetch_contracts(
            author: Option<AccountId>,
            metadata: Option<Bytes>,
        ) -> FetchContractsResult;

        /// Returns a single contract by ID.
        fn fetch_contract_by_id(
            contract_id: Option<Hash>,
        ) -> FetchContractsResult;
    }
}
