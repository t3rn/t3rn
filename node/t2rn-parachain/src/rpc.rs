//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::RpcModule;
use sc_client_api::{AuxStore, Backend, BlockchainEvents, StateBackend, StorageProvider};
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_network::NetworkService;
use sc_network_sync::SyncingService;
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
    Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_core::H256;
use sp_runtime::{traits::BlakeTwo256, OpaqueExtrinsic};
use std::{collections::BTreeMap, sync::Arc};
use t2rn_parachain_runtime::{opaque::Block, AccountId, Balance, Hash, Nonce};

use pallet_portal_rpc::{Portal, PortalApiServer};
use pallet_xdns_rpc::{Xdns, XdnsApiServer};

use sp_api::CallApiAt;

use fc_rpc::{
    pending::ConsensusDataProvider, Eth, EthApiServer, EthBlockDataCacheTask, EthFilter,
    EthFilterApiServer, EthPubSub, EthPubSubApiServer, Net, NetApiServer, OverrideHandle, Web3,
    Web3ApiServer,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};

pub fn open_frontier_backend<C>(
    client: Arc<C>,
    config: &sc_service::Configuration,
) -> Result<Arc<fc_db::kv::Backend<Block>>, String>
    where
        C: sp_blockchain::HeaderBackend<Block>,
{
    let config_dir = config.base_path.config_dir(config.chain_spec.id());
    let path = config_dir.join("frontier").join("db");

    Ok(Arc::new(fc_db::kv::Backend::<Block>::new(
        client,
        &fc_db::kv::DatabaseSettings {
            source: fc_db::DatabaseSource::RocksDb {
                path,
                cache_size: 0,
            },
        },
    )?))
}

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Graph pool instance.
    pub graph: Arc<Pool<A>>,
    /// Network service
    pub network: Arc<NetworkService<Block, Hash>>,
    /// Chain syncing service
    pub sync: Arc<SyncingService<Block>>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// The Node authority flag
    pub is_authority: bool,
    /// Frontier Backend.
    pub frontier_backend: Arc<dyn fc_db::BackendReader<Block> + Send + Sync>,
    /// EthFilterApi pool.
    pub filter_pool: FilterPool,
    /// Maximum fee history cache size.
    pub fee_history_limit: u64,
    /// Fee history cache.
    pub fee_history_cache: FeeHistoryCache,
    /// Ethereum data access overrides.
    pub overrides: Arc<OverrideHandle<Block>>,
    /// Cache for Ethereum block data.
    pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
    /// Enable EVM RPC servers
    pub enable_evm_rpc: bool,
    /// Mandated parent hashes for a given block hash.
    pub forced_parent_hashes: Option<BTreeMap<H256, H256>>,
}

pub fn create_full<C, P, BE, A>(
    deps: FullDeps<C, P, A>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<Block>,
        >,
    >,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: BlockchainEvents<Block>,
    C: CallApiAt<Block>,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: pallet_xdns_rpc::XdnsRuntimeApi<Block, AccountId>,
    C::Api: pallet_portal_rpc::PortalRuntimeApi<Block, AccountId, Balance, Hash>,
    C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
    C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
    BE: Backend<Block> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
    BE::Blockchain: BlockchainBackend<Block>,
    A: ChainApi<Block = Block> + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        graph,
        network,
        sync,
        deny_unsafe,
        is_authority,
        frontier_backend,
        filter_pool,
        fee_history_limit,
        fee_history_cache,
        overrides,
        block_data_cache,
        enable_evm_rpc,
        forced_parent_hashes,
    } = deps;

    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    module.merge(Xdns::new(client.clone()).into_rpc())?;
    module.merge(Portal::new(client).into_rpc())?;

    // Ethereum  modules
    let no_tx_converter: Option<fp_rpc::NoTransactionConverter> = None;

    module.merge(
        Eth::new(
            client.clone(),
            pool.clone(),
            graph.clone(),
            no_tx_converter,
            sync.clone(),
            Default::default(),
            overrides.clone(),
            frontier_backend.clone(),
            is_authority,
            block_data_cache.clone(),
            fee_history_cache,
            fee_history_limit,
            // Allow 10x max allowed weight for non-transactional calls
            10,
            forced_parent_hashes,
            Default::default(),
            None,
        )
        .into_rpc(),
    )?;

    let max_past_logs: u32 = 10_000;
    let max_stored_filters: usize = 500;
    //let tx_pool = TxPool::new(client.clone(), graph);

    module.merge(
        EthFilter::new(
            client.clone(),
            frontier_backend,
            fc_rpc::TxPool::new(client.clone(), graph.clone()),
            filter_pool,
            max_stored_filters,
            max_past_logs,
            block_data_cache,
        )
        .into_rpc(),
    )?;

    module.merge(Net::new(client.clone(), network.clone(), true).into_rpc())?;

    module.merge(Web3::new(client.clone()).into_rpc())?;

    module.merge(
       EthPubSub::new(
           pool,
           client.clone(),
           sync,
           subscription_task_executor,
           overrides,
           pubsub_notification_sinks,
       )
           .into_rpc(),
   )?;


    Ok(module)
}
