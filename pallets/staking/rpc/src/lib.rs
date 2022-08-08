//! RPC interface for the contracts registry pallet.

use std::sync::Arc;
// mod tests;
// mod types;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use pallet_staking_rpc_runtime_api::StakingRuntimeApi;
use sp_api::codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT, MaybeDisplay},
};
use std::convert::TryInto;
use types::*;

#[rpc]
pub trait StakingApi<BlockHash, BlockNumber, AccountId, Balance> {
    #[rpc(name = "circuitPortal_readLatestGatewayHeight")]
    fn read_latest_gateway_height(
        &self,
        gateway_id: [u8; 4],
    ) -> Result<RpcReadLatestGatewayHeight>;
}

/// A struct that implements the [StakingApi].
pub struct Staking<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Staking<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance>
    StakingApi<
        <Block as BlockT>::Hash,
        <<Block as BlockT>::Header as HeaderT>::Number,
        AccountId,
        Balance,
    > for Staking<C, Block>
where
    Block: BlockT,
    AccountId: Codec + MaybeDisplay,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: StakingRuntimeApi<
        Block,
        AccountId,
        Balance,
        <<Block as BlockT>::Header as HeaderT>::Number,
    >,
    Balance: Codec,
{
    //TODO
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
