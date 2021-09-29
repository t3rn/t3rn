// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! A module that implements instrumented code cache.
//!
//! - In order to run contract code we need to instrument it with gas metering.
//! To do that we need to provide the schedule which will supply exact gas costs values.
//! We cache this code in the storage saving the schedule version.
//! - Before running contract code we check if the cached code has the schedule version that
//! is equal to the current saved schedule.
//! If it is equal then run the code, if it isn't reinstrument with the current schedule.
//! - When we update the schedule we want it to have strictly greater version than the current saved one:
//! this guarantees that every instrumented contract code in cache cannot have the version equal to the current one.
//! Thus, before executing a contract it should be reinstrument with new schedule.

use crate::wasm::PrefabWasmModule;
use crate::{CodeHash, CodeStorage, Schedule, Trait};
use codec::{Decode, Encode};
use frame_support::StorageMap;
use sp_std::prelude::*;

/// Put code in the storage. The hash of code is used as a key and is returned
/// as a result of this function.
///
/// This function instruments the given code and caches it in the storage.
pub fn save<T: Trait>(
    _original_code: Vec<u8>,
    _schedule: &Schedule,
) -> Result<CodeHash<T>, &'static str> {
    unimplemented!()
}

/// Load code with the given code hash.
///
/// If the module was instrumented with a lower version of schedule than
/// the current one given as an argument, then this function will perform
/// re-instrumentation and update the cache in the storage.
pub fn load<T: Trait>(
    code_hash: &CodeHash<T>,
    _schedule: &Schedule,
) -> Result<PrefabWasmModule, &'static str> {
    let prefab_module = <CodeStorage<T>>::get(code_hash).ok_or_else(|| "code is not found")?;
    match Decode::decode(&mut prefab_module.encode().as_slice()) {
        Ok(decoded) => Ok(decoded),
        Err(_err) => Err("Can't decode stored contract."),
    }
}
