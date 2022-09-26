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
use frame_support::{assert_ok, traits::Currency};

use frame_system::{EventRecord, Phase};

use sp_io::TestExternalities;
use sp_runtime::{
    traits::{Header as HeaderT, Zero},
    AccountId32,
};
use sp_std::{convert::TryFrom, prelude::*};
use t3rn_primitives::{
    abi::*,
    circuit::{LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
    side_effect::*,
    volatile::LocalState,
    xtx::XtxId,
};
use t3rn_protocol::side_effects::test_utils::*;

use pallet_xbi_portal::{
    sabi::AccountId20,
    xbi_codec::{ActionNotificationTimeouts, XBIFormat, XBIInstr, XBIMetadata},
};
use pallet_xbi_portal_enter::t3rn_sfx::xbi_2_sfx;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB_RELAYER: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

fn set_ids(
    valid_side_effect: SideEffect<AccountId32, BlockNumber, Balance>,
) -> (sp_core::H256, sp_core::H256) {
    let xtx_id: sp_core::H256 =
        hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

    let sfx_id = valid_side_effect
        .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>();

    (xtx_id, sfx_id)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    (array[0] as u32)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

pub fn brute_seed_block_1_to_grandpa_mfv(gateway_id: [u8; 4]) {
    // Brute update storage of MFV::MultiImportedHeaders to blockA = 1 and BestAvailable -> blockA
    let block_hash_1 = sp_core::H256::repeat_byte(1);
    let header_1: Header = Header::new(
        1,
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );

    <pallet_multi_finality_verifier::MultiImportedHeaders<Runtime>>::insert::<
        [u8; 4],
        sp_core::H256,
        Header,
    >(gateway_id, block_hash_1, header_1);

    <pallet_multi_finality_verifier::BestFinalizedMap<Runtime>>::insert::<[u8; 4], sp_core::H256>(
        gateway_id,
        block_hash_1,
    );
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
        prize: 2,
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

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

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
            // assert_eq!(events.len(), 8);
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
                                "388ee470b95c60ecf7e6e1f97b04f423346b443a06b5be4adbc1c219ed7ae636"
                            )
                            .into(),],
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionReadyForExec(
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
            let side_effect_a_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            );

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
                    submission_target_height: vec![1, 0, 0, 0],
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
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                prize: 2 as Balance,
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
                                "df27692efff5ca3e2db6b0c2aed2976970b071d0ba18a82f818d488205004bad"
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

            // Runtime Apply State
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
                reserved_bond: 0,
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
                    submission_target_height: vec![1, 0, 0, 0],
                }]]
            );
        });
}

#[test]
fn circuit_handles_insurance_deposit_for_transfers() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());

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

            // Runtime Apply State
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
                reserved_bond: 0,
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
                    submission_target_height: vec![1, 0, 0, 0],
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
                reserved_bond: 0,
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
            let mut encoded_balance_transfer_event = pallet_balances::Event::<Runtime>::Transfer {
                from: hex!("0909090909090909090909090909090909090909090909090909090909090909")
                    .into(), // variant A
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 1, // variant A
            }
            .encode();

            // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_balance_transfer_event);

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            // Update MFV::MultiImportedHeaders
            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_transfer_side_effect,
                confirmation,
                None,
                None,
            ));

            // Alice should have deducted reward from her balance
            assert_eq!(Balances::free_balance(&ALICE), 1);
            assert_eq!(Balances::free_balance(&BOB_RELAYER), 0);
        });
}

#[test]
fn circuit_handles_dirty_swap_with_no_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
    let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());

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
                    submission_target_height: vec![1, 0, 0, 0],
                }]]
            );

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                None
            );

            // Confirmation start
            let mut encoded_swap_transfer_event = orml_tokens::Event::<Runtime>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 2u128, // amount - variant B
            }
            .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_swap_transfer_event);

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_swap_side_effect,
                confirmation,
                None,
                None,
            ));
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

            // Runtime Apply State
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
                reserved_bond: 0,
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
                    submission_target_height: vec![1, 0, 0, 0],
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
                reserved_bond: 0,
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
            let mut encoded_swap_transfer_event = orml_tokens::Event::<Runtime>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 2u128, // amount - variant B
            }
            .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_swap_transfer_event);

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_swap_side_effect,
                confirmation,
                None,
                None,
            ));

            // Vefify the offered reward has been reserved from Alice account
            assert_eq!(Balances::free_balance(&ALICE), 1);
        });
}

