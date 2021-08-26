// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for pallet-xdns.

use super::*;
use crate::mock::{ExtBuilder, Test, XDNS};

#[test]
fn genesis_should_add_circuit_and_gateway_nodes() {
    let circuit_hash = <Test as frame_system::Config>::Hashing::hash(b"circ");
    let gateway_hash = <Test as frame_system::Config>::Hashing::hash(b"gate");

    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(XDNSRegistry::<Test>::iter().count(), 2);
        assert!(XDNSRegistry::<Test>::get(circuit_hash).is_some());
        assert!(XDNSRegistry::<Test>::get(gateway_hash).is_some());
    });
}

#[test]
fn should_add_a_new_xdns_record_if_it_doesnt_exist() {
    ExtBuilder::default().build().execute_with(|| {
        XDNS::add_new_xdns_record();
    });
}
