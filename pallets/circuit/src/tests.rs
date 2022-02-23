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
use frame_support::assert_ok;
use frame_support::traits::Currency;
use frame_system::{EventRecord, Phase};

use t3rn_primitives::abi::*;
use t3rn_primitives::side_effect::*;

use t3rn_protocol::side_effects::test_utils::*;

use crate::mock::*;
use crate::state::*;

use sp_io::TestExternalities;

use codec::Encode;
use sp_runtime::AccountId32;
use sp_std::prelude::*;
use t3rn_protocol::side_effects::protocol::SideEffectProtocol;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB_RELAYER: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

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
fn on_extrinsic_trigger_works_with_single_transfer_not_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let transfer_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"tran")
        .unwrap();

    let mut local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        Box::new(transfer_protocol_box),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);

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
                        event: Event::Circuit(crate::Event::<Test>::XTransactionReadyForExec(
                            hex!(
                                "7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6"
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
                                    vec![1, 0, 0, 0, 0, 0, 0, 0],
                                    vec![]
                                ],
                                signature: vec![],
                                enforce_executioner: None
                            }]
                        )),
                        topics: vec![]
                    }
                ]
            );
            let xtx_id: sp_core::H256 =
                hex!("7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6").into();
            let side_effect_a_id =
                valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

            // Returns void insurance for that side effect
            let void_insurance_deposit = InsuranceDeposit {
                insurance: 0,
                reward: 0,
                requester: AccountId32::new(hex!(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )),
                bonded_relayer: None,
                status: CircuitStatus::Requested,
                requested_at: 0,
            };

            assert_eq!(
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                void_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: None,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee)
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                }]]
            );
        });
}

#[test]
fn on_extrinsic_trigger_validation_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let transfer_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"tran")
        .unwrap();

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        Box::new(transfer_protocol_box),
    );

    let side_effects = vec![valid_transfer_side_effect];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);

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

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let transfer_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"tran")
        .unwrap();

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        Box::new(transfer_protocol_box),
    );

    let side_effects = vec![valid_transfer_side_effect];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);

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
                        event: Event::Circuit(crate::Event::<Test>::XTransactionReceivedForExec(
                            hex!(
                                "7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(crate::Event::<Test>::NewSideEffectsAvailable(
                            AccountId32::new(hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )),
                            hex!(
                                "7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6"
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
                                    vec![1, 0, 0, 0, 0, 0, 0, 0],
                                    // Insurance goes here
                                    vec![
                                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                                    ]
                                ],
                                signature: vec![],
                                enforce_executioner: None
                            }]
                        )),
                        topics: vec![]
                    }
                ]
            );
        });
}

#[test]
fn on_extrinsic_trigger_apply_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let transfer_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"tran")
        .unwrap();

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        Box::new(transfer_protocol_box),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            System::set_block_number(1);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let xtx_id: sp_core::H256 =
                hex!("7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6").into();
            let side_effect_a_id =
                valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

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
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                valid_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: None,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee)
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                }]]
            );
        });
}

#[test]
fn circuit_handles_insurance_deposit_for_transfers() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let transfer_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"tran")
        .unwrap();

    let mut local_state = LocalState::new();

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, reward = 2
        ],
        &mut local_state,
        Box::new(transfer_protocol_box),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let xtx_id: sp_core::H256 =
                hex!("7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6").into();

            let side_effect_a_id =
                valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

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
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                valid_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: None,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingInsurance,
                    total_reward: Some(fee)
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
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
                Circuit::get_insurance_deposits(xtx_id, side_effect_a_id),
                expected_bonded_insurance_deposit
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: None,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee)
                }
            );

            // Confirmation start
            let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer {
                from: hex!("0909090909090909090909090909090909090909090909090909090909090909")
                    .into(), // variant A
                to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                amount: 1, // variant A
            }
            .encode();

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
                err: None,
                output: None,
                encoded_effect: encoded_balance_transfer_event,
                inclusion_proof: None,
                executioner: BOB_RELAYER,
                received_at: 0,
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                origin_relayer_bob,
                xtx_id,
                valid_transfer_side_effect,
                confirmation,
                None,
                None,
            ));

            // Check that Bob collected the relayer reward
            assert_eq!(Balances::free_balance(&BOB_RELAYER), 1 + 2);
        });
}

#[test]
fn circuit_handles_dirty_swap_with_no_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
    let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

    // Initializing early to query side_effects
    let ext_builder = ExtBuilder::default().with_standard_side_effects();

    let swap_protocol_box = ext_builder
        .standard_side_effects
        .clone()
        .into_iter()
        .find(|se| se.get_id() == *b"swap")
        .unwrap();

    let mut local_state = LocalState::new();

    let valid_swap_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A), // caller
            (Type::Address(32), ArgVariant::B), // to
            (Type::Uint(64), ArgVariant::A),    // amount_from
            (Type::Uint(64), ArgVariant::B),    // amount_to
            (Type::Bytes(4), ArgVariant::A),    // asset_from
            (Type::Bytes(4), ArgVariant::B),    // asset_to
            (Type::Bytes(0), ArgVariant::A),    // empty bytes instead of insurance
        ],
        &mut local_state,
        Box::new(swap_protocol_box),
    );

    let side_effects = vec![valid_swap_side_effect.clone()];
    let fee = 1;
    let sequential = true;

    ext_builder
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                fee,
                sequential,
            ));

            let xtx_id: sp_core::H256 =
                hex!("7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6").into();

            let _side_effect_a_id =
                valid_swap_side_effect.generate_id::<crate::SystemHashing<Test>>();

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: None,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    total_reward: Some(fee)
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id),
                vec![vec![FullSideEffect {
                    input: valid_swap_side_effect.clone(),
                    confirmed: None,
                }]]
            );

            fn as_u32_le(array: &[u8; 4]) -> u32 {
                ((array[0] as u32) << 0)
                    + ((array[1] as u32) << 8)
                    + ((array[2] as u32) << 16)
                    + ((array[3] as u32) << 24)
            }

            // Confirmation start
            let encoded_swap_transfer_event = orml_tokens::Event::<Test>::Transfer(
                as_u32_le(&[0, 1, 2, 3]), // currency_id as u8 bytes [0,1,2,3] -> u32
                BOB_RELAYER,              // executor - Bob
                hex!("0606060606060606060606060606060606060606060606060606060606060606").into(), // variant B (dest)
                2u64, // amount - variant B
            )
            .encode();

            let confirmation = ConfirmedSideEffect::<AccountId32, BlockNumber, BalanceOf> {
                err: None,
                output: None,
                encoded_effect: encoded_swap_transfer_event,
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
