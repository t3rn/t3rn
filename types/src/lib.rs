#![cfg_attr(not(feature = "std"), no_std)]
use scale_info::prelude::vec::Vec;

pub mod abi;
pub mod side_effect;

pub type Bytes = Vec<u8>;
