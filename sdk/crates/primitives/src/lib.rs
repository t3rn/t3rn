#![cfg_attr(not(feature = "std"), no_std)]

use codec::Error as CodecError;

pub use scale_info::prelude::{boxed::Box, collections::BTreeMap, fmt::Debug, vec::Vec};

pub mod error;
pub mod signal;
pub mod state;
pub mod storage;
pub mod xc;

/// A function pointer for getting local state
pub const GET_STATE_FUNCTION_CODE: u32 = 2_8008_8008;
/// A function pointer for submitting side effects
pub const SUBMIT_FUNCTION_CODE: u32 = 3_8008_8008;
/// A function pointer for posting execution signals
pub const POST_SIGNAL_FUNCTION_CODE: u32 = 4_8008_8008;

/// The maximum amount of parameters we allow users to pass to a function
pub const MAX_PARAMETERS_IN_FUNCTION: usize = 16;

/// The maximum call length for a side effect, in bytes
pub const MAX_CALL_LEN: usize = 1024;

/// The ceiling max steps in execution, subject to change
pub const DEFAULT_MAX_STEPS_IN_EXECUTION: usize = 10;
