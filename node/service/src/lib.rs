//! t3rn service. Specialized wrapper over substrate service.

pub mod chain_spec;
// std
use std::sync::Arc;

#[cfg(not(any(
    feature = "with-standalone-runtime",
    feature = "with-parachain-runtime"
)))]
use t3rn_primitives::{AccountId, Balance, Block, Hash, Header, Nonce};

// Local Runtime Types
#[cfg(feature = "with-parachain-runtime")]
use circuit_parachain_runtime::{
    opaque::Block, AccountId, Balance, Hash, Index as Nonce, RuntimeApi,
};
#[cfg(feature = "with-standalone-runtime")]
use circuit_standalone_runtime::{
    opaque::Block, AccountId, Balance, Hash, Index as Nonce, RuntimeApi,
};

// Cumulus Imports
use cumulus_client_consensus_aura::{
    build_aura_consensus, BuildAuraConsensusParams, SlotProportion,
};
use cumulus_client_consensus_common::ParachainConsensus;
use cumulus_client_network::build_block_announce_validator;
use cumulus_client_service::{
    prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::ParaId;

// Substrate Imports
use sc_client_api::ExecutorProvider;
use sc_executor::NativeElseWasmExecutor;
use sc_network::NetworkService;
use sc_service::{Configuration, PartialComponents, Role, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::ConstructRuntimeApi;
use sp_consensus::SlotData;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::BlakeTwo256;
use substrate_prometheus_endpoint::Registry;

mod rpc;

/// Native executor instance.
pub struct CircuitRuntimeExecutor;

#[cfg(feature = "with-parachain-runtime")]
impl sc_executor::NativeExecutionDispatch for CircuitRuntimeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        circuit_parachain_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        circuit_parachain_runtime::native_version()
    }
}

#[cfg(feature = "with-standalone-runtime")]
impl sc_executor::NativeExecutionDispatch for CircuitRuntimeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        circuit_standalone_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        circuit_standalone_runtime::native_version()
    }
}

pub trait IdentifyVariant {
    fn is_standalone(&self) -> bool;
    fn is_parachain(&self) -> bool;
}

impl IdentifyVariant for Box<dyn sc_service::ChainSpec> {
    fn is_standalone(&self) -> bool {
        self.id().starts_with("standalone")
    }

