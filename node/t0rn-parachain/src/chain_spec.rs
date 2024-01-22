use parachain_runtime::{
    opaque::Block, AccountId, AuraId, BalancesConfig, CollatorSelectionConfig, EvmConfig,
    GenesisAccount, ParachainInfoConfig, PolkadotXcmConfig, RuntimeGenesisConfig, SessionConfig,
    SessionKeys, Signature, SudoConfig, SystemConfig, XDNSConfig, TRN, U256, WASM_BINARY,
};

use codec::Encode;

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

const PARACHAIN_ID: u32 = 3333;
const PARACHAIN_ID_KUSAMA: u32 = 3334;
const SUPPLY: u128 = TRN * 100_000_000; // 100 million TRN
const CANDIDACY_BOND: u128 = TRN * 10_000; // 10K TRN
const DESIRED_CANDIDATES: u32 = 32;

const RUNTIME_KSM_NAME: &str = "t1rn";

const SUDO: &str = "t3UH3gWsemHbtan74rWKJsWc8BXyYKoteMdS78PMYeywzRLBX";
const SUDO_T0RN: &str = "5D333eBb5VugHioFoU5nGMbUaR2uYcoyk5qZj9tXRA5ers7A";
const SUDO_T1RN: &str = "t1WfJYwMzegLxyeJNR35XbUWFY6kdSWSBUHpC4inyi8dk2yoQ"; // @t1rn; 32b = 0x5ecd4d9f0255ed3d3c5ac1160a965f0ea743b74533036f1e4d3f4bfc43f9f061
pub(crate) const SS58_FORMAT: u16 = 9935;
pub(crate) const SS58_FORMAT_T0RN: u16 = 42;
pub(crate) const SS58_FORMAT_T1RN: u16 = 4815;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ChainSpecExtension)] // removing ChainSpecGroup since bad blocks wont implement
                                                                                   // #[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
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

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (
        get_public_from_seed::<AuraId>(s),
        get_public_from_seed::<GrandpaId>(s),
    )
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
pub fn session_keys(keys: AuraId) -> SessionKeys {
    SessionKeys { aura: keys }
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "TRN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), SS58_FORMAT.into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            polkadot_genesis_full(
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
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        TRN * 100,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                        TRN * 100000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                        TRN * 100000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                        TRN * 100000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                        TRN * 100000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                        TRN * 100000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                        TRN * 100000,
                    ),
                ],
                PARACHAIN_ID.into(),
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
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

pub fn kusama_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "TIN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), SS58_FORMAT_T1RN.into());

    ChainSpec::from_genesis(
        // Name
        RUNTIME_KSM_NAME,
        // Id
        RUNTIME_KSM_NAME,
        ChainType::Live,
        move || {
            polkadot_genesis_full(
                vec![
                    (
                        // Collator 1: 5Cyauvc374WEMNDuWVpjb1rBKhqGsCZnbXCS9nAizD97eSLT
                        hex!("2854a19e6cb5c712db0e4dddc02b144805ed09523144f88d94c45ae933e56106")
                            .into(),
                        hex!("9064ecbcc5f6358d1cce830a0d1db923b9a7f2493c533eadea14ce6c623d1122")
                            .unchecked_into(),
                    ),
                    (
                        // Collator 2: 5FHUyvsgf7PQ8mprJejXDc3dADfpTXphbtC5Djv9Vr1JLC2x
                        hex!("8e738cd5ba60cd9f5ac551c4e68c9f1367d20a9ad22dbc85f19095e59ca43731")
                            .into(),
                        hex!("8e738cd5ba60cd9f5ac551c4e68c9f1367d20a9ad22dbc85f19095e59ca43731")
                            .unchecked_into(),
                    ),
                ],
                // Prefunded accounts
                vec![
                    // Genesis Account: SUDO (t1WfJYwMzegLxyeJNR35XbUWFY6kdSWSBUHpC4inyi8dk2yoQ = hex!("0x5ecd4d9f0255ed3d3c5ac1160a965f0ea743b74533036f1e4d3f4bfc43f9f061").into()
                    // (get_account_id_from_adrs(SUDO_T1RN), SUPPLY),
                    (hex!("5ecd4d9f0255ed3d3c5ac1160a965f0ea743b74533036f1e4d3f4bfc43f9f061").into(), SUPPLY),
                ],
                PARACHAIN_ID_KUSAMA.into(),
                // Sudo
                // get_account_id_from_adrs(SUDO_T1RN),1
                hex!("5ecd4d9f0255ed3d3c5ac1160a965f0ea743b74533036f1e4d3f4bfc43f9f061").into(),
            )
        },
        // Bootnodes ACCOUNT_STORAGE_ROOT_INDEX
        vec![
            sc_service::config::MultiaddrWithPeerId::from_str(
                "/dns/bootnode-1.t1rn.io/tcp/33333/p2p/12D3KooWKWjFmAzdj3vdxSwvfT62jQFCwMbWMY9yPVhGfRskgWGw",
            )
                .expect("Failed to parse bootnode #1 address"),
            sc_service::config::MultiaddrWithPeerId::from_str(
                "/dns/bootnode-2.t1rn.io/tcp/33333/p2p/12D3KooWRQtXyE2cR3uXva8bPhd8cqHA5L8cZHXiH4kkMf8KtP9H",
            )
                .expect("Failed to parse bootnode #2 address"),
        ],
        // Telemetry
        Some(
            TelemetryEndpoints::new(vec![(
                "/dns/telemetry.kusama.io/tcp/443/x-parity-wss/%2Fsubmit%2F".into(),
                1,
            )])
                .expect("telemetry"),
        ),
        // Protocol ID
        Some(RUNTIME_KSM_NAME),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "kusama".into(), // You MUST set this to the correct network!
            para_id: PARACHAIN_ID,          // You MUST set this correctly!
            bad_blocks: None,
        },
    )
}

