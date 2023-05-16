//! RPC interface for the contracts registry pallet.

use std::sync::Arc;

// pub use self::gen_client::Client as ContractsRegistryClient;
use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};

use jsonrpc_derive::rpc;
pub use pallet_contracts_registry_rpc_runtime_api::ContractsRegistryRuntimeApi;
use pallet_contracts_registry_rpc_runtime_api::FetchContractsResult;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Hash as HashT, MaybeDisplay},
};

const RUNTIME_ERROR: i64 = 1;
const CONTRACT_DOESNT_EXIST: i64 = 2;
const CONTRACT_IS_A_TOMBSTONE: i64 = 3;

#[rpc(server)]
pub trait ContractsRegistryApi<AccountId> {
    /// Returns the contracts searchable by name, author or metadata
    #[rpc(name = "contractsRegistry_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        data: Option<Vec<u8>>,
    ) -> Result<FetchContractsResult>;
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

impl<C, Block, AccountId> ContractsRegistryApi<AccountId> for ContractsRegistry<C, Block>
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
    ) -> Result<FetchContractsResult> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);

        let result = api
            .fetch_contracts(&at, author, metadata)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(result)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{err:?}").into()),
    }
}
