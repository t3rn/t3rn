use crate::Runtime;

pub fn generate_precompile_set(
) -> sp_std::vec::Vec<(sp_core::H160, evm_precompile_util::KnownPrecompile<Runtime>)> {
    // Generate intiial precopile set
    let mut precompile_set = sp_std::vec![
        (
            sp_core::H160([0u8; 20]),
            evm_precompile_util::KnownPrecompile::ECRecover
        ),
        (
            sp_core::H160([1u8; 20]),
            evm_precompile_util::KnownPrecompile::Sha256
        ),
        (
            sp_core::H160([2u8; 20]),
            evm_precompile_util::KnownPrecompile::Ripemd160
        ),
        (
            sp_core::H160([3u8; 20]),
            evm_precompile_util::KnownPrecompile::Identity
        ),
        (
            sp_core::H160([4u8; 20]),
            evm_precompile_util::KnownPrecompile::Modexp
        ),
        (
            sp_core::H160([5u8; 20]),
            evm_precompile_util::KnownPrecompile::Sha3FIPS256
        ),
        (
            sp_core::H160([6u8; 20]),
            evm_precompile_util::KnownPrecompile::Sha3FIPS512
        ),
        (
            sp_core::H160([7u8; 20]),
            evm_precompile_util::KnownPrecompile::ECRecoverPublicKey
        ),
        (
            sp_core::H160([8u8; 20]),
            evm_precompile_util::KnownPrecompile::Portal
        ),
    ];
    // Link the first 10000 asset ids addresses to TokensPrecompile
    // TODO: automate tokens precompile setup so it tokens does not need to be manually added
    for index in (0..10000) {
        precompile_set.push((
            t3rn_primitives::threevm::get_tokens_precompile_address(index),
            evm_precompile_util::KnownPrecompile::Tokens,
        ));
    }
    precompile_set
}
