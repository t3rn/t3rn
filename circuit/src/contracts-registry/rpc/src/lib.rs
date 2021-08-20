//! RPC interface for the contracts registry pallet.

use std::sync::Arc;

mod types;

pub use self::gen_client::Client as ContractsRegistryClient;
use crate::types::RpcFetchContractsResult;
use codec::Codec;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
pub use pallet_contracts_registry_rpc_runtime_api::ContractsRegistryApi as ContractsRegistryRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::traits::{Block as BlockT, Hash as HashT, MaybeDisplay};

#[rpc]
pub trait ContractsRegistryApi<AccountId, Hash> {
    /// Returns the contracts searchable by name, author or metadata
    #[rpc(name = "contractsRegistry_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        data: Option<Bytes>,
    ) -> Result<RpcFetchContractsResult>;

    /// Returns a single contract searchable by id
    #[rpc(name = "contractsRegistry_fetchContractById")]
    fn fetch_contract_by_id(&self, contract_id: Option<Hash>) -> Result<RpcFetchContractsResult>;
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

impl<C, Block, AccountId, Hash> ContractsRegistryApi<AccountId, Hash>
    for ContractsRegistry<C, Block>
where
    AccountId: Codec + MaybeDisplay,
    Block: BlockT,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ContractsRegistryRuntimeApi<Block, AccountId, Hash>,
    Hash: HashT + Codec,
{
    fn fetch_contracts(
        &self,
        author: Option<AccountId>,
        metadata: Option<Bytes>,
    ) -> Result<RpcFetchContractsResult> {
        let api = self.client.runtime_api();

        let result = api
            .fetch_contracts(&api, author, metadata)
            .map_err(|e| e.into());

        if result.is_err() {
            return Err(RpcFetchContractsResult::Error(()).into());
        }
        // TODO: update flags and gas consumed
        Ok(RpcFetchContractsResult::Success {
            data: result.unwrap(),
            flags: 0,
            gas_consumed: 0,
        })
    }

    fn fetch_contract_by_id(&self, contract_id: Option<Hash>) -> Result<RpcFetchContractsResult> {
        let api = self.client.runtime_api();

        let result = api.fetch_contract_by_id(contract_id).map_err(|e| e.into());
        if result.is_err() {
            return Err(RpcFetchContractsResult::Error(()).into());
        }
        // TODO: update flags and gas consumed
        Ok(RpcFetchContractsResult::Success {
            data: result.unwrap().encode(),
            flags: 0,
            gas_consumed: 0,
        })
    }
}
