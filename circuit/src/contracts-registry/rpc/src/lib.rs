//! RPC interface for the contracts registry pallet.

mod types;

use std::sync::Arc;

pub use self::gen_client::Client as ContractsRegistryClient;
use crate::types::RpcFetchContractsResult;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_contracts_registry_rpc_runtime_api::ContractsRegistryApi as ContractsRegistryRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay},
};

#[rpc]
pub trait ContractsRegistryApi<BlockHash, AccountId> {
    /// Returns the contracts searchable by name, author or metadata
    #[rpc(name = "contractsRegistry_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        data: Option<Bytes>,
    ) -> Result<RpcFetchContractsResult>;

    /// Returns a single contract searchable by id
    #[rpc(name = "contractsRegistry_fetchContractById")]
    fn fetch_contract_by_id(&self, contract_id: Option<Bytes>) -> Result<RpcFetchContractsResult>;
}

/// A struct that implements the [ContractsRegistryApi].
pub struct ContractsRegistry<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> ContractsRegistry<C, P> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Possible errors coming from this RPC. Same as the ones in the pallet.
#[derive(Debug)]
pub enum Error {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

impl From<Error> for i64 {
    fn from(e: Error) -> i64 {
        match e {
            Error::DoesntExist => 1,
            Error::IsTombstone => 2,
        }
    }
}

impl From<Error> for RpcError {
    fn from(e: Error) -> Self {
        Self {
            code: ErrorCode::ServerError(e.into()),
            message: match e {
                Error::DoesntExist => "Requested contract does not exist".into(),
                Error::IsTombstone => "Requested contract is inactive".into(),
            },
            data: Some(format!("{:?}", e).into()),
        }
    }
}

impl<C, Block, AccountId> ContractsRegistryApi<<Block as BlockT>::Hash, AccountId>
    for ContractsRegistry<C, Block>
where
    Block: BlockT,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ContractsRegistryRuntimeApi<AccountId, Block::Hash>,
{
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        metadata: Option<Bytes>,
    ) -> Result<RpcFetchContractsResult> {
        let api = self.client.runtime_api();

        api.fetch_contracts(author, metadata).map_err(|e| e.into())
    }

    fn fetch_contract_by_id(&self, contract_id: Option<Bytes>) -> Result<RpcFetchContractsResult> {
        let api = self.client.runtime_api();

        api.fetch_contract_by_id(contract_id).map_err(|e| e.into())
    }
}
