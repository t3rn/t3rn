#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use sp_std::{marker::PhantomData, vec, vec::Vec};
use t3rn_primitives::threevm::{Precompile, EVM_RECODING_BYTE_SELECTOR};

pub struct PortalPrecompile<T: pallet_evm::Config>(PhantomData<T>);

// TODO: this is just the same as 3vm dispatch Right now
impl<T: pallet_evm::Config> EvmPrecompile for PortalPrecompile<T> {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        let _target_gas = handle.gas_limit();
        let _context = handle.context();
        let mut output = Vec::new();
        let callee = handle.context().caller.clone();

        // // TODO: assert the length is at least 2 bytes
        // if input.len() < 2 {
        //     return Err(
        //         ExitError::Other("PortalPrecompile input contained too little bytes".into()).into(),
        //     )
        // }

        let restructured_args =
            [&[EVM_RECODING_BYTE_SELECTOR][..], callee.as_bytes(), &input].concat();

        // TODO: dont hardcode me
        T::ThreeVm::invoke_raw(&70, &restructured_args, &mut output);

        // FIXME: always passes right now, needs error check
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn stub() {
        // 0x7901000000000000000000000000000000000000000a180001343434340000000000000000000000000000000000000000000000000000000000000000404b4c00000000006400000000000000000000000000000000000000000000000000000000000000000000
    }
}
