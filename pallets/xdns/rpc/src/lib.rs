//! RPC interface for the XDNS pallet.

use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::CallError,
};
pub use pallet_xdns_rpc_runtime_api::XdnsRuntimeApi;
use pallet_xdns_rpc_runtime_api::{ChainId, FetchXdnsRecordsResponse, GatewayABIConfig};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};
use std::sync::Arc;

const RUNTIME_ERROR: i64 = 1;

#[rpc(client, server)]
pub trait XdnsApi<AccountId> {
    /// Returns all known XDNS records
    #[method(name = "xdns_fetchRecords")]
    fn fetch_records(&self) -> RpcResult<FetchXdnsRecordsResponse<AccountId>>;

    #[method(name = "xdns_fetchAbi")]
    fn fetch_abi(&self, chain_id: ChainId) -> RpcResult<GatewayABIConfig>;
}

/// A struct that implements the [`XdnsApiServer`].
pub struct Xdns<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> Xdns<C, P> {
    /// Create new `Xdns` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

#[async_trait]
impl<C, Block, AccountId> XdnsApiServer<AccountId> for Xdns<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: XdnsRuntimeApi<Block, AccountId>,
{
    fn fetch_records(&self) -> RpcResult<FetchXdnsRecordsResponse<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result = api.fetch_records(&at).map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }

    fn fetch_abi(&self, chain_id: ChainId) -> RpcResult<GatewayABIConfig> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result: Option<GatewayABIConfig> = api
            .fetch_abi(&at, chain_id)
            .map_err(runtime_error_into_rpc_err)?;

        match result {
            Some(abi) => Ok(abi),
            None => Err("ABI doesn't exist"),
        }
        .map_err(runtime_error_into_rpc_err)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    JsonRpseeError::Call(CallError::Custom(jsonrpsee::types::ErrorObject::owned(
        RUNTIME_ERROR as i32,
        "Runtime Error - XDNS RPC",
        Some(format!("{err:?}")),
    )))
}
