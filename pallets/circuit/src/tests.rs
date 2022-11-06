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

//! Runtime utilities
use circuit_mock_runtime::*;
use circuit_runtime_pallets::pallet_circuit::state::*;

use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, SignalKind},
    xc::*,
};

use codec::{Decode, Encode};
use frame_support::{
    assert_err, assert_noop, assert_ok, dispatch::PostDispatchInfo, traits::Currency,
};

use frame_system::{pallet_prelude::OriginFor, EventRecord, Phase};

pub use pallet_grandpa_finality_verifier::mock::brute_seed_block_1;
use serde_json::Value;
use sp_io::TestExternalities;
use sp_runtime::{AccountId32, DispatchError, DispatchErrorWithPostInfo};
use sp_std::{convert::TryFrom, prelude::*};
use std::{convert::TryInto, fs};
use t3rn_primitives::{
    abi::*,
    circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
    side_effect::*,
    volatile::LocalState,
    xdns::AllowedSideEffect,
    xtx::XtxId,
    Balance, ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};

use t3rn_protocol::side_effects::test_utils::*;

use pallet_xbi_portal::{
    sabi::AccountId20,
    xbi_codec::{ActionNotificationTimeouts, XBIFormat, XBIInstr, XBIMetadata},
};
use pallet_xbi_portal_enter::t3rn_sfx::xbi_2_sfx;

use circuit_runtime_pallets::pallet_circuit::Error as circuit_error;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB_RELAYER: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

pub const FIRST_REQUESTER_NONCE: u32 = 0;
pub const SECOND_REQUESTER_NONCE: u32 = 1;
pub const THIRD_REQUESTER_NONCE: u32 = 2;
pub const FOURTH_REQUESTER_NONCE: u32 = 3;
pub const FIFTH_REQUESTER_NONCE: u32 = 4;
pub const FIRST_SFX_INDEX: u32 = 0;
pub const SECOND_SFX_INDEX: u32 = 1;
pub const THIRD_SFX_INDEX: u32 = 2;
pub const FOURTH_SFX_INDEX: u32 = 3;
pub const FIFTH_SFX_INDEX: u32 = 4;

fn set_ids(
    sfx: SideEffect<AccountId32, Balance>,
    requester: AccountId32,
    requester_nonce: u32,
    sfx_index: u32,
) -> (sp_core::H256, sp_core::H256) {
    let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(requester, requester_nonce);

    let sfx_id = sfx
        .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &xtx_id.0, sfx_index,
        );

    (xtx_id, sfx_id)
}