#[test]
fn circuit_handles_add_liquidity_without_insurance() {
    let origin = Origin::signed(ALICE);

    let origin_relayer_bob = Origin::signed(BOB_RELAYER);

    let ext = ExtBuilder::default();
    let mut local_state = LocalState::new();

    let add_liquidity_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_add_liquidity_interface());

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

            let _events = System::events();

            // 5 events: new account, endowed, transfer, xtransactionreadytoexec, newsideeffectavailable
            // assert_eq!(events.len(), 11);

            // Confirmation start
            let mut encoded_add_liquidity_transfer_event =
                orml_tokens::Event::<Runtime>::Transfer {
                    currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                    from: BOB_RELAYER,                     // executor - Bob
                    to: hex!("0606060606060606060606060606060606060606060606060606060606060606")
                        .into(), // variant B (dest)
                    amount: 1u128,                         // amount - variant B
                }
                .encode();

            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_add_liquidity_transfer_event);

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_add_liquidity_side_effect,
                confirmation,
                None,
                None,
            ));
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

            // Runtime Apply State
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
                reserved_bond: 0,
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
                    submission_target_height: vec![1, 0, 0, 0],
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
                reserved_bond: 0,
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
            let mut encoded_add_liquidity_transfer_event =
                orml_tokens::Event::<Runtime>::Transfer {
                    currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                    from: BOB_RELAYER,                     // executor - Bob
                    to: hex!("0606060606060606060606060606060606060606060606060606060606060606")
                        .into(), // variant B (dest)
                    amount: 1u128,                         // amount - variant B
                }
                .encode();

            // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_add_liquidity_transfer_event);

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_add_liquidity_side_effect,
                confirmation,
                None,
                None,
            ));

            // assert_eq!(Balances::free_balance(&BOB_RELAYER), 1 + 2);
        });
}

// fn successfully_confirm_optimistic(side_effect: SideEffect<AccountId32, BlockNumber, Balance>) {
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

fn successfully_confirm_dirty(
    side_effect: SideEffect<AccountId32, BlockNumber, Balance>,
    xtx_id: XtxId<Runtime>,
    relayer: AccountId32,
) {
    let from = side_effect.encoded_args[0].clone();
    let to = side_effect.encoded_args[1].clone();
    let amount = side_effect.encoded_args[2].clone();

    let mut encoded_balance_transfer_event = pallet_balances::Event::<Runtime>::Transfer {
        from: Decode::decode(&mut &from[..]).unwrap(), // variant A
        to: Decode::decode(&mut &to[..]).unwrap(),     // variant A
        amount: Decode::decode(&mut &amount[..]).unwrap(), // variant A
    }
    .encode();

    // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
    let mut encoded_event = vec![4];
    encoded_event.append(&mut encoded_balance_transfer_event);
    let confirmation_transfer = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
        err: None,
        output: None,
        encoded_effect: encoded_event,
        inclusion_proof: None,
        executioner: BOB_RELAYER,
        received_at: 0,
        cost: None,
    };

    assert_ok!(Circuit::confirm_side_effect(
        Origin::signed(relayer),
        xtx_id,
        side_effect,
        confirmation_transfer,
        None,
        None,
    ));
}

fn successfully_bond_optimistic(
    side_effect: SideEffect<AccountId32, BlockNumber, Balance>,
    xtx_id: XtxId<Runtime>,
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
        side_effect
            .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(),
    ));

    let [insurance, reward]: [u128; 2] = Decode::decode(&mut &optional_insurance[..]).unwrap();

    let created_insurance_deposit = Circuit::get_insurance_deposits(
        xtx_id,
        side_effect
            .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(),
    )
    .unwrap();

    assert_eq!(created_insurance_deposit.insurance, insurance as u128);
    assert_eq!(created_insurance_deposit.reward, reward as u128);
    assert_eq!(
        created_insurance_deposit.requester,
        Decode::decode(&mut &submitter.encode()[..]).unwrap()
    );
    assert_eq!(created_insurance_deposit.bonded_relayer, Some(relayer));
    assert_eq!(created_insurance_deposit.status, CircuitStatus::Bonded);
    assert_eq!(created_insurance_deposit.requested_at, 1);
}

