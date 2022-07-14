//! RPC interface for the XDNS pallet.

use std::sync::Arc;

pub use self::gen_client::Client as XdnsClient;
use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};

use jsonrpc_derive::rpc;
pub use pallet_xdns_rpc_runtime_api::XdnsRuntimeApi;
use pallet_xdns_rpc_runtime_api::{ChainId, FetchXdnsRecordsResponse, GatewayABIConfig};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};

const RUNTIME_ERROR: i64 = 1;

#[rpc]
pub trait XdnsApi<AccountId> {
    /// Returns all known XDNS records
    #[rpc(name = "xdns_fetchRecords")]
    fn fetch_records(&self) -> Result<FetchXdnsRecordsResponse<AccountId>>;

    #[rpc(name = "xdns_fetchAbi")]
    fn fetch_abi(&self, chain_id: ChainId) -> Result<GatewayABIConfig>;
}

/// A struct that implements the [`XdnsApi`].
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

impl<C, Block, AccountId> XdnsApi<AccountId> for Xdns<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: XdnsRuntimeApi<Block, AccountId>,
{
    fn fetch_records(&self) -> Result<FetchXdnsRecordsResponse<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result = api.fetch_records(&at).map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }

    fn fetch_abi(&self, chain_id: ChainId) -> Result<GatewayABIConfig> {
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

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