fn register(
    origin: OriginFor<Runtime>,
    json: Value,
    valid: bool,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let url: Vec<u8> = hex::decode(json["encoded_url"].as_str().unwrap()).unwrap();
    let gateway_id: ChainId =
        Decode::decode(&mut &*hex::decode(json["encoded_gateway_id"].as_str().unwrap()).unwrap())
            .unwrap();
    let gateway_abi: GatewayABIConfig =
        Decode::decode(&mut &*hex::decode(json["encoded_gateway_abi"].as_str().unwrap()).unwrap())
            .unwrap();
    let gateway_vendor: GatewayVendor = Decode::decode(
        &mut &*hex::decode(json["encoded_gateway_vendor"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    let gateway_type: GatewayType =
        Decode::decode(&mut &*hex::decode(json["encoded_gateway_type"].as_str().unwrap()).unwrap())
            .unwrap();
    let gateway_genesis: GatewayGenesisConfig = Decode::decode(
        &mut &*hex::decode(json["encoded_gateway_genesis"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    let gateway_sys_props: GatewaySysProps = Decode::decode(
        &mut &*hex::decode(json["encoded_gateway_sys_props"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    let allowed_side_effects: Vec<AllowedSideEffect> = Decode::decode(
        &mut &*hex::decode(json["encoded_allowed_side_effects"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    let encoded_registration_data: Vec<u8> =
        hex::decode(json["encoded_registration_data"].as_str().unwrap()).unwrap();

    let res = Portal::register_gateway(
        origin,
        url,
        gateway_id,
        gateway_abi.clone(),
        gateway_vendor.clone(),
        gateway_type.clone(),
        gateway_genesis.clone(),
        gateway_sys_props.clone(),
        allowed_side_effects.clone(),
        encoded_registration_data,
    );

    if valid {
        let xdns_record = pallet_xdns::XDNSRegistry::<Runtime>::get(gateway_id).unwrap();
        let stored_side_effects = xdns_record.allowed_side_effects;

        // ensure XDNS writes are correct
        assert_eq!(stored_side_effects, allowed_side_effects);
        assert_eq!(xdns_record.gateway_vendor, gateway_vendor);
        assert_eq!(xdns_record.gateway_abi, gateway_abi);
        assert_eq!(xdns_record.gateway_type, gateway_type);
        assert_eq!(xdns_record.gateway_sys_props, gateway_sys_props);
        assert_eq!(xdns_record.gateway_genesis, gateway_genesis);
    }

    res
}

fn submit_headers(
    origin: OriginFor<Runtime>,
    json: Value,
    index: usize,
) -> Result<(), DispatchError> {
    let encoded_header_data: Vec<u8> =
        hex::decode(json[index]["encoded_data"].as_str().unwrap()).unwrap();
    let gateway_id: ChainId = Decode::decode(
        &mut &*hex::decode(json[index]["encoded_gateway_id"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    Portal::submit_headers(origin, gateway_id, encoded_header_data)
}

fn on_extrinsic_trigger(
    origin: OriginFor<Runtime>,
    json: Value,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let side_effects: Vec<SideEffect<AccountId32, BalanceOf>> =
        Decode::decode(&mut &*hex::decode(json["encoded_side_effects"].as_str().unwrap()).unwrap())
            .unwrap();

    let fee = 0u128;
    let sequential: bool =
        Decode::decode(&mut &*hex::decode(json["encoded_sequential"].as_str().unwrap()).unwrap())
            .unwrap();
    Circuit::on_extrinsic_trigger(origin, side_effects, fee, sequential)
}

fn confirm_side_effect(
    origin: OriginFor<Runtime>,
    json: Value,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let xtx_id: sp_core::H256 =
        Decode::decode(&mut &*hex::decode(json["encoded_xtx_id"].as_str().unwrap()).unwrap())
            .unwrap();
    let side_effect: SideEffect<AccountId32, BalanceOf> =
        Decode::decode(&mut &*hex::decode(json["encoded_side_effect"].as_str().unwrap()).unwrap())
            .unwrap();
    let confirmed_side_effect: ConfirmedSideEffect<AccountId32, BlockNumber, BalanceOf> =
        Decode::decode(
            &mut &*hex::decode(json["encoded_confirmed_side_effect"].as_str().unwrap()).unwrap(),
        )
        .unwrap();

    let sfx_id = side_effect
        .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &xtx_id.0, 0,
        );

    Circuit::confirm_side_effect(origin, sfx_id, confirmed_side_effect)
}

pub fn bid_sfx(
    origin: OriginFor<Runtime>,
    json: Value,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let _xtx_id: sp_core::H256 =
        Decode::decode(&mut &*hex::decode(json["encoded_xtx_id"].as_str().unwrap()).unwrap())
            .unwrap();

    let side_effect_id: SideEffectId<Runtime> =
        Decode::decode(&mut &*hex::decode(json["encoded_id"].as_str().unwrap()).unwrap()).unwrap();

    Circuit::bid_sfx(
        origin, // Active relayer
        side_effect_id,
        2 as Balance,
    )
}

pub fn confirm_sfx(
    executor: AccountId32,
    xtx_id: sp_core::H256,
    sfx: SideEffect<AccountId32, Balance>,
    inclusion_data: Vec<u8>,
) {
    let sfx_confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
        err: None,
        output: None,
        executioner: executor.clone(),
        received_at: 0,
        cost: None,
        inclusion_data,
    };

    let _ = Circuit::confirm_side_effect(
        Origin::signed(executor),
        sfx.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &xtx_id.0, 0,
        ),
        sfx_confirmation,
    );
}

pub fn place_winning_bid_and_advance_3_blocks(
    executor: AccountId32,
    xtx_id: sp_core::H256,
    sfx_id: sp_core::H256,
    bid_amount: Balance,
) {
    assert_ok!(Circuit::bid_sfx(
        Origin::signed(executor.clone()), // Active relayer
        sfx_id,
        bid_amount,
    ));

    assert_eq!(
        Circuit::get_pending_sfx_bids(xtx_id, sfx_id).unwrap().bid,
        bid_amount
    );

    assert_eq!(
        Circuit::get_pending_sfx_bids(xtx_id, sfx_id)
            .unwrap()
            .executor,
        executor
    );

    assert_eq!(
        Circuit::get_pending_xtx_bids_timeouts(xtx_id).unwrap(),
        System::block_number() + 3
    );

    let three_blocks_ahead = System::block_number() + 3;
    System::set_block_number(three_blocks_ahead);

    <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(
        three_blocks_ahead,
    );

    assert_eq!(
        Circuit::get_x_exec_signals(xtx_id).unwrap().status,
        CircuitStatus::Ready
    );
}

fn read_file_and_set_height(path: &str, ignore_submission_height: bool) -> Value {
    let file = fs::read_to_string("src/mock-data/".to_owned() + path).unwrap();
    let json: Value = serde_json::from_str(file.as_str()).unwrap();
    for entry in json.as_array().unwrap() {
        let submission_height: u64 = entry["submission_height"].as_u64().unwrap();
        if submission_height > 0 && !ignore_submission_height {
            System::set_block_number(submission_height.try_into().unwrap());
        }
    }
    json
}

// iterates sequentially though all test files in mock-data
fn run_mock_tests(path: &str) -> Result<(), DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let cli_signer = Origin::signed(
        [
            212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133,
            88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
        ]
        .into(),
    );
    let mut paths: Vec<_> = fs::read_dir("src/mock-data/".to_owned() + path)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    for entry in paths {
        let path = entry.path();
        let file = fs::read_to_string(&path).unwrap();
        let json: Value = serde_json::from_str(file.as_str()).unwrap();
        for entry in json.as_array().unwrap() {
            let submission_height: u64 = entry["submission_height"].as_u64().unwrap();
            if submission_height > 0 {
                System::set_block_number(submission_height.try_into().unwrap());
            }
            match entry["transaction_type"].as_str().unwrap() {
                "register" => {
                    assert_ok!(register(Origin::root(), entry.clone(), true));
                },
                "submit-headers" =>
                    for index in 0..json.as_array().unwrap().len() {
                        assert_ok!(submit_headers(
                            Origin::signed([0u8; 32].into()),
                            json.clone(),
                            index
                        ));
                    },
                "transfer" => {
                    assert_ok!(on_extrinsic_trigger(cli_signer.clone(), entry.clone()));
                },
                "confirm" => {
                    assert_ok!(confirm_side_effect(cli_signer.clone(), entry.clone()));
                },
                _ => unimplemented!(),
            }
        }
    }
    Ok(())
}

#[test]
#[ignore]
fn runs_mock_tests() {
    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = run_mock_tests("auto");
        });
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    (array[0] as u32)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

#[test]
fn on_extrinsic_trigger_works_with_empty_side_effects() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let mut ext = TestExternalities::new_empty();
    let side_effects = vec![];
    let fee = 1;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

        assert_ok!(Circuit::on_extrinsic_trigger(
            origin,
            side_effects,
            fee,
            sequential,
        ));
    });
}

#[test]
fn on_extrinsic_trigger_works_raw_insured_side_effect() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let side_effects = vec![SideEffect {
        target: [0u8, 0u8, 0u8, 0u8],
        max_reward: 2,
        encoded_action: vec![116, 114, 97, 110],
        encoded_args: vec![
            vec![
                53, 71, 114, 119, 118, 97, 69, 70, 53, 122, 88, 98, 50, 54, 70, 122, 57, 114, 99,
                81, 112, 68, 87, 83, 53, 55, 67, 116, 69, 82, 72, 112, 78, 101, 104, 88, 67, 80,
                99, 78, 111, 72, 71, 75, 117, 116, 81, 89,
            ],
            vec![
                53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85, 53,
                110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121, 107, 53, 113, 90, 106, 57,
                116, 88, 82, 65, 53, 101, 114, 115, 55, 65,
            ],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        ],
        signature: vec![],
        enforce_executor: Some(
            [
                53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85, 53,
                110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
            ]
            .into(),
        ),
        insurance: 3,
    }];

    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_works_with_single_transfer_sets_storage_entries() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            // Assert Circuit::emit generates 5 correct events: 3 from charging and 2 Circuit-specific
            let mut events = System::events();
            let event_a = events.pop();
            let event_b = events.pop();

            assert_eq!(
                vec![event_b.unwrap(), event_a.unwrap()],
                vec![
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                max_reward: 1,
                                encoded_action: vec![116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                    ],
                                    vec![
                                        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                                        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6
                                    ],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![
                                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                                    ]
                                ],
                                signature: vec![],
                                enforce_executor: None,
                                insurance: 1,
                            }],
                            vec![hex!(
                                "810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4"
                            )
                            .into(),],
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionReceivedForExec(
                            hex!(
                                "2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                ]
            );
            let xtx_id: sp_core::H256 =
                hex!("2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c").into();
            let side_effect_a_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                &xtx_id.0,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id),
                None
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );
        });
}

#[test]
fn on_extrinsic_trigger_validation_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_works_with_single_transfer_emits_expect_events() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            // Assert Circuit::emit generates 5 correct events: 3 for charging and 2 Circuit-specific
            let mut events = System::events();
            // assert_eq!(events.len(), 10);
            let event_a = events.pop();
            let event_b = events.pop();
            assert_eq!(
                vec![event_b.unwrap(), event_a.unwrap()],
                vec![
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                max_reward: 1 as Balance,
                                insurance: 1 as Balance,
                                encoded_action: vec![116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                    ],
                                    vec![
                                        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                                        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6
                                    ],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    // Insurance goes here
                                    vec![
                                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                                    ]
                                ],
                                signature: vec![],
                                enforce_executor: None
                            }],
                            vec![hex!(
                                "810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4"
                            )
                            .into(),],
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionReceivedForExec(
                            hex!(
                                "2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                ]
            );
        });
}

#[test]
fn circuit_handles_single_bid_for_transfer_sfx() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, reward = 1
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    const REQUESTED_INSURANCE_AMOUNT: Balance = 1;
    const BID_AMOUNT: Balance = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ =
                Balances::deposit_creating(&BOB_RELAYER, REQUESTED_INSURANCE_AMOUNT + BID_AMOUNT); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_transfer_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

            assert_ok!(Circuit::bid_sfx(
                origin_relayer_bob,
                side_effect_a_id,
                BID_AMOUNT,
            ));

            let expected_bonded_sfx_bid = SFXBid {
                bid: BID_AMOUNT,
                requester: ALICE,
                executor: BOB_RELAYER,
                reserved_bond: None,
                insurance: REQUESTED_INSURANCE_AMOUNT,
            };

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
                expected_bonded_sfx_bid
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );
        });
}

