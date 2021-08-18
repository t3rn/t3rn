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

//! Runtime API definition required by Circuit RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
    codec::Codec,
    traits::{MaybeDisplay, MaybeFromStr},
    DispatchError,
};
use sp_std::vec::Vec;
use t3rn_primitives::{ComposableExecResult, Compose};

sp_api::decl_runtime_apis! {
    /// The API to interact with contracts without using executive.
    pub trait CircuitApi<AccountId, Balance, BlockNumber, Hash> where
        AccountId: Codec + MaybeDisplay + MaybeFromStr,
        Balance: Codec + MaybeDisplay + MaybeFromStr,
        BlockNumber: Codec + MaybeDisplay + MaybeFromStr,
        Hash: Codec + MaybeDisplay + MaybeFromStr + sp_std::hash::Hash
    {
        /// Perform a composable execution from a specified account to a appointed gateways.
        ///
        /// See the contracts' `call` dispatchable function for more details.
        fn composable_exec(
            origin: AccountId,
            components: Vec<Compose<AccountId, Balance>>,
            io: Vec<u8>,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> Result<ComposableExecResult, DispatchError>;
    }
}
