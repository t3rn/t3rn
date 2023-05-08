use circuit_parachain_runtime::{
    AccountId, AuraId, EvmConfig, MaintenanceModeConfig, Signature, SudoConfig, XDNSConfig,
};
use cumulus_primitives_core::ParaId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;
use t3rn_primitives::monetary::TRN;

use t3rn_abi::sfx_abi::SFXAbi;
use t3rn_primitives::xdns::GatewayRecord;
use t3rn_types::sfx::Sfx4bId;

const PARACHAIN_ID: u32 = 3333_u32;

fn standard_sfx_abi() -> Vec<(Sfx4bId, SFXAbi)> {
    t3rn_abi::standard::standard_sfx_abi()
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
    _gateway_records: Vec<GatewayRecord<AccountId>>,
    standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
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
            known_xdns_records: vec![],
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
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_start_in_maintenance_mode_is_false() {
        let gen = testnet_genesis(
            Default::default(),
            Default::default(),
            Default::default(),
            sp_runtime::AccountId32::new([0; 32]),
            Default::default(),
            Default::default(),
        );
        assert!(
            !gen.maintenance_mode.start_in_maintenance_mode,
            "start_in_maintenance_mode should be false"
        );
    }
}
