//! RPC interface for the Portal pallet.

use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
};
use sp_std::vec::Vec;

use pallet_portal_rpc_runtime_api::ChainId;
pub use pallet_portal_rpc_runtime_api::PortalRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};
use t3rn_types::sfx::SideEffect;

const RUNTIME_ERROR: i64 = 1;

#[rpc(client, server)]
pub trait PortalApi<AccountId, Balance, Hash> {
    /// Returns latest finalized header of a gateway if available
    #[method(name = "portal_fetchHeadHeight")]
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128>;

    #[method(name = "portal_allActiveXtx")]
    fn fetch_all_active_xtx(
        &self,
        for_executor: AccountId,
    ) -> RpcResult<
        Vec<(
            Hash,                                // xtx_id
            Vec<SideEffect<AccountId, Balance>>, // side_effects
            Vec<Hash>,                           // sfx_ids
        )>,
    >;
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

impl<C, Block, AccountId, Balance, Hash> PortalApiServer<AccountId, Balance, Hash>
    for Portal<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Balance: Codec + MaybeDisplay,
    Hash: Codec + MaybeDisplay,
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: PortalRuntimeApi<Block, AccountId, Balance, Hash>,
{
    // ToDo ChainId decoding is not working, like in XDNS
    fn fetch_head_height(&self, chain_id: ChainId) -> RpcResult<u128> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result: Option<u128> = api
            .fetch_head_height(at, chain_id)
            .map_err(runtime_error_into_rpc_err)?;

        match result {
            Some(height) => Ok(height),
            None => Err(runtime_error_into_rpc_err("ABI doesn't exist")),
        }
    }

    fn fetch_all_active_xtx(
        &self,
        for_executor: AccountId,
    ) -> RpcResult<
        Vec<(
            Hash,                                // xtx_id
            Vec<SideEffect<AccountId, Balance>>, // side_effects
            Vec<Hash>,                           // sfx_ids
        )>,
    > {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result: Vec<(
            Hash,                                // xtx_id
            Vec<SideEffect<AccountId, Balance>>, // side_effects
            Vec<Hash>,                           // sfx_ids
        )> = api
            .fetch_all_active_xtx(at, for_executor)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(result)
        // match result {
        //     Some() => Ok(height),
        //     None => Err(runtime_error_into_rpc_err(
        //         "fetch_all_active_xtx failed exist",
        //     )),
        // }
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    JsonRpseeError::Custom(format!("{err:?}"))
}
