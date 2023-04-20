#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::threevm::{Precompile, EVM_RECODING_BYTE_SELECTOR, PORTAL};

pub struct PortalPrecompile<T: pallet_evm::Config>(PhantomData<T>);

impl<T: pallet_evm::Config> EvmPrecompile for PortalPrecompile<T> {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        let _target_gas = handle.gas_limit();
        let _context = handle.context();
        let mut output = Vec::new();

        let restructured_args =
            [&[EVM_RECODING_BYTE_SELECTOR][..], callee.as_bytes(), &input].concat();

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
                exit_status: ExitError::Other("invalid output".into()),
            })
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn stub() {}
}
