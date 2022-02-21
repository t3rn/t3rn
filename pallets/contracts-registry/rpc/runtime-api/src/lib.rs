//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
pub use pallet_contracts_registry::FetchContractsResult;
use sp_core::Bytes;

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait ContractsRegistryRuntimeApi<AccountId> where
        AccountId: Codec
    {
        /// Returns the contracts searchable by name, author or metadata
        fn fetch_contracts(
            author: Option<AccountId>,
            metadata: Option<Vec<u8>>,
        ) -> FetchContractsResult;
    }
}
