// This file is part of Substrate.

// Copyright (C) 2019-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

//! Node-specific RPC methods for interaction with contracts.

use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use pallet_contracts_primitives::RentProjection;
use serde::{Deserialize, Serialize};
use sp_rpc::number;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{Bytes, H256};
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT},
};
use std::convert::TryInto;

pub use self::gen_client::Client as ContractsClient;
pub use pallet_contracts_rpc_runtime_api::{
	self as runtime_api, ContractExecResult, ContractsApi as ContractsRuntimeApi,
};

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
pub enum RpcContractExecResult {
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


/// Contracts RPC methods.
#[rpc]
pub trait EscrowGatewayApi<BlockHash, BlockNumber, AccountId, Balance> {
	/// Executes a call to a contract.
	///
	/// This call is performed locally without submitting any transactions. Thus executing this
	/// won't change any state. Nonetheless, the calling state-changing contracts is still possible.
	///
	/// This method is useful for calling getter-like methods on contracts.
	#[rpc(name = "gateway_escrowCall")]
	fn escrow_call(
		&self,
		call_request: CallRequest<AccountId, Balance>,
		at: Option<BlockHash>,
	) -> Result<RpcContractExecResult>;

	/// Returns the value under a specified storage `key` in a contract given by `address` param,
	/// or `None` if it is not set.
	#[rpc(name = "gateway_getStorage")]
	fn get_storage(
		&self,
		address: AccountId,
		key: H256,
		at: Option<BlockHash>,
	) -> Result<Option<Bytes>>;

	/// Returns the projected time a given contract will be able to sustain paying its rent.
	///
	/// The returned projection is relevant for the given block, i.e. it is as if the contract was
	/// accessed at the beginning of that block.
	///
	/// Returns `None` if the contract is exempted from rent.
	#[rpc(name = "gateway_rentProjection")]
	fn rent_projection(
		&self,
		address: AccountId,
		at: Option<BlockHash>,
	) -> Result<Option<BlockNumber>>;
}


/// An implementation of contract specific RPC methods.
pub struct EscrowGateway<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> EscrowGateway<C, B> {
	/// Create new `Contracts` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		EscrowGateway {
			client,
			_marker: Default::default(),
		}
	}
}
impl<C, Block, AccountId, Balance>
EscrowGatewayApi<
	<Block as BlockT>::Hash,
	<<Block as BlockT>::Header as HeaderT>::Number,
	AccountId,
	Balance,
> for EscrowGateway<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
		C::Api: EscrowGatewayApi<
			<Block as BlockT>::Hash,
			AccountId,
			Balance,
			<<Block as BlockT>::Header as HeaderT>::Number,
		>,
		AccountId: Codec,
		Balance: Codec,
{
	fn escrow_call(
		&self,
		call_request: CallRequest<AccountId, Balance>,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<RpcContractExecResult> {
		println!("api -- escrow_call");
		unimplemented!();
	}

	fn get_storage(
		&self,
		address: AccountId,
		key: H256,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		unimplemented!();

	}

	fn rent_projection(
		&self,
		address: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<<<Block as BlockT>::Header as HeaderT>::Number>> {
		unimplemented!();
	}
}
