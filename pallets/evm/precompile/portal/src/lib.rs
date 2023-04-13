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

        // TODO: assert the length is at least 2 bytes
        if input.len() < 2 {
            return Err(
                ExitError::Other("PortalPrecompile input contained too little bytes".into()).into(),
            )
        }

        // TODO; assert on first byte that it is indeed portal
        let precompile_selector_index = input[0];

        // TODO: add the evm selector here
        let args_with_evm_selector = vec![&[EVM_RECODING_BYTE_SELECTOR][..], &input[1..]].concat();

        T::ThreeVm::invoke_raw(
            &precompile_selector_index,
            &args_with_evm_selector,
            &mut output,
        );

        // Hmm, maybe we just recode the entire thing

        // FIXME: always passes right now, needs error check
        if !output.is_empty() {
            Ok(PrecompileOutput {
                exit_status: ExitSucceed::Returned,
                output,
            })
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
