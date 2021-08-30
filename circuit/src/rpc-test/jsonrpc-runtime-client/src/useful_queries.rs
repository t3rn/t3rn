use crate::polkadot_like_chain::PolkadotLike;
use codec::Encode;
use num_traits::Zero;
use relay_substrate_client::Client as SubstrateClient;

/// Get first header of Substrate network
pub async fn get_first_header(
    sub_client: &SubstrateClient<PolkadotLike>,
) -> Result<Vec<u8>, String> {
    let initial_header = sub_client.header_by_number(Zero::zero()).await;
    initial_header
        .map(|header| header.encode())
        .map_err(|error| format!("Error reading Substrate genesis header: {:?}", error))
}
