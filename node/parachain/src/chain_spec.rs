use circuit_parachain_runtime::{
    AccountId, AuraId, MultiFinalityVerifierDefaultConfig, MultiFinalityVerifierEthereumLikeConfig,
    MultiFinalityVerifierGenericLikeConfig, MultiFinalityVerifierPolkadotLikeConfig,
    MultiFinalityVerifierSubstrateLikeConfig, Signature, SudoConfig, XDNSConfig,
    EXISTENTIAL_DEPOSIT,
};
use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use jsonrpc_runtime_client::{
    create_rpc_client, get_gtwy_init_data, get_metadata, ConnectionParams,
};
use pallet_xdns::types::{SideEffectInterface, XdnsRecord};
use sp_core::Encode;
/// t3rn-pallets chain spec config -- START
use t3rn_primitives::{
    abi::Type,
    bridges::{
        header_chain::InitializationData,
        runtime::{KUSAMA_CHAIN_ID, POLKADOT_CHAIN_ID, ROCOCO_CHAIN_ID},
    },
    ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor, Header,
};

use log::info;
use std::{
    convert::TryFrom,
    io::{Error, ErrorKind},
};
use t3rn_primitives::{side_effect::interface::SideEffectInterface, xdns::XdnsRecord};

/// Helper function that fetches metadata from live networks and generates an XdnsRecord
fn fetch_xdns_record_from_rpc(
    params: &ConnectionParams,
    chain_id: t3rn_primitives::ChainId,
) -> Result<XdnsRecord<AccountId>, Error> {
    async_std::task::block_on(async move {
        let client = create_rpc_client(params).await.unwrap();

        let _runtime_version = client.clone().runtime_version().await.unwrap();
        let metadata = get_metadata(&client.clone()).await.unwrap();

        let gateway_sys_props = GatewaySysProps::try_from(&chain_id)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        let mut modules_vec = vec![];
        let mut extension_vec = vec![];
        metadata.pallets.encode_to(&mut modules_vec);
        metadata
            .extrinsic
            .signed_extensions
            .encode_to(&mut extension_vec);

        Ok(<XdnsRecord<AccountId>>::new(
            format!("wss://{}", params.host).as_bytes().to_vec(),
            chain_id,
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            GatewayGenesisConfig {
                modules_encoded: Some(modules_vec),
                extrinsics_version: metadata.extrinsic.version,
                genesis_hash: client.genesis_hash.0.to_vec(),
            },
            gateway_sys_props,
            vec![],
        ))
    })
}

/// Helper function to generate Polkadot and Kusama XdnsRecords from RPC
fn seed_xdns_registry() -> Result<Vec<XdnsRecord<AccountId>>, Error> {
    let polkadot_connection_params: ConnectionParams = ConnectionParams {
        host: String::from("rpc.polkadot.io"),
        port: 443,
        secure: true,
    };

    let kusama_connection_params: ConnectionParams = ConnectionParams {
        host: String::from("kusama-rpc.polkadot.io"),
        port: 443,
        secure: true,
    };

    let polkadot_xdns =
        fetch_xdns_record_from_rpc(&polkadot_connection_params, POLKADOT_CHAIN_ID).unwrap();
    info!("Fetched Polkadot metadata successfully!");
    let kusama_xdns =
        fetch_xdns_record_from_rpc(&kusama_connection_params, KUSAMA_CHAIN_ID).unwrap();
    info!("Fetched Kusama metadata successfully!");

    Ok(vec![polkadot_xdns, kusama_xdns])
}

