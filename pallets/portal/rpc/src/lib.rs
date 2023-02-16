//! RPC interface for the Portal pallet.

use std::sync::Arc;

pub use self::gen_client::Client as PortalClient;
use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};

use jsonrpc_derive::rpc;
use pallet_portal_rpc_runtime_api::ChainId;
pub use pallet_portal_rpc_runtime_api::PortalRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};

const RUNTIME_ERROR: i64 = 1;

#[rpc]
pub trait PortalApi<AccountId> {
    /// Returns latest finalized header of a gateway if available
    #[rpc(name = "portal_getLatestFinalizedHeader")]
    fn get_latest_finalized_header(&self, chain_id: ChainId) -> Result<Vec<u8>>;
}

/// A struct that implements the [`PortalApi`].
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

impl<C, Block, AccountId> PortalApi<AccountId> for Portal<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PortalRuntimeApi<Block, AccountId>,
{
    // ToDo ChainId decoding is not working, like in XDNS
    fn get_latest_finalized_header(&self, gateway_id: ChainId) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result: Option<Vec<u8>> = api.get_latest_finalized_header(&at, gateway_id).unwrap();

        match result {
            Some(header_hash) => Ok(header_hash),
            None => Err("No Finalized Header Found"),
        }
        .map_err(runtime_error_into_rpc_err)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{err:?}").into()),
    }
}
