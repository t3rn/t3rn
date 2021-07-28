// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

use bp_circuit::derive_account_from_gateway_id;
use circuit_runtime::{
    AccountId, AuraConfig, BalancesConfig, EVMConfig, GenesisConfig, GrandpaConfig, SessionConfig,
    SessionKeys, Signature, SudoConfig, SystemConfig, XDNSConfig, WASM_BINARY,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

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
pub fn get_authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
    )
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> ChainSpec {
        let properties = Some(
            serde_json::json!({
                "tokenDecimals": 9,
                "tokenSymbol": "MLAU",
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

fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { aura, grandpa }
}

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: SystemConfig {
            code: WASM_BINARY
                .expect("Circuit development WASM not available")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 50))
                .collect(),
        },
        pallet_aura: AuraConfig {
            authorities: Vec::new(),
        },
        pallet_grandpa: GrandpaConfig {
            authorities: Vec::new(),
        },
        pallet_sudo: SudoConfig { key: root_key },
        pallet_session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.1.clone(), x.2.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        pallet_evm: EVMConfig {
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
		pallet_xdns: XDNSConfig {
			known_xdns_records: Vec::new(),
		}
        //ToDo: Uncomment when upgrading to v4.0.0 substrate
        // system: SystemConfig {
        //     code: WASM_BINARY
        //         .expect("Circuit development WASM not available")
        //         .to_vec(),
        //     changes_trie_config: Default::default(),
        // },
        // balances: BalancesConfig {
        //     balances: endowed_accounts
        //         .iter()
        //         .cloned()
        //         .map(|k| (k, 1 << 50))
        //         .collect(),
        // },
        // aura: AuraConfig {
        //     authorities: Vec::new(),
        // },
        // grandpa: GrandpaConfig {
        //     authorities: Vec::new(),
        // },
        // sudo: SudoConfig { key: root_key },
        // session: SessionConfig {
        //     keys: initial_authorities
        //         .iter()
        //         .map(|x| {
        //             (
        //                 x.0.clone(),
        //                 x.0.clone(),
        //                 session_keys(x.1.clone(), x.2.clone()),
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
