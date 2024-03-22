#![cfg_attr(not(feature = "std"), no_std)]
#![feature(assert_matches)]
extern crate alloc;

pub use pallet_3vm_evm_primitives::{
    ExitError, ExitRevert, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use sp_std::{borrow, borrow::ToOwned};

pub mod costs;
pub mod data;
pub mod handle;
pub mod modifier;
pub mod precompile_set;
pub mod substrate;

//#[cfg(feature = "precompile-testing")]
pub mod testing;

/// Alias for Result returning an EVM precompile error.
pub type EvmResult<T = ()> = Result<T, PrecompileFailure>;

/// Trait similar to `fp_evm::Precompile` but with a `&self` parameter to manage some
/// state (this state is only kept in a single transaction and is lost afterward).
pub trait StatefulPrecompile {
    /// Instanciate the precompile.
    /// Will be called once when building the PrecompileSet at the start of each
    /// Ethereum transaction.
    fn new() -> Self;

    /// Execute the precompile with a reference to its state.
    fn execute(&self, handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput>;
}

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
