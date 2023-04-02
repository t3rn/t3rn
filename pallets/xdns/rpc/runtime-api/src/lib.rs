//! Runtime API definition required by XDNS RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding XDNS access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_runtime::sp_std;
use sp_std::prelude::*;
use t3rn_primitives::xdns::GatewayRecord;
pub use t3rn_primitives::{gateway::GatewayABIConfig, ChainId};

sp_api::decl_runtime_apis! {
    /// The API to interact with pallet XDNS
    pub trait XdnsRuntimeApi<AccountId> where
        AccountId: Codec,
    {
        /// Returns metadata for all known Blockchains
        fn fetch_records() -> Vec<GatewayRecord<AccountId>>;

        /// Returns the GatewayABIConfig for a given ChainId
        fn fetch_abi(chain_id: ChainId) -> Option<GatewayABIConfig>;
    }
}
