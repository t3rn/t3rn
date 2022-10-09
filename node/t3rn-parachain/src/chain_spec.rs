use circuit_parachain_runtime::{AccountId, AuraId, Signature, SudoConfig};
use cumulus_primitives_core::ParaId;

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;
use t3rn_primitives::{
    bridges::runtime::{KUSAMA_CHAIN_ID, POLKADOT_CHAIN_ID, ROCOCO_CHAIN_ID},
    monetary::TRN,
    ChainId,
};

const PARACHAIN_ID: u32 = 3333_u32;

fn is_relaychain(chain_id: &ChainId) -> bool {
    match *chain_id {
        POLKADOT_CHAIN_ID | KUSAMA_CHAIN_ID | ROCOCO_CHAIN_ID => true,
        _ => false,
    }
}

/// t3rn-pallets chain spec config -- END

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
    TPublic::Pair::from_string(&format!("//{}", seed), None)
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

pub fn polkadot_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "TRN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "t3rn",
        // Id
        "t3rn_polkadot",
        ChainType::Live,
        move || {
            polkadot_genesis(
                // Invulnerable collators
                vec![
                    (
                        get_account_id_from_adrs(
                            "t3rn2cUTJPwjhCXRGjUMm9jViuLxa4oZsz32nPJZJxftHDeb",
                        ),
                        get_aura_id_from_adrs("t3rn2cUTJPwjhCXRGjUMm9jViuLxa4oZsz32nPJZJxftHDeb"),
                    ),
                    (
                        get_account_id_from_adrs(
                            "t3rn36tUEi4swq6uDNef5nZzdrBn4X5vKrSUsxx94HVeHXGq",
                        ),
                        get_aura_id_from_adrs("t3rn36tUEi4swq6uDNef5nZzdrBn4X5vKrSUsxx94HVeHXGq"),
                    ),
                ],
                // Prefunded accounts
                vec![get_account_id_from_adrs(
                    "t3rnTimebvS7RsUGsN6u7N6sR1gqWtXiUVLG9nSeSon4Q7K",
                )],
                PARACHAIN_ID.into(),
                // Sudo
                get_account_id_from_adrs("t3rnTimebvS7RsUGsN6u7N6sR1gqWtXiUVLG9nSeSon4Q7K"),
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
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

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

fn polkadot_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
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
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        parachain_info: circuit_parachain_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: circuit_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: (TRN as u128) * 10_000_u128,
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
    }
}
