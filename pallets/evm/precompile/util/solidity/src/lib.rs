#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use pallet_3vm_evm_primitives::{
    ExitError, ExitRevert, ExitSucceed, PrecompileFailure, PrecompileOutput,
};
use sp_std::{borrow, borrow::ToOwned};

pub mod costs;
pub mod data;
pub mod handle;
pub mod modifier;
pub mod substrate;

#[cfg(test)]
pub mod testing;

/// Alias for Result returning an EVM precompile error.
pub type EvmResult<T = ()> = Result<T, PrecompileFailure>;

/// Return an error with provided (static) text.
/// Using the `revert` function of `Gasometer` is preferred as erroring
/// consumed all the gas limit and the error message is not easily
/// retrievable.
#[must_use]
pub fn error<T: Into<borrow::Cow<'static, str>>>(text: T) -> PrecompileFailure {
    PrecompileFailure::Error {
        exit_status: ExitError::Other(text.into()),
    }
}

#[must_use]
pub fn revert(output: impl AsRef<[u8]>) -> PrecompileFailure {
    PrecompileFailure::Revert {
        exit_status: ExitRevert::Reverted,
        output: output.as_ref().to_owned(),
    }
}

#[must_use]
pub fn succeed(output: impl AsRef<[u8]>) -> PrecompileOutput {
    PrecompileOutput {
        exit_status: ExitSucceed::Returned,
        output: output.as_ref().to_owned(),
    }
}
