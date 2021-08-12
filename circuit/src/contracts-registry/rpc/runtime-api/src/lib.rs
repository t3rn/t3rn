#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_contracts_registry::FetchContractsResult;
use sp_runtime::traits::MaybeDisplay;

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait ContractsRegistryRuntimeApi<AccountId, BlockNumber> where
        AccountId: Codec + MaybeDisplay,
        BlockNumber: Codec + MaybeDisplay,
    {
        /// Returns the contracts searchable by name, author or metadata
        fn fetch_contracts(
            origin: AccountId,
            // contract_id: Option<Vec<u8>>,
            // author: Option<AccountId>,
            // metadata: Option<Vec<u8>>,
        ) -> FetchContractsResult;
    }
}
