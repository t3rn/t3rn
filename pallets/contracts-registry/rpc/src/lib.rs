//! RPC interface for the contracts registry pallet.

use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
};
pub use pallet_contracts_registry_rpc_runtime_api::ContractsRegistryRuntimeApi;
use pallet_contracts_registry_rpc_runtime_api::FetchContractsResult;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    traits::{Block as BlockT, MaybeDisplay},
};

const RUNTIME_ERROR: i64 = 1;

#[rpc(client, server)]
pub trait ContractsRegistryApi<AccountId> {
    /// Returns the contracts searchable by name, author or metadata
    #[method(name = "contractsRegistry_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        metadata: Option<Vec<u8>>,
    ) -> RpcResult<FetchContractsResult>;
}

/// A struct that implements the [ContractsRegistryApi].
pub struct ContractsRegistry<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> ContractsRegistry<C, B> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}
impl<C, Block, AccountId> ContractsRegistryApiServer<AccountId> for ContractsRegistry<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ContractsRegistryRuntimeApi<Block, AccountId>,
{
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        metadata: Option<Vec<u8>>,
    ) -> RpcResult<FetchContractsResult> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        let result = api
            .fetch_contracts(at, author, metadata)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    JsonRpseeError::Custom(format!("{err:?}"))
}
