use std::{collections::BTreeMap, str::FromStr};

use async_std::task;
use beefy_primitives::crypto::AuthorityId as BeefyId;
use bp_circuit::derive_account_from_gateway_id;
use bp_runtime::{CIRCUIT_CHAIN_ID, GATEWAY_CHAIN_ID, KUSAMA_CHAIN_ID, POLKADOT_CHAIN_ID};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Encode, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use circuit_runtime::{
    AccountId, AuraConfig, BalancesConfig, BeefyConfig, ContractsRegistryConfig, EVMConfig,
    GenesisConfig, GrandpaConfig, MultiFinalityVerifierConfig, SessionConfig, SessionKeys,
    Signature, SudoConfig, SystemConfig, XDNSConfig, WASM_BINARY,
};
use jsonrpc_runtime_client::{create_rpc_client, get_metadata, ConnectionParams};
use pallet_xdns::XdnsRecord;
use t3rn_primitives::{GatewayGenesisConfig, GatewayType, GatewayVendor};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob/Charlie/Dave/Eve auths.
    LocalTestnet,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn get_authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId, BeefyId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<BeefyId>(s),
    )
}

/// Helper function that fetches metadata from live networks and generates an XdnsRecord
fn fetch_xdns_record_from_rpc(
    params: &ConnectionParams,
    chain_id: t3rn_primitives::ChainId,
) -> Result<XdnsRecord<AccountId>, std::io::Error> {
    task::block_on(async move {
        let client = create_rpc_client(params).await.unwrap();

        let runtime_version = client.clone().runtime_version().await.unwrap();
        let metadata = get_metadata(&client.clone()).await.unwrap();

        let mut modules_vec = vec![];
        let mut extension_vec = vec![];
        metadata.modules.encode_to(&mut modules_vec);
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
                extrinsics_version: metadata.extrinsic.version.into(),
                signed_extension: Some(extension_vec),
                runtime_version,
                genesis_hash: client.genesis_hash.0.to_vec(),
            },
        ))
    })
}

