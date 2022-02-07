use codec::Encode;
use jsonrpc_runtime_client::ConnectionParams;
use log::info;
use pallet_xdns::types::XdnsRecord;
use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::IdentifyAccount;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};
use t3rn_primitives::bridges::runtime::{KUSAMA_CHAIN_ID, POLKADOT_CHAIN_ID};
use t3rn_primitives::{
    AccountId, AccountPublic, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};

#[cfg(feature = "with-parachain-runtime")]
pub mod parachain;

#[cfg(feature = "with-standalone-runtime")]
pub mod standalone;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
    /// Known bad block hashes.
    #[serde(default)]
    pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v1::Block>,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

#[cfg(feature = "with-standalone-runtime")]
use beefy_primitives::crypto::AuthorityId as BeefyId;
/// Generate an Aura authority key.

#[cfg(feature = "with-standalone-runtime")]
use sp_finality_grandpa::AuthorityId as GrandpaId;
#[cfg(feature = "with-standalone-runtime")]
pub fn get_authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId, BeefyId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<BeefyId>(s),
    )
}

/// Generate an Aura authority key for Karura.
pub fn get_parachain_authority_keys_from_seed(seed: &str) -> (AccountId, AuraId) {
    (
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<AuraId>(seed),
    )
}

/// Helper function that fetches metadata from live networks and generates an XdnsRecord
fn fetch_xdns_record_from_rpc(
    params: &ConnectionParams,
    chain_id: t3rn_primitives::ChainId,
) -> Result<XdnsRecord<AccountId>, Error> {
    async_std::task::block_on(async move {
        let client = jsonrpc_runtime_client::create_rpc_client(params)
            .await
            .unwrap();

        let runtime_version = client.clone().runtime_version().await.unwrap();
        let metadata = jsonrpc_runtime_client::get_metadata(&client.clone())
            .await
            .unwrap();

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
                // signed_extensions: Some(extension_vec),
                runtime_version,
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
