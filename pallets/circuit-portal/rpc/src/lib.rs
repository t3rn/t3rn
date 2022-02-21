//! RPC interface for the contracts registry pallet.

use std::sync::Arc;
mod tests;
mod types;
pub use self::gen_client::Client as ContractsRegistryClient;
use crate::types::GAS_PER_SECOND;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use pallet_circuit_circuit_portal_rpc_runtime_api::CircuitPortalRuntimeApi;
use sp_api::codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT, MaybeDisplay},
};
use std::convert::TryInto;
use t3rn_primitives::Compose;
use types::*;

#[rpc]
pub trait CircuitPortalApi<BlockHash, BlockNumber, AccountId, Balance> {
    /// Executes all attached or appointed by ID composable contracts on appointed gateways.
    ///
    /// IO flow between components on different chains can be described using Input-Output schedule.
    ///
    /// Circuit queues the request and awaits for an execution agent to volounteer to facilitate the execution
    /// across connected chains via gateways - acts as an escrow account and is accountable
    /// with her stake for proven misbehaviour.
    #[rpc(name = "circuitPortal_composableExec")]
    fn composable_exec(
        &self,
        call_request: InterExecRequest<AccountId, Balance>,
        at: Option<BlockHash>,
    ) -> Result<RpcComposableExecResult>;
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
    fn composable_exec(
        &self,
        inter_exec_request: InterExecRequest<AccountId, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<RpcComposableExecResult> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let InterExecRequest {
            origin,
            components,
            io,
            gas_limit,
            input_data,
        } = inter_exec_request;

        // Make sure that gas_limit fits into 64 bits.
        let gas_limit: u64 = gas_limit.try_into().map_err(|_| Error {
            code: ErrorCode::InvalidParams,
            message: format!("{:?} doesn't fit in 64 bit unsigned value", gas_limit),
            data: None,
        })?;

        let max_gas_limit = 5 * GAS_PER_SECOND;
        if gas_limit > max_gas_limit {
            return Err(Error {
                code: ErrorCode::InvalidParams,
                message: format!(
                    "Requested gas limit is greater than maximum allowed: {} > {}",
                    gas_limit, max_gas_limit
                ),
                data: None,
            });
        }

        let mut components_runtime: Vec<Compose<AccountId, Balance>> = vec![];

        for component_rpc in components.into_iter() {
            components_runtime.push(Compose {
                name: component_rpc.name.into_boxed_bytes().to_vec(),
                code_txt: component_rpc.code_txt.into_boxed_bytes().to_vec(),
                exec_type: component_rpc.exec_type.into_boxed_bytes().to_vec(),
                dest: component_rpc.dest,
                value: component_rpc.value,
                bytes: component_rpc.bytes.to_vec(),
                input_data: component_rpc.input_data.to_vec(),
            });
        }

        let exec_result = api
            .composable_exec(
                &at,
                origin,
                components_runtime,
                io.into_boxed_bytes().to_vec(),
                gas_limit,
                input_data.to_vec(),
            )
            .map_err(|e| runtime_error_into_rpc_err(e))?;

        Ok(exec_result.into())
    }
}

fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
