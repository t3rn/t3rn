#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use frame_support::{sp_runtime::app_crypto::sp_core, traits::Currency};
use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::{
    threevm::{Precompile, PORTAL},
    T3rnCodec,
};

use precompile_util_solidity::data::EvmData;

pub struct PortalPrecompile<T>(PhantomData<T>);

impl<T> EvmPrecompile for PortalPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        let _target_gas = handle.gas_limit();
        let _context = handle.context();
        let mut output = Vec::new();
        let callee = handle.context().caller;

        let restructured_args = [&[T3rnCodec::Rlp.into()][..], callee.as_bytes(), input].concat();

        T::ThreeVm::invoke_raw(&PORTAL, &restructured_args, &mut output);

        if let Some(result_byte) = output.first() {
            if *result_byte == 0 {
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output,
                })
            } else {
                Err(PrecompileFailure::Error {
                    exit_status: ExitError::Other("invalid output".into()),
                })
            }
        } else {
            Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Empty buffer".into()),
            })
        }
    }
}
