#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_3vm_evm_primitives::{
    Context, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use pallet_3vm_evm_primitives::{ExitError, PrecompileFailure};
pub use pallet_evm_precompile_modexp::Modexp;
pub use pallet_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
pub use pallet_evm_precompile_simple::{
    ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256,
};
use portal_precompile::PortalPrecompile;
use sp_core::H160;
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, vec::Vec};

pub enum KnownPrecompile<T: pallet_3vm_evm::Config> {
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
    // T3rn precompiles:
    Portal,
    Noop(T),
}

impl<T: pallet_3vm_evm::Config> KnownPrecompile<T> {
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
            KnownPrecompile::Portal => PortalPrecompile::<T>::execute(handle),
            KnownPrecompile::Noop(_) => PrecompileResult::Err(PrecompileFailure::from(
                ExitError::Other("Noop precompile".into()),
            )),
        }
    }
}

pub struct Precompiles<T: pallet_3vm_evm::Config> {
    pub inner: BTreeMap<H160, KnownPrecompile<T>>,
    phantom: PhantomData<T>,
}

impl<T: pallet_3vm_evm::Config> Precompiles<T> {
    pub fn new(inner: BTreeMap<u64, KnownPrecompile<T>>) -> Self {
        Self {
            inner: inner.into_iter().map(|(k, v)| (hash(&k), v)).collect(),
            phantom: Default::default(),
        }
    }

    pub fn used_addresses(&self) -> Vec<H160> {
        self.inner.keys().cloned().collect()
    }
}

impl<T: pallet_3vm_evm::Config> PrecompileSet for Precompiles<T> {
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