#[test]
fn two_dirty_and_three_optimistic_transfers_are_allocated_to_3_steps_and_all_5_is_confirmed() {
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
        transfer_protocol_box.clone(),
    );

    let valid_optimistic_transfer_side_effect_3 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::C),
            (Type::Address(32), ArgVariant::A),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let valid_optimistic_transfer_side_effect_4 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::C),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::B), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let valid_optimistic_transfer_side_effect_5 = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::C),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(128), ArgVariant::B),
            (Type::OptionalInsurance, ArgVariant::B), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
    );

    let side_effects = vec![
        valid_transfer_side_effect_1.clone(),
        valid_transfer_side_effect_2.clone(),
        valid_optimistic_transfer_side_effect_3.clone(),
        valid_optimistic_transfer_side_effect_4.clone(),
        valid_optimistic_transfer_side_effect_5.clone(),
    ];

    let fee = 1;
    let sequential = true;

    ExtBuilder::default()
        .with_standard_side_effects()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);
            let _ = Balances::deposit_creating(&BOB_RELAYER, 50);

            System::set_block_number(1);
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

            // Confirmation start - 3
            successfully_bond_optimistic(
                valid_optimistic_transfer_side_effect_3.clone(),
                xtx_id,
                BOB_RELAYER,
                ALICE,
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::PendingInsurance
            );

            // Confirmation start - 4
            successfully_bond_optimistic(
                valid_optimistic_transfer_side_effect_4.clone(),
                xtx_id,
                BOB_RELAYER,
                ALICE,
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::PendingInsurance
            );

            // Confirmation start - 5
            successfully_bond_optimistic(
                valid_optimistic_transfer_side_effect_5.clone(),
                xtx_id,
                BOB_RELAYER,
                ALICE,
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::Ready
            );

            // Confirmation start - 3
            successfully_confirm_dirty(
                valid_optimistic_transfer_side_effect_3,
                xtx_id,
                BOB_RELAYER,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::PendingExecution
            );

            // Confirmation start - 4
            successfully_confirm_dirty(
                valid_optimistic_transfer_side_effect_4,
                xtx_id,
                BOB_RELAYER,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::PendingExecution
            );

            // Confirmation start - 5
            successfully_confirm_dirty(
                valid_optimistic_transfer_side_effect_5,
                xtx_id,
                BOB_RELAYER,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::Finished
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().steps_cnt,
                (1, 3)
            );
            // Confirmation start - 1
            successfully_confirm_dirty(valid_transfer_side_effect_1, xtx_id, BOB_RELAYER);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::Finished
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().steps_cnt,
                (2, 3)
            );

            // Confirmation start - 2
            successfully_confirm_dirty(valid_transfer_side_effect_2, xtx_id, BOB_RELAYER);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().status,
                CircuitStatus::FinishedAllSteps
            );
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap().steps_cnt,
                (3, 3)
            );
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
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

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

            // Confirmation start - 1
            successfully_confirm_dirty(valid_transfer_side_effect_1, xtx_id, BOB_RELAYER);

            // Confirmation start - 2
            successfully_confirm_dirty(valid_transfer_side_effect_2, xtx_id, BOB_RELAYER);
        });
}

