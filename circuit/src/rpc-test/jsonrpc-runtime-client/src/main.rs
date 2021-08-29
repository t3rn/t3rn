// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
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

#![warn(missing_docs)]

//! Example substrate RPC client code.
//!
//! This module shows how you can write a Rust RPC client that connects to a running
//! substrate node and use statically typed RPC wrappers.

use relay_substrate_client::{Client as SubstrateClient, ConnectionParams};

/// Implement Chain with Polkadot-like types for relay-client
pub mod polkadot_like_chain;
pub use polkadot_like_chain::PolkadotLike;

/// Useful Substrate network RPC queries
pub mod useful_queries;
pub use useful_queries::get_first_header;

fn main() {
    let sub_params = ConnectionParams {
        host: "localhost".into(),
        port: 9944,
        secure: false,
    };

    // use jsonrspee-websocket behind relay_substrate_client
    async_std::task::block_on(async move {
        let client = create_rpc_client(&sub_params).await.unwrap();
        let first_header = get_first_header(&client).await.unwrap();
        println!("first header {:?}", first_header);
    });
}

/// Run single transaction proof relay and stop.
pub async fn create_rpc_client(
    sub_params: &ConnectionParams,
) -> Result<SubstrateClient<PolkadotLike>, String> {
    let sub_client = SubstrateClient::<PolkadotLike>::try_connect(sub_params.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(sub_client)
}
