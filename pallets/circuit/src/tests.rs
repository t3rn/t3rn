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

//! Test utilities
use crate::{
    mock::*,
    sdk_primitives::{
        signal::ExecutionSignal,
        xc::{Chain, Operation},
    },
    state::*,
    SignalKind,
    Error
};
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok, dispatch::PostDispatchInfo, traits::Currency};
use frame_system::{pallet_prelude::OriginFor, EventRecord, Phase};
use pallet_circuit_portal::bp_circuit;
use serde_json::Value;
use sp_io::TestExternalities;
use sp_runtime::{traits::Header, AccountId32, DispatchErrorWithPostInfo};
use sp_std::prelude::*;
use std::{convert::TryInto, fs};
use t3rn_primitives::{
    abi::*,
    circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
    side_effect::*,
    volatile::LocalState,
    xdns::AllowedSideEffect,
    xtx::XtxId,
    ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};
use t3rn_protocol::side_effects::test_utils::*;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB_RELAYER: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

fn set_ids(
    valid_side_effect: SideEffect<AccountId32, BlockNumber, BalanceOf>,
) -> (sp_core::H256, sp_core::H256) {
    let xtx_id: sp_core::H256 =
        hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

    let side_effect_a_id = valid_side_effect.generate_id::<crate::SystemHashing<Test>>();

    (xtx_id, side_effect_a_id)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

pub fn brute_seed_block_1_to_grandpa_mfv(gateway_id: [u8; 4]) {
    // Brute update storage of MFV::MultiImportedHeaders to blockA = 1 and BestAvailable -> blockA
    let block_hash_1 = sp_core::H256::repeat_byte(1);
    let header_1: bp_circuit::Header = bp_circuit::Header::new(
        1,
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );

    <pallet_multi_finality_verifier::MultiImportedHeaders<Test>>::insert::<
        [u8; 4],
        sp_core::H256,
        bp_circuit::Header,
    >(gateway_id, block_hash_1, header_1);

    <pallet_multi_finality_verifier::BestFinalizedMap<Test>>::insert::<[u8; 4], sp_core::H256>(
        gateway_id,
        block_hash_1,
    );
}

fn register_file(
    origin: OriginFor<Test>,
    file: &str,
    valid: bool,
    index: usize,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let raw_data = fs::read_to_string("./src/mock-data/".to_owned() + file).unwrap();
    let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
    register(origin, json[index].clone(), valid)
}

fn register(
    origin: OriginFor<Test>,
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
        url.clone(),
        gateway_id.clone(),
        gateway_abi.clone(),
        gateway_vendor.clone(),
        gateway_type.clone(),
        gateway_genesis.clone(),
        gateway_sys_props.clone(),
        allowed_side_effects.clone(),
        encoded_registration_data.clone(),
    );

    if valid {
        let xdns_record = pallet_xdns::XDNSRegistry::<Test>::get(gateway_id).unwrap();
        let stored_side_effects = xdns_record.allowed_side_effects;

        // ensure XDNS writes are correct
        assert_eq!(stored_side_effects, allowed_side_effects);
        assert_eq!(xdns_record.gateway_vendor, gateway_vendor);
        assert_eq!(xdns_record.gateway_abi, gateway_abi);
        assert_eq!(xdns_record.gateway_type, gateway_type);
        assert_eq!(xdns_record.gateway_sys_props, gateway_sys_props);
        assert_eq!(xdns_record.gateway_genesis, gateway_genesis);
    }

    return res
}

fn submit_header_file(
    origin: OriginFor<Test>,
    file: &str,
    index: usize, //might have an index (for relaychains)
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let raw_data = fs::read_to_string("./src/mock-data/".to_owned() + file).unwrap();
    let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
    submit_headers(origin, json, index)
}

fn submit_headers(
    origin: OriginFor<Test>,
    json: Value,
    index: usize,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let encoded_header_data: Vec<u8> =
        hex::decode(json[index]["encoded_data"].as_str().unwrap()).unwrap();
    let gateway_id: ChainId = Decode::decode(
        &mut &*hex::decode(json[index]["encoded_gateway_id"].as_str().unwrap()).unwrap(),
    )
    .unwrap();
    Portal::submit_headers(origin, gateway_id, encoded_header_data)
}