pub fn polkadot_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "TRN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), SS58_FORMAT.into());

    ChainSpec::from_genesis(
        // Name
        "t3rn",
        // Id
        "t3rn",
        ChainType::Live,
        move || {
            polkadot_genesis_full(
                vec![
                    (
                        // Collator 1: t3XXX7FGKAsG3pwE188CP91zCgt4p2mEQkdeELwocRJ4kCrSw
                        hex!("9064ecbcc5f6358d1cce830a0d1db923b9a7f2493c533eadea14ce6c623d1122")
                            .into(),
                        hex!("9064ecbcc5f6358d1cce830a0d1db923b9a7f2493c533eadea14ce6c623d1122")
                            .unchecked_into(),
                    ),
                    (
                        // Collator 2: t3VVV3XoajCLGHp7kRWjeV37x43eDPb2XPxXJM92jmwCa1Y5h
                        hex!("365f04d23363f74c2239cb0071d7e6c97ce9b8e9372240887570e290ac78f85f")
                            .into(),
                        hex!("365f04d23363f74c2239cb0071d7e6c97ce9b8e9372240887570e290ac78f85f")
                            .unchecked_into(),
                    ),
                ],
                // Prefunded accounts
                vec![
                    // Genesis Account: SUDO (t3UH3gWsemHbtan74rWKJsWc8BXyYKoteMdS78PMYeywzRLBX = hex!("0x00a6769855d6df941f09e0743f8879f66bad2dde6534a268dfe478449a16312b").into()
                    (get_account_id_from_adrs(SUDO), SUPPLY),
                ],
                PARACHAIN_ID.into(),
                // Sudo
                get_account_id_from_adrs(SUDO),
            )
        },
        // Bootnodes
        vec![
            sc_service::config::MultiaddrWithPeerId::from_str(
                "/dns/bootnode-1.t3rn.io/tcp/33333/p2p/12D3KooWDWGoYHhsVUtLehNEdwp8JNi4DLTJVB2L53HMHarBXw66",
            )
                .expect("Failed to parse bootnode #1 address"),
            sc_service::config::MultiaddrWithPeerId::from_str(
                "/dns/bootnode-2.t3rn.io/tcp/33333/p2p/12D3KooWLGtGEf92p8CbUmzwFYavEtDUaJNJCbBSp4muSqs2cVz1",
            )
                .expect("Failed to parse bootnode #2 address"),
        ],
        // Telemetry
        Some(
            TelemetryEndpoints::new(vec![(
                "/dns/telemetry.polkadot.io/tcp/443/x-parity-wss/%2Fsubmit%2F".into(),
                1,
            )])
                .expect("telemetry"),
        ),
        // Protocol ID
        Some("t3rn"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "polkadot".into(), // You MUST set this to the correct network!
            para_id: PARACHAIN_ID,          // You MUST set this correctly!
            bad_blocks: None,
        },
    )
}

