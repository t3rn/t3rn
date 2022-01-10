use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;

/// Polkadot header id.
// pub type HeaderId = relay_utils::HeaderId<bp_polkadot_core::Hash, bp_polkadot_core::BlockNumber>;
/// Polkadot header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_polkadot_core::Header>;

/// Polkadot chain definition
#[derive(Debug, Clone, Copy)]
pub struct PolkadotLike;

impl ChainBase for PolkadotLike {
    type BlockNumber = bp_polkadot_core::BlockNumber;
    type Hash = bp_polkadot_core::Hash;
    type Hasher = bp_polkadot_core::Hasher;
    type Header = bp_polkadot_core::Header;
}

impl Chain for PolkadotLike {
    const NAME: &'static str = "Polkadot";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

    type AccountId = bp_polkadot_core::AccountId;
    type Index = bp_polkadot_core::Nonce;
    type SignedBlock = bp_polkadot_core::SignedBlock;
    type Call = ();
}