#[test]
fn circuit_handles_dropped_at_bidding() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, reward = 1
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    const REQUESTED_INSURANCE_AMOUNT: Balance = 1;
    const BID_AMOUNT: Balance = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ =
                Balances::deposit_creating(&BOB_RELAYER, REQUESTED_INSURANCE_AMOUNT + BID_AMOUNT); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_eq!(Balances::free_balance(ALICE), 3);
            assert_eq!(Balances::reserved_balance(ALICE), 0);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            assert_eq!(Balances::free_balance(ALICE), 2);
            assert_eq!(Balances::reserved_balance(ALICE), 1);

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_transfer_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: ALICE,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            System::reset_events();
            System::set_block_number(4);
            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(4);
            let events = System::events();

            assert_eq!(Balances::free_balance(ALICE), 3);
            assert_eq!(Balances::reserved_balance(ALICE), 0);

            assert_eq!(
                events[0],
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Balances(
                        circuit_runtime_pallets::pallet_balances::Event::Unreserved {
                            who: ALICE,
                            amount: fee
                        }
                    ),
                    topics: vec![]
                }
            );
            assert_eq!(
                events[1],
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                        Runtime,
                    >::XTransactionXtxDroppedAtBidding(
                        xtx_id
                    )),
                    topics: vec![]
                }
            );
            // ToDo activate once #504 is fixed
            // assert_eq!(Circuit::get_x_exec_signals(xtx_id), None);
        })
}

#[test]
fn circuit_selects_best_bid_out_of_3_for_transfer_sfx() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::C), // insurance = 3, max_reward/reward = 3
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    const REQUESTED_INSURANCE_AMOUNT: Balance = 3;
    const BID_AMOUNT_A: Balance = 1;
    const BID_AMOUNT_B: Balance = 2;
    const BID_AMOUNT_C: Balance = 3;
    const MAX_FEE: Balance = 3;

    const INITIAL_BALANCE: Balance = 10;

    const REQUESTER: AccountId32 = ALICE;
    const BID_WINNER: AccountId32 = BOB_RELAYER;
    const BID_LOOSER: AccountId32 = CHARLIE;

    const BIDDING_BLOCK_NO: BlockNumber = 1;
    const BIDDING_TIMEOUT: BlockNumber = 3;
    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&REQUESTER, INITIAL_BALANCE);
            let _ = Balances::deposit_creating(&BID_WINNER, INITIAL_BALANCE);
            let _ = Balances::deposit_creating(&BID_LOOSER, INITIAL_BALANCE);

            System::set_block_number(BIDDING_BLOCK_NO);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                true,
            ));
            // Requester should have offered SFX::max_reward reserved
            assert_eq!(Balances::reserved_balance(&REQUESTER), MAX_FEE);
            assert_eq!(
                Balances::free_balance(&REQUESTER),
                INITIAL_BALANCE - MAX_FEE
            );

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_transfer_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: REQUESTER,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            System::reset_events();
            // Bob opens bid with bid = max_reward, the highest possible
            assert_ok!(Circuit::bid_sfx(
                Origin::signed(BID_WINNER),
                side_effect_a_id,
                BID_AMOUNT_C,
            ));
            let events = System::events();

            assert_eq!(
                events[1],
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                        Runtime,
                    >::SFXNewBidReceived(
                        side_effect_a_id,
                        BID_WINNER,
                        BID_AMOUNT_C
                    )),
                    topics: vec![]
                }
            );

            // Reserve insurance + bid amounts of the current winner
            assert_eq!(
                Balances::reserved_balance(&BID_WINNER),
                BID_AMOUNT_C + REQUESTED_INSURANCE_AMOUNT
            );

            // Charlie bids better offer
            assert_ok!(Circuit::bid_sfx(
                Origin::signed(BID_LOOSER),
                side_effect_a_id,
                BID_AMOUNT_B,
            ));

            // Reserve insurance + bid amounts of the current winner
            assert_eq!(
                Balances::reserved_balance(&BID_LOOSER),
                BID_AMOUNT_B + REQUESTED_INSURANCE_AMOUNT
            );
            // Unreserve insurance + bid amounts of the previous bidder
            assert_eq!(Balances::reserved_balance(&BID_WINNER), 0);
            // Bidding with the same amount should not be accepted
            assert_err!(
                Circuit::bid_sfx(Origin::signed(BID_WINNER), side_effect_a_id, BID_AMOUNT_B,),
                circuit_error::<Runtime>::BiddingRejectedBetterBidFound,
            );

            // Bob submits the winning bid
            assert_ok!(Circuit::bid_sfx(
                Origin::signed(BID_WINNER),
                side_effect_a_id,
                BID_AMOUNT_A,
            ));
            // Reserve insurance + bid amounts of the current winner
            assert_eq!(
                Balances::reserved_balance(&BID_WINNER),
                BID_AMOUNT_A + REQUESTED_INSURANCE_AMOUNT
            );
            // Unreserve insurance + bid amounts of the previous bidder
            assert_eq!(Balances::reserved_balance(&BID_LOOSER), 0);

            let expected_bonded_sfx_bid = SFXBid {
                bid: BID_AMOUNT_A,
                requester: REQUESTER,
                executor: BID_WINNER,
                reserved_bond: None,
                insurance: REQUESTED_INSURANCE_AMOUNT,
            };

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
                expected_bonded_sfx_bid
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: REQUESTER,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            System::set_block_number(BIDDING_BLOCK_NO + BIDDING_TIMEOUT);

            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(
                BIDDING_BLOCK_NO + BIDDING_TIMEOUT,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: REQUESTER,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                })
            );
        });
}

#[test]
fn circuit_handles_swap_with_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let ext = ExtBuilder::default();

    let mut local_state = LocalState::new();
    let swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());
    let valid_swap_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),       // caller
            (Type::Address(32), ArgVariant::B),       // to
            (Type::Uint(128), ArgVariant::A),         // amount_from
            (Type::Uint(128), ArgVariant::B),         // amount_to
            (Type::Bytes(4), ArgVariant::A),          // asset_from
            (Type::Bytes(4), ArgVariant::B),          // asset_to
            (Type::OptionalInsurance, ArgVariant::A), // insurance
        ],
        &mut local_state,
        swap_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_swap_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext.with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1 + 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_swap_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_swap_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            place_winning_bid_and_advance_3_blocks(
                BOB_RELAYER,
                xtx_id,
                side_effect_a_id,
                1 as Balance,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );
        });
}

#[test]
fn circuit_handles_add_liquidity_without_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();
    let mut local_state = LocalState::new();

    let add_liquidity_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_add_liquidity_interface());

    let valid_add_liquidity_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),       // argument_0: caller
            (Type::Address(32), ArgVariant::B),       // argument_1: to
            (Type::Bytes(4), ArgVariant::A),          // argument_2: asset_left
            (Type::Bytes(4), ArgVariant::B),          // argument_3: asset_right
            (Type::Bytes(4), ArgVariant::C),          // argument_4: liquidity_token
            (Type::Uint(128), ArgVariant::A),         // argument_5: amount_left
            (Type::Uint(128), ArgVariant::B),         // argument_6: amount_right
            (Type::Uint(128), ArgVariant::A),         // argument_7: amount_liquidity_token
            (Type::OptionalInsurance, ArgVariant::A), // argument_8: no insurance, empty bytes
        ],
        &mut local_state,
        add_liquidity_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_add_liquidity_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext.with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2);
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_add_liquidity_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id),
                None
            );
        });
}

