//! Types for contracts registry RPC.

use jsonrpc_core::{Error as RpcError, ErrorCode};
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

/// Possible errors coming from this RPC. Same as the ones in the pallet.
#[derive(Debug)]
pub enum Error {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

impl From<Error> for i64 {
    fn from(e: Error) -> i64 {
        match e {
            Error::DoesntExist => 1,
            Error::IsTombstone => 2,
        }
    }
}

impl From<Error> for RpcError {
    fn from(e: Error) -> Self {
        Self {
            code: ErrorCode::ServerError(e.into()),
            message: match e {
                Error::DoesntExist => "Requested contract does not exist".into(),
                Error::IsTombstone => "Requested contract is inactive".into(),
            },
            data: Some(format!("{:?}", e).into()),
        }
    }
}
