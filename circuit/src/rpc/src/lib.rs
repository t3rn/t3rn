// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Node-specific RPC methods for interaction with circuit.

use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT},
};
use sp_std::{prelude::*, str};
use std::convert::TryInto;

pub use self::gen_client::Client as ContractsClient;
pub use circuit_rpc_runtime_api::{self as runtime_api, CircuitApi as CircuitRuntimeApi};
use t3rn_primitives::{ComposableExecResult, Compose, ContractAccessError};

const RUNTIME_ERROR: i64 = 1;
const CONTRACT_DOESNT_EXIST: i64 = 2;
const CONTRACT_IS_A_TOMBSTONE: i64 = 3;

/// A rough estimate of how much gas a decent hardware consumes per second,
/// using native execution.
/// This value is used to set the upper bound for maximal contract calls to
/// prevent blocking the RPC for too long.
///
/// As 1 gas is equal to 1 weight we base this on the conducted benchmarks which
/// determined runtime weights:
/// https://github.com/paritytech/substrate/pull/5446
const GAS_PER_SECOND: u64 = 1_000_000_000_000;

/// A private newtype for converting `ContractAccessError` into an RPC error.
struct RPCContractAccessError(ContractAccessError);
impl From<RPCContractAccessError> for Error {
    fn from(e: RPCContractAccessError) -> Error {
        use t3rn_primitives::ContractAccessError::*;
        match e.0 {
            DoesntExist => Error {
                code: ErrorCode::ServerError(CONTRACT_DOESNT_EXIST),
                message: "The specified contract doesn't exist.".into(),
                data: None,
            },
            IsTombstone => Error {
                code: ErrorCode::ServerError(CONTRACT_IS_A_TOMBSTONE),
                message: "The contract is a tombstone and doesn't have any storage.".into(),
                data: None,
            },
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InterExecRequest<AccountId, Balance> {
    origin: AccountId,
    components: Vec<ComposeRPC<AccountId, Balance>>,
    io: Box<str>,
    gas_limit: number::NumberOrHex,
    input_data: Bytes,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComposeRPC<Account, Balance> {
    name: Box<str>,
    code_txt: Box<str>,
    gateway_id: Account,
    exec_type: Box<str>,
    dest: Account,
    value: Balance,
    bytes: Bytes,
    input_data: Bytes,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CallRequest<AccountId, Balance> {
    origin: AccountId,
    dest: AccountId,
    value: Balance,
    gas_limit: number::NumberOrHex,
    input_data: Bytes,
}

/// An RPC serializable result of contract execution
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum RpcComposableExecResult {
    /// Successful execution
    Success {
        /// The return flags
        flags: u32,
        /// Output data
        data: Bytes,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// Error execution
    Error(()),
}

/// An RPC serializable result of contracts fetch.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum RpcFetchContractsResult {
    /// Successful execution
    Success {
        /// The return flags
        flags: u32,
        /// Output data
        data: Bytes,
        /// How much gas was consumed by the call.
        gas_consumed: u64,
    },
    /// Error execution
    Error(()),
}

impl From<ComposableExecResult> for RpcComposableExecResult {
    fn from(r: ComposableExecResult) -> Self {
        match r {
            ComposableExecResult::Success {
                flags,
                data,
                gas_consumed,
            } => RpcComposableExecResult::Success {
                flags,
                data: data.into(),
                gas_consumed,
            },
            ComposableExecResult::Error => RpcComposableExecResult::Error(()),
        }
    }
}

/// Circuit RPC methods.
#[rpc]
pub trait CircuitApi<BlockHash, BlockNumber, AccountId, Balance> {
    /// Executes all attached or appointed by ID composable contracts on appointed gateways.
    ///
    /// IO flow between components on different chains can be described using Input-Output schedule.
    ///
    /// Circuit queues the request and awaits for an execution agent to volounteer to facilitate the execution
    /// across connected chains via gateways - acts as an escrow account and is accountable
    /// with her stake for proven misbehaviour.
    #[rpc(name = "composable_exec")]
    fn composable_exec(
        &self,
        call_request: InterExecRequest<AccountId, Balance>,
        at: Option<BlockHash>,
    ) -> Result<RpcComposableExecResult>;

    /// Returns the contracts searchable by name or author
    #[rpc(name = "circuit_fetchContracts")]
    fn fetch_contracts(
        &self,
        author: AccountId,
        name: Box<str>,
        at: Option<BlockHash>,
    ) -> Result<RpcFetchContractsResult>;
}

/// An implementation of contract specific RPC methods.
pub struct Circuit<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Circuit<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Circuit {
            client,
            _marker: Default::default(),
        }
    }
}
impl<C, Block, AccountId, Balance>
    CircuitApi<
        <Block as BlockT>::Hash,
        <<Block as BlockT>::Header as HeaderT>::Number,
        AccountId,
        Balance,
    > for Circuit<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: CircuitRuntimeApi<
        Block,
        AccountId,
        Balance,
        <<Block as BlockT>::Header as HeaderT>::Number,
    >,
    AccountId: Codec,
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

    fn fetch_contracts(
        &self,
        _author: AccountId,
        _name: Box<str>,
        _at: Option<<Block as BlockT>::Hash>,
    ) -> Result<RpcFetchContractsResult> {
        Ok(RpcFetchContractsResult::Error(()))
    }
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime trapped".into(),
        data: Some(format!("{:?}", err).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::primitive::str;
    use sp_core::U256;

    #[test]
    fn composable_execution_request_should_serialize_deserialize_properly() {
        type Req = InterExecRequest<String, u128>;
        let req: Req = serde_json::from_str(
            r#"
		{
			"origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
			"components": [],
			"io": "component1, component2 | component3;",
			"gasLimit": 1000000000000,
			"inputData": "0x8c97db39"
		}
		"#,
        )
        .unwrap();
        assert_eq!(req.gas_limit.into_u256(), U256::from(0xe8d4a51000u64));
        // Deserialize io schedule from string to vec<u8>
        let io_vec: Vec<u8> = req.io.into_boxed_bytes().to_vec();
        assert_eq!(
            io_vec,
            vec![
                99, 111, 109, 112, 111, 110, 101, 110, 116, 49, 44, 32, 99, 111, 109, 112, 111,
                110, 101, 110, 116, 50, 32, 124, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116,
                51, 59
            ]
        );
        // Serialize io schedule from Vec<u8> to string again with core::str
        let io_vec_back_to_str: &str = core::str::from_utf8(io_vec.as_slice()).unwrap();
        assert_eq!(io_vec_back_to_str, "component1, component2 | component3;");
    }

    #[test]
    fn compose_of_request_should_serialize_deserialize_properly() {
        type Req = InterExecRequest<String, u128>;
        let req: Req = serde_json::from_str(
            r#"
		{
			"origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
			"components": [{
                "name": "component1",
                "codeTxt": "let a = \"hello\"",
                "gatewayId": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
                "execType": "exec-volatile",
                "dest": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
                "value": 0,
                "bytes": "0x8c97db398c97db398c97db398c97db39",
                "inputData": "0x00"
			}],
			"io": "component1, component2 | component3;",
			"gasLimit": 1000000000000,
			"inputData": "0x8c97db39"
		}
		"#,
        )
        .unwrap();
        // Deserializes string fields correctly
        let name_str: &str = &req.components[0].name;
        assert_eq!(name_str, "component1");
        let code_str: &str = &req.components[0].code_txt;
        assert_eq!(code_str, "let a = \"hello\"");
        let exec_type_str: &str = &req.components[0].exec_type;
        assert_eq!(exec_type_str, "exec-volatile");
    }
}