    fn is_parachain(&self) -> bool {
        self.id().starts_with("parachain")
    }
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
#[cfg(feature = "with-parachain-runtime")]
pub fn new_partial<RuntimeApi, Executor, BIQ>(
    config: &Configuration,
    build_import_queue: BIQ,
) -> Result<
    PartialComponents<
        TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
        TFullBackend<Block>,
        (),
        sc_consensus::DefaultImportQueue<
            Block,
            TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
        >,
        sc_transaction_pool::FullPool<
            Block,
            TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
        >,
        (Option<Telemetry>, Option<TelemetryWorkerHandle>),
    >,
    sc_service::Error,
>
where
    RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>
        + Send
        + Sync
        + 'static,
    RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::Metadata<Block>
        + sp_session::SessionKeys<Block>
        + sp_api::ApiExt<
            Block,
            StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
        > + sp_offchain::OffchainWorkerApi<Block>
        + sp_block_builder::BlockBuilder<Block>,
    sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
    Executor: sc_executor::NativeExecutionDispatch + 'static,
    BIQ: FnOnce(
        Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>,
        &Configuration,
        Option<TelemetryHandle>,
        &TaskManager,
    ) -> Result<
        sc_consensus::DefaultImportQueue<
            Block,
            TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
        >,
        sc_service::Error,
    >,
{
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = sc_executor::NativeElseWasmExecutor::<Executor>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let import_queue = build_import_queue(
        client.clone(),
        config,
        telemetry.as_ref().map(|telemetry| telemetry.handle()),
        &task_manager,
    )?;

    let params = PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: (),
        other: (telemetry, telemetry_worker_handle),
    };

    Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
#[cfg(feature = "with-parachain-runtime")]
async fn start_node_impl<RuntimeApi, Executor, RB, BIQ, BIC>(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    id: ParaId,
    _rpc_ext_builder: RB,
    build_import_queue: BIQ,
    build_consensus: BIC,
) -> sc_service::error::Result<(
    TaskManager,
    Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>,
)>
where
    RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>
        + Send
        + Sync
        + 'static,
    RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::Metadata<Block>
        + sp_session::SessionKeys<Block>
        + sp_api::ApiExt<
            Block,
            StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
        > + sp_offchain::OffchainWorkerApi<Block>
        + sp_block_builder::BlockBuilder<Block>
        + cumulus_primitives_core::CollectCollationInfo<Block>
        + pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
        + substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
    Executor: sc_executor::NativeExecutionDispatch + 'static,
    RB: Fn(
            Arc<TFullClient<Block, RuntimeApi, Executor>>,
        ) -> Result<jsonrpc_core::IoHandler<sc_rpc::Metadata>, sc_service::Error>
        + Send
        + 'static,
    BIQ: FnOnce(
            Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>,
            &Configuration,
            Option<TelemetryHandle>,
            &TaskManager,
        ) -> Result<
            sc_consensus::DefaultImportQueue<
                Block,
                TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
            >,
            sc_service::Error,
        > + 'static,
    BIC: FnOnce(
        Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>>,
        Option<&Registry>,
        Option<TelemetryHandle>,
        &TaskManager,
        &polkadot_service::NewFull<polkadot_service::Client>,
        Arc<
            sc_transaction_pool::FullPool<
                Block,
                TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>,
            >,
        >,
        Arc<NetworkService<Block, Hash>>,
        SyncCryptoStorePtr,
        bool,
    ) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error>,
{
    if matches!(parachain_config.role, Role::Light) {
        return Err("Light client not supported!".into());
    }

    let parachain_config = prepare_node_config(parachain_config);

    let params = new_partial::<RuntimeApi, Executor, BIQ>(&parachain_config, build_import_queue)?;
    let (mut telemetry, telemetry_worker_handle) = params.other;

    let relay_chain_full_node =
        cumulus_client_service::build_polkadot_full_node(polkadot_config, telemetry_worker_handle)
            .map_err(|e| match e {
                polkadot_service::Error::Sub(x) => x,
                s => format!("{}", s).into(),
            })?;

    let client = params.client.clone();
    let backend = params.backend.clone();
    let block_announce_validator = build_block_announce_validator(
        relay_chain_full_node.client.clone(),
        id,
        Box::new(relay_chain_full_node.network.clone()),
        relay_chain_full_node.backend.clone(),
    );

    let force_authoring = parachain_config.force_authoring;
    let validator = parachain_config.role.is_authority();
    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let transaction_pool = params.transaction_pool.clone();
    let mut task_manager = params.task_manager;
    let import_queue = cumulus_client_service::SharedImportQueue::new(params.import_queue);
    let (network, system_rpc_tx, start_network) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue: import_queue.clone(),
            block_announce_validator_builder: Some(Box::new(|_| block_announce_validator)),
            warp_sync: None,
        })?;

    let rpc_extensions_builder = {
        let client = client.clone();
        let transaction_pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: transaction_pool.clone(),
                deny_unsafe,
            };

            Ok(crate::rpc::create_full(deps))
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_extensions_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: params.keystore_container.sync_keystore(),
        backend: backend.clone(),
        network: network.clone(),
        system_rpc_tx,
        telemetry: telemetry.as_mut(),
    })?;

    let announce_block = {
        let network = network.clone();
        Arc::new(move |hash, data| network.announce_block(hash, data))
    };

    if validator {
        let parachain_consensus = build_consensus(
            client.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            &relay_chain_full_node,
            transaction_pool,
            network,
            params.keystore_container.sync_keystore(),
            force_authoring,
        )?;

        let spawner = task_manager.spawn_handle();

        let params = StartCollatorParams {
            para_id: id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_full_node,
            spawner,
            parachain_consensus,
            import_queue,
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id: id,
            relay_chain_full_node,
        };

        start_full_node(params)?;
    }

    start_network.start_network();

    Ok((task_manager, client))
}

