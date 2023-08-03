use circuit_parachain_runtime::{AccountId, AuraId, Signature, SudoConfig, TRN};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

const PARACHAIN_ID: u32 = 3333;
const SUPPLY: u128 = TRN * 100_000_000; // 100 million TRN
const CANDIDACY_BOND: u128 = TRN * 10_000; // 10K TRN
const DESIRED_CANDIDATES: u32 = 32;
const SUDO: &str = "t3UH3gWsemHbtan74rWKJsWc8BXyYKoteMdS78PMYeywzRLBX";
pub(crate) const SS58_FORMAT: u16 = 9935;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<circuit_parachain_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension,
)]
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
            polkadot_genesis(
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
            // TODO: needs updating
            polkadot_genesis(
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
        },
    )
}

fn polkadot_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<(AccountId, u128)>,
    id: ParaId,
    root_key: AccountId,
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
                .map(|(acc, amt)| (acc, amt))
                .collect(),
        },
        treasury: Default::default(),
        parachain_info: circuit_parachain_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: circuit_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: CANDIDACY_BOND,
            desired_candidates: DESIRED_CANDIDATES,
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
        transaction_payment: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_is_right() {
        assert_eq!(SUPPLY, 100_000_000_000_000_000_000);
    }
}
