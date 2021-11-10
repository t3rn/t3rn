use crate::polkadot_like_chain::PolkadotLike;
use codec::{Decode, Encode};
use frame_metadata::v14::RuntimeMetadataV14;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams};
use num_traits::Zero;
use relay_substrate_client::Client as SubstrateClient;
use sp_core::Bytes;

/// Get first header of Substrate network
pub async fn get_first_header(
    sub_client: &SubstrateClient<PolkadotLike>,
) -> Result<Vec<u8>, String> {
    let initial_header = sub_client.header_by_number(Zero::zero()).await;
    initial_header
        .map(|header| header.encode())
        .map_err(|error| format!("Error reading Substrate genesis header: {:?}", error))
}

pub async fn get_metadata(
    sub_client: &SubstrateClient<PolkadotLike>,
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
