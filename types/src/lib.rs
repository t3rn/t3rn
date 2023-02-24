#![feature(box_syntax)]
#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::prelude::vec::Vec;

pub mod abi;
pub mod bid;
pub mod fsx;
pub mod interface;
#[cfg(test)]
pub mod mini_mock;
pub mod recode;
pub mod sfx;
pub mod sfx_abi;
pub mod standard;
pub mod to_abi;
pub mod to_filled_abi;
pub mod types;

pub type Bytes = Vec<u8>;