// ToDo: Order for multiple should now be fixed - verify t3rn#261 is solved
#[test]
#[ignore]
fn circuit_handles_transfer_dirty_and_optimistic_and_swap() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    let transfer_protocol_box =
        Box::new(t3rn_protocol::side_effects::standards::get_transfer_interface());
    let swap_protocol_box = Box::new(t3rn_protocol::side_effects::standards::get_swap_interface());

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
        valid_transfer_side_effect_2,
        valid_swap_side_effect.clone(),
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
            brute_seed_block_1_to_grandpa_mfv([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let _events = System::events();
            // assert_eq!(events.len(), 9);

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

            // Confirmation start
            let mut encoded_balance_transfer_event = pallet_balances::Event::<Runtime>::Transfer {
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

            let confirmation_transfer = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_balance_transfer_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob.clone(),
                xtx_id,
                valid_transfer_side_effect_1,
                confirmation_transfer,
                None,
                None,
            ));

            println!(
                "exec signals after 1st confirmation, transfer: {:?}",
                Circuit::get_x_exec_signals(xtx_id).unwrap()
            );

            // Confirmation start
            let mut encoded_swap_transfer_event = orml_tokens::Event::<Runtime>::Transfer {
                currency_id: as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                from: BOB_RELAYER,                     // executor - Bob
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 2u128, // amount - variant B
            }
            .encode();

            // Adding 4 since Balances Pallet = 4 in construct_runtime! enum
            let mut encoded_event = vec![4];
            encoded_event.append(&mut encoded_swap_transfer_event);

            let confirmation_swap = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                encoded_effect: encoded_swap_transfer_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            println!(
                "full side effects after confirmation: {:?}",
                Circuit::get_full_side_effects(xtx_id).unwrap()
            );

            println!(
                "exec signals after confirmation: {:?}",
                Circuit::get_x_exec_signals(xtx_id).unwrap()
            );

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_swap_side_effect,
                confirmation_swap,
                None,
                None,
            ));
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
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
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
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(401u32)); // 100 offset + current block height 1 = 101

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: 401u32, // 100 offset + current block height 1 = 101
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee),
                    steps_cnt: (0, 1),
                })
            );

            System::set_block_number(410);

            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(110);

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
                Some(
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<
                            Runtime,
                        >::XTransactionXtxRevertedAfterTimeOut(
                            hex!(
                                "2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59"
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
            hex!("b09a43d4886048104b526ce9b29d77e10dd27e263d329888b73562b0b9068a0a").into();

        assert_eq!(res.xtx_id, xtx_id_new);
        assert_eq!(res.local_state, LocalState::new());
        assert_eq!(res.steps_cnt, (0, 0));
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
            brute_seed_block_1_to_grandpa_mfv(*b"pdot");

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
            <Circuit as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(100);
            System::set_block_number(100);

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
            brute_seed_block_1_to_grandpa_mfv(*b"pdot");

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
            brute_seed_block_1_to_grandpa_mfv(*b"ksma");

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
            brute_seed_block_1_to_grandpa_mfv(*b"pdot");

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
            brute_seed_block_1_to_grandpa_mfv(*b"pdot");

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
            brute_seed_block_1_to_grandpa_mfv(*b"pdot");

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

    let se = SideEffect::<[u8; 32], u128, u128>::try_from(ch).unwrap();

    assert_eq!(
        se,
        SideEffect {
            target: [112u8, 100u8, 111u8, 116u8],
            prize: 0,
            ordered_at: 0,
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
            enforce_executioner: None,
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
const INITIAL_BALANCE: Balance = 3;
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
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box,
    );

    valid_transfer_side_effect.target = [3, 3, 3, 3];

    let side_effects = vec![valid_transfer_side_effect.clone()];
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
            brute_seed_block_1_to_grandpa_mfv([3, 3, 3, 3]);

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();
            let _side_effect_a_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
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
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Escrowed,
                    submission_target_height: vec![1, 0, 0, 0],
                }]]
            );

            assert_ok!(Circuit::execute_side_effects_with_xbi(
                origin,
                xtx_id,
                valid_transfer_side_effect,
                MAX_EXECUTION_COST as u128,
                MAX_NOTIFICATION_COST as u128,
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                INITIAL_BALANCE - MAX_EXECUTION_COST - MAX_NOTIFICATION_COST
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
    >(xbi_evm, vec![], Zero::zero())
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
            brute_seed_block_1_to_grandpa_mfv([3, 3, 3, 3]);
            brute_seed_block_1_to_grandpa_mfv([1, 1, 1, 1]);

            let xtx_id: sp_core::H256 =
                hex!("2637d56ea21c04df03463decc4aa8d2916c96e59ac45e451d7133eedc621de59").into();

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin.clone(),
                side_effects,
                fee,
                sequential,
            ));

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
                    input: valid_evm_sfx.clone(),
                    confirmed: None,
                    security_lvl: SecurityLvl::Escrowed,
                    submission_target_height: vec![1, 0, 0, 0],
                }]]
            );

            assert_ok!(Circuit::execute_side_effects_with_xbi(
                origin,
                xtx_id,
                valid_evm_sfx,
                MAX_EXECUTION_COST as u128,
                MAX_NOTIFICATION_COST as u128,
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                INITIAL_BALANCE - MAX_EXECUTION_COST - MAX_NOTIFICATION_COST
            );
        });
}
