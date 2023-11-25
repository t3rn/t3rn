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
use crate::Pallet as Vacuum;

use super::*;

use sp_std::*;

use t3rn_primitives::{ExecutionVendor, GatewayVendor, SpeedMode, SubstrateToken, TokenInfo};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::{EventRecord, Pallet as System, RawOrigin};
use pallet_xdns::Pallet as XDNS;
use t3rn_primitives::monetary::EXISTENTIAL_DEPOSIT;

fn assume_last_xtx_event<T: Config>() -> T::Hash {
    let events = System::<T>::events();
    // let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    let event_record = &events[events.len() - 1];
    let event_topics = &event_record.topics;
    *event_topics
        .first()
        .expect("Expect System Event to emit XTX ID")
}

use frame_support::traits::OriginTrait;

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
    .expect("Gateway should register to XBI in Vacuum::runtime_benchmarks");

    // Advance 100 blocks ahead and call global on_initialize loop in order to process_all_verifier_overviews and set Gateway as active
    System::<T>::set_block_number(100u8.into());
    XDNS::<T>::check_for_manual_verifier_overview_process(100u8.into());

    // Advance next 100 blocks ahead and call global on_initialize loop in order to process_all_verifier_overviews and set Gateway as active
    System::<T>::set_block_number(200u8.into());
    XDNS::<T>::check_for_manual_verifier_overview_process(200u8.into());
}

const NATIVE_ASSET: u32 = 0;
fn submit_one_single_order_as_tran<T: Config>(
    gateway_id: [u8; 4],
    caller: T::AccountId,
    order_amount: BalanceOf<T>,
    max_reward: BalanceOf<T>,
    speed_mode: SpeedMode,
) {
    let insurance = max_reward / BalanceOf::<T>::from(10u8);
    // Mint enough of local currency for caller as requester and executor
    <T as Config>::Currency::deposit_creating(
        &caller,
        insurance.clone()
            + max_reward
            + order_amount
            + BalanceOf::<T>::from(EXISTENTIAL_DEPOSIT as u8),
    );

    assert_ok!(Vacuum::<T>::single_order(
        RawOrigin::Signed(caller.clone()).into(),
        gateway_id,
        NATIVE_ASSET,
        order_amount,
        NATIVE_ASSET,
        max_reward,
        insurance,
        caller.clone(),
        speed_mode
    ));
}

benchmarks! {

    single_order {
        let caller: T::AccountId = whitelisted_caller();
        let gateway_id: TargetId = [4u8; 4];
        let order_amount = BalanceOf::<T>::from(100u8);
        let max_reward = BalanceOf::<T>::from(200u8);
        let insurance = max_reward / BalanceOf::<T>::from(10u8);
        register_gateway_under_xbi_vendor::<T>(gateway_id.clone(), caller.clone());

        // Mint enough of local currency for caller as requester and executor
        <T as Config>::Currency::deposit_creating(&caller, insurance.clone() + max_reward + order_amount + BalanceOf::<T>::from(EXISTENTIAL_DEPOSIT as u8));
    }: _(RawOrigin::Signed(caller.clone()), gateway_id, NATIVE_ASSET, order_amount, NATIVE_ASSET, max_reward, insurance, caller.clone(), SpeedMode::Fast)

    read_order_status {
        let caller: T::AccountId = whitelisted_caller();
        let gateway_id: TargetId = [4u8; 4];
        let order_amount = BalanceOf::<T>::from(100u8);
        let reward_amount = BalanceOf::<T>::from(200u8);

        register_gateway_under_xbi_vendor::<T>(gateway_id.clone(), caller.clone());

        submit_one_single_order_as_tran::<T>(
            gateway_id,
            caller.clone(),
            order_amount,
            reward_amount,
            SpeedMode::Fast,
        );

        let xtx_id = assume_last_xtx_event::<T>();
    }: _(RawOrigin::Signed(caller), xtx_id)
}
