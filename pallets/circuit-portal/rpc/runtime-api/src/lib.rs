//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use sp_runtime::codec::Codec;
use sp_std::vec::Vec;
use t3rn_primitives::ReadLatestGatewayHeight;

sp_api::decl_runtime_apis! {
    /// The API to interact with circuit portal
    pub trait CircuitPortalRuntimeApi<AccountId, Balance, BlockNumber> where
        AccountId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
    {
        fn read_latest_gateway_height(
            gateway_id: [u8; 4],
        ) -> ReadLatestGatewayHeight;
    }
}
