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
use crate::mock_rpc_setup::{TestSetup, REMOTE_CLIENT};
use codec::{Compact, Encode};

use sp_core::crypto::{AccountId32, UncheckedFrom};
use sp_core::storage::StorageKey;
use sp_core::Bytes;
use sp_io::TestExternalities;
use sp_keystore::KeystoreExt;

use circuit_runtime::{Runtime};
use circuit_test_utils::create_gateway_protocol_from_client;
use sp_keyring::Sr25519Keyring;
use volatile_vm::wasm::PrefabWasmModule;

use jsonrpc_runtime_client::polkadot_like_chain::PolkadotLike;
use jsonrpsee_types::{traits::Client, JsonValue};

use codec::Decode;
use relay_substrate_client::Client as RemoteClient;
use std::{thread, time};
use t3rn_primitives::*;

/// Determine the address of a contract : copied from pallet-contracts
/// Formula: `hash(deploying_address ++ code_hash ++ salt)`
pub fn contract_address(
    deploying_address: Sr25519Keyring,
    code_hash: <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::Output,
    salt: &[u8],
) -> AccountId32 {
    let buf: Vec<_> = deploying_address
        .to_raw_public_vec()
        .iter()
        .chain(code_hash.as_ref())
        .chain(salt)
        .cloned()
        .collect();

    let hash = <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::hash(&buf);
    let account_id: AccountId32 = UncheckedFrom::unchecked_from(hash);
    account_id
}

pub fn collect_args(args: Vec<Vec<u8>>) -> Vec<u8> {
    args.iter().fold(vec![], |mut a, b| {
        a.extend(b);
        a
    })
}

/**
Compiles wat file into wasm and returns Vec<u8> and code_hash
*/
fn compile_module(
    fixture_name: &str,
) -> wat::Result<(
    Vec<u8>,
    <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::Output,
)> {
    let fixture_path = [
        "../../../gateway/pallets/contracts-gateway/",
        "fixtures/",
        fixture_name,
        ".wat",
    ]
    .concat();
    let wasm_binary = wat::parse_file(fixture_path).unwrap();
    let code_hash =
        <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::hash(&wasm_binary);
    Ok((wasm_binary, code_hash))
}

/**
Reads wasm file and returns Vec<u8> and code_hash
*/
fn get_module(
    fixture_name: &str,
) -> wat::Result<(
    Vec<u8>,
    <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::Output,
)> {
    let fixture_path = [
        "../../../gateway/pallets/contracts-gateway/",
        "fixtures/",
        fixture_name,
        ".wasm",
    ]
    .concat();
    let wasm_binary = std::fs::read(fixture_path).unwrap();
    let code_hash =
        <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::hash(&wasm_binary);
    Ok((wasm_binary, code_hash))
}

pub fn storage_map_key<K: Encode>(
    module_prefix: &str,
    storage_prefix: &str,
    mapkey: &K,
) -> Vec<u8> {
    let mut bytes = sp_core::twox_128(module_prefix.as_bytes()).to_vec();
    bytes.extend(&sp_core::twox_128(storage_prefix.as_bytes())[..]);
    // mapkey is already hashed
    bytes.extend(&mapkey.encode().to_vec());
    bytes
}

pub fn assert_contract_present_on_chain(
    client: &RemoteClient<PolkadotLike>,
    code_hash: <sp_runtime::traits::BlakeTwo256 as sp_runtime::traits::Hash>::Output,
    code_length: usize,
) {
    // lets wait for the block to be finanlized
    println!("Waiting for 10 seconds..");
    thread::sleep(time::Duration::from_secs(10));
    // Keyring only supports test accounts and it will be None since it tries to find it in the map.
    let key = storage_map_key("Contracts", "CodeStorage", &code_hash);
    let storage_key = StorageKey(key);

    async_std::task::block_on(async move {
        let decoded_response = client
            .storage_value::<PrefabWasmModule<Runtime>>(storage_key)
            .await;

        assert!(decoded_response.is_ok());
        let response = decoded_response.unwrap().unwrap();
        assert_eq!(
            (response.original_code_len as usize),
            code_length,
            "Retrived code len : {}, Original code len : {}",
            (response.original_code_len as usize),
            code_length
        );
    });
}

