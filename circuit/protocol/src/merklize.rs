use sp_core::H256;

use sp_io::hashing::{blake2_256, keccak_256};
use t3rn_primitives::abi::HasherAlgo;

/// Check ethereum merkle proof.
/// Returns Ok(computed-root) if check succeeds.
/// Returns Err(computed-root) if check fails.
pub fn check_merkle_proof<T: AsRef<[u8]>>(
    expected_root: H256,
    items: impl Iterator<Item = T>,
    hasher_alg: HasherAlgo,
) -> Result<H256, H256> {
    let computed_root = match hasher_alg {
        HasherAlgo::Blake2 => compute_merkle_root_blake2(items),
        HasherAlgo::Keccak256 => compute_merkle_root_keccak256(items),
    };

    if computed_root == expected_root {
        Ok(computed_root)
    } else {
        Err(computed_root)
    }
}

/// Compute ethereum merkle root.
pub fn compute_merkle_root_keccak256<T: AsRef<[u8]>>(items: impl Iterator<Item = T>) -> H256 {
    struct Keccak256Hasher;

    impl hash_db::Hasher for Keccak256Hasher {
        type Out = H256;
        type StdHasher = plain_hasher::PlainHasher;
        const LENGTH: usize = 32;
        fn hash(x: &[u8]) -> Self::Out {
            keccak_256(x).into()
        }
    }

    triehash::ordered_trie_root::<Keccak256Hasher, _>(items)
}

/// Compute ethereum merkle root.
pub fn compute_merkle_root_blake2<T: AsRef<[u8]>>(items: impl Iterator<Item = T>) -> H256 {
    struct Blake2Hasher;

    impl hash_db::Hasher for Blake2Hasher {
        type Out = H256;
        type StdHasher = plain_hasher::PlainHasher;
        const LENGTH: usize = 32;
        fn hash(x: &[u8]) -> Self::Out {
            blake2_256(x).into()
        }
    }

    triehash::ordered_trie_root::<Blake2Hasher, _>(items)
}
