//! Types for contracts registry RPC.

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::Bytes;

/// An RPC serializable result of contracts fetch.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum RpcFetchContractsResult {
    /// Successful execution
    Success {
        /// The return flags
        flags: u32,
        /// Output data
        data: Bytes,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// Error execution
    Error(()),
}
