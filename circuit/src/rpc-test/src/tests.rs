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

use std::sync::Arc;

use sp_core::crypto::KeyTypeId;

use sp_application_crypto::AppPair;

use sc_transaction_pool::{BasicPool, FullChainApi};
use substrate_test_runtime_client::{
    self,
    runtime::{Block, Extrinsic, Transfer},
    AccountKeyring, Backend, Client, DefaultTestClientBuilderExt, TestClientBuilderExt,
};

use sp_keystore::{KeystoreExt, SyncCryptoStore, SyncCryptoStorePtr};

use sp_io::TestExternalities;

use hex_literal::hex;
use sc_rpc::author::{Author, AuthorApi};
use sc_rpc::state::{new_full, State, StateApi};
use sc_rpc::system::{System, SystemApi};
use sc_rpc_api::{system::SystemInfo, DenyUnsafe};

use sp_utils::mpsc::tracing_unbounded;

use jsonrpc_pubsub::manager::SubscriptionManager;
use t3rn_primitives::*;

use pallet_circuit_execution_delivery::message_assembly::test_utils::create_test_stuffed_gateway_protocol;

use futures::{compat::Future01CompatExt, executor, FutureExt};
use jsonrpc_core::futures::future as future01;

use codec::Encode;
use sc_keystore::LocalKeystore;
use sp_keyring::Sr25519Keyring;

// Executor shared by all tests.
//
// This shared executor is used to prevent `Too many open files` errors
// on systems with a lot of cores.
lazy_static::lazy_static! {
    static ref EXECUTOR: executor::ThreadPool = executor::ThreadPool::new()
        .expect("Failed to create thread pool executor for tests");
}

type Boxed01Future01 = Box<dyn future01::Future<Item = (), Error = ()> + Send + 'static>;

/// Executor for use in testing
pub struct TaskExecutor;
impl future01::Executor<Boxed01Future01> for TaskExecutor {
    fn execute(
        &self,
        future: Boxed01Future01,
    ) -> std::result::Result<(), future01::ExecuteError<Boxed01Future01>> {
        EXECUTOR.spawn_ok(future.compat().map(drop));
        Ok(())
    }
}