#[test]
fn circuit_handles_add_liquidity_with_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();
    let mut local_state = LocalState::new();

    let add_liquidity_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_add_liquidity_interface());

    let valid_add_liquidity_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),       // argument_0: caller
            (Type::Address(32), ArgVariant::B),       // argument_1: to
            (Type::Bytes(4), ArgVariant::A),          // argument_2: asset_left
            (Type::Bytes(4), ArgVariant::B),          // argument_3: asset_right
            (Type::Bytes(4), ArgVariant::A),          // argument_4: liquidity_token
            (Type::Uint(128), ArgVariant::A),         // argument_5: amount_left
            (Type::Uint(128), ArgVariant::B),         // argument_6: amount_right
            (Type::Uint(128), ArgVariant::A),         // argument_7: amount_liquidity_token
            (Type::OptionalInsurance, ArgVariant::A), // argument_8: Variant A insurance = 1, reward = 2
        ],
        &mut local_state,
        add_liquidity_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_add_liquidity_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext.with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1 + 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_add_liquidity_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_add_liquidity_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            place_winning_bid_and_advance_3_blocks(
                BOB_RELAYER,
                xtx_id,
                side_effect_a_id,
                1 as Balance,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );
        });
}

// fn successfully_confirm_optimistic(side_effect: SideEffect<AccountId32, Balance>) {
//
//     let from = side_effect.encoded_args[0].clone();
//     let to = side_effect.encoded_args[1].clone();
//     let amount = side_effect.encoded_args[2].clone();
//
//     let mut encoded_balance_transfer_event_1 = pallet_balances::Event::<Runtime>::Transfer {
//         from: from.into(), // variant A
//         to: to.into(), // variant B (dest)
//         amount: amount.into(), // variant A
//     }
//         .encode();
//
//     // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
//     let mut encoded_event_1 = vec![4];
//     encoded_event_1.append(&mut encoded_balance_transfer_event_1);
//     let confirmation_transfer_1 =
//         ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
//             err: None,
//             output: None,
//             encoded_effect: encoded_event_1,
//             inclusion_proof: None,
//             executioner: BOB_RELAYER,
//             received_at: 0,
//             cost: None,
//         };
//
//     assert_ok!(Circuit::confirm_side_effect(
//         origin_relayer_bob.clone(),
//         xtx_id.clone(),
//         valid_transfer_side_effect_1,
//         confirmation_transfer_1,
//         None,
//         None,
//     ));
//
// }

fn successfully_bond_optimistic(
    side_effect: SideEffect<AccountId32, Balance>,
    sfx_index: u32,
    xtx_id: XtxId<Runtime>,
    relayer: AccountId32,
    submitter: AccountId32,
) {
    let optional_insurance = side_effect.encoded_args[3].clone();

    assert!(
        optional_insurance.len() == 32,
        "Wrong test value - optimistic transfer assumes optimistic arguments"
    );

    assert_ok!(Circuit::bid_sfx(
        Origin::signed(relayer.clone()),
        side_effect.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &xtx_id.0, sfx_index
        ),
        2 as Balance,
    ));

    let [insurance, reward]: [u128; 2] = Decode::decode(&mut &optional_insurance[..]).unwrap();

    let created_sfx_bid = Circuit::get_pending_sfx_bids(
        xtx_id,
        side_effect.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &xtx_id.0, sfx_index,
        ),
    )
    .unwrap();

    assert_eq!(created_sfx_bid.insurance, insurance as Balance);
    // assert_eq!(created_sfx_bid.reserved_bond, Some(insurance as Balance));
    assert_eq!(created_sfx_bid.bid, reward as Balance);
    assert_eq!(
        created_sfx_bid.requester,
        Decode::decode(&mut &submitter.encode()[..]).unwrap()
    );
    assert_eq!(created_sfx_bid.executor, relayer);
}

#[test]
fn two_dirty_transfers_are_allocated_to_2_steps_and_can_be_submitted() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::B),
            (Type::Address(32), ArgVariant::A),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        SECOND_REQUESTER_NONCE,
        SECOND_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect_1, valid_transfer_side_effect_2];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 10);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let events = System::events();
            assert_eq!(events.len(), 6);
        });
}

#[test]
fn two_dirty_transfers_are_allocated_to_2_steps_and_can_be_confirmed() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::B),
            (Type::Address(32), ArgVariant::A),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        SECOND_REQUESTER_NONCE,
        SECOND_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect_1, valid_transfer_side_effect_2];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

// ToDo: Order for multiple should now be fixed - verify t3rn#261 is solved
#[test]
#[ignore]
fn circuit_handles_transfer_dirty_and_optimistic_and_swap() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());
    let swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A),
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        SECOND_REQUESTER_NONCE,
        SECOND_SFX_INDEX,
    );

    let valid_swap_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),       // caller
            (Type::Address(32), ArgVariant::B),       // to
            (Type::Uint(128), ArgVariant::A),         // amount_from
            (Type::Uint(128), ArgVariant::B),         // amount_to
            (Type::Bytes(4), ArgVariant::A),          // asset_from
            (Type::Bytes(4), ArgVariant::B),          // asset_to
            (Type::OptionalInsurance, ArgVariant::A), // no insurance
        ],
        &mut local_state,
        swap_protocol_box,
        ALICE,
        THIRD_REQUESTER_NONCE,
        THIRD_SFX_INDEX,
    );

    let side_effects = vec![
        valid_transfer_side_effect_1,
        valid_transfer_side_effect_2,
        valid_swap_side_effect,
    ];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

#[test]
fn circuit_cancels_xtx_with_bids_after_timeout() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());
    let _swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A),
        ],
        &mut local_state,
        transfer_protocol_box,
        FIRST_REQUESTER_NONCE,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = false;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let _events = System::events();
            // assert_eq!(events.len(), 8);

            let xtx_id: sp_core::H256 =
                hex!("9946104f0d553532303b8a763d5828d75ed4493c585c948d10b7e9317ade6331").into();

            // The tiemout links that will be checked at on_initialize are there
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(401u32)); // 100 offset + current block height 1 = 101

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32, // 400 offset + current block height 1 = 101
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                })
            );

            let sfx_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            );
            place_winning_bid_and_advance_3_blocks(ALICE, xtx_id, sfx_id, 1);

            System::set_block_number(410);

            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(410);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::RevertTimedOut,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                })
            );

            assert_eq!(Circuit::get_active_timing_links(xtx_id), None);

            // Emits event notifying about cancellation
            let mut events = System::events();
            // assert_eq!(events.len(), 9);
            assert_eq!(
                events.pop(),
                Some(
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionXtxRevertedAfterTimeOut(
                            hex!(
                                "9946104f0d553532303b8a763d5828d75ed4493c585c948d10b7e9317ade6331"
                            )
                            .into()
                        )),
                        topics: vec![]
                    }
                ),
            );
            // Voids all associated side effects with Xtx by setting their confirmation to Err
        });
}

#[test]
fn circuit_cancels_xtx_after_timeout() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());
    let _swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let side_effects = vec![valid_transfer_side_effect];
    let fee = 1;
    let sequential = false;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let _events = System::events();
            // assert_eq!(events.len(), 8);

            let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);

            // The tiemout links that will be checked at on_initialize are there
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(401u32)); // 100 offset + current block height 1 = 101

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32, // 100 offset + current block height 1 = 101
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                })
            );

            System::set_block_number(410);

            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(410);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::RevertTimedOut,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                })
            );

            assert_eq!(Circuit::get_active_timing_links(xtx_id), None);

            // Emits event notifying about cancellation
            let mut events = System::events();
            // assert_eq!(events.len(), 9);
            assert_eq!(
                events.pop(),
                Some(
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionXtxRevertedAfterTimeOut(
                            hex!(
                                "2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c"
                            )
                            .into()
                        )),
                        topics: vec![]
                    }
                ),
            );
            // Voids all associated side effects with Xtx by setting their confirmation to Err
        });
}

