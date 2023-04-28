//! RPC interface for the XDNS pallet.

use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::CallError,
};
use pallet_portal_rpc_runtime_api::ChainId;
pub use pallet_portal_rpc_runtime_api::PortalRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::sp_std;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};
use sp_std::prelude::*;
use std::sync::Arc;

const RUNTIME_ERROR: i64 = 1;

#[rpc(client, server)]
pub trait PortalApi<AccountId> {
    /// Returns all known XDNS records
    #[method(name = "portal_fetchHeadHeight")]
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128>;
}

/// A struct that implements the [`PortalApiServer`].
pub struct Portal<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> Portal<C, P> {
    /// Create new `Portal` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

#[async_trait]
impl<C, Block, AccountId> PortalApiServer<AccountId> for Portal<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PortalRuntimeApi<Block, AccountId>,
{
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result: Option<u128> = api
            .fetch_head_height(&at, chain_id)
            .map_err(runtime_error_into_rpc_err)?;

        match result {
            Some(height) => Ok(height),
            None => Err("ABI doesn't exist"),
        }
        .map_err(runtime_error_into_rpc_err)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    JsonRpseeError::Call(CallError::Custom(jsonrpsee::types::ErrorObject::owned(
        RUNTIME_ERROR as i32,
        "Runtime Error - Portal RPC",
        Some(format!("{err:?}")),
    )))
}
