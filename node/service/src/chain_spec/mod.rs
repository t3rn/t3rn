use std::collections::BTreeMap;
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
use pallet_xdns::SideEffectInterface;
use t3rn_primitives::abi::Type;

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

fn standard_side_effects_map() -> BTreeMap<[u8; 4], SideEffectInterface> {
    let transfer_side_effect = SideEffectInterface {
            id: *b"tran",
            name: b"transfer".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: from
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: value
                Type::OptionalInsurance, // argument_3: insurance
            ],
            argument_to_state_mapper: vec![
                b"from".to_vec(),
                b"to".to_vec(),
                b"value".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"Transfer(from,to,value)".to_vec()],
            escrowed_events: vec![b"EscrowTransfer(from,to,value)".to_vec()],
            commit_events: vec![b"Transfer(executor,to,value)".to_vec()],
            revert_events: vec![b"Transfer(executor,from,value)".to_vec()],
        };

        let swap_side_effect = SideEffectInterface {
            id: *b"swap",
            name: b"swap".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: caller
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: amount_from
                Type::Value,             // argument_3: amount_to
                Type::DynamicBytes,      // argument_4: asset_from
                Type::DynamicBytes,      // argument_5: asset_to
                Type::OptionalInsurance, // argument_6: insurance
            ],
            argument_to_state_mapper: vec![
                b"caller".to_vec(),
                b"to".to_vec(),
                b"amount_from".to_vec(),
                b"amount_to".to_vec(),
                b"asset_from".to_vec(),
                b"asset_to".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
            escrowed_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
            commit_events: vec![b"MultiTransfer(executor,to,asset_to,amount_to)".to_vec()],
            revert_events: vec![b"MultiTransfer(executor,caller,asset_from,amount_from)".to_vec()],
        };

        let add_liquidity_side_effect = SideEffectInterface {
            id: *b"aliq",
            name: b"add_liquidity".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: caller
                Type::DynamicAddress,    // argument_1: to
                Type::DynamicBytes,      // argument_2: asset_left
                Type::DynamicBytes,      // argument_3: asset_right
                Type::DynamicBytes,      // argument_4: liquidity_token
                Type::Value,             // argument_5: amount_left
                Type::Value,             // argument_6: amount_right
                Type::Value,             // argument_7: amount_liquidity_token
                Type::OptionalInsurance, // argument_8: insurance
            ],
            argument_to_state_mapper: vec![
                b"caller".to_vec(),
                b"to".to_vec(),
                b"asset_left".to_vec(),
                b"assert_right".to_vec(),
                b"liquidity_token".to_vec(),
                b"amount_left".to_vec(),
                b"amount_right".to_vec(),
                b"amount_liquidity_token".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![
                b"ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            escrowed_events: vec![
                b"ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            commit_events: vec![
                b"MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            revert_events: vec![
                b"MultiTransfer(executor,caller,asset_left,amount_left)".to_vec(),
                b"MultiTransfer(executor,caller,asset_right,amount_right)".to_vec(),
            ],
        };

        let call_evm_side_effect = SideEffectInterface {
            id: *b"call",
            name: b"call:generic".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress, // argument_0: source
                Type::DynamicAddress, // argument_1: target
                Type::DynamicBytes,   // argument_2: target
                Type::Value,          // argument_3: value
                Type::Uint(64),       // argument_4: gas_limit
                Type::Value,          // argument_5: max_fee_per_gas
                Type::Value,          // argument_6: max_priority_fee_per_gas
                Type::Value,          // argument_7: nonce
                Type::DynamicBytes,   // argument_8: access_list (since HF Berlin?)
            ],
            argument_to_state_mapper: vec![
                b"source".to_vec(),
                b"target".to_vec(),
                b"input".to_vec(),
                b"value".to_vec(),
                b"gas_limit".to_vec(),
                b"max_fee_per_gas".to_vec(),
                b"max_priority_fee_per_gas".to_vec(),
                b"nonce".to_vec(),
                b"access_list".to_vec(),
            ],
            confirm_events: vec![
                b"TransactCall(Append<caller>,source,value,input,gas_limit)".to_vec()
            ],
            escrowed_events: vec![],
            commit_events: vec![],
            revert_events: vec![],
        };

        let get_data_side_effect = SideEffectInterface {
            id: *b"data",
            name: b"data:get".to_vec(),
            argument_abi: vec![
                Type::DynamicBytes, // argument_0: key
            ],
            argument_to_state_mapper: vec![b"key".to_vec()],
            confirm_events: vec![b"<InclusionOnly>".to_vec()],
            escrowed_events: vec![],
            commit_events: vec![],
            revert_events: vec![],
        };

        BTreeMap::from([
            (transfer_side_effect.id, transfer_side_effect),
            (swap_side_effect.id, swap_side_effect),
            (add_liquidity_side_effect.id, add_liquidity_side_effect),
            (call_evm_side_effect.id, call_evm_side_effect),
            (get_data_side_effect.id, get_data_side_effect),
        ])
}
