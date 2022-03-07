// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(not(feature = "std"))]
pub use alloc::{boxed::Box, collections::btree_map::BTreeMap, vec, vec::Vec};

#[cfg(feature = "std")]
pub use std::{boxed::Box, collections::btree_map::BTreeMap, vec, vec::Vec};
