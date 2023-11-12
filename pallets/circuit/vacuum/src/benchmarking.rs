// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

#![cfg(feature = "runtime-benchmarks")]
// use crate::{Pallet as Vacuum, *};

use super::*;

use sp_std::*;

use t3rn_primitives::{ExecutionVendor, GatewayVendor, SpeedMode};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{EventRecord, Pallet as System, RawOrigin};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = System::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn register_gateway_under_xbi_vendor<T: Config>(gateway_id: [u8; 4], caller: T::AccountId) {
    // Add new Gateway using T::Config::Xdns
    <T::Xdns>::add_new_gateway(
        gateway_id,
        GatewayVendor::XBI,
        ExecutionVendor::Substrate,
        Codec::Scale,
        Some(caller.clone()),
        Some(caller),
        vec![
            (*b"tran", Some(2)),
            (*b"tass", Some(4)),
            (*b"swap", Some(3)),
            (*b"aliq", Some(3)),
            (*b"cevm", Some(10)),
            (*b"wasm", Some(10)),
        ],
    )
    .expect("Gateway should register to XBI in Vacuum::runtime_benchmarks")
}

benchmarks! {
    read_order_status {
        let caller: T::AccountId = whitelisted_caller();

        register_gateway_under_xbi_vendor::<T>([4u8; 4], caller.clone());

        let xtx_id_zero_32b = sp_core::H256::zero();

        let xtx_id_zero: T::Hash = T::Hash::decode(&mut &xtx_id_zero_32b.as_bytes()[..]).expect("XTX hash should decode from 32b");

    }: _(RawOrigin::Signed(caller), xtx_id_zero)
}