fn on_extrinsic_trigger(
    origin: OriginFor<Test>,
    json: Value,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let side_effects: Vec<SideEffect<AccountId32, BlockNumber, BalanceOf>> =
        Decode::decode(&mut &*hex::decode(json["encoded_side_effects"].as_str().unwrap()).unwrap())
            .unwrap();

    let fee = 0;
    let sequential: bool =
        Decode::decode(&mut &*hex::decode(json["encoded_sequential"].as_str().unwrap()).unwrap())
            .unwrap();
    Circuit::on_extrinsic_trigger(origin, side_effects, fee, sequential)
}

fn confirm_side_effect(
    origin: OriginFor<Test>,
    json: Value,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let xtx_id: sp_core::H256 =
        Decode::decode(&mut &*hex::decode(json["encoded_xtx_id"].as_str().unwrap()).unwrap())
            .unwrap();
    let side_effect: SideEffect<AccountId32, BlockNumber, BalanceOf> =
        Decode::decode(&mut &*hex::decode(json["encoded_side_effect"].as_str().unwrap()).unwrap())
            .unwrap();
    let confirmed_side_effect: ConfirmedSideEffect<AccountId32, BlockNumber, BalanceOf> =
        Decode::decode(&mut &*hex::decode(json["encoded_confirmed"].as_str().unwrap()).unwrap())
            .unwrap();

    Circuit::confirm_side_effect(
        origin,
        xtx_id,
        side_effect,
        confirmed_side_effect,
        None,
        None,
    )
}

pub fn bond_insurance_deposit(
    origin: OriginFor<Test>,
    json: Value
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let xtx_id: sp_core::H256 =
        Decode::decode(&mut &*hex::decode(json["encoded_xtx_id"].as_str().unwrap()).unwrap())
            .unwrap();

    let side_effect_id: SideEffectId<Test> =
        Decode::decode(&mut &*hex::decode(json["encoded_id"].as_str().unwrap()).unwrap())
            .unwrap();

    Circuit::bond_insurance_deposit(
        origin, // Active relayer
        xtx_id,
        side_effect_id,
    )
}

fn read_file_and_set_height(
    path: &str
) -> Value {
    let file = fs::read_to_string("src/mock-data/".to_owned() + path).unwrap();
    let json: Value = serde_json::from_str(file.as_str()).unwrap();
     for entry in json.as_array().unwrap() {
        let submission_height: u64 = entry["submission_height"].as_u64().unwrap();
        if submission_height > 0 {
            System::set_block_number(submission_height.try_into().unwrap());
        }
     }
    json
}

// iterates sequentially though all test files in mock-data
fn run_mock_tests(
    path: &str
) -> Result<(), DispatchErrorWithPostInfo<PostDispatchInfo>> {
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
    Ok(().into())
}

#[test]
fn runs_mock_tests() {
    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = run_mock_tests("auto");
        });
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
        prize: 1,
        ordered_at: 0,
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
        enforce_executioner: Some(
            [
                53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85, 53,
                110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
            ]
            .into(),
        ),
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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_works_with_single_transfer_not_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            // Assert Circuit::emit generates 5 correct events: 3 from charging and 2 Circuit-specific
            let events = System::events();
            assert_eq!(events.len(), 8);
            assert_eq!(
                vec![events[6].clone(), events[7].clone()],
                vec![
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                prize: 0,
                                ordered_at: 0,
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
                                    vec![]
                                ],
                                signature: vec![],
                                enforce_executioner: None
                            }],
                            vec![hex!(
                                "84a5512d2a624231c0d3748ec11a94d01d9366d310f057f12913e40c1267b4e1"
                            )
                            .into(),],
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::XTransactionReadyForExec(
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                ]
            );
            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();
            let side_effect_a_id =
                valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
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
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                    security_lvl: SecurityLvl::Dirty,
                    submission_target_height: vec![0],
                }]]
            );
        });
}