/// Build the import queue for the parachain runtime.
#[allow(clippy::type_complexity)]
#[cfg(feature = "with-parachain-runtime")]
pub fn parachain_build_import_queue(
    client: Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<CircuitRuntimeExecutor>>>,
    config: &Configuration,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
) -> Result<
    sc_consensus::DefaultImportQueue<
        Block,
        TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<CircuitRuntimeExecutor>>,
    >,
    sc_service::Error,
> {
    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

    cumulus_client_consensus_aura::import_queue::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
        _,
        _,
    >(cumulus_client_consensus_aura::ImportQueueParams {
        block_import: client.clone(),
        client: client.clone(),
        create_inherent_data_providers: move |_, _| async move {
            let time = sp_timestamp::InherentDataProvider::from_system_time();

            let slot =
                sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
                    *time,
                    slot_duration.slot_duration(),
                );

            Ok((time, slot))
        },
        registry: config.prometheus_registry(),
        can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
        spawner: &task_manager.spawn_essential_handle(),
        telemetry,
    })
    .map_err(Into::into)
}

/// Start a parachain node.
#[cfg(feature = "with-parachain-runtime")]
pub async fn start_parachain_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    id: ParaId,
) -> sc_service::error::Result<(
    TaskManager,
    Arc<TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<CircuitRuntimeExecutor>>>,
)> {
    start_node_impl::<RuntimeApi, CircuitRuntimeExecutor, _, _, _>(
        parachain_config,
        polkadot_config,
        id,
        |_| Ok(Default::default()),
        parachain_build_import_queue,
        |client,
         prometheus_registry,
         telemetry,
         task_manager,
         relay_chain_node,
         transaction_pool,
         sync_oracle,
         keystore,
         force_authoring| {
            let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

            let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
                task_manager.spawn_handle(),
                client.clone(),
                transaction_pool,
                prometheus_registry,
                telemetry.clone(),
            );

            let relay_chain_backend = relay_chain_node.backend.clone();
            let relay_chain_client = relay_chain_node.client.clone();
            Ok(build_aura_consensus::<
                sp_consensus_aura::sr25519::AuthorityPair,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            >(BuildAuraConsensusParams {
                proposer_factory,
                create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
                    let parachain_inherent =
					cumulus_primitives_parachain_inherent::ParachainInherentData::create_at_with_client(
						relay_parent,
						&relay_chain_client,
						&*relay_chain_backend,
						&validation_data,
						id,
					);
                    async move {
                        let time = sp_timestamp::InherentDataProvider::from_system_time();

                        let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
							*time,
							slot_duration.slot_duration(),
						);

                        let parachain_inherent = parachain_inherent.ok_or_else(|| {
                            Box::<dyn std::error::Error + Send + Sync>::from(
                                "Failed to create parachain inherent",
                            )
                        })?;
                        Ok((time, slot, parachain_inherent))
                    }
                },
                block_import: client.clone(),
                relay_chain_client: relay_chain_node.client.clone(),
                relay_chain_backend: relay_chain_node.backend.clone(),
                para_client: client,
                backoff_authoring_blocks: Option::<()>::None,
                sync_oracle,
                keystore,
                force_authoring,
                slot_duration,
                // We got around 500ms for proposing
                block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
                // And a maximum of 750ms if slots are skipped
                max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
                telemetry,
            }))
        },
    )
    .await
}

#[cfg(feature = "with-standalone-runtime")]
pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            sc_finality_grandpa::GrandpaBlockImport<
                FullBackend,
                Block,
                FullClient,
                FullSelectChain,
            >,
            sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
