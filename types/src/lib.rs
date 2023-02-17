#![feature(box_syntax)]
#![cfg_attr(not(feature = "std"), no_std)]
use scale_info::prelude::vec::Vec;

pub mod abi;
pub mod bid;
pub mod fsx;
pub mod interface;
pub mod sfx;
pub mod standard;
// pub mod standard2;
// pub mod vendor;

pub type Bytes = Vec<u8>;
