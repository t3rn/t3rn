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
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{typed::Subscriber, SubscriptionId};
use sc_rpc_api::author::error::Error;
use serde::{Deserialize, Serialize};
use sp_core::Bytes;
use sp_transaction_pool::TransactionStatus;

/// Gateway RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Gateway RPC future Result type.
pub type FutureResult<T> = Box<dyn jsonrpc_core::futures::Future<Item = T, Error = Error> + Send>;

/// RPC Extrinsic or hash
///
/// Allows to refer to extrinsic either by its raw representation or its hash.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtrinsicOrHash<Hash> {
    /// The hash of the extrinsic.
    Hash(Hash),
    /// Raw extrinsic bytes.
    Extrinsic(Bytes),
}

/// Substrate Gatewaying RPC API
#[rpc]
pub trait GatewayApi<Hash, BlockHash> {
    /// RPC metadata
    type Metadata;

    /// Submit hex-encoded extrinsic for inclusion in block.
    #[rpc(name = "gateway_submitExtrinsic")]
    fn submit_extrinsic(&self, extrinsic: Bytes) -> FutureResult<Hash>;

    /// Returns all pending extrinsics, potentially grouped by sender.
    #[rpc(name = "gateway_pendingExtrinsics")]
    fn pending_extrinsics(&self) -> Result<Vec<Bytes>>;

    /// Remove given extrinsic from the pool and temporarily ban it to prevent reimporting.
    #[rpc(name = "gateway_removeExtrinsic")]
    fn remove_extrinsic(&self, bytes_or_hash: Vec<ExtrinsicOrHash<Hash>>) -> Result<Vec<Hash>>;

    /// Submit an extrinsic to watch.
    ///
    /// See [`TransactionStatus`](sp_transaction_pool::TransactionStatus) for details on transaction
    /// life cycle.
    #[pubsub(
        subscription = "gateway_extrinsicUpdate",
        subscribe,
        name = "gateway_submitAndWatchExtrinsic"
    )]
    fn watch_extrinsic(
        &self,
        metadata: Self::Metadata,
        subscriber: Subscriber<TransactionStatus<Hash, BlockHash>>,
        bytes: Bytes,
    );

    /// Unsubscribe from extrinsic watching.
    #[pubsub(
        subscription = "gateway_extrinsicUpdate",
        unsubscribe,
        name = "gateway_unwatchExtrinsic"
    )]
    fn unwatch_extrinsic(
        &self,
        metadata: Option<Self::Metadata>,
        id: SubscriptionId,
    ) -> Result<bool>;
}
