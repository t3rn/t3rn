use crate::ProofTriePointer;
use snowbridge_core::Verifier;
use sp_std::vec::Vec;
use sp_trie::StorageProof;

pub trait CircuitPortal<T: frame_system::Config> {
    type EthVerifier: Verifier;

    fn confirm_inclusion(
        gateway_id: [u8; 4],
        _encoded_message: Vec<u8>,
        trie_type: ProofTriePointer,
        maybe_block_hash: Option<Vec<u8>>,
        maybe_proof: Option<Vec<Vec<u8>>>,
    ) -> Result<(), &'static str>;

     fn confirm_parachain(
        gateway_id: [u8; 4],
        block_hash: Vec<u8>,
        proof: StorageProof,
    ) -> Result<Vec<u8>, &'static str>;
}