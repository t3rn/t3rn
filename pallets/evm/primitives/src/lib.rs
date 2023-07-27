#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unused_crate_dependencies)]

/// This is a thin wrapper around fp_evm, exposing some additional traits we need for self execution
pub use frontier_fp_evm::*;

pub mod traits;