#[test]
fn load_local_state_can_generate_and_read_state() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
    let mut ext = TestExternalities::new_empty();

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

        let res = Circuit::load_local_state(&origin, None).unwrap();

        let xtx_id_new: sp_core::H256 =
            hex!("2dd1ccea5b1d02d46b19803b55f7de8ee5dabc951faf617c28c7933dae30719c").into();

        assert_eq!(res.xtx_id, xtx_id_new);
        assert_eq!(res.local_state, LocalState::new());
        assert_eq!(res.steps_cnt, (0, 0));
    });
}

#[test]
#[ignore]
fn uninsured_unrewarded_single_rococo_transfer() {
    let path = "uninsured_unrewarded_single_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.01",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 1);
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 1);
            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);
            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            let confirm = read_file_and_set_height(
                &(path.to_owned() + "5-confirm-transfer-325d16cb.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm[0].clone()
            ));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 1);
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), 1);
        });
}

#[test]
#[ignore]
fn insured_unrewarded_single_rococo_transfer() {
    let path = "insured_unrewarded_single_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.1",
    //             bond: "1",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 3-submit-transfer", 50);
    // await execute("submit-headers roco --export -o 5-headers-roco", 5);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 10 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 10 * 10u128.pow(12)); // 10 trn

            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);
            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 10u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            let confirm = read_file_and_set_height(
                &(path.to_owned() + "6-confirm-transfer-8eb5521e.json"),
                false,
            );
            // Can't confirm without header in light client
            assert_noop!(
                confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()),
                "SideEffect confirmation failed!"
            );

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "5-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            // // Can't confirm without bond posted
            // assert_noop!(
            //     confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()),
            //     Circuit::Error::<Runtime>::ApplyFailed
            // );

            let post_bond = read_file_and_set_height(
                &(path.to_owned() + "4-bond-insurance-8eb5521e.json"),
                false,
            );
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond[0].clone()
            ));

            assert_eq!(
                Balances::free_balance(&CLI_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                9u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                10u128.pow(12)
            );

            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm[0].clone()
            ));
            assert_eq!(
                Balances::free_balance(&CLI_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn insured_rewarded_single_rococo_transfer() {
    let path = "insured_rewarded_single_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.1",
    //             bond: "1",
    //             reward: "1",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 3-submit-transfer", 50);
    // await execute("submit-headers roco --export -o 5-headers-roco", 5);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 10 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 10 * 10u128.pow(12)); // 10 trn

            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }
            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 9u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            assert_eq!(Balances::reserved_balance(&CLI_DEFAULT), 10u128.pow(12));
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );

            let post_bond = read_file_and_set_height(
                &(path.to_owned() + "4-bond-insurance-3c964de9.json"),
                false,
            );
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond[0].clone()
            ));

            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 9u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                9u128 * 10u128.pow(12)
            );

            assert_eq!(Balances::reserved_balance(&CLI_DEFAULT), 10u128.pow(12));
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                10u128.pow(12)
            );

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "5-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            let confirm = read_file_and_set_height(
                &(path.to_owned() + "6-confirm-transfer-3c964de9.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm[0].clone()
            ));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 9u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                11u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn insured_rewarded_multi_rococo_transfer() {
    let path = "insured_rewarded_multi_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.1",
    //             bond: "1",
    //             reward: "1",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5GducktTqf8KKeatpex4kwkg1PZZimY1xUDUFoBZ2s5EDfVf",
    //             amount: "0.1",
    //             bond: "2",
    //             reward: "2",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 3-submit-transfer", 50);
    // await execute("submit-headers roco --export -o 6-headers-roco", 5);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 10 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 10 * 10u128.pow(12)); // 10 trn

            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 7u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                3u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );

            let post_bond_1 = read_file_and_set_height(
                &(path.to_owned() + "4-bond-insurance-f0a3de08.json"),
                false,
            );
            let post_bond_2 = read_file_and_set_height(
                &(path.to_owned() + "5-bond-insurance-3c964de9.json"),
                false,
            );

            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond_1[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond_2[0].clone()
            ));

            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 7u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                3u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                3u128 * 10u128.pow(12)
            );

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "6-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }
            let confirm_2 = read_file_and_set_height(
                &(path.to_owned() + "8-confirm-transfer-f0a3de08.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_2[0].clone()
            ));

            let confirm_1 = read_file_and_set_height(
                &(path.to_owned() + "7-confirm-transfer-3c964de9.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_1[0].clone()
            ));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 7u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn insured_unrewarded_multi_rococo_transfer() {
    let path = "insured_unrewarded_multi_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.1",
    //             bond: "1",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5GducktTqf8KKeatpex4kwkg1PZZimY1xUDUFoBZ2s5EDfVf",
    //             amount: "0.1",
    //             bond: "2",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 3-submit-transfer", 50);
    // await execute("submit-headers roco --export -o 6-headers-roco", 5);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 10 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 10 * 10u128.pow(12)); // 10 trn

            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }
            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 10u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );

            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );

            let post_bond_1 = read_file_and_set_height(
                &(path.to_owned() + "4-bond-insurance-863c7bc6.json"),
                false,
            );
            let post_bond_2 = read_file_and_set_height(
                &(path.to_owned() + "5-bond-insurance-8eb5521e.json"),
                false,
            );

            // Bond can be submitted in arbitrary order
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond_2[0].clone()
            ));

            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                post_bond_1[0].clone()
            ));

            assert_eq!(
                Balances::free_balance(&CLI_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                3u128 * 10u128.pow(12)
            );

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "6-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            // the confirmation order for these side effect doesn't matter, as they're all insured
            let confirm_2 = read_file_and_set_height(
                &(path.to_owned() + "7-confirm-transfer-863c7bc6.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_2[0].clone()
            ));
            let confirm_1 = read_file_and_set_height(
                &(path.to_owned() + "8-confirm-transfer-8eb5521e.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_1[0].clone()
            ));
            assert_eq!(
                Balances::free_balance(&CLI_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

// ToDo add rewarded_unbonded_multi test

#[test]
#[ignore]
fn uninsured_unrewarded_multi_rococo_transfer() {
    let path = "uninsured_unrewarded_multi_rococo_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.1",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5GducktTqf8KKeatpex4kwkg1PZZimY1xUDUFoBZ2s5EDfVf",
    //             amount: "0.1",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         }
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 3-submit-transfer", 50);
    // await execute("submit-headers roco --export -o 4-headers-roco", 50);
    // await execute("submit-headers roco --export -o 6-headers-roco", 15);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 10 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 10 * 10u128.pow(12)); // 10 trn

            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }
            let transfer =
                read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 10u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }
            let confirm_2 = read_file_and_set_height(
                &(path.to_owned() + "7-confirm-transfer-3fdd994b.json"),
                false,
            );
            // shouldn't confirm in wrong order, as these are uninsured
            assert_noop!(
                confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm_2[0].clone()),
                "Unable to find matching Side Effect in given Xtx to confirm"
            );

            let submit_header_3 =
                read_file_and_set_height(&(path.to_owned() + "6-headers-roco.json"), false);
            for index in 0..submit_header_3.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_3.clone(),
                    index
                ));
            }

            let confirm_1 = read_file_and_set_height(
                &(path.to_owned() + "5-confirm-transfer-846c03c6.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_1[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_2[0].clone()
            ));
            assert_eq!(
                Balances::free_balance(&CLI_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(&EXECUTOR_DEFAULT),
                10u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn multi_mixed_rococo() {
    let path = "multi_mixed_rococo/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.01111",
    //             bond: "1",
    //             reward: "2",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.02222",
    //             bond: "3",
    //             reward: "3",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "12",
    //             bond: "1",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "11",
    //             bond: "2",
    //             reward: "3",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.011",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "3",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.012",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //     ],
    //     sequential: false,
    // }

    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("register bslk --export -o 3-register-bslk", 10)
    // await execute("submit-headers roco --export -o 4-headers-roco", 15);
    // await execute("submit-headers bslk --export -o 5-headers-bslk", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 6-submit-transfer", 70);
    // await execute("submit-headers roco --export -o 11-headers-roco", 10);
    // await execute("submit-headers bslk --export -o 14-headers-bslk", 70);
    // await execute("submit-headers roco --export -o 17-headers-roco", 90);
    // await execute("submit-headers roco --export -o 19-headers-roco", 10);
    // await execute("submit-headers bslk --export -o 20-headers-bslk", 90);
    // await execute("submit-headers roco --export -o 22-headers-roco", 0);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 20 * 10u128.pow(12)); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 20 * 10u128.pow(12)); // 10 trn

            let register_roco =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_roco[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }

            let register_bslk =
                read_file_and_set_height(&(path.to_owned() + "3-register-bslk.json"), false);
            assert_ok!(register(Origin::root(), register_bslk[0].clone(), true));

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            let submit_header_3 =
                read_file_and_set_height(&(path.to_owned() + "5-headers-bslk.json"), false);
            for index in 0..submit_header_3.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_3.clone(),
                    index
                ));
            }

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "6-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                20u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );

            let bond_insurance_1 = read_file_and_set_height(
                &(path.to_owned() + "10-bond-insurance-09268618.json"),
                true,
            );
            let bond_insurance_2 = read_file_and_set_height(
                &(path.to_owned() + "7-bond-insurance-c29dce66.json"),
                true,
            );
            let bond_insurance_3 = read_file_and_set_height(
                &(path.to_owned() + "8-bond-insurance-7a39c710.json"),
                true,
            );
            let bond_insurance_4 = read_file_and_set_height(
                &(path.to_owned() + "9-bond-insurance-2d6e40f6.json"),
                true,
            );

            let confirm_1 = read_file_and_set_height(
                &(path.to_owned() + "12-confirm-transfer-7a39c710.json"),
                true,
            );

            // Bond can be submitted in arbitrary order
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_3[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_4[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_2[0].clone()
            ));

            // can't execute until header was submitted
            assert_noop!(
                confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm_1[0].clone()),
                "SideEffect confirmation failed!"
            );

            // Submit header next roco range randomly
            let submit_header_4 =
                read_file_and_set_height(&(path.to_owned() + "11-headers-roco.json"), false);
            for index in 0..submit_header_4.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_4.clone(),
                    index
                ));
            }

            // ToDo can't import error here
            // assert_noop!(
            //     confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm_1[0].clone()),
            //     pallet_circuit::Error::<Runtime>::ApplyFailed
            // );

            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_1[0].clone()
            ));

            // Other executor can submit, but wont be rewarded once complete
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            // ______Confirm insured step:________

            // can confirm with all bonds paid
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_1[0].clone()
            ));

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            let confirm_2 = read_file_and_set_height(
                &(path.to_owned() + "16-confirm-transfer-c29dce66.json"),
                false,
            );
            let confirm_3 = read_file_and_set_height(
                &(path.to_owned() + "15-confirm-transfer-2d6e40f6.json"),
                false,
            );
            let confirm_4 = read_file_and_set_height(
                &(path.to_owned() + "13-confirm-transfer-09268618.json"),
                false,
            );

            let submit_header_5 =
                read_file_and_set_height(&(path.to_owned() + "14-headers-bslk.json"), false);
            for index in 0..submit_header_5.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_5.clone(),
                    index
                ));
            }

            // can confirm in random order within a step
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_2[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_4[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_3[0].clone()
            ));

            //no rewards paid after step was confirmed
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            let submit_header_6 =
                read_file_and_set_height(&(path.to_owned() + "17-headers-roco.json"), false);
            for index in 0..submit_header_6.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_6.clone(),
                    index
                ));
            }

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            let confirm_5 = read_file_and_set_height(
                &(path.to_owned() + "18-confirm-transfer-58c5be47.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_5[0].clone()
            ));

            let submit_header_7 =
                read_file_and_set_height(&(path.to_owned() + "19-headers-roco.json"), false);
            for index in 0..submit_header_7.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_7.clone(),
                    index
                ));
            }

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            let submit_header_8 =
                read_file_and_set_height(&(path.to_owned() + "20-headers-bslk.json"), false);
            for index in 0..submit_header_8.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_8.clone(),
                    index
                ));
            }

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                13u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                8u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                7u128 * 10u128.pow(12)
            );

            let confirm_6 = read_file_and_set_height(
                &(path.to_owned() + "21-confirm-transfer-f6307e35.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_6[0].clone()
            ));

            let submit_header_8 =
                read_file_and_set_height(&(path.to_owned() + "22-headers-roco.json"), false);
            for index in 0..submit_header_8.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_8.clone(),
                    index
                ));
            }

            let confirm_7 = read_file_and_set_height(
                &(path.to_owned() + "23-confirm-transfer-cee25b9a.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_7[0].clone()
            ));

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 12u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                28u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn insured_multi_rococo_multiple_executors() {
    let path = "insured_multi_chain_rococo/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.01111",
    //             bond: "1",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "roco",
    //             type: "tran",
    //             receiver: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
    //             amount: "0.02222",
    //             bond: "2",
    //             reward: "2",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "12",
    //             bond: "1",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "11",
    //             bond: "2",
    //             reward: "3",
    //             signature: null,
    //             executioner: null
    //         },
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10);
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("register bslk --export -o 3-register-bslk", 30)
    // await execute("submit-headers roco --export -o 4-headers-roco", 10);
    // await execute("submit-headers bslk --export -o 5-headers-bslk", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 6-submit-transfer", 90);
    // await execute("submit-headers roco --export -o 11-headers-roco", 10);
    // await execute("submit-headers bslk --export -o 14-headers-bslk", 10);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 20 * 10u128.pow(12));
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 20 * 10u128.pow(12));
            let _ = Balances::deposit_creating(&EXECUTOR_SECOND, 20 * 10u128.pow(12));

            let register_roco =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_roco[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }

            let register_bslk =
                read_file_and_set_height(&(path.to_owned() + "3-register-bslk.json"), false);
            assert_ok!(register(Origin::root(), register_bslk[0].clone(), true));

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            let submit_header_3 =
                read_file_and_set_height(&(path.to_owned() + "5-headers-bslk.json"), false);
            for index in 0..submit_header_3.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_3.clone(),
                    index
                ));
            }

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "6-submit-transfer.json"), false);

            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), 15u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                20u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(EXECUTOR_SECOND),
                20u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                5u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_SECOND),
                0u128 * 10u128.pow(12)
            );

            // bslk bonds
            let bond_insurance_1 = read_file_and_set_height(
                &(path.to_owned() + "8-bond-insurance-6e724b39.json"),
                true,
            );
            let bond_insurance_2 = read_file_and_set_height(
                &(path.to_owned() + "9-bond-insurance-c29dce66.json"),
                true,
            );
            // // roco bonds
            let bond_insurance_3 = read_file_and_set_height(
                &(path.to_owned() + "10-bond-insurance-3a7e3223.json"),
                true,
            );
            let bond_insurance_4 = read_file_and_set_height(
                &(path.to_owned() + "7-bond-insurance-09268618.json"),
                true,
            );

            // Bond can be submitted in arbitrary order, by different executors
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_SECOND),
                bond_insurance_3[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_SECOND),
                bond_insurance_4[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_2[0].clone()
            ));
            assert_ok!(bid_sfx(
                Origin::signed(EXECUTOR_DEFAULT),
                bond_insurance_1[0].clone()
            ));

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 15u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                17u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(EXECUTOR_SECOND),
                17u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                5u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                3u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_SECOND),
                3u128 * 10u128.pow(12)
            );

            let submit_header_4 =
                read_file_and_set_height(&(path.to_owned() + "11-headers-roco.json"), false);
            for index in 0..submit_header_4.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_4.clone(),
                    index
                ));
            }

            let submit_header_5 =
                read_file_and_set_height(&(path.to_owned() + "14-headers-bslk.json"), false);
            for index in 0..submit_header_5.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_5.clone(),
                    index
                ));
            }

            let confirm_1 = read_file_and_set_height(
                &(path.to_owned() + "16-confirm-transfer-09268618.json"),
                false,
            );
            let confirm_2 = read_file_and_set_height(
                &(path.to_owned() + "12-confirm-transfer-3a7e3223.json"),
                false,
            );
            let confirm_3 = read_file_and_set_height(
                &(path.to_owned() + "13-confirm-transfer-6e724b39.json"),
                false,
            );
            let confirm_4 = read_file_and_set_height(
                &(path.to_owned() + "15-confirm-transfer-c29dce66.json"),
                false,
            );

            // can confirm with all bonds paid
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_1[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm_4[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_SECOND),
                confirm_2[0].clone()
            ));
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_SECOND),
                confirm_3[0].clone()
            ));

            assert_eq!(Balances::free_balance(CLI_DEFAULT), 15u128 * 10u128.pow(12));
            assert_eq!(
                Balances::free_balance(EXECUTOR_DEFAULT),
                23u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::free_balance(EXECUTOR_SECOND),
                22u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&CLI_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_DEFAULT),
                0u128 * 10u128.pow(12)
            );
            assert_eq!(
                Balances::reserved_balance(&EXECUTOR_SECOND),
                0u128 * 10u128.pow(12)
            );
        });
}

