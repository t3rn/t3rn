//! Runtime API definition required by XDNS RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding XDNS access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_xdns::types::FetchXdnsRecordsResponse;

sp_api::decl_runtime_apis! {
    /// The API to interact with pallet XDNS
    pub trait XdnsRuntimeApi<AccountId> where
        AccountId: Codec,
    {
        /// Returns metadata for all known Blockchains
        fn fetch_chains() -> FetchXdnsRecordsResponse<AccountId>;
    }
}
