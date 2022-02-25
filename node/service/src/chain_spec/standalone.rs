use crate::chain_spec::get_authority_keys_from_seed;

use beefy_primitives::crypto::AuthorityId as BeefyId;
use jsonrpc_core::serde_json;

use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::Ss58Codec;

use sp_core::{sr25519, Pair};
use sp_finality_grandpa::AuthorityId as GrandpaId;


use t3rn_primitives::bridges::runtime::{
    SourceAccount, GATEWAY_CHAIN_ID,
};

use crate::chain_spec::{get_account_id_from_seed, seed_xdns_registry};
use circuit_standalone_runtime::{
    AuraConfig, BalancesConfig, BeefyConfig, ContractsRegistryConfig, GenesisConfig, GrandpaConfig,
    MultiFinalityVerifierConfig, SessionKeys, SudoConfig, SystemConfig, XDNSConfig,
    WASM_BINARY,
};
use pallet_xdns::XdnsRecord;
use t3rn_primitives::bridges::chain_circuit::derive_account_from_gateway_id;
use t3rn_primitives::{
    AccountId,
};

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

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub fn load(self) -> ChainSpec {
        let properties = Some(
            serde_json::json!({
                "tokenDecimals": 9_u8,
                "tokenSymbol": "TRN",
                "bridgeIds": {
                    "Gateway": GATEWAY_CHAIN_ID,
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
                            derive_account_from_gateway_id(SourceAccount::Account(
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
                            // pallet_bridge_messages::Pallet::<
                            //     circuit_standalone_runtime::Runtime,
                            //     pallet_bridge_messages::DefaultInstance,
                            // >::relayer_fund_account_id(),
                            derive_account_from_gateway_id(SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Alice"),
                            )),
                            derive_account_from_gateway_id(SourceAccount::Account(
                                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                            )),
                            derive_account_from_gateway_id(SourceAccount::Account(
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
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 50))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.2.clone(), 2))
                .collect(),
        },
        beefy: BeefyConfig {
            authorities: initial_authorities.iter().map(|x| (x.3.clone())).collect(),
        },
        sudo: SudoConfig {
            key: root_key,
        },
        // session: SessionConfig {
        //     keys: initial_authorities
        //         .iter()
        //         .map(|x| {
        //             (
        //                 x.0.clone(),
        //                 x.0.clone(),
        //                 session_keys(x.1.clone(), x.2.clone(), x.3.clone()),
        //             )
        //         })
        //         .collect::<Vec<_>>(),
        // },
        // evm: EVMConfig {
        //     accounts: {
        //         let mut map = BTreeMap::new();
        //         map.insert(
        //             sp_core::H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
        //                 .expect("internal H160 is valid; qed"),
        //             pallet_evm::GenesisAccount {
        //                 balance: sp_core::U256::from_str("0xffffffffffffffffffffffffffffffff")
        //                     .expect("internal U256 is valid; qed"),
        //                 code: Default::default(),
        //                 nonce: Default::default(),
        //                 storage: Default::default(),
        //             },
        //         );
        //         map
        //     },
        // },
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
        ethereum_light_client: circuit_standalone_runtime::EthereumLightClientConfig {
            initial_header: Default::default(),
            initial_difficulty: Default::default(),
        },
        basic_outbound_channel: circuit_standalone_runtime::BasicOutboundChannelConfig {
            // this is the account for pal/exec module_id
            principal: AccountId::from_string("5FmrGR9YMhgHqcrNc4W9enTbmCLRE6sbAJKs3kqA5kJfWQoN")
                .expect("Should not fail"),
            interval: 1,
        },
        orml_tokens: Default::default(),
    }
}

fn is_standalone() -> bool {
    true
}

#[test]
fn derived_dave_account_is_as_expected() {
    let dave = get_account_id_from_seed::<sr25519::Public>("Dave");
    let derived: AccountId = derive_account_from_gateway_id(SourceAccount::Account(dave));
    assert_eq!(
        derived.to_string(),
        "5C9NFeDzVveQeCvyUDA7fJv47NygtdL69i6JjmBAGf1KEDv5".to_string()
    );
}

#[test]
fn fetch_xdns_should_return_results() {
    let actual = seed_xdns_registry().unwrap();
    assert_eq!(actual.len(), 2);
}
