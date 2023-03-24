use circuit_parachain_runtime::{AccountId, AuraId, EvmConfig, Signature, SudoConfig, XDNSConfig};
use cumulus_primitives_core::ParaId;
use jsonrpc_runtime_client::{
    create_rpc_client, get_gtwy_init_data, get_metadata, get_parachain_id, ConnectionParams,
};
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Encode, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{
    convert::TryFrom,
    io::{Error, ErrorKind},
    str::FromStr,
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
    monetary::TRN,
    xdns::{Parachain, XdnsRecord},
    ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor, Header, TokenSysProps,
};

use t3rn_abi::sfx_abi::SFXAbi;
use t3rn_types::sfx::Sfx4bId;

const PARACHAIN_ID: u32 = 3333_u32;

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

    let gateway_sys_props = TokenSysProps::try_from(&chain_id)
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
        GatewayVendor::Rococo,
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

fn standard_sfx_abi() -> Vec<(Sfx4bId, SFXAbi)> {
    t3rn_abi::standard::standard_sfx_abi()
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

        let is_relay_chain = matches!(
            *gateway_id,
            POLKADOT_CHAIN_ID | KUSAMA_CHAIN_ID | ROCOCO_CHAIN_ID
        );

        let (authority_set, header) = get_gtwy_init_data(&client.clone(), is_relay_chain)
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
        .map(|gateway_id| fetch_gtwy_init_data(gateway_id))
        .collect::<Result<_, Error>>()?;

    Ok(init_data)
}

/// t3rn-pallets chain spec config -- END

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<circuit_parachain_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<circuit_parachain_runtime::Block>,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed.
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{seed}"), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to derive an account ID from a SS58 address.
pub fn get_account_id_from_adrs(adrs: &str) -> AccountId {
    AccountId::from_str(adrs).expect("account id from SS58 address")
}

/// Helper function to derive a public key from a SS58 address.
pub fn get_public_from_adrs<TPublic: Public>(adrs: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(adrs, None)
        .expect("keypair from SS58 address")
        .public()
}

/// Derive an Aura id from a SS58 address.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_aura_id_from_adrs(adrs: &str) -> AuraId {
    use sp_core::crypto::Ss58Codec;
    AuraId::from_string(adrs).expect("aura id from SS58 address")
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn session_keys(keys: AuraId) -> circuit_parachain_runtime::SessionKeys {
    circuit_parachain_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_collator_keys_from_seed("Alice"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_collator_keys_from_seed("Bob"),
                    ),
                ],
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
                PARACHAIN_ID.into(),
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![],
                standard_sfx_abi(),
                vec![],
                // initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                //     .expect("initial gateways"),
            )
        },
        Vec::new(),
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: PARACHAIN_ID,              // You MUST set this correctly!
            bad_blocks: None,
        },
    )
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "TRN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_collator_keys_from_seed("Alice"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_collator_keys_from_seed("Bob"),
                    ),
                ],
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
                PARACHAIN_ID.into(),
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![],
                standard_sfx_abi(),
                vec![],
                // initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                //     .expect("initial gateways"),
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("circuit-local"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: PARACHAIN_ID,              // You MUST set this correctly!
            bad_blocks: None,
        },
    )
}

pub fn rococo_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "T0RN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "t0rn",
        // Id
        "t0rn_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                // Invulnerable collators
                vec![
                    (
                        get_account_id_from_adrs(
                            "5FKjxoi5Yfjwa1aXesFWRXMpvs4vJMXFeG2ydFPyNwUn4qiW",
                        ),
                        get_aura_id_from_adrs("5FKjxoi5Yfjwa1aXesFWRXMpvs4vJMXFeG2ydFPyNwUn4qiW"),
                    ),
                    (
                        get_account_id_from_adrs(
                            "5DcyZrktqRvmpHQGLJEucnYM6qwJpG8HN8zaL8dvGUsBb67v",
                        ),
                        get_aura_id_from_adrs("5DcyZrktqRvmpHQGLJEucnYM6qwJpG8HN8zaL8dvGUsBb67v"),
                    ),
                ],
                // Prefunded accounts
                vec![
                    get_account_id_from_adrs("5D333eBb5VugHioFoU5nGMbUaR2uYcoyk5qZj9tXRA5ers7A"),
                    get_account_id_from_adrs("5CAYyLZxG4oYQP8CGTYgPPhkoT42NyMvi2J3hKPCLGyKHAC4"),
                    get_account_id_from_adrs("5GducktTqf8KKeatpex4kwkg1PZZimY1xUDUFoBZ2s5EDfVf"),
                    get_account_id_from_adrs("5CqRUh9fiVgzMftXmacNSNMXF4TDfkUXCTZvXfuYXA33knRC"),
                    get_account_id_from_adrs("5DXBQResSqHCGijMH1UtpQNZzpjdCqHtad14FUnwaSA7xmRL"),
                    get_account_id_from_adrs("5HomG74gKivcZfCLixXyZbuGg57Bc8ZR55BkAX2jus2dSYS1"),
                    get_account_id_from_adrs("5FQpivNZCVw3LQWoQwrF44CLeP1g5j8RSAtcR4kURbZwXgXg"),
                    get_account_id_from_adrs("5GLMuTmTvNCWkYCYc2DNPWjTMKa2nKW6yYH8xKqeiPHgcLNs"),
                    get_account_id_from_adrs("5DWyim48gMrAhoHz9pjb6qu5Q8paDmeWhisALuV9cS8NvScG"),
                    get_account_id_from_adrs("5FU77XnhRuBD6VSA8ZvwqB6BSjyYEGtS4HPwMMU6WwqpVmmV"),
                ],
                PARACHAIN_ID.into(),
                // Sudo
                get_account_id_from_adrs("5D333eBb5VugHioFoU5nGMbUaR2uYcoyk5qZj9tXRA5ers7A"),
                vec![],
                standard_sfx_abi(),
                vec![],
                // initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                //     .expect("initial gateways"),
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("t0rn"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo".into(), // You MUST set this to the correct network!
            para_id: PARACHAIN_ID,        // You MUST set this correctly!
            bad_blocks: None,
        },
    )
}

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

fn testnet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    _initial_gateways: Vec<InitializationData<Header>>,
) -> circuit_parachain_runtime::GenesisConfig {
    circuit_parachain_runtime::GenesisConfig {
        system: circuit_parachain_runtime::SystemConfig {
            code: circuit_parachain_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: circuit_parachain_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        treasury: Default::default(),
        assets: Default::default(),
        parachain_info: circuit_parachain_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: circuit_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: (TRN as u128) * 10_u128,
            desired_candidates: 32_u32,
            ..Default::default()
        },
        session: circuit_parachain_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),        // account id
                        acc,                // validator id
                        session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: circuit_parachain_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        xdns: XDNSConfig {
            known_xdns_records: xdns_records,
            known_gateway_records: vec![],
            standard_sfx_abi,
        },
        contracts_registry: Default::default(),
        account_manager: Default::default(),
        clock: Default::default(),
        three_vm: Default::default(), // TODO: genesis for this needs to be setup for the function pointers
        evm: EvmConfig {
            // We need _some_ code inserted at the precompile address so that
            // the evm will actually call the address.
            accounts: circuit_parachain_runtime::contracts_config::PrecompilesValue::get()
                .used_addresses()
                .into_iter()
                .map(|addr| {
                    (
                        addr,
                        circuit_parachain_runtime::contracts_config::EvmGenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: REVERT_BYTECODE.into(),
                        },
                    )
                })
                .collect(),
        },
    }
}
