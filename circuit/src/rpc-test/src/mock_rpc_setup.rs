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

use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};

use sc_rpc::author::Author;
use sc_rpc::state::{new_full, State};
use sc_rpc::system::System;
use sc_rpc_api::{system::SystemInfo, DenyUnsafe};

use sp_utils::mpsc::tracing_unbounded;

use jsonrpc_pubsub::manager::SubscriptionManager;

use futures::{compat::Future01CompatExt, executor, FutureExt};
use jsonrpc_core::futures::future as future01;

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

pub type Boxed01Future01 = Box<dyn future01::Future<Item = (), Error = ()> + Send + 'static>;

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
pub fn create_temp_keystore<P: AppPair>(
    authority: Sr25519Keyring,
) -> (SyncCryptoStorePtr, tempfile::TempDir) {
    let keystore_path = tempfile::tempdir().expect("Creates keystore path");
    let keystore =
        Arc::new(LocalKeystore::open(keystore_path.path(), None).expect("Creates keystore"));
    SyncCryptoStore::sr25519_generate_new(&*keystore, KEY_TYPE, Some(&authority.to_seed()))
        .expect("Creates authority key");

    (keystore, keystore_path)
}

pub fn uxt(sender: AccountKeyring, nonce: u64) -> Extrinsic {
    let tx = Transfer {
        amount: Default::default(),
        nonce,
        from: sender.into(),
        to: Default::default(),
    };
    tx.into_signed_tx()
}

pub type FullTransactionPool = BasicPool<FullChainApi<Client<Backend>, Block>, Block>;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

pub struct TestSetup {
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
    pub fn author(&self) -> Author<FullTransactionPool, Client<Backend>> {
        Author::new(
            self.client.clone(),
            self.pool.clone(),
            SubscriptionManager::new(Arc::new(TaskExecutor)),
            self.keystore.clone(),
            DenyUnsafe::No,
        )
    }

    pub fn state(&self) -> State<Block, Client<Backend>> {
        let (state, _) = new_full(
            self.client.clone(),
            SubscriptionManager::new(Arc::new(TaskExecutor)),
            DenyUnsafe::No,
            None,
        );
        state
    }

    pub fn system(&self) -> System<Block> {
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
