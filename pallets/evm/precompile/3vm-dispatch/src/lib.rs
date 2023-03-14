#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;
use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use t3rn_primitives::threevm::Precompile;

// TODO: build this for the next phase of evm artifacts.
pub struct ThreeVmDispatch<T> {
    _marker: PhantomData<T>,
}

impl<T> EvmPrecompile for ThreeVmDispatch<T>
where
    T: pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        let precompilePointer = handle.input();
         // cut pointer from input // cut byte
        let _target_gas = handle.gas_limit();
        let _context = handle.context();

        let mut output = Vec::new();

        // TODO;  here replace pointer with the one from handle.input()
        T::ThreeVm::invoke_raw(&55_u8, input, &mut output); // TODO: dummy ptr for now

        if !output.is_empty() {
            Ok(PrecompileOutput {
                exit_status: ExitSucceed::Stopped,
                output,
            })
        } else {
            Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("invalid output".into()),
            })
        }
    }
}
