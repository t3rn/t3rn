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

use t3rn_protocol::side_effects::standards::TransferSideEffectProtocol;
use t3rn_protocol::side_effects::test_utils::*;

use crate::mock::*;
use crate::state::*;

use sp_io::TestExternalities;

use sp_runtime::AccountId32;
// use crate::*;

pub fn new_test_ext() -> TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    TestExternalities::new(t)
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

#[test]
fn on_extrinsic_trigger_works_with_empty_side_effects() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let mut ext = TestExternalities::new_empty();
    let side_effects = vec![];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        assert_ok!(Circuit::on_extrinsics_trigger(
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

    let mut ext = TestExternalities::new_empty();

    let mut local_state = LocalState::new();
    let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        System::set_block_number(1);

        assert_ok!(Circuit::on_extrinsics_trigger(
            origin,
            side_effects,
            fee,
            sequential,
        ));

        // Assert Circuit::emit generates 5 correct events: 3 from charging and 2 Circuit-specific
        let events = System::events();
        assert_eq!(events.len(), 5);
        assert_eq!(
            vec![events[3].clone(), events[4].clone()],
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    // event: Event::call_dispatch(call_dispatch::Event::<TestRuntime>::MessageVersionSpecMismatch(
                    event: Event::Circuit(crate::Event::<Test>::XTransactionReceivedForExec(
                        hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef")
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
                        hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef")
                            .into(),
                        vec![SideEffect {
                            target: [0u8, 0u8, 0u8, 0u8],
                            prize: 0,
                            ordered_at: 0,
                            encoded_action: vec![116, 114, 97, 110],
                            encoded_args: vec![
                                vec![
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                ],
                                vec![
                                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6
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
            hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef").into();
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
                total_reward: Some(1000000)
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

    let mut ext = TestExternalities::new_empty();

    let mut local_state = LocalState::new();
    let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        System::set_block_number(1);

        assert_ok!(Circuit::on_extrinsics_trigger(
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

    let mut ext = TestExternalities::new_empty();

    let mut local_state = LocalState::new();
    let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        System::set_block_number(1);

        assert_ok!(Circuit::on_extrinsics_trigger(
            origin,
            side_effects,
            fee,
            sequential,
        ));

        // Assert Circuit::emit generates 5 correct events: 3 for charging and 2 Circuit-specific
        let events = System::events();
        assert_eq!(events.len(), 5);
        assert_eq!(
            vec![events[3].clone(), events[4].clone()],
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::Circuit(crate::Event::<Test>::XTransactionReceivedForExec(
                        hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef")
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
                        hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef")
                            .into(),
                        vec![SideEffect {
                            target: [0u8, 0u8, 0u8, 0u8],
                            prize: 0,
                            ordered_at: 0,
                            encoded_action: vec![116, 114, 97, 110],
                            encoded_args: vec![
                                vec![
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                ],
                                vec![
                                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6
                                ],
                                vec![1, 0, 0, 0, 0, 0, 0, 0],
                                // Insurance goes here
                                vec![
                                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
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

    let mut ext = TestExternalities::new_empty();

    let mut local_state = LocalState::new();
    let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        System::set_block_number(1);

        assert_ok!(Circuit::on_extrinsics_trigger(
            origin,
            side_effects,
            fee,
            sequential,
        ));

        let xtx_id: sp_core::H256 =
            hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef").into();
        let side_effect_a_id =
            valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

        // Test Apply State
        // Returns void insurance for that side effect
        let valid_insurance_deposit = InsuranceDeposit {
            insurance: 1,
            reward: 0,
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
                total_reward: Some(1000000)
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

    let mut ext = TestExternalities::new_empty();

    let mut local_state = LocalState::new();
    let transfer_protocol_box = Box::new(TransferSideEffectProtocol {});
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
        ],
        &mut local_state,
        transfer_protocol_box.clone(),
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let fee = 1_000_000;
    let sequential = true;

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_001);
        System::set_block_number(1);

        assert_ok!(Circuit::on_extrinsics_trigger(
            origin,
            side_effects,
            fee,
            sequential,
        ));

        let xtx_id: sp_core::H256 =
            hex!("6aa7d045405e48f6badcdc58fbb1183031bb74895de69ff51ea785f778e573ef").into();
        let side_effect_a_id =
            valid_transfer_side_effect.generate_id::<crate::SystemHashing<Test>>();

        // Test Apply State
        // Returns void insurance for that side effect
        let valid_insurance_deposit = InsuranceDeposit {
            insurance: 1,
            reward: 0,
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
                total_reward: Some(1000000)
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
