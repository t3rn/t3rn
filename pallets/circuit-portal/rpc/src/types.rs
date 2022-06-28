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

use serde::{Deserialize, Serialize};
use sp_core::Bytes;
use sp_rpc::number;
use t3rn_primitives::{ChainId};

pub const RUNTIME_ERROR: i64 = 1;

/// A rough estimate of how much gas a decent hardware consumes per second,
/// using native execution.
/// This value is used to set the upper bound for maximal contract calls to
/// prevent blocking the RPC for too long.
///
/// As 1 gas is equal to 1 weight we base this on the conducted benchmarks which
/// determined runtime weights:
/// https://github.com/paritytech/substrate/pull/5446
pub const GAS_PER_SECOND: u64 = 1_000_000_000_000;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InterExecRequest<AccountId, Balance> {
    pub origin: AccountId,
    pub components: Vec<ComposeRPC<AccountId, Balance>>,
    pub io: Box<str>,
    pub gas_limit: number::NumberOrHex,
    pub input_data: Bytes,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComposeRPC<Account, Balance> {
    pub name: Box<str>,
    pub code_txt: Box<str>,
    pub gateway_id: ChainId,
    pub exec_type: Box<str>,
    pub dest: Account,
    pub value: Balance,
    pub bytes: Bytes,
    pub input_data: Bytes,
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CallRequest<AccountId, Balance> {
    pub origin: AccountId,
    pub dest: AccountId,
    pub value: Balance,
    pub gas_limit: number::NumberOrHex,
    pub input_data: Bytes,
}

/// An RPC serializable result of contract execution
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum RpcReadLatestGatewayHeight {
    /// Successful execution
    Success {
        /// Output data
        encoded_height: Bytes,
    },
    /// Error execution
    Error(()),
}
