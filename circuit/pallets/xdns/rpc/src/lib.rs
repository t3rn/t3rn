//! RPC interface for the XDNS pallet.

use std::sync::Arc;

pub use self::gen_client::Client as ContractsRegistryClient;
use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_core_client::RpcError;
use jsonrpc_derive::rpc;
use pallet_xdns::types::FetchXdnsRecordsResponse;
use pallet_xdns_rpc_runtime_api::*;
use sp_api::{ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};

const RUNTIME_ERROR: i64 = 1;
const NO_KNOWN_RECORDS: i64 = 2;

#[rpc]
pub trait XdnsApi<AccountId> {
    /// Returns all known XDNS records
    #[rpc(name = "xdns_fetchRecords")]
    fn fetch_records(&self) -> Result<FetchXdnsRecordsResponse<AccountId>>;
}

pub struct Xdns<Client, Block> {
    client: Arc<Client>,
    _marker: std::marker::PhantomData<Block>,
}

impl<Client, Block, AccountId> XdnsApi<AccountId> for Xdns<Client, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    Client: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    Client::Api: XdnsApi<AccountId>
{
    fn fetch_records(&self) -> Result<FetchXdnsRecordsResponse<AccountId>> {
        let api = self.client.runtime_api();

        let result = api.fetch_records()
            .map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
