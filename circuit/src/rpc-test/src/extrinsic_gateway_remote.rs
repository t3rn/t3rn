// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::mock_rpc_setup::TestSetup;

use sp_keystore::KeystoreExt;
use sp_core::Bytes;
use sp_io::TestExternalities;

use t3rn_primitives::*;

use pallet_circuit_execution_delivery::message_assembly::test_utils::create_test_stuffed_gateway_protocol;

use jsonrpc_runtime_client::{create_rpc_client, ConnectionParams};
use jsonrpsee_types::{traits::Client, JsonValue};
use sp_keyring::Sr25519Keyring;

// FixMe: Fails when submmited against DEV node, since for now multisig is unsupported
#[test]
fn successfully_dispatches_signed_transfer_outbound_message_with_protocol_from_circuit_to_remote_target(
) {

    let localhost_params: ConnectionParams = ConnectionParams {
        host: "localhost".to_string(),
        port: 9944,
        secure: false,
    };
    let devnet_params: ConnectionParams = ConnectionParams {
        host: "dev.net.t3rn.io".to_string(),
        port: 443,
        secure: true,
    };
    // Re-use mocked test setup since signing must happen in test externalities env
    let p = TestSetup::default();
    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(p.keystore));

    let test_protocol = create_test_stuffed_gateway_protocol(Sr25519Keyring::Alice.public().into());

    ext.execute_with(|| {
        let transfer_outbound_message = test_protocol
            .transfer(
                Default::default(),
                Default::default(),
                GatewayType::ProgrammableExternal,
            )
            .unwrap();

        // FixMe: Create the signed params, where params is the encoded UncheckedExtrisicV4
        let request_message: RpcPayloadSigned =
            transfer_outbound_message.to_jsonrpc_signed().unwrap();

        // Start a remote WS request block
        async_std::task::block_on(async move {
            let client = create_rpc_client(&localhost_params).await.unwrap();

            let param_hex_str = format!(
                r#"0x{}"#,
                hex::encode(request_message.signed_extrinsic.clone())
            );

            // Send requests either via client.request(...) or already wrapped into output types submit_unsigned_extrinsic
            let result_req: Result<Bytes, jsonrpsee_types::Error> = client
                .client
                .request(
                    "author_submitExtrinsic",
                    vec![JsonValue::String(param_hex_str)].into(),
                )
                .await;

            // Full list of custom requests implemented into SubstrateClient
            // vendor/bridges/relays/client-substrate/src/rpc.rs
            let result_wrap = client
                .submit_unsigned_extrinsic(Bytes(request_message.signed_extrinsic))
                .await;

            println!("client.author_submit_extrinsic result = {:?}", result_req);
            println!(
                "client.submit_unsigned_extrinsic result = {:?}",
                result_wrap
            );
            // ToDo: Fix error `Err(RestartNeeded("Custom error: Unparsable response"))'
            assert_eq!(true, false);
        });
    });
}
