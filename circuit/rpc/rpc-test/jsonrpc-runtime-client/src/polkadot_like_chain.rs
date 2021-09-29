use frame_support::weights::IdentityFee;
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
    type AccountId = bp_polkadot_core::AccountId;
    type Balance = bp_polkadot_core::Balance;
    type Index = bp_polkadot_core::Index;
    type Signature = bp_polkadot_core::Signature;
}

impl Chain for PolkadotLike {
    const NAME: &'static str = "Polkadot";
    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

    const STORAGE_PROOF_OVERHEAD: u32 = 0;
    const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = 0;
    type SignedBlock = bp_polkadot_core::SignedBlock;
    type Call = ();
    type WeightToFee = IdentityFee<bp_polkadot_core::Balance>;
}
