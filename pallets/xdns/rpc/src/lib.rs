//! RPC interface for the XDNS pallet.

use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::CallError,
};
pub use pallet_xdns_rpc_runtime_api::XdnsRuntimeApi;
use pallet_xdns_rpc_runtime_api::{ChainId, GatewayABIConfig};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::sp_std;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};
use sp_std::prelude::*;
use std::sync::Arc;
use t3rn_primitives::xdns::{FullGatewayRecord, GatewayRecord};

const RUNTIME_ERROR: i64 = 1;

#[rpc(client, server)]
pub trait XdnsApi<AccountId> {
    /// Returns all known XDNS records
    #[method(name = "xdns_fetchRecords")]
    fn fetch_records(&self) -> RpcResult<Vec<GatewayRecord<AccountId>>>;

    #[method(name = "xdns_fetchAbi")]
    fn fetch_abi(&self, chain_id: ChainId) -> RpcResult<GatewayABIConfig>;

    #[method(name = "xdns_fetchFullRecords")]
    fn fetch_full_gateway_records(&self) -> RpcResult<Vec<FullGatewayRecord<AccountId>>>;
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
    fn fetch_records(&self) -> RpcResult<Vec<GatewayRecord<AccountId>>> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result = api.fetch_records(at).map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }

    fn fetch_full_gateway_records(&self) -> RpcResult<Vec<FullGatewayRecord<AccountId>>> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result = api
            .fetch_full_gateway_records(at)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }

    fn fetch_abi(&self, chain_id: ChainId) -> RpcResult<GatewayABIConfig> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result: Option<GatewayABIConfig> = api
            .fetch_abi(at, chain_id)
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