#[test]
fn on_extrinsic_trigger_validation_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_emit_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            // Assert Circuit::emit generates 5 correct events: 3 for charging and 2 Circuit-specific
            let events = System::events();
            assert_eq!(events.len(), 10);
            assert_eq!(
                vec![events[8].clone(), events[9].clone()],
                vec![
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                prize: 0,
                                ordered_at: 0,
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
                                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                                    ]
                                ],
                                signature: vec![],
                                enforce_executioner: None
                            }],
                            vec![hex!(
                                "cf58f0709ebeac8cc014467972fe1eaa88355b41c02c4dfa1b0608313849c3be"
                            )
                            .into(),],
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::XTransactionReceivedForExec(
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
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
fn on_extrinsic_trigger_apply_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_transfer_side_effect.clone());

            // Test Apply State
            // Returns void insurance for that side effect
            let valid_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: None,
                status: CircuitStatus::Requested,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                valid_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1)
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                }]]
            );
        });
}

#[test]
fn circuit_handles_insurance_deposit_for_transfers() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, reward = 2
        ],
        &mut local_state,
        transfer_protocol_box,
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
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_transfer_side_effect.clone());

            // Test Apply State
            // Returns void insurance for that side effect
            let valid_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: None,
                status: CircuitStatus::Requested,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                valid_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                }]]
            );

            let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

            assert_ok!(Circuit::bond_insurance_deposit(
                origin_relayer_bob.clone(),
                xtx_id,
                side_effect_a_id,
            ));

            let expected_bonded_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: Some(BOB_RELAYER),
                status: CircuitStatus::Bonded,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                expected_bonded_insurance_deposit
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
                    steps_cnt: (0, 1),
                }
            );

            // Confirmation start
            let mut encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer {
                from: hex!("0909090909090909090909090909090909090909090909090909090909090909")
                    .into(), // variant A
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 1, // variant A
            }
            .encode();

            // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_balance_transfer_event);

            // let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
            //     err: None,
            //     output: None,
            //     encoded_effect: encoded_event,
            //     inclusion_proof: None,
            //     executioner: BOB_RELAYER,
            //     received_at: 0,
            //     cost: None,
            // };
            //
            // // Update MFV::MultiImportedHeaders
            // assert_ok!(Circuit::confirm_side_effect(
            //     origin_relayer_bob,
            //     xtx_id,
            //     valid_transfer_side_effect,
            //     confirmation,
            //     None,
            //     None,
            // ));

            // Check that Bob collected the relayer reward
            // assert_eq!(Balances::free_balance(&BOB_RELAYER), 1 + 2);
        });
}

#[test]
fn circuit_handles_dirty_swap_with_no_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
    let swap_protocol_box = ExtBuilder::get_swap_protocol_box();

    let mut local_state = LocalState::new();

    let valid_swap_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A), // caller
            (Type::Address(32), ArgVariant::B), // to
            (Type::Uint(128), ArgVariant::A),   // amount_from
            (Type::Uint(128), ArgVariant::B),   // amount_to
            (Type::Bytes(4), ArgVariant::A),    // asset_from
            (Type::Bytes(4), ArgVariant::B),    // asset_to
            (Type::Bytes(0), ArgVariant::A),    // empty bytes instead of insurance
        ],
        &mut local_state,
        swap_protocol_box,
    );

    let side_effects = vec![valid_swap_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_swap_side_effect.clone());

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
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_swap_side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Dirty,
                    submission_target_height: vec![0],
                }]]
            );

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                None
            );

            // Confirmation start
            let mut encoded_swap_transfer_event = orml_tokens::Event::<Test>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 2u128, // amount - variant B
            }
            .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_swap_transfer_event);

            // let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
            //     err: None,
            //     output: None,
            //     encoded_effect: encoded_event,
            //     inclusion_proof: None,
            //     executioner: BOB_RELAYER,
            //     received_at: 0,
            //     cost: None,
            // };
            //
            // assert_ok!(Circuit::confirm_side_effect(
            //     origin_relayer_bob,
            //     xtx_id,
            //     valid_swap_side_effect,
            //     confirmation,
            //     None,
            //     None,
            // ));
        });
}