fn standard_side_effects() -> Vec<SideEffectInterface> {
    let transfer_side_effect = SideEffectInterface {
        id: *b"tran",
        name: b"transfer".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: from
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: value
            Type::OptionalInsurance, // argument_3: insurance
        ],
        argument_to_state_mapper: vec![
            b"from".to_vec(),
            b"to".to_vec(),
            b"value".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"Transfer(_from,to,value)".to_vec()],
        escrowed_events: vec![b"EscrowTransfer(from,to,value)".to_vec()],
        commit_events: vec![b"Transfer(executor,to,value)".to_vec()],
        revert_events: vec![b"Transfer(executor,from,value)".to_vec()],
    };

    let swap_side_effect = SideEffectInterface {
        id: *b"swap",
        name: b"swap".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: amount_from
            Type::Value,             // argument_3: amount_to
            Type::DynamicBytes,      // argument_4: asset_from
            Type::DynamicBytes,      // argument_5: asset_to
            Type::OptionalInsurance, // argument_6: insurance
        ],
        argument_to_state_mapper: vec![
            b"caller".to_vec(),
            b"to".to_vec(),
            b"amount_from".to_vec(),
            b"amount_to".to_vec(),
            b"asset_from".to_vec(),
            b"asset_to".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
        escrowed_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
        commit_events: vec![b"MultiTransfer(executor,to,asset_to,amount_to)".to_vec()],
        revert_events: vec![b"MultiTransfer(executor,caller,asset_from,amount_from)".to_vec()],
    };

    let add_liquidity_side_effect = SideEffectInterface {
        id: *b"aliq",
        name: b"add_liquidity".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::DynamicBytes,      // argument_2: asset_left
            Type::DynamicBytes,      // argument_3: asset_right
            Type::DynamicBytes,      // argument_4: liquidity_token
            Type::Value,             // argument_5: amount_left
            Type::Value,             // argument_6: amount_right
            Type::Value,             // argument_7: amount_liquidity_token
            Type::OptionalInsurance, // argument_8: insurance
        ],
        argument_to_state_mapper: vec![
            b"caller".to_vec(),
            b"to".to_vec(),
            b"asset_left".to_vec(),
            b"assert_right".to_vec(),
            b"liquidity_token".to_vec(),
            b"amount_left".to_vec(),
            b"amount_right".to_vec(),
            b"amount_liquidity_token".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![
            b"ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        escrowed_events: vec![
            b"ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        commit_events: vec![
            b"MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        revert_events: vec![
            b"MultiTransfer(executor,caller,asset_left,amount_left)".to_vec(),
            b"MultiTransfer(executor,caller,asset_right,amount_right)".to_vec(),
        ],
    };

    let call_evm_side_effect = SideEffectInterface {
        id: *b"call",
        name: b"call:generic".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress, // argument_0: source
            Type::DynamicAddress, // argument_1: target
            Type::DynamicBytes,   // argument_2: target
            Type::Value,          // argument_3: value
            Type::Uint(64),       // argument_4: gas_limit
            Type::Value,          // argument_5: max_fee_per_gas
            Type::Value,          // argument_6: max_priority_fee_per_gas
            Type::Value,          // argument_7: nonce
            Type::DynamicBytes,   // argument_8: access_list (since HF Berlin?)
        ],
        argument_to_state_mapper: vec![
            b"source".to_vec(),
            b"target".to_vec(),
            b"input".to_vec(),
            b"value".to_vec(),
            b"gas_limit".to_vec(),
            b"max_fee_per_gas".to_vec(),
            b"max_priority_fee_per_gas".to_vec(),
            b"nonce".to_vec(),
            b"access_list".to_vec(),
        ],
        confirm_events: vec![b"TransactCall(Append<caller>,source,value,input,gas_limit)".to_vec()],
        escrowed_events: vec![],
        commit_events: vec![],
        revert_events: vec![],
    };

    let get_data_side_effect = SideEffectInterface {
        id: *b"data",
        name: b"data:get".to_vec(),
        argument_abi: vec![
            Type::DynamicBytes, // argument_0: key
        ],
        argument_to_state_mapper: vec![b"key".to_vec()],
        confirm_events: vec![b"<InclusionOnly>".to_vec()],
        escrowed_events: vec![],
        commit_events: vec![],
        revert_events: vec![],
    };

    vec![
        transfer_side_effect,
        swap_side_effect,
        add_liquidity_side_effect,
        call_evm_side_effect,
        get_data_side_effect,
    ]
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

        let is_relay_chain = match *gateway_id {
            POLKADOT_CHAIN_ID | KUSAMA_CHAIN_ID | ROCOCO_CHAIN_ID => true,
            _ => false,
        };

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
        .map(|gateway_id| fetch_gtwy_init_data(*gateway_id))
        .collect::<Result<_, Error>>()?;

    Ok(init_data)
}

/// t3rn-pallets chain spec config -- END

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<circuit_parachain_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
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
                3333_u32.into(),
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                seed_xdns_registry().unwrap_or_default(),
                standard_side_effects(),
                initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                    .expect("initial gateways"),
            )
        },
        Vec::new(),
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 3333_u32,                  // You MUST set this correctly!
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
                3333_u32.into(),
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                seed_xdns_registry().unwrap_or_default(),
                standard_side_effects(),
                initial_gateways(vec![&POLKADOT_CHAIN_ID, &KUSAMA_CHAIN_ID, &ROCOCO_CHAIN_ID])
                    .expect("initial gateways"),
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
            para_id: 3333_u32,                  // You MUST set this correctly!
        },
    )
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_side_effects: Vec<SideEffectInterface>,
    initial_gateways: Vec<InitializationData<Header>>,
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
        parachain_info: circuit_parachain_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: circuit_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
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
        // transaction_payment: Default::default(),
        // beefy: BeefyConfig {
        // 	// authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
        // 	authorities: Vec::new()
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
            init_data: None,
        },
        orml_tokens: Default::default(),
    }
}
