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

pub struct VacuumPrecompile<T>(PhantomData<T>);

pub enum VacuumAction {
    VacuumOrder = 90,
    Vacuum3DOrder = 91,
    VacuumConfirm = 92,
    VacuumSubmitCorrectnessProof = 93,
    VacuumSubmitFaultProof = 94,
}

impl<T> EvmPrecompile for VacuumPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config + frame_system::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
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

        // Assume first byte of input is the action selector
        let action = match restructured_args.first() {
            Some(byte) => match byte {
                0 => VacuumAction::VacuumOrder,
                1 => VacuumAction::Vacuum3DOrder,
                2 => VacuumAction::VacuumConfirm,
                3 => VacuumAction::VacuumSubmitCorrectnessProof,
                4 => VacuumAction::VacuumSubmitFaultProof,
                _ =>
                    return Err(PrecompileFailure::Error {
                        exit_status: ExitError::Other("Invalid action selector".into()),
                    }),
            },
            None =>
                return Err(PrecompileFailure::Error {
                    exit_status: ExitError::Other("Empty buffer".into()),
                }),
        };

        T::ThreeVm::invoke_raw(&(action as u8), &restructured_args, &mut output);

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
