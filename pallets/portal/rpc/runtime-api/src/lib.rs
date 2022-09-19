//! Runtime API definition required by XDNS RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding XDNS access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{alloc::vec::Vec, Codec};
pub use t3rn_primitives::ChainId;

sp_api::decl_runtime_apis! {
    /// The API to interact with pallet XDNS
    pub trait PortalRuntimeApi<AccountId> where
        AccountId: Codec,
    {
        /// Returns hash of latest finalized header
        fn get_latest_finalized_header(gateway_id: ChainId) -> Option<Vec<u8>>;
    }
}
