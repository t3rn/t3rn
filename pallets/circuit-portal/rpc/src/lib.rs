//! RPC interface for the contracts registry pallet.

use std::sync::Arc;
mod tests;
mod types;
pub use self::gen_client::Client as ContractsRegistryClient;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_circuit_portal_rpc_runtime_api::CircuitPortalRuntimeApi;
use sp_api::{codec::Codec, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT, MaybeDisplay},
};
use t3rn_primitives::ReadLatestGatewayHeight;
use types::*;

#[rpc]
pub trait CircuitPortalApi<BlockHash, BlockNumber, AccountId, Balance> {
    #[rpc(name = "circuitPortal_readLatestGatewayHeight")]
    fn read_latest_gateway_height(&self, gateway_id: [u8; 4]) -> Result<ReadLatestGatewayHeight>;
}

/// A struct that implements the [CircuitPortalApi].
pub struct CircuitPortal<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> CircuitPortal<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance>
    CircuitPortalApi<
        <Block as BlockT>::Hash,
        <<Block as BlockT>::Header as HeaderT>::Number,
        AccountId,
        Balance,
    > for CircuitPortal<C, Block>
where
    Block: BlockT,
    AccountId: Codec + MaybeDisplay,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: CircuitPortalRuntimeApi<
        Block,
        AccountId,
        Balance,
        <<Block as BlockT>::Header as HeaderT>::Number,
    >,
    Balance: Codec,
{
    fn read_latest_gateway_height(&self, gateway_id: [u8; 4]) -> Result<ReadLatestGatewayHeight> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result = api
            .read_latest_gateway_height(&at, gateway_id)
            .map_err(|e| runtime_error_into_rpc_err(e))?;

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
