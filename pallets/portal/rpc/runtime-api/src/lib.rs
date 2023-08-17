//! Runtime API definition required by Portal RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding XDNS access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;

pub use t3rn_primitives::ChainId;

sp_api::decl_runtime_apis! {
    /// The API to interact with pallet XDNS
    pub trait PortalRuntimeApi<AccountId> where
        AccountId: Codec,
    {
        /// Returns the current head height of the given chain
        fn fetch_head_height(chain_id: ChainId) -> Option<u128>;
    }
}
