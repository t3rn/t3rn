#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_3vm_evm_primitives::{
    Context, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
pub use pallet_evm_precompile_modexp::Modexp;
pub use pallet_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
pub use pallet_evm_precompile_simple::{
    ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256,
};

use sp_core::H160;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

pub enum KnownPrecompile {
    // Ethereum precompiles:
    ECRecover,
    Sha256,
    Ripemd160,
    Identity,
    Modexp,
    // Non-Frontier specific nor Ethereum precompiles:
    Sha3FIPS256,
    Sha3FIPS512,
    ECRecoverPublicKey,
    // 3VM precompiles:
}

impl KnownPrecompile {
    pub fn execute(&self, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        match self {
            // Ethereum:
            KnownPrecompile::ECRecover => ECRecover::execute(handle),
            KnownPrecompile::Sha256 => Sha256::execute(handle),
            KnownPrecompile::Ripemd160 => Ripemd160::execute(handle),
            KnownPrecompile::Identity => Identity::execute(handle),
            KnownPrecompile::Modexp => Modexp::execute(handle),
            // Non-Frontier specific nor Ethereum:
            KnownPrecompile::Sha3FIPS256 => Sha3FIPS256::execute(handle),
            KnownPrecompile::Sha3FIPS512 => Sha3FIPS512::execute(handle),
            KnownPrecompile::ECRecoverPublicKey => ECRecoverPublicKey::execute(handle),
        }
    }
}

pub struct Precompiles {
    inner: BTreeMap<H160, KnownPrecompile>,
}

impl Precompiles {
    pub fn new(inner: BTreeMap<u64, KnownPrecompile>) -> Self {
        Self {
            inner: inner.into_iter().map(|(k, v)| (hash(&k), v)).collect(),
        }
    }

    pub fn used_addresses(&self) -> Vec<H160> {
        self.inner.keys().cloned().collect()
    }
}

impl PrecompileSet for Precompiles {
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        self.inner
            .get(&handle.code_address())
            .map(|precompile| precompile.execute(handle))
    }

    fn is_precompile(&self, address: H160) -> bool {
        self.used_addresses().contains(&address)
    }
}

fn hash(a: &u64) -> H160 {
    H160::from_low_u64_be(*a)
}