#[test]
fn circuit_handles_swap_with_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let ext = ExtBuilder::default();

    let mut local_state = LocalState::new();
    let swap_protocol_box = ExtBuilder::get_swap_protocol_box();
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
    );

    let side_effects = vec![valid_swap_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext.with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_swap_side_effect.clone());

            // Test Apply State
            // Returns valid insurance for that side effect
            let valid_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: None,
                status: CircuitStatus::Requested,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                valid_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_swap_side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                }]]
            );

            let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

            assert_ok!(Circuit::bond_insurance_deposit(
                origin_relayer_bob.clone(),
                xtx_id,
                side_effect_a_id,
            ));

            let expected_bonded_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: Some(BOB_RELAYER),
                status: CircuitStatus::Bonded,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                expected_bonded_insurance_deposit
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
                    steps_cnt: (0, 1),
                }
            );

            // Confirmation start
            let mut encoded_swap_transfer_event = orml_tokens::Event::<Test>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 2u128, // amount - variant B
            }
            .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_swap_transfer_event);
            //
            // let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
            //     err: None,
            //     output: None,
            //     encoded_effect: encoded_event,
            //     inclusion_proof: None,
            //     executioner: BOB_RELAYER,
            //     received_at: 0,
            //     cost: None,
            // };
            //
            // assert_ok!(Circuit::confirm_side_effect(
            //     origin_relayer_bob,
            //     xtx_id,
            //     valid_swap_side_effect,
            //     confirmation,
            //     None,
            //     None,
            // ));

            // assert_eq!(Balances::free_balance(&BOB_RELAYER), 1 + 2);
        });
}

#[test]
fn circuit_handles_add_liquidity_without_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();
    let mut local_state = LocalState::new();

    let add_liquidity_protocol_box = ExtBuilder::get_add_liquidity_protocol_box();

    let valid_add_liquidity_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A), // argument_0: caller
            (Type::Address(32), ArgVariant::B), // argument_1: to
            (Type::Bytes(4), ArgVariant::A),    // argument_2: asset_left
            (Type::Bytes(4), ArgVariant::B),    // argument_3: asset_right
            (Type::Bytes(4), ArgVariant::C),    // argument_4: liquidity_token
            (Type::Uint(128), ArgVariant::A),   // argument_5: amount_left
            (Type::Uint(128), ArgVariant::B),   // argument_6: amount_right
            (Type::Uint(128), ArgVariant::A),   // argument_7: amount_liquidity_token
            (Type::Bytes(0), ArgVariant::A),    // argument_8: no insurance, empty bytes
        ],
        &mut local_state,
        add_liquidity_protocol_box,
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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_add_liquidity_side_effect.clone());

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                None
            );

            let events = System::events();

            // 5 events: new account, endowed, transfer, xtransactionreadytoexec, newsideeffectavailable
            assert_eq!(events.len(), 11);

            // Confirmation start
            let mut encoded_add_liquidity_transfer_event = orml_tokens::Event::<Test>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 1u128, // amount - variant B
            }
            .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_add_liquidity_transfer_event);

            // let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
            //     err: None,
            //     output: None,
            //     encoded_effect: encoded_event,
            //     inclusion_proof: None,
            //     executioner: BOB_RELAYER,
            //     received_at: 0,
            //     cost: None,
            // };
            //
            // assert_ok!(Circuit::confirm_side_effect(
            //     origin_relayer_bob,
            //     xtx_id,
            //     valid_add_liquidity_side_effect,
            //     confirmation,
            //     None,
            //     None,
            // ));
        });
}

#[test]
fn circuit_handles_add_liquidity_with_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();
    let mut local_state = LocalState::new();

    let add_liquidity_protocol_box = ExtBuilder::get_add_liquidity_protocol_box();

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
            (Type::OptionalInsurance, ArgVariant::A), // argument_8: no insurance, empty bytes
        ],
        &mut local_state,
        add_liquidity_protocol_box,
    );

    let side_effects = vec![valid_add_liquidity_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext.with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(valid_add_liquidity_side_effect.clone());

            // Test Apply State
            // Returns valid insurance for that side effect
            let valid_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: None,
                status: CircuitStatus::Requested,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                valid_insurance_deposit
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_add_liquidity_side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: vec![0],
                }]]
            );

            let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

            assert_ok!(Circuit::bond_insurance_deposit(
                origin_relayer_bob.clone(),
                xtx_id,
                side_effect_a_id,
            ));

            let expected_bonded_insurance_deposit = InsuranceDeposit {
                insurance: 1,
                reward: 2,
                requester: AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )),
                bonded_relayer: Some(BOB_RELAYER),
                status: CircuitStatus::Bonded,
                requested_at: 1,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id).unwrap(),
                expected_bonded_insurance_deposit
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
                    steps_cnt: (0, 1),
                }
            );
        });
}

