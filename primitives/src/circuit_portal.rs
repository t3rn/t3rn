use snowbridge_core::Verifier;
use sp_std::vec::Vec;

pub trait CircuitPortal<T: frame_system::Config> {
    type EthVerifier: Verifier;

    fn confirm_event_inclusion(
        gateway_id: [u8; 4],
        encoded_event: Vec<u8>,
        encoded_submission_target_height: Vec<u8>,
        maybe_proof: Option<Vec<Vec<u8>>>,
        maybe_block_hash: Option<Vec<u8>>,
    ) -> Result<(), &'static str>;

    fn read_cmp_latest_target_height(
        gateway_id: [u8; 4],
        gateway_block_hash: Option<Vec<u8>>,
        maybe_cmp_height: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, &'static str>;
}