fn polkadot_genesis_full(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<(AccountId, u128)>,
    id: ParaId,
    root_key: AccountId,
) -> RuntimeGenesisConfig {
    return RuntimeGenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            _config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|(acc, amt)| (acc, amt))
                .collect(),
        },

        clock: Default::default(),
        account_manager: Default::default(),
        treasury: Default::default(),
        ethereum: Default::default(),
        escrow_treasury: Default::default(),
        fee_treasury: Default::default(),
        parachain_treasury: Default::default(),
        slash_treasury: Default::default(),
        parachain_info: ParachainInfoConfig {
            parachain_id: id,
            _config: Default::default(),
        },
        collator_selection: CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: CANDIDACY_BOND,
            desired_candidates: DESIRED_CANDIDATES,
            ..Default::default()
        },
        session: SessionConfig {
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
        assets: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
            _config: Default::default(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
        contracts_registry: Default::default(),
        attesters: Default::default(),
        evm: EvmConfig {
            accounts: {
                // Prefund the "ALICE" account
                let mut accounts = std::collections::BTreeMap::new();
                accounts.insert(
                    /*SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                     * hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                     * Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
                     *H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558").expect("internal H160 is valid;
                     * qed"), */
                    H160::from_slice(&hex_literal::hex!(
                        "d43593c715fdd31c61141abd04a99fd6822c8558"
                    )),
                    GenesisAccount {
                        nonce: U256::zero(),
                        // Using a larger number, so I can tell the accounts apart by balance.
                        balance: U256::from(2u64 << 61),
                        code: vec![],
                        storage: std::collections::BTreeMap::new(),
                    },
                );
                accounts.insert(
                    // H160 address of CI test runner account
                    H160::from_str("CEB58Fc447ee30D2104dD00ABFe6Fe29fe470e5C")
                        .expect("internal H160 is valid; qed"),
                    GenesisAccount {
                        balance: U256::from(10u64 << 62),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                accounts.encode()
            },
            _marker: Default::default(),
        },
        three_vm: Default::default(),
        rewards: Default::default(),
        maintenance: Default::default(),
        xdns: XDNSConfig {
            known_gateway_records: vec![],
            standard_sfx_abi: t3rn_abi::standard::standard_sfx_abi().encode(),
            _marker: Default::default(),
        },
    }
}

pub fn rococo_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "T0RN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), SS58_FORMAT_T0RN.into());

    ChainSpec::from_genesis(
        // Name
        "t0rn",
        // Id
        "t0rn_testnet",
        ChainType::Live,
        move || {
            polkadot_genesis_full(
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
                    (
                        get_account_id_from_adrs(
                            "5D333eBb5VugHioFoU5nGMbUaR2uYcoyk5qZj9tXRA5ers7A",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5CAYyLZxG4oYQP8CGTYgPPhkoT42NyMvi2J3hKPCLGyKHAC4",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5GducktTqf8KKeatpex4kwkg1PZZimY1xUDUFoBZ2s5EDfVf",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5CqRUh9fiVgzMftXmacNSNMXF4TDfkUXCTZvXfuYXA33knRC",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5DXBQResSqHCGijMH1UtpQNZzpjdCqHtad14FUnwaSA7xmRL",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5HomG74gKivcZfCLixXyZbuGg57Bc8ZR55BkAX2jus2dSYS1",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5FQpivNZCVw3LQWoQwrF44CLeP1g5j8RSAtcR4kURbZwXgXg",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5GLMuTmTvNCWkYCYc2DNPWjTMKa2nKW6yYH8xKqeiPHgcLNs",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5DWyim48gMrAhoHz9pjb6qu5Q8paDmeWhisALuV9cS8NvScG",
                        ),
                        1 << 60,
                    ),
                    (
                        get_account_id_from_adrs(
                            "5FU77XnhRuBD6VSA8ZvwqB6BSjyYEGtS4HPwMMU6WwqpVmmV",
                        ),
                        1 << 60,
                    ),
                ],
                PARACHAIN_ID.into(),
                // Sudo
                get_account_id_from_adrs(SUDO_T0RN),
            )
        },
        // Bootnodes
        vec![
            sc_service::config::MultiaddrWithPeerId::from_str(
                "/dns/bootnode.t0rn.io/tcp/33333/p2p/12D3KooWKt2YedCEqxmUtidvfQQBRdj84XiebfVPptHLQnGdGkyy",
            ).expect("Failed to parse bootnode #1 address"),
        ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_is_right() {
        assert_eq!(SUPPLY, 100_000_000_000_000_000_000);
    }

    #[test]
    fn should_match_correct_aura_keys_for_t0rn() {
        assert_eq!(
            get_aura_id_from_adrs("5FKjxoi5Yfjwa1aXesFWRXMpvs4vJMXFeG2ydFPyNwUn4qiW").encode(),
            hex!("902c7861618ce57396b7052b9ca769f7ea38cdf7d6287783e11e7ac740423942").to_vec()
        );
    }
}