fn successfully_bond_optimistic(
    side_effect: SideEffect<AccountId32, BlockNumber, BalanceOf>,
    xtx_id: XtxId<Test>,
    relayer: AccountId32,
    submitter: AccountId32,
) {
    let optional_insurance = side_effect.encoded_args[3].clone();

    assert!(
        optional_insurance.len() == 32,
        "Wrong test value - optimistic transfer assumes optimistic arguments"
    );

    assert_ok!(Circuit::bond_insurance_deposit(
        Origin::signed(relayer.clone()),
        xtx_id,
        side_effect.generate_id::<crate::SystemHashing<Test>>(),
    ));

    let [insurance, reward]: [u128; 2] = Decode::decode(&mut &optional_insurance[..]).unwrap();

    assert_eq!(
        Circuit::get_insurance_deposits(
            xtx_id,
            side_effect.generate_id::<crate::SystemHashing<Test>>()
        )
        .unwrap(),
        InsuranceDeposit {
            insurance: insurance as u128,
            reward: reward as u128,
            requester: Decode::decode(&mut &submitter.encode()[..]).unwrap(),
            bonded_relayer: Some(relayer.clone()),
            status: CircuitStatus::Bonded,
            requested_at: 1,
        }
    );
}

#[test]
fn two_dirty_transfers_are_allocated_to_2_steps_and_can_be_submitted() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::B),
            (Type::Address(32), ArgVariant::A),
            (Type::Uint(128), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
    );

    let side_effects = vec![
        valid_transfer_side_effect_1.clone(),
        valid_transfer_side_effect_2.clone(),
    ];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 10);

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let events = System::events();
            assert_eq!(events.len(), 8);
        });
}

// ToDo: Order for multiple should now be fixed - verify t3rn#261 is solved
#[test]
fn circuit_handles_transfer_dirty_and_optimistic_and_swap() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();
    let swap_protocol_box = ExtBuilder::get_swap_protocol_box();

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
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
    );

    let valid_swap_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A), // caller
            (Type::Address(32), ArgVariant::B), // to
            (Type::Uint(128), ArgVariant::A),   // amount_from
            (Type::Uint(128), ArgVariant::B),   // amount_to
            (Type::Bytes(4), ArgVariant::A),    // asset_from
            (Type::Bytes(4), ArgVariant::B),    // asset_to
            (Type::Bytes(0), ArgVariant::A),    // no insurance
        ],
        &mut local_state,
        swap_protocol_box,
    );

    let side_effects = vec![
        valid_transfer_side_effect_1.clone(),
        valid_transfer_side_effect_2.clone(),
        valid_swap_side_effect.clone(),
    ];
    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 10);

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let events = System::events();
            assert_eq!(events.len(), 9);

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

            // Confirmation start
            let mut encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer {
                from: hex!("0909090909090909090909090909090909090909090909090909090909090909")
                    .into(), // variant A
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 1, // variant A
            }
            .encode();

            // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_balance_transfer_event);

            println!(
                "full side effects before confirmation: {:?}",
                Circuit::get_full_side_effects(xtx_id).unwrap()
            );

            println!(
                "exec signals before confirmation: {:?}",
                Circuit::get_x_exec_signals(xtx_id).unwrap()
            );

            println!(
                "exec signals after 1st confirmation, transfer: {:?}",
                Circuit::get_x_exec_signals(xtx_id).unwrap()
            );

            println!(
                "full side effects after confirmation: {:?}",
                Circuit::get_full_side_effects(xtx_id).unwrap()
            );

            println!(
                "exec signals after confirmation: {:?}",
                Circuit::get_x_exec_signals(xtx_id).unwrap()
            );
        });
}