> {
    if config.keystore_remote.is_some() {
        return Err(ServiceError::Other(format!(
            "Remote Keystores are not supported."
        )));
    }

    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            &config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let slot_duration = sc_consensus_aura::slot_duration(&*client)?.slot_duration();

    let import_queue =
        sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(ImportQueueParams {
            block_import: grandpa_block_import.clone(),
            justification_import: Some(Box::new(grandpa_block_import.clone())),
            client: client.clone(),
            create_inherent_data_providers: move |_, ()| async move {
                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                let slot =
                    sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
                        *timestamp,
                        slot_duration,
                    );

                Ok((timestamp, slot))
            },
            spawner: &task_manager.spawn_essential_handle(),
            can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(
                client.executor().clone(),
            ),
            registry: config.prometheus_registry(),
            check_for_equivocation: Default::default(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        })?;

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (grandpa_block_import, grandpa_link, telemetry),
    })
}

/// Builds a new service for a full client.
#[cfg(feature = "with-standalone-runtime")]
pub fn new_full(mut config: Configuration) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        mut keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, grandpa_link, mut telemetry),
    } = new_partial(&config)?;

    if let Some(url) = &config.keystore_remote {
        match remote_keystore(url) {
            Ok(k) => keystore_container.set_remote_keystore(k),
            Err(e) => {
                return Err(ServiceError::Other(format!(
                    "Error hooking up remote keystore for {}: {}",
                    url, e
                )))
            }
        };
    }

    config
        .network
        .extra_sets
        .push(sc_finality_grandpa::grandpa_peers_set_config());
    let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync: Some(warp_sync),
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks: Option<()> = None;
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };

            Ok(crate::rpc::create_full(deps))
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_extensions_builder,
        backend,
        system_rpc_tx,
        config,
        telemetry: telemetry.as_mut(),
    })?;

    if role.is_authority() {
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let can_author_with =
            sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

        let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
        let raw_slot_duration = slot_duration.slot_duration();

        let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _, _>(
            StartAuraParams {
                slot_duration,
                client: client.clone(),
                select_chain,
                block_import,
                proposer_factory,
                create_inherent_data_providers: move |_, ()| async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                    let slot =
                        sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
                            *timestamp,
                            raw_slot_duration,
                        );

                    Ok((timestamp, slot))
                },
                force_authoring,
                backoff_authoring_blocks,
                keystore: keystore_container.sync_keystore(),
                can_author_with,
                sync_oracle: network.clone(),
                justification_sync_link: network.clone(),
                block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
                max_block_proposal_slot_portion: None,
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            },
        )?;

        // the AURA authoring task is considered essential, i.e. if it
        // fails we take down the service with it.
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("aura", Some("block-authoring"), aura);
    }

    // if the node isn't actively participating in consensus then it doesn't
    // need a keystore, regardless of which protocol we use below.
    let keystore = if role.is_authority() {
        Some(keystore_container.sync_keystore())
    } else {
        None
    };

    let grandpa_config = sc_finality_grandpa::Config {
        // FIXME #1578 make this available through chainspec
        gossip_duration: Duration::from_millis(333),
        justification_period: 512,
        name: Some(name),
        observer_enabled: false,
        keystore,
        local_role: role,
        telemetry: telemetry.as_ref().map(|x| x.handle()),
    };

    if enable_grandpa {
        // start the full GRANDPA voter
        // NOTE: non-authorities could run the GRANDPA observer protocol, but at
        // this point the full voter should provide better guarantees of block
        // and vote data availability than the observer. The observer has not
        // been tested extensively yet and having most nodes in a network run it
        // could lead to finality stalls.
        let grandpa_config = sc_finality_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network,
            voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state: SharedVoterState::empty(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };

        // the GRANDPA voter task is considered infallible, i.e.
        // if it fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
        );
    }

    network_starter.start_network();
    Ok(task_manager)
}
