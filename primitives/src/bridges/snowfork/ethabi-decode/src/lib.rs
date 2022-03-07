// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod decoder;
mod encoder;
mod event;
mod param;
mod std;
mod token;
mod util;

pub use crate::{
	decoder::decode,
	encoder::{encode, encode_function},
	event::Event,
	param::{Param, ParamKind},
	token::Token,
};

#[derive(Debug)]
pub enum Error {
	/// Invalid entity such as a bad function name.
	InvalidName,
	/// Invalid data.
	InvalidData
}

/// ABI Address
pub use ethereum_types::Address;

/// ABI word.
pub type Word = [u8; 32];

/// ABI Int and UInt
pub use ethereum_types::U256;

/// Hash
pub use ethereum_types::H256;