#[test]
fn circuit_cancels_xtx_after_timeout() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box = ExtBuilder::get_transfer_protocol_box();
    let _swap_protocol_box = ExtBuilder::get_swap_protocol_box();

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = false;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 10);

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let _events = System::events();
            // assert_eq!(events.len(), 8);

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

            // The tiemout links that will be checked at on_initialize are there
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(401u32)); // 400 offset + current block height 1 = 401

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32, // 400 offset + current block height 1 = 401
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                })
            );

            System::set_block_number(410);

            <Circuit as frame_support::traits::OnInitialize<u32>>::on_initialize(410);

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
                    steps_cnt: (0, 1),
                })
            );

            assert_eq!(Circuit::get_active_timing_links(xtx_id), None);

            // Emits event notifying about cancellation
            let mut events = System::events();
            // assert_eq!(events.len(), 9);
            assert_eq!(
                events.pop(),
                Some(EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(
                        crate::Event::<Test>::XTransactionXtxRevertedAfterTimeOut(
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
                            )
                            .into()
                        )
                    ),
                    topics: vec![]
                }),
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
            hex!("b09a43d4886048104b526ce9b29d77e10dd27e263d329888b73562b0b9068a0a").into();
        assert_eq!(res.xtx_id, xtx_id_new);
        assert_eq!(res.local_state, LocalState::new());
        assert_eq!(res.steps_cnt, (0, 0));
    });
}

#[test]
fn unbonded_unrewarded_unsequential_single_rococo_transfer_confirms() {
    let path = "unbonded_unrewarded_unsequential_single_rococo_transfer_confirms/";
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
            let register_values = read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"));
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 = read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"));
            for index in 0..submit_header_1.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            };

            let transfer =  read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"));
            assert_ok!(on_extrinsic_trigger(Origin::signed(CLI_DEFAULT), transfer[0].clone()));

            let submit_header_2 = read_file_and_set_height(&(path.to_owned() + "4-headers-roco.json"));
            for index in 0..submit_header_2.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            };

            let confirm =  read_file_and_set_height(&(path.to_owned() + "5-confirm-transfer-roco.json"));
            assert_ok!(confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), 1);
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), 1);

        });
}

#[test]
fn bonded_unrewarded_unsequential_single_rococo_transfer_confirms() {
    let path = "bonded_unrewarded_unsequential_single_rococo_transfer_confirms/";
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


    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, (10 * 10u128.pow(12)).into()); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, (10 * 10u128.pow(12)).into()); // 10 trn

            // Read data from files
            let register_values = read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"));
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 = read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"));
            for index in 0..submit_header_1.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                 assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            };

            let transfer =  read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"));
            assert_ok!(on_extrinsic_trigger(Origin::signed(CLI_DEFAULT), transfer[0].clone()));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), (10u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(EXECUTOR_DEFAULT), (10u128 * 10u128.pow(12)).into());

            let confirm =  read_file_and_set_height(&(path.to_owned() + "6-confirm-transfer-roco.json"));
            // Can't confirm without header in light client
            assert_noop!(
                confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()),
                "SideEffect confirmation failed!"
            );

            let submit_header_2 = read_file_and_set_height(&(path.to_owned() + "5-headers-roco.json"));
            for index in 0..submit_header_2.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            };

            // Can't confirm without bond posted
            assert_noop!(
                confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()),
                Error::<Test>::ApplyFailed
            );

            let post_bond = read_file_and_set_height(&(path.to_owned() + "4-post-bond-roco.json"));
            assert_ok!(bond_insurance_deposit(Origin::signed(EXECUTOR_DEFAULT), post_bond[0].clone()));

            assert_eq!(Balances::free_balance(&CLI_DEFAULT), (10u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), (9u128 * 10u128.pow(12)).into());

            assert_ok!(confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), (10u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), (10u128 * 10u128.pow(12)).into());

        });
}

