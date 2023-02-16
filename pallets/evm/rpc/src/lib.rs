//! Node-specific RPC methods for interaction with evm contracts.

use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::CallError,
};
pub use pallet_evm_rpc_runtime_api::EvmRuntimeRPCApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{H160, U256};
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

const RUNTIME_ERROR: i64 = 1;

/// EVM RPC methods.
#[rpc(client, server)]
pub trait EvmApi<BlockHash, AccountId, Balance> {
    /// Returns an evm address if it is stored in the virtual machine
    #[method(name = "evm_get_evm_address")]
    fn get_evm_address(&self, account_id: AccountId, at: Option<BlockHash>) -> RpcResult<H160>;
    /// Returns an account if it is stored in the virtual machine or calculate it from the evm address
    #[method(name = "evm_get_or_into_account_id")]
    fn get_or_into_account_id(&self, address: H160, at: Option<BlockHash>) -> RpcResult<AccountId>;
    /// Returns an account if it is stored in the virtual machine or calculate it from the evm address
    #[method(name = "evm_get_threevm_info")]
    fn get_threevm_info(
        &self,
        address: H160,
        at: Option<BlockHash>,
    ) -> RpcResult<(AccountId, Balance, u8)>;
    /// Returns the account basic info if it is stored in the virtual machine
    #[method(name = "evm_account_info")]
    fn account_info(
        &self,
        address: H160,
        at: Option<BlockHash>,
    ) -> RpcResult<(U256, U256, Vec<u8>)>;
}

pub enum Error {
    NoEvmAddress,
    NoAccountId,
}
/// An implementation of contract specific RPC methods.
pub struct Evm<C, Block, AccountId, Balance> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<(Block, AccountId, Balance)>,
}

impl<C, Block, AccountId, Balance> Evm<C, Block, AccountId, Balance> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Evm {
            client,
            _marker: Default::default(),
        }
    }
}

#[async_trait]
impl<C, Block, AccountId, Balance> EvmApiServer<<Block as BlockT>::Hash, AccountId, Balance>
    for Evm<C, Block, AccountId, Balance>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: EvmRuntimeRPCApi<Block, AccountId, Balance>,
    AccountId: Codec + Clone + Send + Sync + 'static,
    Balance: Codec + Clone + Send + Sync + 'static,
{
    fn get_evm_address(
        &self,
        account_id: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<H160> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        match api.get_evm_address(&at, account_id) {
            Ok(Some(tuple)) => Ok(tuple),
            _ => Err("No evm address"),
        }
        .map_err(runtime_error_into_rpc_err)
    }

    fn get_or_into_account_id(
        &self,
        address: H160,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<AccountId> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));
        api.get_or_into_account_id(&at, address)
            .map_err(runtime_error_into_rpc_err)
    }

    fn get_threevm_info(
        &self,
        address: H160,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<(AccountId, Balance, u8)> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));
        match api.get_threevm_info(&at, address) {
            Ok(Some(tuple)) => Ok(tuple),
            _ => Err("No 3vm info"),
        }
        .map_err(runtime_error_into_rpc_err)
    }

    fn account_info(
        &self,
        address: H160,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<(U256, U256, Vec<u8>)> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));
        api.account_info(&at, address)
            .map_err(runtime_error_into_rpc_err)
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    JsonRpseeError::Call(CallError::Custom(jsonrpsee::types::ErrorObject::owned(
        RUNTIME_ERROR as i32,
        "Runtime Error - EVM RPC",
        Some(format!("{err:?}")),
    )))
}
