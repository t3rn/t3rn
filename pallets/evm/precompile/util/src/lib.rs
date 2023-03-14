#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

pub use pallet_3vm_evm_primitives::{
    Context, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use pallet_evm_precompile_3vm_dispatch::ThreeVmDispatch;
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
    ThreeVmDispatch,
}


// impl<R> PrecompileSet for LocalNetworkPrecompiles<R>
impl<T> PrecompileSet for Precompiles<T> where
T: pallet_evm::Config,
{
     fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {

        let address = handle.code_address();
        match address {
            // Ethereum:
            a if a == hash(&0) => Some(ECRecover::execute(handle)),
            a if a == hash(&1) => Some(Sha256::execute(handle)),
            a if a == hash(&2) => Some(Ripemd160::execute(handle)),
            a if a == hash(&3) => Some(Identity::execute(handle)),
            a if a == hash(&4) => Some(Modexp::execute(handle)),
            // Non-Frontier specific nor Ethereum:
            a if a == hash(&5) => Some(Sha3FIPS256::execute(handle)),
            a if a == hash(&6) => Some(Sha3FIPS512::execute(handle)),
            a if a == hash(&7) => Some(ECRecoverPublicKey::execute(handle)),
            // 3VM
            a if a == hash(&8) => Some(ThreeVmDispatch::<T>::execute(handle)),
            _ => None,
            
        }
    }

    fn is_precompile(&self, address: H160) -> bool {
        self.used_addresses().contains(&address)
    }
}

pub struct Precompiles<T> {
    _marker: PhantomData<T>,
    inner: BTreeMap<H160, KnownPrecompile>,
}

impl<T> Precompiles <T> {
    pub fn new(inner: BTreeMap<u64, KnownPrecompile>) -> Self {
        Self {
            inner: inner.into_iter().map(|(k, v)| (hash(&k), v)).collect(),
            _marker: PhantomData,
        }
    }

    pub fn used_addresses(&self) -> Vec<H160> {
        self.inner.keys().cloned().collect()
    }
}

// impl <T>PrecompileSet for Precompiles<T> {
//     fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
//         self.inner
//             .get(&handle.code_address())
//             .map(|precompile| precompile.execute(handle))
//     }

//     fn is_precompile(&self, address: H160) -> bool {
//         self.used_addresses().contains(&address)
//     }
// }

fn hash(a: &u64) -> H160 {
    H160::from_low_u64_be(*a)
}
