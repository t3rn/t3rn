// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Tests for finality synchronization loop.

#![cfg(test)]

use crate::{
    finality_loop::{
        prune_recent_finality_proofs, read_finality_proofs_from_stream, run,
        select_better_recent_finality_proof, FinalityProofs, FinalitySyncParams, SourceClient,
        TargetClient,
    },
    FinalityProof, FinalitySyncPipeline, SourceHeader,
};

use async_trait::async_trait;
use futures::{FutureExt, Stream, StreamExt};
use parking_lot::Mutex;
use relay_utils::{
    metrics::MetricsParams, relay_loop::Client as RelayClient, MaybeConnectionError,
};
use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};

type IsMandatory = bool;
type TestNumber = u64;

#[derive(Debug, Clone)]
enum TestError {
    NonConnection,
}

impl MaybeConnectionError for TestError {
    fn is_connection_error(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
struct TestFinalitySyncPipeline;

impl FinalitySyncPipeline for TestFinalitySyncPipeline {
    type FinalityProof = TestFinalityProof;
    type Hash = u64;
    type Header = TestSourceHeader;
    type Number = TestNumber;

    const SOURCE_NAME: &'static str = "TestSource";
    const TARGET_NAME: &'static str = "TestTarget";
}

#[derive(Debug, Clone, PartialEq)]
struct TestSourceHeader(IsMandatory, TestNumber);

impl SourceHeader<TestNumber> for TestSourceHeader {
    fn number(&self) -> TestNumber {
        self.1
    }

    fn is_mandatory(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TestFinalityProof(TestNumber);

impl FinalityProof<TestNumber> for TestFinalityProof {
    fn target_header_number(&self) -> TestNumber {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
struct ClientsData {
    source_best_block_number: TestNumber,
    source_headers: HashMap<TestNumber, (TestSourceHeader, Option<TestFinalityProof>)>,
    source_proofs: Vec<TestFinalityProof>,

    target_best_block_number: TestNumber,
    target_headers: Vec<(TestSourceHeader, TestFinalityProof)>,
}

#[derive(Clone)]
struct TestSourceClient {
    on_method_call: Arc<dyn Fn(&mut ClientsData) + Send + Sync>,
    data: Arc<Mutex<ClientsData>>,
}

#[async_trait]
impl RelayClient for TestSourceClient {
    type Error = TestError;

    async fn reconnect(&mut self) -> Result<(), TestError> {
        unreachable!()
    }
}

#[async_trait]
impl SourceClient<TestFinalitySyncPipeline> for TestSourceClient {
    type FinalityProofsStream = Pin<Box<dyn Stream<Item = TestFinalityProof> + 'static + Send>>;

    async fn best_finalized_block_number(&self) -> Result<TestNumber, TestError> {
        let mut data = self.data.lock();
        (self.on_method_call)(&mut *data);
        Ok(data.source_best_block_number)
    }

    async fn header_and_finality_proof(
        &self,
        number: TestNumber,
    ) -> Result<(TestSourceHeader, Option<TestFinalityProof>), TestError> {
        let mut data = self.data.lock();
        (self.on_method_call)(&mut *data);
        data.source_headers
            .get(&number)
            .cloned()
            .ok_or(TestError::NonConnection)
    }

    async fn finality_proofs(&self) -> Result<Self::FinalityProofsStream, TestError> {
        let mut data = self.data.lock();
        (self.on_method_call)(&mut *data);
        Ok(futures::stream::iter(data.source_proofs.clone()).boxed())
    }
}

#[derive(Clone)]
struct TestTargetClient {
    on_method_call: Arc<dyn Fn(&mut ClientsData) + Send + Sync>,
    data: Arc<Mutex<ClientsData>>,
}

#[async_trait]
impl RelayClient for TestTargetClient {
    type Error = TestError;

    async fn reconnect(&mut self) -> Result<(), TestError> {
        unreachable!()
    }
}

#[async_trait]
impl TargetClient<TestFinalitySyncPipeline> for TestTargetClient {
    async fn best_finalized_source_block_number(&self) -> Result<TestNumber, TestError> {
        let mut data = self.data.lock();
        (self.on_method_call)(&mut *data);
        Ok(data.target_best_block_number)
    }

    async fn submit_finality_proof(
        &self,
        header: TestSourceHeader,
        proof: TestFinalityProof,
    ) -> Result<(), TestError> {
        let mut data = self.data.lock();
        (self.on_method_call)(&mut *data);
        data.target_best_block_number = header.number();
        data.target_headers.push((header, proof));
        Ok(())
    }
}

fn run_sync_loop(
    state_function: impl Fn(&mut ClientsData) -> bool + Send + Sync + 'static,
) -> ClientsData {
    let (exit_sender, exit_receiver) = futures::channel::mpsc::unbounded();
    let internal_state_function: Arc<dyn Fn(&mut ClientsData) + Send + Sync> =
        Arc::new(move |data| {
            if state_function(data) {
                exit_sender.unbounded_send(()).unwrap();
            }
        });
    let clients_data = Arc::new(Mutex::new(ClientsData {
        source_best_block_number: 10,
        source_headers: vec![
            (6, (TestSourceHeader(false, 6), None)),
            (7, (TestSourceHeader(false, 7), Some(TestFinalityProof(7)))),
            (8, (TestSourceHeader(true, 8), Some(TestFinalityProof(8)))),
            (9, (TestSourceHeader(false, 9), Some(TestFinalityProof(9)))),
            (10, (TestSourceHeader(false, 10), None)),
        ]
        .into_iter()
        .collect(),
        source_proofs: vec![TestFinalityProof(12), TestFinalityProof(14)],

        target_best_block_number: 5,
        target_headers: vec![],
    }));
    let source_client = TestSourceClient {
        on_method_call: internal_state_function.clone(),
        data: clients_data.clone(),
    };
    let target_client = TestTargetClient {
        on_method_call: internal_state_function,
        data: clients_data.clone(),
    };
    let sync_params = FinalitySyncParams {
        tick: Duration::from_secs(0),
        recent_finality_proofs_limit: 1024,
        stall_timeout: Duration::from_secs(1),
    };

    let _ = async_std::task::block_on(run(
        source_client,
        target_client,
        sync_params,
        MetricsParams::disabled(),
        exit_receiver.into_future().map(|(_, _)| ()),
    ));

    let clients_data = clients_data.lock().clone();
    clients_data
}

#[test]
fn select_better_recent_finality_proof_works() {
    // if there are no unjustified headers, nothing is changed
    assert_eq!(
        select_better_recent_finality_proof::<TestFinalitySyncPipeline>(
            &[(5, TestFinalityProof(5))],
            &mut vec![],
            Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
        ),
        Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
    );

    // if there are no recent finality proofs, nothing is changed
    assert_eq!(
        select_better_recent_finality_proof::<TestFinalitySyncPipeline>(
            &[],
            &mut vec![TestSourceHeader(false, 5)],
            Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
        ),
        Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
    );

    // if there's no intersection between recent finality proofs and unjustified headers, nothing is changed
    let mut unjustified_headers = vec![TestSourceHeader(false, 9), TestSourceHeader(false, 10)];
    assert_eq!(
        select_better_recent_finality_proof::<TestFinalitySyncPipeline>(
            &[(1, TestFinalityProof(1)), (4, TestFinalityProof(4))],
            &mut unjustified_headers,
            Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
        ),
        Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
    );

    // if there's intersection between recent finality proofs and unjustified headers, but there are no
    // proofs in this intersection, nothing is changed
    let mut unjustified_headers = vec![
        TestSourceHeader(false, 8),
        TestSourceHeader(false, 9),
        TestSourceHeader(false, 10),
    ];
    assert_eq!(
        select_better_recent_finality_proof::<TestFinalitySyncPipeline>(
            &[(7, TestFinalityProof(7)), (11, TestFinalityProof(11))],
            &mut unjustified_headers,
            Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
        ),
        Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
    );
    assert_eq!(
        unjustified_headers,
        vec![
            TestSourceHeader(false, 8),
            TestSourceHeader(false, 9),
            TestSourceHeader(false, 10)
        ]
    );

    // if there's intersection between recent finality proofs and unjustified headers and there's
    // a proof in this intersection:
    // - this better (last from intersection) proof is selected;
    // - 'obsolete' unjustified headers are pruned.
    let mut unjustified_headers = vec![
        TestSourceHeader(false, 8),
        TestSourceHeader(false, 9),
        TestSourceHeader(false, 10),
    ];
    assert_eq!(
        select_better_recent_finality_proof::<TestFinalitySyncPipeline>(
            &[(7, TestFinalityProof(7)), (9, TestFinalityProof(9))],
            &mut unjustified_headers,
            Some((TestSourceHeader(false, 2), TestFinalityProof(2))),
        ),
        Some((TestSourceHeader(false, 9), TestFinalityProof(9))),
    );
}

#[test]
fn read_finality_proofs_from_stream_works() {
    // when stream is currently empty, nothing is changed
    let mut recent_finality_proofs = vec![(1, TestFinalityProof(1))];
    let mut stream = futures::stream::pending().into();
    read_finality_proofs_from_stream::<TestFinalitySyncPipeline, _>(
        &mut stream,
        &mut recent_finality_proofs,
    );
    assert_eq!(recent_finality_proofs, vec![(1, TestFinalityProof(1))]);
    assert!(!stream.needs_restart);

    // when stream has entry with target, it is added to the recent proofs container
    let mut stream = futures::stream::iter(vec![TestFinalityProof(4)])
        .chain(futures::stream::pending())
        .into();
    read_finality_proofs_from_stream::<TestFinalitySyncPipeline, _>(
        &mut stream,
        &mut recent_finality_proofs,
    );
    assert_eq!(
        recent_finality_proofs,
        vec![(1, TestFinalityProof(1)), (4, TestFinalityProof(4))]
    );
    assert!(!stream.needs_restart);

    // when stream has ended, we'll need to restart it
    let mut stream = futures::stream::empty().into();
    read_finality_proofs_from_stream::<TestFinalitySyncPipeline, _>(
        &mut stream,
        &mut recent_finality_proofs,
    );
    assert_eq!(
        recent_finality_proofs,
        vec![(1, TestFinalityProof(1)), (4, TestFinalityProof(4))]
    );
    assert!(stream.needs_restart);
}

#[test]
fn prune_recent_finality_proofs_works() {
    let original_recent_finality_proofs: FinalityProofs<TestFinalitySyncPipeline> = vec![
        (10, TestFinalityProof(10)),
        (13, TestFinalityProof(13)),
        (15, TestFinalityProof(15)),
        (17, TestFinalityProof(17)),
        (19, TestFinalityProof(19)),
    ]
    .into_iter()
    .collect();

    // when there's proof for justified header in the vec
    let mut recent_finality_proofs = original_recent_finality_proofs.clone();
    prune_recent_finality_proofs::<TestFinalitySyncPipeline>(10, &mut recent_finality_proofs, 1024);
    assert_eq!(
        &original_recent_finality_proofs[1..],
        recent_finality_proofs,
    );

    // when there are no proof for justified header in the vec
    let mut recent_finality_proofs = original_recent_finality_proofs.clone();
    prune_recent_finality_proofs::<TestFinalitySyncPipeline>(11, &mut recent_finality_proofs, 1024);
    assert_eq!(
        &original_recent_finality_proofs[1..],
        recent_finality_proofs,
    );

    // when there are too many entries after initial prune && they also need to be pruned
    let mut recent_finality_proofs = original_recent_finality_proofs.clone();
    prune_recent_finality_proofs::<TestFinalitySyncPipeline>(10, &mut recent_finality_proofs, 2);
    assert_eq!(
        &original_recent_finality_proofs[3..],
        recent_finality_proofs,
    );

    // when last entry is pruned
    let mut recent_finality_proofs = original_recent_finality_proofs.clone();
    prune_recent_finality_proofs::<TestFinalitySyncPipeline>(19, &mut recent_finality_proofs, 2);
    assert_eq!(
        &original_recent_finality_proofs[5..],
        recent_finality_proofs,
    );

    // when post-last entry is pruned
    let mut recent_finality_proofs = original_recent_finality_proofs.clone();
    prune_recent_finality_proofs::<TestFinalitySyncPipeline>(20, &mut recent_finality_proofs, 2);
    assert_eq!(
        &original_recent_finality_proofs[5..],
        recent_finality_proofs,
    );
}