#[test]
fn bonded_rewarded_unsequential_single_rococo_transfer_confirms() {
    let path = "bonded_rewarded_unsequential_single_rococo_transfer_confirms/";
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


    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&CLI_DEFAULT, (10 * 10u128.pow(12)).into()); // 10 trn
            let _ = Balances::deposit_creating(&EXECUTOR_DEFAULT, (10 * 10u128.pow(12)).into()); // 10 trn

            // Read data from files
            let register_values = read_file_and_set_height(&(path.to_owned() + "1-register-roco.json"));
            assert_ok!(register(Origin::root(), register_values[0].clone(), true));

            let submit_header_1 = read_file_and_set_height(&(path.to_owned() + "2-headers-roco.json"));
            for index in 0..submit_header_1.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                 assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_1.clone(),
                    index
                ));
            };
            let transfer =  read_file_and_set_height(&(path.to_owned() + "3-submit-transfer.json"));

            assert_ok!(on_extrinsic_trigger(Origin::signed(CLI_DEFAULT), transfer[0].clone()));
            assert_eq!(Balances::free_balance(CLI_DEFAULT), (9u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(EXECUTOR_DEFAULT), (10u128 * 10u128.pow(12)).into());

            let post_bond = read_file_and_set_height(&(path.to_owned() + "4-post-bond-roco.json"));
            assert_ok!(bond_insurance_deposit(Origin::signed(EXECUTOR_DEFAULT), post_bond[0].clone()));

            assert_eq!(Balances::free_balance(&CLI_DEFAULT), (9u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), (9u128 * 10u128.pow(12)).into());

            let submit_header_2 = read_file_and_set_height(&(path.to_owned() + "5-headers-roco.json"));
            for index in 0..submit_header_2.as_array().unwrap().len() { // we have to loop, because this might be seperate transactions
                assert_ok!(submit_headers(
                    Origin::signed(CLI_DEFAULT),
                    submit_header_2.clone(),
                    index
                ));
            };

            let confirm =  read_file_and_set_height(&(path.to_owned() + "6-confirm-transfer-roco.json"));
            assert_ok!(confirm_side_effect(Origin::signed(EXECUTOR_DEFAULT), confirm[0].clone()));
            assert_eq!(Balances::free_balance(&CLI_DEFAULT), (9u128 * 10u128.pow(12)).into());
            assert_eq!(Balances::free_balance(&EXECUTOR_DEFAULT), (11u128 * 10u128.pow(12)).into());

        });
}

#[test]
fn sdk_basic_success() {
    let origin = Origin::signed(ALICE);
    let mut ext = TestExternalities::new_empty();

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 50);

        let res = setup_fresh_state(&origin);

        // then it sets up some side effects
        let trigger = LocalTrigger::new(
            DJANGO,
            vec![Chain::<_, u32, [u8; 32]>::Polkadot(Operation::Transfer {
                caller: ALICE,
                to: CHARLIE,
                amount: 50,
            })
            .encode()],
            Some(res.xtx_id),
        );

        // then it submits to circuit
        assert_ok!(<Circuit as OnLocalTrigger<Test>>::on_local_trigger(
            &origin,
            trigger.clone()
        ));

        System::set_block_number(10);

        // submits a signal
        let signal = ExecutionSignal::new(&res.xtx_id, Some(res.steps_cnt.0), SignalKind::Complete);
        assert_ok!(Circuit::on_signal(&origin, signal.clone()));

        // validate the state
        check_queue(QueueValidator::Elements(vec![(ALICE, signal)].into()));

        // async process the signal
        <Circuit as frame_support::traits::OnInitialize<u32>>::on_initialize(100);
        System::set_block_number(100);

        // no signal left
        check_queue(QueueValidator::Length(0));
    });
}

#[test]
fn sdk_can_send_multiple_states() {
    let origin = Origin::signed(ALICE);
    let mut ext = TestExternalities::new_empty();

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 50);

        let res = setup_fresh_state(&origin);

        assert_ok!(<Circuit as OnLocalTrigger<Test>>::on_local_trigger(
            &origin,
            LocalTrigger::new(
                DJANGO,
                vec![Chain::<_, _, [u8; 32]>::Polkadot(Operation::Transfer {
                    caller: ALICE,
                    to: CHARLIE,
                    amount: 50,
                })
                .encode()],
                Some(res.xtx_id.clone()),
            )
        ));

        System::set_block_number(10);

        assert_ok!(<Circuit as OnLocalTrigger<Test>>::on_local_trigger(
            &origin,
            LocalTrigger::new(
                DJANGO,
                vec![Chain::<_, _, [u8; 32]>::Kusama(Operation::Transfer {
                    caller: ALICE,
                    to: DJANGO,
                    amount: 1,
                })
                .encode()],
                Some(res.xtx_id),
            )
        ));
    });
}
//
// #[test]
// fn post_kill_signal_updates_states() {}

enum QueueValidator {
    Length(usize),
    Elements(
        Vec<(
            AccountId32,
            ExecutionSignal<<Test as frame_system::Config>::Hash>,
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

fn setup_fresh_state(origin: &Origin) -> LocalStateExecutionView<Test> {
    let res = Circuit::load_local_state(&origin, None).unwrap();
    assert_ne!(Some(res.xtx_id), None);
    res
}