/// Helper function to generate Polkadot and Kusama XdnsRecords from RPC
fn seed_xdns_registry() -> Result<Vec<XdnsRecord<AccountId>>, std::io::Error> {
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

    let circuit_connection_params: ConnectionParams = ConnectionParams {
        host: String::from("dev.net.t3rn.io"),
        port: 443,
        secure: true,
    };

    let demo_gateway_connection_params: ConnectionParams = ConnectionParams {
        host: String::from("dev.net.t3rn.io/gateway"),
        port: 443,
        secure: true,
    };

    let polkadot_xdns =
        fetch_xdns_record_from_rpc(&polkadot_connection_params, POLKADOT_CHAIN_ID).unwrap();

    let kusama_xdns =
        fetch_xdns_record_from_rpc(&kusama_connection_params, KUSAMA_CHAIN_ID).unwrap();

    let circuit_xdns =
        fetch_xdns_record_from_rpc(&circuit_connection_params, CIRCUIT_CHAIN_ID).unwrap();

    let demo_gateway_xdns =
        fetch_xdns_record_from_rpc(&demo_gateway_connection_params, GATEWAY_CHAIN_ID).unwrap();

    Ok(vec![
        polkadot_xdns,
        kusama_xdns,
        circuit_xdns,
        demo_gateway_xdns,
    ])
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> ChainSpec {
        let properties = Some(
            serde_json::json!({
                "tokenDecimals": 9,
                "tokenSymbol": "TRN",
                "bridgeIds": {
                    "Gateway": bp_runtime::GATEWAY_CHAIN_ID,
                }
            })
            .as_object()
            .expect("Map given; qed")
            .clone(),
        );

        match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Development",
                "dev",
                sc_service::ChainType::Development,
                || {
                    testnet_genesis(
                        vec![get_authority_keys_from_seed("Alice")],
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        vec![
                            get_account_id_from_seed::<sr25519::Public>("Alice"),
                            get_account_id_from_seed::<sr25519::Public>("Bob"),
                            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                            derive_account_from_gateway_id(bp_runtime::SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Alice"),
                            )),
                        ],
                        seed_xdns_registry().unwrap_or_default(),
                        true,
                    )
                },
                vec![],
                None,
                None,
                properties,
                None,
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Local Testnet",
                "local_testnet",
                sc_service::ChainType::Local,
                || {
                    testnet_genesis(
                        vec![
                            get_authority_keys_from_seed("Alice"),
                            get_authority_keys_from_seed("Bob"),
                            get_authority_keys_from_seed("Charlie"),
                            get_authority_keys_from_seed("Dave"),
                            get_authority_keys_from_seed("Eve"),
                        ],
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        vec![
                            get_account_id_from_seed::<sr25519::Public>("Alice"),
                            get_account_id_from_seed::<sr25519::Public>("Bob"),
                            get_account_id_from_seed::<sr25519::Public>("Charlie"),
                            get_account_id_from_seed::<sr25519::Public>("Dave"),
                            get_account_id_from_seed::<sr25519::Public>("Eve"),
                            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                            get_account_id_from_seed::<sr25519::Public>("George"),
                            get_account_id_from_seed::<sr25519::Public>("Harry"),
                            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                            get_account_id_from_seed::<sr25519::Public>("George//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Harry//stash"),
                            pallet_bridge_messages::Pallet::<
                                circuit_runtime::Runtime,
                                pallet_bridge_messages::DefaultInstance,
                            >::relayer_fund_account_id(),
                            derive_account_from_gateway_id(bp_runtime::SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Alice"),
                            )),
                            derive_account_from_gateway_id(bp_runtime::SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                            )),
                            derive_account_from_gateway_id(bp_runtime::SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Eve"),
                            )),
                        ],
                        seed_xdns_registry().unwrap_or_default(),
                        true,
                    )
                },
                vec![],
                None,
                None,
                properties,
                None,
            ),
        }
    }
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, beefy: BeefyId) -> SessionKeys {
    SessionKeys {
        aura,
        grandpa,
        beefy,
    }
}

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId, BeefyId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    xdns_records: Vec<XdnsRecord<AccountId>>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY
                .expect("Circuit development WASM not available")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 50))
                .collect(),
        },
        aura: AuraConfig {
            authorities: vec![],
        },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        beefy: BeefyConfig {
            authorities: vec![],
        },
        sudo: SudoConfig {
            key: root_key.clone(),
        },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.1.clone(), x.2.clone(), x.3.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        evm: EVMConfig {
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    sp_core::H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: sp_core::U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map
            },
        },
        xdns: XDNSConfig {
            known_xdns_records: xdns_records,
        },
        contracts_registry: ContractsRegistryConfig {
            known_contracts: Vec::new(),
        },
        multi_finality_verifier: MultiFinalityVerifierConfig {
            owner: None,
            init_data: None,
        },
        ethereum_light_client: circuit_runtime::EthereumLightClientConfig {
            initial_header: Default::default(),
            initial_difficulty: Default::default(),
        },
    }
}

#[test]
fn derived_dave_account_is_as_expected() {
    let dave = get_account_id_from_seed::<sr25519::Public>("Dave");
    let derived: AccountId =
        derive_account_from_gateway_id(bp_runtime::SourceAccount::Account(dave));
    assert_eq!(
        derived.to_string(),
        "5C9NFeDzVveQeCvyUDA7fJv47NygtdL69i6JjmBAGf1KEDv5".to_string()
    );
}

/// Ignore as this includes actual HTTP calls
#[test]
#[ignore]
fn fetch_xdns_should_return_results() {
    let actual = seed_xdns_registry();
    assert_ok!(actual);
    assert_eq!(actual.unwrap().len(), 4);
}
