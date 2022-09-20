//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use circuit_standalone_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index};
use pallet_3vm_contracts_rpc::{Contracts, ContractsApi};
use pallet_portal_rpc::{Portal, PortalApi};
use pallet_3vm_evm_rpc::{Evm, EvmApi};
use pallet_xdns_rpc::{Xdns, XdnsApi};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api:
        pallet_3vm_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
    C::Api: pallet_xdns_rpc::XdnsRuntimeApi<Block, AccountId>,
    C::Api: pallet_3vm_evm_rpc::EvmRuntimeRPCApi<Block, AccountId, Balance>,
    // C::Api: pallet_portal_rpc::PortalRuntimeApi<Block, AccountId>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client.clone(),
        pool,
        deny_unsafe,
    )));
    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        client.clone(),
    )));
    io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));
    io.extend_with(XdnsApi::to_delegate(Xdns::new(client.clone())));
    io.extend_with(EvmApi::to_delegate(Evm::new(client)));
    // io.extend_with(PortalApi::to_delegate(Portal::new(client.clone())));

    io
}
