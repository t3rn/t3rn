//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use circuit_standalone_runtime::{
    opaque::Block, AccountId, Balance, BlockNumber, Hash, Index as Nonce,
};
use pallet_3vm_contracts_rpc::{Contracts, ContractsApiServer};
use pallet_3vm_evm_rpc::{Evm, EvmApiServer};
use pallet_portal_rpc::{Portal, PortalApiServer};
use pallet_xdns_rpc::{Xdns, XdnsApiServer};
use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api:
        pallet_3vm_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
    C::Api: pallet_xdns_rpc::XdnsRuntimeApi<Block, AccountId>,
    C::Api: pallet_portal_rpc::PortalRuntimeApi<Block, AccountId>,
    C::Api: pallet_3vm_evm_rpc::EvmRuntimeRPCApi<Block, AccountId, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    module.merge(Contracts::new(client.clone()).into_rpc())?;
    module.merge(Xdns::new(client.clone()).into_rpc())?;
    module.merge(Portal::new(client.clone()).into_rpc())?;
    module.merge(Evm::new(client.clone()).into_rpc())?;

    Ok(module)
}
