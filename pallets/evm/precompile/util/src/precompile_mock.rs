use crate::{IsPrecompileResult, PrecompileHandle, PrecompileSet};
use frame_support::traits::Currency;
use pallet_3vm_evm::{ExitError, PrecompileFailure};
use pallet_3vm_evm_primitives::{Precompile, PrecompileResult};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use portal_precompile::PortalPrecompile;
use precompile_util_solidity::data::EvmData;
use sp_core::H160;
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, vec::Vec};
use tokens_precompile::TokensPrecompile;

pub enum KnownPrecompile<T: pallet_3vm_evm::Config + pallet_assets::Config + frame_system::Config>
where
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
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
    Tokens,
    Noop(T),
}

impl<T: pallet_3vm_evm::Config + pallet_assets::Config + frame_system::Config> KnownPrecompile<T>
where
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    pub fn execute(&self, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        match self {
            // Ethereum:
            KnownPrecompile::ECRecover => <ECRecover as Precompile>::execute(handle),
            KnownPrecompile::Sha256 => <Sha256 as Precompile>::execute(handle),
            KnownPrecompile::Ripemd160 => <Ripemd160 as Precompile>::execute(handle),
            KnownPrecompile::Identity => <Identity as Precompile>::execute(handle),
            KnownPrecompile::Modexp => <Modexp as Precompile>::execute(handle),
            // Non-Frontier specific nor Ethereum:
            KnownPrecompile::Sha3FIPS256 => <Sha3FIPS256 as Precompile>::execute(handle),
            KnownPrecompile::Sha3FIPS512 => <Sha3FIPS512 as Precompile>::execute(handle),
            KnownPrecompile::ECRecoverPublicKey =>
                <ECRecoverPublicKey as Precompile>::execute(handle),
            KnownPrecompile::Portal => PortalPrecompile::<T>::execute(handle),
            KnownPrecompile::Tokens => TokensPrecompile::<T>::execute(handle),
            KnownPrecompile::Noop(_) => PrecompileResult::Err(PrecompileFailure::from(
                ExitError::Other("Noop precompile".into()),
            )),
            _ => PrecompileResult::Err(PrecompileFailure::from(ExitError::Other(
                "Unknown precompile".into(),
            ))),
        }
    }
}

pub struct MockPrecompileSet<
    T: pallet_3vm_evm::Config + pallet_assets::Config + frame_system::Config,
> where
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    pub inner: BTreeMap<H160, KnownPrecompile<T>>,
    phantom: PhantomData<T>,
}

impl<T: pallet_3vm_evm::Config + pallet_assets::Config + frame_system::Config> MockPrecompileSet<T>
where
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    pub fn new(inner: BTreeMap<H160, KnownPrecompile<T>>) -> Self {
        Self {
            inner: inner.into_iter().map(|(k, v)| (k, v)).collect(),
            phantom: Default::default(),
        }
    }

    pub fn used_addresses(&self) -> Vec<H160> {
        self.inner.keys().cloned().collect()
    }
}

impl<T: pallet_3vm_evm::Config + pallet_assets::Config + frame_system::Config> PrecompileSet
    for MockPrecompileSet<T>
where
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_3vm_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_3vm_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        self.inner
            .get(&handle.code_address())
            .map(|precompile| precompile.execute(handle))
    }

    /// Check if the given address is a precompile. Should only be called to
    /// perform the check while not executing the precompile afterward, since
    /// `execute` already performs a check internally.
    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        IsPrecompileResult::Answer {
            is_precompile: self.used_addresses().contains(&address),
            extra_cost: 0,
        }
    }
}
