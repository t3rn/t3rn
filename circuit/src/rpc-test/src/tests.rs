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


use std::{sync::Arc};

use sp_keystore::testing::KeyStore;

use substrate_test_runtime_client::{
	self, AccountKeyring, runtime::{Extrinsic, Transfer, Block},
	DefaultTestClientBuilderExt, TestClientBuilderExt, Backend, Client,
};
use sc_transaction_pool::{BasicPool, FullChainApi};

use sc_rpc_api::{DenyUnsafe, system::SystemInfo};
use sc_rpc::author::{Author, AuthorApi};
use sc_rpc::system::{System, SystemApi};
use sc_rpc::state::{new_full, State, StateApi};

use sp_utils::mpsc::tracing_unbounded;

use jsonrpc_pubsub::{manager::SubscriptionManager};

fn _uxt(sender: AccountKeyring, nonce: u64) -> Extrinsic {
	let tx = Transfer {
		amount: Default::default(),
		nonce,
		from: sender.into(),
		to: Default::default(),
	};
	tx.into_signed_tx()
}

type FullTransactionPool = BasicPool<
	FullChainApi<Client<Backend>, Block>,
	Block,
>;

struct TestSetup {
	pub client: Arc<Client<Backend>>,
	pub keystore: Arc<KeyStore>,
	pub pool: Arc<FullTransactionPool>,
}

impl Default for TestSetup {
	fn default() -> Self {
		let keystore = Arc::new(KeyStore::new());
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
			SubscriptionManager::new(Arc::new(crate::testing::TaskExecutor)),
			self.keystore.clone(),
			DenyUnsafe::No,
		)
	}

	fn state(&self) -> State<Block, Client<Backend>> {
		let (state, _) = new_full(
			self.client.clone(),
			SubscriptionManager::new(Arc::new(crate::testing::TaskExecutor)),
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
