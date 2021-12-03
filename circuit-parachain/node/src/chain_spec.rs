use circuit_parachain_runtime::{AccountId, AuraId, Balance, Signature, EXISTENTIAL_DEPOSIT, UNIT};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<circuit_parachain_runtime::GenesisConfig, Extensions>;

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

pub fn development_config(id: ParaId) -> ChainSpec {
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
				get_account_id_from_seed::<sr25519::Public>("Alice"),
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
				id,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
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
				get_account_id_from_seed::<sr25519::Public>("Alice"),
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
				id,
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("circuit-local"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> circuit_parachain_runtime::GenesisConfig {
	circuit_parachain_runtime::GenesisConfig {
		system: circuit_parachain_runtime::SystemConfig {
			code: circuit_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: circuit_parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
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
		sudo: circuit_parachain_runtime::SudoConfig { key: root_key.clone() },
		multi_finality_verifier_polkadot_like: Default::default(),
		xdns: Default::default(),
		elections: Default::default(),
		council: Default::default(),
		democracy: Default::default(),
		ethereum_light_client: Default::default(),
	}
}

pub fn circuit_config(id: ParaId) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "T3RN".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 1333.into());

	ChainSpec::from_genesis(
		// Name
		"Circuit",
		// ID
		"circuit",
		ChainType::Live,
		move || {
			genesis(
				// initial collators.
				vec![
					(
						hex!["4c5d8611f64604f0413b8277be968a7aa8295972399eb466989d1969e2bfbc7d"]
							.into(),
						hex!["4c5d8611f64604f0413b8277be968a7aa8295972399eb466989d1969e2bfbc7d"]
							.unchecked_into(),
					),
					(
						hex!["089fcfb60670b8c86237df9acdafc59b27b7c5a5bd3990da7bf753940767b023"]
							.into(),
						hex!["089fcfb60670b8c86237df9acdafc59b27b7c5a5bd3990da7bf753940767b023"]
							.unchecked_into(),
					),
				],
				// sudo
				hex!["2af656d7541a1911ade17e1eea634ea932f700767da7bf0f5e5e617efaec163e"].into(),
				vec![
					(
						// sudo 97 million
						hex!["2af656d7541a1911ade17e1eea634ea932f700767da7bf0f5e5e617efaec163e"]
							.into(),
						(97_000_000 * UNIT).into(),
					),
					(
						// beqa 1 million
						hex!["6818cda9ffb645241c3b1f11539cabbd347520ce3e73e83571502a6ff05dc220"]
							.into(),
						(1_000_000 * UNIT).into(),
					),
					(
						// zannis 1 million
						hex!["8ef3b1c8708e01ed0b707acc9ae1a9e0a0f3057c142c62e714652431aa9c6247"]
							.into(),
						(1_000_000 * UNIT).into(),
					),
					(
						// ved 1 million
						hex!["58eff1cf80796776dca1ffc26983c905ec35bc298f5f2e694fce682564c07f51"]
							.into(),
						(1_000_000 * UNIT).into(),
					),
				],
				id,
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("circuit"),
		// Properties
		Some(properties),
		// Extensions
		Extensions { relay_chain: "".into(), para_id: id.into() },
	)
}

fn genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	id: ParaId,
) -> circuit_parachain_runtime::GenesisConfig {
	circuit_parachain_runtime::GenesisConfig {
		system: circuit_parachain_runtime::SystemConfig {
			code: circuit_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: circuit_parachain_runtime::BalancesConfig { balances: endowed_accounts },
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
		sudo: circuit_parachain_runtime::SudoConfig { key: root_key.clone() },
		multi_finality_verifier_polkadot_like: Default::default(),
		xdns: Default::default(),
		elections: Default::default(),
		council: Default::default(),
		democracy: Default::default(),
		ethereum_light_client: Default::default(),
	}
}
