//! Runtime API definition required by Portal RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding XDNS access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;

use sp_std::prelude::*;

pub use t3rn_primitives::ChainId;
use t3rn_types::sfx::SideEffect;

sp_api::decl_runtime_apis! {
    /// The API to interact with pallet XDNS
    pub trait PortalRuntimeApi<AccountId, Balance, Hash> where
        AccountId: Codec,
        Balance: Codec,
        Hash: Codec,
    {
        /// Returns the current head height of the given chain
        fn fetch_head_height(chain_id: ChainId) -> Option<u128>;
        fn fetch_all_active_xtx(for_executor: AccountId) -> Vec<(
            Hash,                              // xtx_id
            Vec<SideEffect<AccountId, Balance>>, // side_effects
            Vec<Hash>,                         // sfx_ids
        )>;
    }
}