#[test]
#[ignore]
fn uninsured_unrewarded_parachain_transfer() {
    let path = "uninsured_unrewarded_parachain_transfer/";
    // generated via CLI with:
    // export default {
    //     sideEffects: [
    //         {
    //             target: "bslk",
    //             type: "tran",
    //             receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
    //             amount: "10",
    //             bond: "0",
    //             reward: "0",
    //             signature: null,
    //             executioner: null
    //         },
    //     ],
    //     sequential: false,
    // }
    // await execute("register roco --export -o 1-register-roco", 10)
    // await execute("submit-headers roco --export -o 2-headers-roco", 15);
    // await execute("register bslk --export -o 3-register-bslk", 10)
    // await execute("submit-headers roco --export -o 4-headers-roco", 15);
    // await execute("submit-headers bslk --export -o 5-headers-blsk", 15);
    // await execute("submit-side-effects config/transfer.ts -e -o 6-submit-transfer", 80);
    // await execute("submit-headers roco --export -o 7-headers-roco", 5);
    // await execute("submit-headers bslk --export -o 8-headers-bslk", 5);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, 1);
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, 1);
            // Read data from files
            let register_values =
                read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 =
                read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"), false);
            for index in 0..submit_header_1.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            }

            let register_values =
                read_file_and_set_height(&(path.to_owned() + "3-register-bslk.json"), false);
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_2 =
                read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"), false);
            for index in 0..submit_header_2.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            }

            let submit_header_3 =
                read_file_and_set_height(&(path.to_owned() + "5-headers-bslk.json"), false);
            for index in 0..submit_header_3.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_3.clone(),
                    index
                ));
            }

            let transfer =
                read_file_and_set_height(&(path.to_owned() + "6-submit-transfer.json"), false);
            assert_ok!(on_extrinsic_trigger(
                Origin::signed(CLI_DEFAULT),
                transfer[0].clone()
            ));

            let submit_header_4 =
                read_file_and_set_height(&(path.to_owned() + "7-headers-roco.json"), false);
            for index in 0..submit_header_4.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_4.clone(),
                    index
                ));
            }

            let submit_header_5 =
                read_file_and_set_height(&(path.to_owned() + "8-headers-bslk.json"), false);
            for index in 0..submit_header_5.as_array().unwrap().len() {
                // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_5.clone(),
                    index
                ));
            }

            let confirm = read_file_and_set_height(
                &(path.to_owned() + "9-confirm-transfer-b29a43e5.json"),
                false,
            );
            assert_ok!(confirm_side_effect(
                Origin::signed(EXECUTOR_DEFAULT),
                confirm[0].clone()
            ));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 1);
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), 1);
        });
}

