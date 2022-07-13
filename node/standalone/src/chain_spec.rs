use circuit_standalone_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
    MultiFinalityVerifierDefaultConfig, MultiFinalityVerifierEthereumLikeConfig,
    MultiFinalityVerifierGenericLikeConfig, MultiFinalityVerifierPolkadotLikeConfig,
    MultiFinalityVerifierSubstrateLikeConfig, Signature, SudoConfig, SystemConfig, XDNSConfig,
    WASM_BINARY,
};

use jsonrpc_runtime_client::{
    create_rpc_client, get_gtwy_init_data, get_metadata, get_parachain_id, ConnectionParams,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Encode, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{
    convert::TryFrom,
    io::{Error, ErrorKind},
    time::Duration,
};
use t3rn_primitives::{
    bridges::{
        header_chain::InitializationData,
        runtime::{
            BASILISK_CHAIN_ID, CATALYST_CHAIN_ID, DALI_CHAIN_ID, DOLPHIN_CHAIN_ID,
            GENSHIRO_CHAIN_ID, KUSAMA_CHAIN_ID, PANGOLIN_CHAIN_ID, POLKADOT_CHAIN_ID,
            ROCFINITY_CHAIN_ID, ROCOCO_CHAIN_ID, ROCOCO_ENCOINTER_CHAIN_ID, SNOWBLINK_CHAIN_ID,
            SOONSOCIAL_CHAIN_ID,
        },
    },
    side_effect::interface::SideEffectInterface,
    xdns::{Parachain, XdnsRecord},
    ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor, Header,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

fn is_relaychain(chain_id: &ChainId) -> bool {
    match *chain_id {
        POLKADOT_CHAIN_ID | KUSAMA_CHAIN_ID | ROCOCO_CHAIN_ID => true,
        _ => false,
    }
}

/// Helper function that fetches metadata from live networks and generates a XdnsRecord.
async fn fetch_xdns_record_from_rpc(
    provider: &str,
    chain_id: t3rn_primitives::ChainId,
) -> Result<XdnsRecord<AccountId>, Error> {
    let params = ConnectionParams {
        host: String::from(provider),
        port: 443,
        secure: true,
    };

    let client = async_std::future::timeout(Duration::from_secs(12), create_rpc_client(&params))
        .await
        .map_err(|_| Error::new(ErrorKind::TimedOut, provider))?
        .map_err(|err| Error::new(ErrorKind::NotConnected, err))?;

    let metadata = get_metadata(&client.clone())
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gateway_sys_props = GatewaySysProps::try_from(&chain_id)
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    let mut modules_vec = vec![];
    metadata.pallets.encode_to(&mut modules_vec);

    let parachain_info = if is_relaychain(&chain_id) {
        None
    } else {
        let parachain_id = get_parachain_id(&client.clone())
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        Some(Parachain {
            relay_chain_id: chain_id,
            id: parachain_id,
        })
    };

    Ok(<XdnsRecord<AccountId>>::new(
        format!("wss://{}", params.host).as_bytes().to_vec(),
        chain_id,
        parachain_info,
        Default::default(),
        GatewayVendor::PolkadotLike,
        GatewayType::ProgrammableExternal(0),
        GatewayGenesisConfig {
            modules_encoded: Some(modules_vec),
            extrinsics_version: metadata.extrinsic.version,
            genesis_hash: client.genesis_hash.0.to_vec(),
        },
        gateway_sys_props,
        vec![],
        vec![*b"tran"],
    ))
}

/// Helper function to generate XdnsRecords from RPC.
fn seed_xdns_registry() -> Result<Vec<XdnsRecord<AccountId>>, Error> {
    async_std::task::block_on(async {
        let chains = vec![
            // Relaychains...
            ("rpc.polkadot.io", POLKADOT_CHAIN_ID),
            ("kusama-rpc.polkadot.io", KUSAMA_CHAIN_ID),
            ("rococo-rpc.polkadot.io", ROCOCO_CHAIN_ID),
            // Rococo parachains...
            ("rococo.api.encointer.org", ROCOCO_ENCOINTER_CHAIN_ID),
            ("rpc-01.basilisk-rococo.hydradx.io", BASILISK_CHAIN_ID),
            ("fullnode.catalyst.cntrfg.com", CATALYST_CHAIN_ID),
            ("rpc.composablefinance.ninja", DALI_CHAIN_ID),
            ("ws.rococo.dolphin.engineering", DOLPHIN_CHAIN_ID),
            ("rpc.rococo.efinity.io", ROCFINITY_CHAIN_ID),
            (
                "parachain-testnet.equilab.io/rococo/collator/node1/wss",
                GENSHIRO_CHAIN_ID,
            ),
            ("pangolin-parachain-rpc.darwinia.network", PANGOLIN_CHAIN_ID),
            ("rococo-rpc.snowbridge.network", SNOWBLINK_CHAIN_ID),
            ("rco-para.subsocial.network", SOONSOCIAL_CHAIN_ID),
        ];

        let mut records = Vec::with_capacity(chains.len());

        for (provider, chain_id) in chains.into_iter() {
            let r = fetch_xdns_record_from_rpc(provider, chain_id).await;
            if r.is_ok() {
                records.push(r.unwrap());
                log::info!("🧭 fetched XDNS info from wss://{}", provider);
            } else {
                log::warn!(
                    "⚠️  unable to fetch XDNS info from wss://{} {:?}",
                    provider,
                    r.unwrap_err()
                );
            }
        }

        Ok(records)
    })
}

fn standard_side_effects() -> Vec<SideEffectInterface> {
    t3rn_protocol::side_effects::standards::standard_side_effects()
}

/// Fetches gateway initialization data by chain id.
fn fetch_gtwy_init_data(gateway_id: &ChainId) -> Result<InitializationData<Header>, Error> {
    async_std::task::block_on(async move {
        let endpoint = match *gateway_id {
            POLKADOT_CHAIN_ID => "rpc.polkadot.io",
            KUSAMA_CHAIN_ID => "kusama-rpc.polkadot.io",
            ROCOCO_CHAIN_ID => "rococo-rpc.polkadot.io",
            _ => return Err(Error::new(ErrorKind::InvalidInput, "unknown gateway id")),
        };

        let client = create_rpc_client(&ConnectionParams {
            host: endpoint.to_string(),
            port: 443,
            secure: true,
        })
        .await
        .map_err(|error| Error::new(ErrorKind::NotConnected, error))?;

        let (authority_set, header) =
            get_gtwy_init_data(&client.clone(), is_relaychain(gateway_id))
                .await
                .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;

        Ok(InitializationData {
            header,
            authority_list: authority_set.authorities,
            set_id: authority_set.set_id,
            is_halted: false,
            gateway_id: *gateway_id,
        })
    })
}

/// Lists initialization data for indicated gateways.
fn initial_gateways(gateway_ids: Vec<&ChainId>) -> Result<Vec<InitializationData<Header>>, Error> {
    let init_data = gateway_ids
        .iter()
        .map(|gateway_id| fetch_gtwy_init_data(*gateway_id))
        .collect::<Result<_, Error>>()?;

    Ok(init_data)
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                seed_xdns_registry().unwrap_or_default(),
                standard_side_effects(),
                vec![],
                // initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                //     .expect("initial gateways"),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                seed_xdns_registry().unwrap_or_default(),
                standard_side_effects(),
                vec![],
                // initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                //     .expect("initial gateways"),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_side_effects: Vec<SideEffectInterface>,
    initial_gateways: Vec<InitializationData<Header>>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
        // beefy: BeefyConfig {
        //     // authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
        //     authorities: Vec::new(),
        // },
        xdns: XDNSConfig {
            known_xdns_records: xdns_records,
            standard_side_effects,
        },
        contracts_registry: Default::default(),
        multi_finality_verifier_substrate_like: MultiFinalityVerifierSubstrateLikeConfig {
            owner: None,
            init_data: None,
        },
        multi_finality_verifier_generic_like: MultiFinalityVerifierGenericLikeConfig {
            owner: None,
            init_data: None,
        },
        multi_finality_verifier_ethereum_like: MultiFinalityVerifierEthereumLikeConfig {
            owner: None,
            init_data: None,
        },
        multi_finality_verifier_polkadot_like: MultiFinalityVerifierPolkadotLikeConfig {
            owner: None,
            init_data: None,
        },
        multi_finality_verifier_default: MultiFinalityVerifierDefaultConfig {
            owner: None,
            init_data: Some(initial_gateways),
        },
        orml_tokens: Default::default(),
        account_manager: Default::default(),
    }
}
