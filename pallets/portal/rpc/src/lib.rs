//! RPC interface for the XDNS pallet.

<<<<<<< HEAD
use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::CallError,
=======
use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorCode, ErrorObject},
>>>>>>> origin/chore/update-flow
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
<<<<<<< HEAD
    /// Returns all known XDNS records
    #[method(name = "portal_fetchHeadHeight")]
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128>;
=======
    /// Returns latest finalized header of a gateway if available
    #[method(name = "portal_getLatestFinalizedHeader")]
    fn get_latest_finalized_header(&self, chain_id: ChainId) -> RpcResult<Vec<u8>>;
>>>>>>> origin/chore/update-flow
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

<<<<<<< HEAD
#[async_trait]
=======
>>>>>>> origin/chore/update-flow
impl<C, Block, AccountId> PortalApiServer<AccountId> for Portal<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: PortalRuntimeApi<Block, AccountId>,
{
<<<<<<< HEAD
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128> {
=======
    // ToDo ChainId decoding is not working, like in XDNS
    fn get_latest_finalized_header(&self, gateway_id: ChainId) -> RpcResult<Vec<u8>> {
>>>>>>> origin/chore/update-flow
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

<<<<<<< HEAD
        let result: Option<u128> = api
            .fetch_head_height(&at, chain_id)
            .map_err(runtime_error_into_rpc_err)?;
=======
        let result: Option<Vec<u8>> = api.get_latest_finalized_header(at, gateway_id).unwrap();
>>>>>>> origin/chore/update-flow

        match result {
            Some(height) => Ok(height),
            None => Err("ABI doesn't exist"),
        }
        .map_err(runtime_error_into_rpc_err)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
<<<<<<< HEAD
    JsonRpseeError::Call(CallError::Custom(jsonrpsee::types::ErrorObject::owned(
        RUNTIME_ERROR as i32,
        "Runtime Error - Portal RPC",
        Some(format!("{err:?}")),
    )))
=======
    JsonRpseeError::Custom(format!("{err:?}").into())
>>>>>>> origin/chore/update-flow
}