pub fn get_contract_storage(
    client: &RemoteClient<PolkadotLike>,
    contract_address: AccountId32,
    key: &str,
) -> Bytes {
    async_std::task::block_on(async move {
        let result_req: Result<Bytes, jsonrpsee_types::Error> = client
            .client
            .request(
                "contracts_getStorage",
                vec![
                    JsonValue::String(contract_address.to_string()),
                    JsonValue::String(key.to_string()),
                ]
                .into(),
            )
            .await;

        assert!(
            result_req.is_ok(),
            "contracts_getStorage failed : {}",
            result_req.unwrap_err()
        );
        result_req.unwrap()
    })
}
#[test]
fn successfully_deploys_smart_contract() {
    async_std::task::block_on(async move {
        // Arrange

        let (wasm, code_hash) = compile_module("call_flipper_runtime").unwrap();

        let p = TestSetup::default();
        let opt_client = REMOTE_CLIENT.lock().unwrap();
        let client = opt_client.as_ref().unwrap();

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(p.keystore));

        let signer = Sr25519Keyring::Alice;
        let test_protocol =
            create_gateway_protocol_from_client(client, signer.public().into()).await;

        let empty: Vec<u8> = vec![];
        let deploy_arguments = vec![
            Compact::from(3_000_000_000u128).encode(),
            Compact::from(10_000_000_000u64).encode(),
            wasm.encode(),
            empty.encode(),
            empty.encode(),
        ];

        // Act

        let ext_hash = client
            .submit_signed_extrinsic(signer.to_account_id(), |nonce| {
                let signed_ext = ext.execute_with(|| {
                    let payload = test_protocol
                        .produce_signed_payload(
                            "Contracts",
                            "instantiate_with_code",
                            collect_args(deploy_arguments),
                            nonce,
                        )
                        .unwrap();
                    payload.tx_signed
                });
                signed_ext.into()
            })
            .await;

        // Assert
        assert!(
            ext_hash.is_ok(),
            "Contract deploy not successful : {}",
            ext_hash.unwrap_err()
        );
        assert_contract_present_on_chain(client, code_hash, wasm.len());
    });
}

#[test]
fn successfully_deploys_flipper_and_calls_flip() {
    async_std::task::block_on(async move {
        // Arrange

        let (wasm, code_hash) = get_module("flipper").unwrap();

        let p = TestSetup::default();
        let opt_client = REMOTE_CLIENT.lock().unwrap();
        let client = opt_client.as_ref().unwrap();

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(p.keystore));

        let signer = Sr25519Keyring::Alice;
        let test_protocol =
            create_gateway_protocol_from_client(client, signer.public().into()).await;

        // Prepare data field for deploy
        // Please check metadata file for these hex selector values
        let mut constructor_selector = hex::decode("9bae9d5e").unwrap();
        let constructor_argument = false.encode();
        constructor_selector.extend(constructor_argument);

        // Prepare data field for flip call
        let message_selector = hex::decode("633aa551").unwrap();

        let empty: Vec<u8> = vec![];
        let deploy_arguments = vec![
            Compact::from(3_000_000_000u128).encode(),
            Compact::from(1_714_624_000u64).encode(),
            wasm.encode(),
            constructor_selector.encode(),
            empty.encode(),
        ];

        let dest = contract_address(signer, code_hash, &empty);

        let call_arguments = vec![
            GenericAddress::Id(dest.clone()).encode(),
            Compact::from(0u128).encode(),
            Compact::from(446_000_000u64).encode(),
            message_selector.encode(),
        ];

        // Act : Deploy smart contract

        let ext_hash_deploy = client
            .submit_signed_extrinsic(signer.to_account_id(), |nonce| {
                let signed_ext = ext.execute_with(|| {
                    let payload = test_protocol
                        .produce_signed_payload(
                            "Contracts",
                            "instantiate_with_code",
                            collect_args(deploy_arguments),
                            nonce,
                        )
                        .unwrap();
                    payload.tx_signed
                });
                signed_ext.into()
            })
            .await;

        assert!(
            ext_hash_deploy.is_ok(),
            "Contract deploy not successful : {}",
            ext_hash_deploy.unwrap_err()
        );
        assert_contract_present_on_chain(client, code_hash, wasm.len());

        // Get current bool value
        let response = get_contract_storage(
            client,
            dest.clone(),
            r#"0x0000000000000000000000000000000000000000000000000000000000000000"#,
        );
        let current_value: bool = Decode::decode(&mut &response[..]).unwrap();

        // Act : Call flip() function on smart contract
        let ext_hash_call = client
            .submit_signed_extrinsic(signer.to_account_id(), |nonce| {
                let signed_ext = ext.execute_with(|| {
                    let payload = test_protocol
                        .produce_signed_payload(
                            "Contracts",
                            "call",
                            collect_args(call_arguments),
                            nonce,
                        )
                        .unwrap();
                    payload.tx_signed
                });
                signed_ext.into()
            })
            .await;

        assert!(
            ext_hash_call.is_ok(),
            "Contract call not successful : {}",
            ext_hash_call.unwrap_err()
        );
        // Wait for block finalization
        thread::sleep(time::Duration::from_secs(20));

        // Get current bool value
        let response = get_contract_storage(
            client,
            dest.clone(),
            r#"0x0000000000000000000000000000000000000000000000000000000000000000"#,
        );
        let new_value: bool = Decode::decode(&mut &response[..]).unwrap();
        assert_eq!(current_value, !new_value, "Contract value not flipped");
    });
}