#[test]
fn sdk_basic_success() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            // then it sets up some side effects
            let trigger = LocalTrigger::new(
                DJANGO,
                vec![Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Transfer {
                    caller: ALICE,
                    to: CHARLIE,
                    amount: 50,
                    insurance: None,
                })
                .encode()],
                Some(res.xtx_id),
            );

            System::set_block_number(1);
            brute_seed_block_1(*b"pdot");

            // then it submits to circuit
            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(&origin, trigger)
            );

            System::set_block_number(10);

            // submits a signal
            let signal =
                ExecutionSignal::new(&res.xtx_id, Some(res.steps_cnt.0), SignalKind::Complete);
            assert_ok!(Circuit::on_signal(&origin, signal.clone()));

            // validate the state
            check_queue(QueueValidator::Elements(vec![(ALICE, signal)]));

            // async process the signal
            System::set_block_number(100);
            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(100);

            // no signal left
            check_queue(QueueValidator::Length(0));
        });
}

#[test]
#[ignore]
fn sdk_can_send_multiple_states() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            System::set_block_number(1);
            brute_seed_block_1(*b"pdot");

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Transfer {
                            caller: ALICE,
                            to: CHARLIE,
                            amount: 50,
                            insurance: None
                        })
                        .encode()],
                        Some(res.xtx_id),
                    )
                )
            );

            System::set_block_number(10);
            brute_seed_block_1(*b"ksma");

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![Chain::<_, u128, [u8; 32]>::Kusama(Operation::Transfer {
                            caller: ALICE,
                            to: DJANGO,
                            amount: 1,
                            insurance: None
                        })
                        .encode()],
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

#[test]
fn transfer_is_validated_correctly() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            System::set_block_number(1);
            brute_seed_block_1(*b"pdot");

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Transfer {
                            caller: ALICE,
                            to: CHARLIE,
                            amount: 50,
                            insurance: None
                        })
                        .encode()],
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

#[test]
fn swap_is_validated_correctly() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            System::set_block_number(1);
            brute_seed_block_1(*b"pdot");

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Swap {
                            caller: ALICE,
                            to: CHARLIE,
                            amount_from: 100,
                            amount_to: 10,
                            asset_from: [7_u8; 32],
                            asset_to: [8_u8; 32],
                            insurance: None
                        })
                        .encode()],
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

#[test]
fn add_liquidity_is_validated_correctly() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            System::set_block_number(1);
            brute_seed_block_1(*b"pdot");

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, Balance>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![Chain::<_, u128, _>::Polkadot(Operation::AddLiquidity {
                            caller: ALICE,
                            to: CHARLIE,
                            asset_left: [7_u8; 32],
                            asset_right: [8_u8; 32],
                            liquidity_token: [9_u8; 32],
                            amount_left: 100,
                            amount_right: 10,
                            amount_liquidity_token: 100,
                            insurance: None,
                        })
                        .encode()],
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

use t3rn_sdk_primitives::{
    storage::BoundedVec,
    xc::{Call as CallVM, Operation},
};

