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
use codec::{Compact, Encode};

use crate::mock_rpc_setup::{TestSetup, REMOTE_CLIENT};

use sp_core::Bytes;
use sp_io::TestExternalities;
use sp_keystore::KeystoreExt;

use t3rn_primitives::*;

use pallet_circuit_execution_delivery::message_test_utils::{
    create_gateway_protocol_from_client, create_test_stuffed_gateway_protocol,
};

use jsonrpc_runtime_client::{create_rpc_client, ConnectionParams};
use jsonrpsee_types::{traits::Client, JsonValue};
use sp_keyring::Sr25519Keyring;

#[test]
#[ignore] // ToDo: Won't run at CI for additional localhost target
fn successfully_dispatches_signed_transfer_outbound_message_with_protocol_from_circuit_to_remote_target(
) {
    async_std::task::block_on(async move {
        // Re-use mocked test setup since signing must happen in test externalities env
        let p = TestSetup::default();
        let opt_client = REMOTE_CLIENT.lock().unwrap();
        let client = opt_client.as_ref().unwrap();

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(p.keystore));

        let signer = Sr25519Keyring::Alice;
        let test_protocol =
            create_gateway_protocol_from_client(client, signer.public().into()).await;

        let signed_ext = ext.execute_with(|| {
            let transfer_outbound_message = test_protocol
                .transfer(
                    GenericAddress::Id(Sr25519Keyring::Bob.to_account_id()),
                    Compact::from(100000000000000u128),
                    GatewayType::ProgrammableExternal,
                )
                .unwrap();

            transfer_outbound_message
                .to_jsonrpc_signed()
                .unwrap()
                .signed_extrinsic
        });

        let ext_hash = client
            .submit_signed_extrinsic(signer.to_account_id(), |_nonce| signed_ext.into())
            .await;

        assert!(ext_hash.is_ok(), "result should be true");

        println!("client.author_submit_extrinsic result = {:?}", ext_hash);
    });
}

#[test]
#[ignore] // ToDo: Fails since only one signed RPC can be submitted at the time until nonce in mock are supported
fn successfully_dispatches_signed_call_outbound_message_with_protocol_from_circuit_to_remote_target(
) {
    let localhost_params: ConnectionParams = ConnectionParams {
        host: "localhost".to_string(),
        port: 9944,
        secure: false,
    };
    let _devnet_params: ConnectionParams = ConnectionParams {
        host: "dev.net.t3rn.io".to_string(),
        port: 443,
        secure: true,
    };
    // Re-use mocked test setup since signing must happen in test externalities env
    let p = TestSetup::default();
    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(p.keystore));

    let test_protocol = create_test_stuffed_gateway_protocol(Sr25519Keyring::Alice.public().into());

    let (
        module_name,
        fn_name,
        data,
        escrow_account,
        requester,
        to,
        value,
        gas,
        gateway_type,
        return_value,
    ) = (
        b"state".to_vec(),
        b"getStorage".to_vec(),
        vec![],                                                               // input
        GenericAddress::Id(Sr25519Keyring::Alice.to_account_id()).encode(),   // escrow
        GenericAddress::Id(Sr25519Keyring::Bob.to_account_id()).encode(),     // requester
        GenericAddress::Id(Sr25519Keyring::Charlie.to_account_id()).encode(), // to/dest
        Compact::from(100000000000000u128).encode(),                          // value
        Compact::from(200000000000000u128).encode(),                          // gas
        GatewayType::ProgrammableExternal,
        None, // optional return value which already might have been known
    );
    ext.execute_with(|| {
        let call_outbound_message = test_protocol
            .call(
                module_name,
                fn_name,
                data,
                escrow_account,
                requester,
                to,
                value,
                gas,
                gateway_type,
                return_value,
            )
            .unwrap();

        // FixMe: Create the signed params, where params is the encoded UncheckedExtrisicV4
        let request_message: RpcPayloadSigned = call_outbound_message.to_jsonrpc_signed().unwrap();

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
                    "state_call",
                    vec![
                        JsonValue::String("BabeApi_current_epoch".to_string()),
                        JsonValue::String(param_hex_str),
                    ]
                    .into(),
                )
                .await;

            println!("client.author_submit_extrinsic result = {:?}", result_req);
            // ToDo: Plug-in BabeApi RPC API to dev runtime
            assert_eq!(true, false);
        });
    });
}
