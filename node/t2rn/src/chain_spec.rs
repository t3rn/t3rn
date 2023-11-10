use circuit_t2rn_runtime::{
    AccountId,
    AuraConfig,
    BalancesConfig,
    GrandpaConfig,
    RuntimeGenesisConfig,
    Signature,
    SudoConfig,
    SystemConfig,
    // SessionConfig,
    // CollatorSelectionConfig,
    XDNSConfig, // EvmConfig
    WASM_BINARY,
};

const CANDIDACY_BOND: u128 = 0; // 10K TRN
const DESIRED_CANDIDATES: u32 = 2;

use codec::Encode;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use t3rn_abi::sfx_abi::SFXAbi;
use t3rn_primitives::xdns::GatewayRecord;
use t3rn_types::sfx::Sfx4bId;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<circuit_t2rn_runtime::RuntimeGenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{seed}"), None)
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

fn standard_sfx_abi() -> Vec<(Sfx4bId, SFXAbi)> {
    t3rn_abi::standard::standard_sfx_abi()
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Blazing Fast Testnet",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("0x70cc1a691b08a41e87a4e78d4ded96b6eb2d0a1311b181abda0b37d03cfa8b26"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("0x2252662f0d97138c24e6b4fcd16ab5d90de4b9ee3d7ad66ea4f48a6bc25e101c"),
                    get_account_id_from_seed::<sr25519::Public>("0xe0326c64f378729b5e0292bbae5b9f9131d7b3d2227676cef628182fd9aff37c"),
                ],
                vec![],
                standard_sfx_abi(),
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
        "Blazing Fast Testnet",
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
                get_account_id_from_seed::<sr25519::Public>("0x70cc1a691b08a41e87a4e78d4ded96b6eb2d0a1311b181abda0b37d03cfa8b26"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("0x2252662f0d97138c24e6b4fcd16ab5d90de4b9ee3d7ad66ea4f48a6bc25e101c"),
                    get_account_id_from_seed::<sr25519::Public>("0xe0326c64f378729b5e0292bbae5b9f9131d7b3d2227676cef628182fd9aff37c"),
                ],
                vec![],
                standard_sfx_abi(),
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

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];
/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
// pub fn session_keys(keys: AuraId) -> SessionKeys {
//     SessionKeys { aura: keys, grandpa: keys }
// }
/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _gateway_records: Vec<GatewayRecord<AccountId>>,
    _standard_sfx_abi: Vec<(Sfx4bId, SFXAbi)>,
    _enable_println: bool,
) -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            _config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, (10000 * 10u128.pow(12))))
                .collect(),
        },
        // session: SessionConfig {
        //     keys: initial_authorities
        //         .iter()
        //         .map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone())))
        //         .collect(),
        // },
        // collator_selection: CollatorSelectionConfig {
        //     invulnerables: initial_authorities.iter().cloned().map(|(acc, _)| acc).collect(),
        //     candidacy_bond: CANDIDACY_BOND,
        //     desired_candidates: DESIRED_CANDIDATES,
        //     ..Default::default()
        // },
        treasury: Default::default(),
        escrow_treasury: Default::default(),
        fee_treasury: Default::default(),
        parachain_treasury: Default::default(),
        slash_treasury: Default::default(),
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
            _config: Default::default(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
        assets: Default::default(),
        rewards: Default::default(),
        xdns: XDNSConfig {
            known_gateway_records: vec![],
            standard_sfx_abi: t3rn_abi::standard::standard_sfx_abi().encode(),
            _marker: Default::default(),
        },
        contracts_registry: Default::default(),
        account_manager: Default::default(),
        attesters: Default::default(),
        clock: Default::default(),
        three_vm: Default::default(), // TODO: genesis for this needs to be setup for the function pointers\
        evm: Default::default(),
        maintenance_mode: Default::default(),
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
            sp_runtime::AccountId32::new([0; 32]),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );
        assert!(
            !gen.maintenance_mode.start_in_maintenance_mode,
            "start_in_maintenance_mode should be false by default"
        );
    }
}
