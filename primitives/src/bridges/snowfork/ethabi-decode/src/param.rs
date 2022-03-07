// Copyright 2015-2020 Parity Technologies
// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be 
// copied, modified, or distributed except according to those terms.

use crate::std::{Box, Vec};


/// Event param specification.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
	/// Param type.
	pub kind: ParamKind,
	/// Indexed flag. If true, param is used to build block bloom.
	pub indexed: bool,
}

/// Function and event param types.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
	/// Address.
	Address,
	/// Bytes.
	Bytes,
	/// Signed integer.
	Int(usize),
	/// Unsigned integer.
	Uint(usize),
	/// Boolean.
	Bool,
	/// String.
	String,
	/// Array of unknown size.
	Array(Box<ParamKind>),
	/// Vector of bytes with fixed size.
	FixedBytes(usize),
	/// Array with fixed size.
	FixedArray(Box<ParamKind>, usize),
	/// Tuple containing different types
	Tuple(Vec<Box<ParamKind>>),
}

impl ParamKind {
	/// returns whether a zero length byte slice (`0x`) is
	/// a valid encoded form of this param type
	pub fn is_empty_bytes_valid_encoding(&self) -> bool {
		match self {
			ParamKind::FixedBytes(len) => *len == 0,
			ParamKind::FixedArray(_, len) => *len == 0,
			_ => false,
		}
	}

	/// returns whether a ParamKind is dynamic
	/// used to decide how the ParamKind should be encoded
	pub fn is_dynamic(&self) -> bool {
		match self {
			ParamKind::Bytes | ParamKind::String | ParamKind::Array(_) => true,
			ParamKind::FixedArray(elem_type, _) => elem_type.is_dynamic(),
			ParamKind::Tuple(params) => params.iter().any(|param| param.is_dynamic()),
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::ParamKind;

	#[test]
	fn test_is_dynamic() {
		assert_eq!(ParamKind::Address.is_dynamic(), false);
		assert_eq!(ParamKind::Bytes.is_dynamic(), true);
		assert_eq!(ParamKind::FixedBytes(32).is_dynamic(), false);
		assert_eq!(ParamKind::Uint(256).is_dynamic(), false);
		assert_eq!(ParamKind::Int(64).is_dynamic(), false);
		assert_eq!(ParamKind::Bool.is_dynamic(), false);
		assert_eq!(ParamKind::String.is_dynamic(), true);
		assert_eq!(ParamKind::Array(Box::new(ParamKind::Bool)).is_dynamic(), true);
		assert_eq!(ParamKind::FixedArray(Box::new(ParamKind::Uint(256)), 2).is_dynamic(), false);
		assert_eq!(ParamKind::FixedArray(Box::new(ParamKind::String), 2).is_dynamic(), true);
		assert_eq!(ParamKind::FixedArray(Box::new(ParamKind::Array(Box::new(ParamKind::Bool))), 2).is_dynamic(), true);
	}
}
