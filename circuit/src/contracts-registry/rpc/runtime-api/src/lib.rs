#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_contracts_registry::{FetchContractsResult, RegistryContractId};
use sp_runtime::{sp_std::vec::Vec, traits::MaybeDisplay};

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait ContractsRegistryApi<AccountId, Hash> where
        AccountId: Codec + MaybeDisplay,
        Hash: Codec + MaybeDisplay + frame_system::Config,
    {
        /// Returns the contracts searchable by name, author or metadata
        fn fetch_contracts(
            author: Option<AccountId>,
            metadata: Option<Vec<u8>>,
        ) -> FetchContractsResult;

        /// Returns a single contract by ID.
        fn fetch_contract_by_id(
            contract_id: Option<RegistryContractId<Hash>>,
        ) -> FetchContractsResult;
    }
}
