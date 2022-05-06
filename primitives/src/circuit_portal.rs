use snowbridge_core::Verifier;
use sp_std::vec::Vec;

pub trait CircuitPortal<T: frame_system::Config> {
    type EthVerifier: Verifier;

    fn confirm_event_inclusion(
        gateway_id: [u8; 4],
        encoded_event: Vec<u8>,
        maybe_proof: Option<Vec<Vec<u8>>>,
        maybe_block_hash: Option<Vec<u8>>,
    ) -> Result<(), &'static str>;
}
