use crate::polkadot_like_chain::Rococo;
use codec::{Decode, Encode};
use frame_metadata::{v14::RuntimeMetadataV14, RuntimeMetadata, RuntimeMetadataPrefixed};
use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams};
use num_traits::Zero;
use relay_substrate_client::Client as SubstrateClient;
use sc_finality_grandpa::FinalityProof;
use sp_core::Bytes;
use t3rn_primitives::{
    bridges::header_chain::{justification::GrandpaJustification, AuthoritySet},
    Header,
};

/// Get first header of Substrate network
pub async fn get_first_header(sub_client: &SubstrateClient<Rococo>) -> Result<Vec<u8>, String> {
    let initial_header = sub_client.header_by_number(Zero::zero()).await;
    initial_header
        .map(|header| header.encode())
        .map_err(|error| format!("Error reading Substrate genesis header: {error:?}"))
}

pub async fn get_metadata(
    sub_client: &SubstrateClient<Rococo>,
) -> Result<RuntimeMetadataV14, String> {
    let bytes: Bytes = sub_client
        .client
        .request("state_getMetadata", JsonRpcParams::NoParams)
        .await
        .unwrap();

    let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..]).unwrap();
    match meta.1 {
        RuntimeMetadata::V14(md14) => Ok(md14),
        _ => Err("Could not parse metadata".into()),
    }
}

/// Gets the current authority set id, the actual authority set, and header for the latest finalized block.
pub async fn get_gtwy_init_data(
    sub_client: &SubstrateClient<Rococo>,
    is_relay_chain: bool,
) -> Result<(AuthoritySet, Header), String> {
    let block_hash: serde_json::value::Value = sub_client
        .client
        .request("chain_getFinalizedHead", JsonRpcParams::NoParams)
        .await
        .map_err(|error| format!("chain_getFinalizedHead failed: {error:?}"))?;

    let header: Header = sub_client
        .client
        .request(
            "chain_getHeader",
            JsonRpcParams::Array(vec![block_hash.clone()]),
        )
        .await
        .map_err(|error| format!("chain_getHeader failed: {error:?}"))?;

    if is_relay_chain {
        let encoded_finality_proof: Bytes = sub_client
            .client
            .request(
                "grandpa_proveFinality",
                JsonRpcParams::Array(vec![header.number.into()]),
            )
            .await
            .map_err(|error| format!("grandpa_proveFinality failed: {error:?}"))?;

        let finality_proof = <FinalityProof<Header>>::decode(&mut &encoded_finality_proof[..])
            .map_err(|error| format!("finality proof decoding failed: {error:?}"))?;

        let justification =
            GrandpaJustification::<Header>::decode(&mut &*finality_proof.justification)
                .map_err(|error| format!("justification decoding failed: {error:?}"))?;

        let mut authorities = Vec::with_capacity(justification.commit.precommits.len());

        for signed in &justification.commit.precommits {
            authorities.push((signed.id.clone(), 1)); // FIXME: rm hardcoded authority weight
        }

        let authority_set = AuthoritySet::new(authorities, justification.round);

        Ok((authority_set, header))
    } else {
        Err("parachain gateway preregistration is not supported yet".to_string())
    }
}

pub async fn get_parachain_id(sub_client: &SubstrateClient<Rococo>) -> Result<u32, String> {
    let bytes: Bytes = sub_client
        .client
        .request(
            "state_getStorage",
            JsonRpcParams::Array(vec![
                "0x0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f".into(),
            ]),
        )
        .await
        .map_err(|err| format!("state_getStorage failed: {err:?}"))?;

    let parachain_id: u32 = Decode::decode(&mut &bytes[..])
        .map_err(|err| format!("parachain id decoding failed: {err:?}"))?;

    Ok(parachain_id)
}