// TODO: this fails because the side effect doesnt work for the gateway, will be fixed in the future
#[ignore]
#[test]
fn call_to_vm_is_validated_correctly() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, Balance>>::on_local_trigger(
                    &origin,
                    LocalTrigger::new(
                        DJANGO,
                        vec![
                            Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Call(Box::new(
                                CallVM {
                                    caller: ALICE,
                                    call: t3rn_sdk_primitives::xc::VM::Evm {
                                        dest: BOB_RELAYER,
                                        value: 1,
                                    },
                                    data: BoundedVec::default(),
                                }
                            )))
                            .encode()
                        ],
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

#[test]
fn into_se_from_chain() {
    let ch = Chain::<_, u128, [u8; 32]>::Polkadot(Operation::Transfer {
        caller: ALICE,
        to: CHARLIE,
        amount: 50,
        insurance: None,
    })
    .encode();

    let se = SideEffect::<[u8; 32], u128>::try_from(ch).unwrap();

    assert_eq!(
        se,
        SideEffect {
            target: [112u8, 100u8, 111u8, 116u8],
            max_reward: 0,
            insurance: 0,
            encoded_action: vec![116, 114, 97, 110],
            encoded_args: vec![
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1
                ],
                vec![
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3
                ],
                vec![50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![]
            ],
            signature: vec![],
            enforce_executor: None,
        }
    )
}

#[test]
fn post_kill_signal_updates_states() {}

enum QueueValidator {
    Length(usize),
    Elements(
        Vec<(
            AccountId32,
            ExecutionSignal<<Runtime as frame_system::Config>::Hash>,
        )>,
    ),
}
fn check_queue(validation: QueueValidator) {
    let q = Circuit::get_signal_queue();

    match validation {
        QueueValidator::Length(len) => {
            assert_eq!(q.len(), len);
        },
        QueueValidator::Elements(elements) => {
            assert_eq!(q.into_inner(), elements);
        },
    }
}

fn setup_fresh_state(origin: &Origin) -> LocalStateExecutionView<Runtime, Balance> {
    let res = Circuit::load_local_state(origin, None).unwrap();
    assert_ne!(Some(res.xtx_id), None);
    res
}

/// XBI
const INITIAL_BALANCE: Balance = 50;
const MAX_EXECUTION_COST: Balance = 1;
const MAX_NOTIFICATION_COST: Balance = 2;
#[test]
fn execute_side_effects_with_xbi_works_for_transfers() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();
    let mut valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, max_fee = 1
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    valid_transfer_side_effect.target = [3, 3, 3, 3];

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    const MAX_FEE: Balance = 1;
    const INSURANCE: Balance = 1;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // XTX SETUP

            let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE);

            System::set_block_number(1);
            brute_seed_block_1([3, 3, 3, 3]);

            let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);

            let sfx_id_a = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                &xtx_id.0,
                FIRST_SFX_INDEX,
            );

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin.clone(),
                side_effects,
                fee,
                sequential,
            ));

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: ALICE,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Escrow,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            place_winning_bid_and_advance_3_blocks(ALICE, xtx_id, sfx_id_a, MAX_FEE);

            assert_ok!(Circuit::execute_side_effects_with_xbi(
                origin,
                xtx_id,
                valid_transfer_side_effect,
                MAX_EXECUTION_COST as Balance,
                MAX_NOTIFICATION_COST as Balance,
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                INITIAL_BALANCE
                    - MAX_EXECUTION_COST
                    - MAX_NOTIFICATION_COST
                    - 2 * MAX_FEE
                    - INSURANCE
            );
        });
}

#[test]
fn execute_side_effects_with_xbi_works_for_call_evm() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let xbi_evm = XBIFormat {
        instr: XBIInstr::CallEvm {
            source: AccountId20::repeat_byte(3),
            target: AccountId20::repeat_byte(2),
            value: sp_core::U256([1, 0, 0, 0]),
            input: vec![8, 9],
            gas_limit: 2,
            max_fee_per_gas: sp_core::U256([4, 5, 6, 7]),
            max_priority_fee_per_gas: None,
            nonce: Some(sp_core::U256([3, 4, 6, 7])),
            access_list: vec![],
        },
        metadata: XBIMetadata {
            id: sp_core::H256::repeat_byte(2),
            dest_para_id: 3333u32,
            src_para_id: 4u32,
            sent: ActionNotificationTimeouts {
                action: 1u32,
                notification: 2u32,
            },
            delivered: ActionNotificationTimeouts {
                action: 3u32,
                notification: 4u32,
            },
            executed: ActionNotificationTimeouts {
                action: 4u32,
                notification: 5u32,
            },
            max_exec_cost: 6u128,
            max_notifications_cost: 8u128,
            maybe_known_origin: None,
            actual_aggregated_cost: None,
        },
    };

    let mut valid_evm_sfx = xbi_2_sfx::<
        Runtime,
        <Runtime as circuit_runtime_pallets::pallet_circuit::Config>::Escrowed,
    >(
        xbi_evm,
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        1,
        1,
    )
    .unwrap();

    // assert target
    valid_evm_sfx.target = [1u8, 1u8, 1u8, 1u8];
    let side_effects = vec![valid_evm_sfx.clone()];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // XTX SETUP

            let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([3, 3, 3, 3]);
            brute_seed_block_1([1, 1, 1, 1]);

            let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin.clone(),
                side_effects,
                fee,
                sequential,
            ));

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: ALICE,
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    total_reward: Some(fee),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_evm_sfx.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Escrow,
                    submission_target_height: vec![0],
                    index: FIRST_SFX_INDEX,
                }]]
            );

            place_winning_bid_and_advance_3_blocks(
                ALICE,
                xtx_id,
                valid_evm_sfx
                    .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                        &xtx_id.0, 0,
                    ),
                1,
            );

            assert_ok!(Circuit::execute_side_effects_with_xbi(
                origin,
                xtx_id,
                valid_evm_sfx,
                MAX_EXECUTION_COST as Balance,
                MAX_NOTIFICATION_COST as Balance,
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                INITIAL_BALANCE - MAX_EXECUTION_COST - MAX_NOTIFICATION_COST - 3
            );
        });
}
#[test]
fn no_duplicate_xtx_and_sfx_ids() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::C), // insurance = 3, max_reward/reward = 3
        ],
        &mut local_state,
        transfer_protocol_box,
        ALICE,
        FIRST_REQUESTER_NONCE,
        FIRST_SFX_INDEX,
    );

    let expected_xtx_id_1 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
    let expected_xtx_id_2 = generate_xtx_id::<Hashing>(ALICE, SECOND_REQUESTER_NONCE);

    let expected_sfx_id_1 = valid_transfer_side_effect
        .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
        &expected_xtx_id_1.0,
        0,
    );

    let expected_sfx_id_2 = valid_transfer_side_effect
        .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
        &expected_xtx_id_2.0,
        0,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 6); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin.clone(),
                side_effects.clone(),
                fee,
                sequential,
            ));

            let events = System::events();
            assert_eq!(
                events[4],
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                        Runtime,
                    >::NewSideEffectsAvailable(
                        AccountId32::new(hex!(
                            "0101010101010101010101010101010101010101010101010101010101010101"
                        )),
                        expected_xtx_id_1,
                        side_effects.clone(),
                        vec![expected_sfx_id_1],
                    )),
                    topics: vec![]
                }
            );

            // manually increment nonce to simulate production environment
            frame_system::Pallet::<Runtime>::inc_account_nonce(ALICE);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects.clone(),
                fee,
                sequential,
            ));

            let next_events = System::events();
            assert_eq!(
                next_events[7],
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                        Runtime,
                    >::NewSideEffectsAvailable(
                        AccountId32::new(hex!(
                            "0101010101010101010101010101010101010101010101010101010101010101"
                        )),
                        expected_xtx_id_2,
                        side_effects,
                        vec![expected_sfx_id_2],
                    )),
                    topics: vec![]
                }
            );

            assert_ne!(expected_xtx_id_1, expected_xtx_id_2);
            assert_ne!(expected_sfx_id_1, expected_sfx_id_2);
        });
}