/// creates keystore backed by a temp file
fn create_temp_keystore<P: AppPair>(
    authority: Sr25519Keyring,
) -> (SyncCryptoStorePtr, tempfile::TempDir) {
    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore =
        Arc::new(LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore"));
    SyncCryptoStore::sr25519_generate_new(&*keystore, KEY_TYPE, Some(&authority.to_seed()))
        .expect("Creates authority key");

    (keystore, keystore_path)
}

fn uxt(sender: AccountKeyring, nonce: u64) -> Extrinsic {
    let tx = Transfer {
        amount: Default::default(),
        nonce,
        from: sender.into(),
        to: Default::default(),
    };
    tx.into_signed_tx()
}

type FullTransactionPool = BasicPool<FullChainApi<Client<Backend>, Block>, Block>;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

struct TestSetup {
    pub client: Arc<Client<Backend>>,
    pub keystore: SyncCryptoStorePtr,
    pub pool: Arc<FullTransactionPool>,
}

impl Default for TestSetup {
    fn default() -> Self {
        let keystore = create_temp_keystore::<
            pallet_circuit_execution_delivery::message_assembly::signer::app::Pair,
        >(Sr25519Keyring::Alice)
        .0;
        let client_builder = substrate_test_runtime_client::TestClientBuilder::new();
        let client = Arc::new(client_builder.set_keystore(keystore.clone()).build());

        let spawner = sp_core::testing::TaskExecutor::new();
        let pool = BasicPool::new_full(
            Default::default(),
            true.into(),
            None,
            spawner,
            client.clone(),
        );

        TestSetup {
            client,
            keystore,
            pool,
        }
    }
}

impl TestSetup {
    fn author(&self) -> Author<FullTransactionPool, Client<Backend>> {
        Author::new(
            self.client.clone(),
            self.pool.clone(),
            SubscriptionManager::new(Arc::new(TaskExecutor)),
            self.keystore.clone(),
            DenyUnsafe::No,
        )
    }

    fn state(&self) -> State<Block, Client<Backend>> {
        let (state, _) = new_full(
            self.client.clone(),
            SubscriptionManager::new(Arc::new(TaskExecutor)),
            DenyUnsafe::No,
            None,
        );
        state
    }

    fn system(&self) -> System<Block> {
        let (tx, _rx) = tracing_unbounded("rpc_circuit_tests");
        System::new(
            SystemInfo {
                impl_name: "testclient".into(),
                impl_version: "0.1.0".into(),
                chain_name: "testchain".into(),
                properties: Default::default(),
                chain_type: Default::default(),
            },
            tx,
            DenyUnsafe::No,
        )
    }
}

#[test]
fn rpc_prints_system_version() {
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    let request = r#"{"jsonrpc":"2.0","method":"system_version","params":[],"id":1}"#;

    let response = r#"{"jsonrpc":"2.0","result":"0.1.0","id":1}"#;

    let meta = sc_rpc::Metadata::default();
    assert_eq!(io.handle_request_sync(request, meta), Some(response.into()));
}

#[test]
fn successfully_dispatches_unsigned_get_storage_outbound_message_from_circuit_to_external_gateway()
{
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    const NON_EMPTY_STORAGE_KEY: &str = "0x0befda6e1ca4ef40219d588a727f1271";

    let key: Vec<u8> = hex!("0befda6e1ca4ef40219d588a727f1271").to_vec();

    let expected_storage = GatewayExpectedOutput::Storage {
        key: vec![key.clone()],
        value: vec![None],
    };

    let arguments = vec![key];

    let get_storage_outbound_message: CircuitOutboundMessage = CircuitOutboundMessage {
        name: b"state_getStorage".to_vec(),
        module_name: b"state".to_vec(),
        method_name: b"getStorage".to_vec(),
        arguments,
        expected_output: vec![expected_storage],
        extra_payload: None,
        sender: None,
        target: None,
    };

    let request_message: RpcPayloadUnsigned = get_storage_outbound_message
        .to_jsonrpc_unsigned()
        .expect("must not fail");

    let request = format!(
        r#"{{"jsonrpc":"2.0","method":"{}","params":["0x{}"],"id":1}}"#,
        request_message.method_name,
        hex::encode(request_message.params.encode())
    );

    let response = r#"{"jsonrpc":"2.0","result":"null","id":1}"#;

    let meta = sc_rpc::Metadata::default();
    assert_eq!(
        io.handle_request_sync(&request, meta),
        Some(response.into())
    );
}

#[test]
fn successfully_dispatches_signed_transfer_outbound_message_from_circuit_to_external_gateway() {
    let p = TestSetup::default();

    let mut io = jsonrpc_core::MetaIoHandler::<sc_rpc::Metadata>::default();

    io.extend_with(AuthorApi::to_delegate(p.author()));
    io.extend_with(SystemApi::to_delegate(p.system()));
    io.extend_with(StateApi::to_delegate(p.state()));

    let test_protocol = create_test_stuffed_gateway_protocol(Sr25519Keyring::Alice.public().into());

    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(p.keystore));
    ext.execute_with(|| {
        let transfer_message = test_protocol
            .transfer(
                Sr25519Keyring::Alice.to_raw_public_vec(),
                Default::default(),
                GatewayType::ProgrammableExternal,
            )
            .expect("shouldn't fail");

        let transfer = uxt(Sr25519Keyring::Alice, 0);
        let signed = transfer_message
            .to_jsonrpc_signed()
            .expect("should have a signed payload");

        use sp_core::Encode;
        println!("0x{}", hex::encode(signed.signed_extrinsic.clone()));
        println!("0x{}", hex::encode(transfer.encode()));

        let request = format!(
            r#"{{"jsonrpc":"2.0","method":"author_submitExtrinsic","params":["0x{}"],"id":1}}"#,
            hex::encode(signed.signed_extrinsic)
        );

        println!("{}", request);

        let response = r#"{"jsonrpc":"2.0","result":null,"id":1}"#;

        let meta = sc_rpc::Metadata::default();
        assert_eq!(
            io.handle_request_sync(&request, meta),
            Some(response.into())
        );
    });
}
