use sp_std::vec::Vec;
use codec::{Encode, Decode, MaxEncodedLen};
use frame_support::pallet_prelude::TypeInfo;
pub use ethereum_types::{H256, U256, H160};
pub trait BinanceFV<T: frame_system::Config> {

    fn init_bridge_instance (
        encoded_header: Vec<u8>
    ) -> Result<(), &'static str>;

    fn check_inclusion(
        enc_receipt: Vec<u8>,
        enc_proof: Option<Vec<u8>>,
        enc_block_hash: Vec<u8>
    ) -> Result<(), &'static str>;

}
#[derive(Debug, Clone, Decode, PartialEq, Eq)]
pub struct Topics(pub Vec<H256>);

#[derive(Debug, Clone, Eq, PartialEq, Decode)]
pub struct Proof {
    pub bytes: Vec<Vec<u8>>,
    pub index: Vec<u8>
}

#[derive(Debug, Clone, Eq, PartialEq, Decode)]
pub struct Receipt {
    pub status: bool,
    pub cumulative_gas_used: U256,
    pub logs_bloom: LogsBloom,
    pub logs: Vec<Event>,
}

#[derive(Debug, Clone, Eq, PartialEq, Decode)]
pub struct Event {
    pub address: H160,
    pub topics: Topics,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub struct LogsBloom(pub [u8; 256]);