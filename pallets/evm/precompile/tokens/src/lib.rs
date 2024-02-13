#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::threevm::Precompile;

pub struct TokensPrecompile<T: pallet_evm::Config>(PhantomData<T>);

impl<T: pallet_evm::Config> EvmPrecompile for TokensPrecompile<T> {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        let _target_gas = handle.gas_limit();
        let _context = handle.context();
        Err(PrecompileFailure::Error {
            exit_status: ExitError::Other("Not implemented".into()),
        })
    }
}
