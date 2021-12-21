//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::codec::Codec;
use sp_std::vec::Vec;
use t3rn_primitives::{ComposableExecResult, Compose};

sp_api::decl_runtime_apis! {
    /// The API to interact with execution delivery
    pub trait ExecutionDeliveryRuntimeApi<AccountId, Balance, BlockNumber> where
        AccountId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
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
        ) -> ComposableExecResult;
    }
}
