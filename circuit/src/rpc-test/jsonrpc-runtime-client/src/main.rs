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

use jsonrpc_runtime_client::{create_rpc_client, get_first_header, ConnectionParams};

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
