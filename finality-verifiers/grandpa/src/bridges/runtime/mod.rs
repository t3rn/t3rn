// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Primitives that may be used at (bridges) runtime level.

use codec::Encode;
use sp_core::hash::H256;
use sp_io::hashing::blake2_256;
use sp_std::convert::TryFrom;

pub use chain::{BlockNumberOf, Chain, HashOf, HasherOf, HeaderOf};
pub use storage_proof::{Error as StorageProofError, StorageProofChecker};

#[cfg(feature = "std")]
pub use storage_proof::craft_valid_storage_proof;

mod chain;
mod storage_proof;

pub type ChainId = [u8; 4];
