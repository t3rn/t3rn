use sp_std::vec::Vec;

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
