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
use circuit_runtime_pallets::{pallet_circuit, pallet_circuit::state::*};

use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, SignalKind},
    xc::*,
};

use codec::Encode;
use frame_support::{assert_err, assert_ok, traits::Currency};
use frame_system::{EventRecord, Phase};

use circuit_mock_runtime::test_utils::*;
use hex_literal::hex;
pub use pallet_grandpa_finality_verifier::mock::brute_seed_block_1;
use serde_json::Value;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::AccountId32;
use sp_std::{
    convert::{TryFrom, TryInto},
    prelude::*,
    str::FromStr,
};
use std::fs;
use t3rn_types::sfx::*;

use t3rn_primitives::{
    circuit::{AdaptiveTimeout, LocalStateExecutionView, LocalTrigger, OnLocalTrigger},
    volatile::LocalState,
    Balance,
};

use circuit_runtime_pallets::pallet_circuit::Error as circuit_error;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);
pub const BOB_RELAYER: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);
pub const ESCROW_ACCOUNT: AccountId32 = AccountId32::new([51_u8; 32]);

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

pub const ED: Balance = 1_u128;

fn advance_to_block(block: BlockNumber) {
    System::set_block_number(block);
    <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(block);
}

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
        Circuit::get_pending_sfx_bids(xtx_id, sfx_id)
            .unwrap()
            .unwrap()
            .amount,
        bid_amount
    );

    assert_eq!(
        Circuit::get_pending_sfx_bids(xtx_id, sfx_id)
            .unwrap()
            .unwrap()
            .executor,
        executor
    );

    assert_eq!(
        Circuit::get_pending_xtx_bids_timeouts(xtx_id).unwrap(),
        System::block_number() + 3
    );

    let three_blocks_ahead = System::block_number() + 3;
    advance_to_block(three_blocks_ahead);

    <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(three_blocks_ahead);

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
            advance_to_block(submission_height.try_into().unwrap());
        }
    }
    json
}

#[test]
fn on_extrinsic_trigger_works_with_empty_side_effects() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let mut ext = TestExternalities::new_empty();
    let side_effects = vec![];

    ext.execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

        assert_ok!(Circuit::on_extrinsic_trigger(
            origin,
            side_effects,
            SpeedMode::Finalized,
            SecurityLvl::Optimistic,
        ));
    });
}

#[test]
fn on_extrinsic_trigger_works_raw_insured_side_effect() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let side_effects = vec![SideEffect {
        target: [0u8, 0u8, 0u8, 0u8],
        max_reward: 2,
        action: [116, 114, 97, 110],
        encoded_args: vec![
            vec![
                42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247, 0,
                118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
            ],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
        reward_asset_id: None,
    }];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_works_with_single_transfer_sets_storage_entries() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
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
                                "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                max_reward: 1,
                                action: [116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                    ],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                ],
                                signature: vec![],
                                enforce_executor: None,
                                reward_asset_id: None,
                                insurance: 1,
                            }],
                            vec![hex!(
                                "0d24d4c519a7fa4f636d4d64127967b704047b08c40cf6dd49068daa75ce5ffe"
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
                                "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
                            )
                            .into()
                        )),
                        topics: vec![]
                    },
                ]
            );
            let xtx_id: sp_core::H256 =
                hex!("c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537").into();
            let side_effect_a_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                &xtx_id.0,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
                None
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect,
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 0,
                    index: FIRST_SFX_INDEX,
                }]]
            );
        });
}

#[test]
fn on_extrinsic_trigger_validation_works_with_single_transfer_insured() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));
        });
}

#[test]
fn on_extrinsic_trigger_works_with_single_transfer_emits_expect_events() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
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
                                "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
                            )
                            .into(),
                            vec![SideEffect {
                                target: [0u8, 0u8, 0u8, 0u8],
                                max_reward: 1 as Balance,
                                insurance: 1 as Balance,
                                action: [116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
                                    ],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                ],
                                signature: vec![],
                                enforce_executor: None,
                                reward_asset_id: None,
                            }],
                            vec![hex!(
                                "0d24d4c519a7fa4f636d4d64127967b704047b08c40cf6dd49068daa75ce5ffe"
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
                                "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
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

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];

    const REQUESTED_INSURANCE_AMOUNT: Balance = 1;
    const BID_AMOUNT: Balance = 1;

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2 + ED); // Alice should have at least: fee (1) + insurance reward (2) + ED (for VariantA)
            let _ = Balances::deposit_creating(
                &BOB_RELAYER,
                REQUESTED_INSURANCE_AMOUNT + BID_AMOUNT + ED,
            ); // Bob should have at least: insurance deposit (1) + ED (for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 0,
                    index: FIRST_SFX_INDEX,
                }]]
            );

            let origin_relayer_bob = Origin::signed(BOB_RELAYER); // Only sudo access to register new gateways for now

            assert_ok!(Circuit::bid_sfx(
                origin_relayer_bob,
                side_effect_a_id,
                BID_AMOUNT,
            ));

            // assert_eq!(
            //     Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
            //     SFXBid::<AccountId32, Balance, u32> {
            //                 bid: BID_AMOUNT,
            //                 requester: ALICE,
            //                 executor: BOB_RELAYER,
            //                 reserved_bond: None,
            //                 insurance: REQUESTED_INSURANCE_AMOUNT,
            //                 reward_asset_id: None,
            //             }
            // );

            // changes status to InBidding
            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::InBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );
        });
}

#[test]
fn circuit_handles_dropped_at_bidding() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    let _fee = 1;
    const REQUESTED_INSURANCE_AMOUNT: Balance = 1;
    const INITIAL_BALANCE: Balance = 3;
    const BID_AMOUNT: Balance = 1;
    const MAX_REWARD: Balance = 1;

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ =
                Balances::deposit_creating(&BOB_RELAYER, REQUESTED_INSURANCE_AMOUNT + BID_AMOUNT); // Bob should have at least: insurance deposit (1)(for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_eq!(Balances::free_balance(ALICE), INITIAL_BALANCE);
            assert_eq!(Balances::reserved_balance(ALICE), 0);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            assert_eq!(Balances::free_balance(ALICE), INITIAL_BALANCE - MAX_REWARD);

            let (xtx_id, _side_effect_a_id) = set_ids(
                valid_transfer_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: ALICE,
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                }
            );

            System::reset_events();
            advance_to_block(4);
            <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(4);
            let events = System::events();

            assert_eq!(Balances::free_balance(ALICE), INITIAL_BALANCE);

            assert!(
                events.iter().any(|record| {
                    if let Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::XTransactionXtxDroppedAtBidding(xtx_id)) = record.event {
                        assert_eq!(xtx_id, xtx_id);
                        true
                    } else {
                        false
                    } })
            );
            assert_eq!(Circuit::get_x_exec_signals(xtx_id), None);
        })
}

const SINGLE_XTX_DEL_WEIGHT: u64 = 450000000;

#[test]
fn circuit_updates_weight_after_killing_xtx_in_on_initialize_hook() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            crate::machine::test_extra::stage_single();
            let xtx_id = crate::machine::test_extra::setup_empty_xtx_and_force_set_status(None);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: ALICE,
                    timeouts_at: AdaptiveTimeout::default_401(),
                    delay_steps_at: None,
                    status: CircuitStatus::Reserved,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                })
            );

            let weight =
                <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(1 + 4);

            assert_eq!(weight, SINGLE_XTX_DEL_WEIGHT);

            assert_eq!(Circuit::get_x_exec_signals(xtx_id), None);
        });
}

#[test]
fn circuit_selects_best_bid_out_of_3_for_transfer_sfx() {
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        3, // insurance
        3, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];

    const REQUESTED_INSURANCE_AMOUNT: Balance = 3;
    const BID_AMOUNT_A: Balance = 1;
    const BID_AMOUNT_B: Balance = 2;
    const BID_AMOUNT_C: Balance = 3;
    const MAX_REWARD: Balance = 3;

    const INITIAL_BALANCE: Balance = 10;

    const REQUESTER: AccountId32 = ALICE;
    const BID_WINNER: AccountId32 = BOB_RELAYER;
    const BID_LOOSER: AccountId32 = CHARLIE;

    const BIDDING_BLOCK_NO: BlockNumber = 1;
    const BIDDING_TIMEOUT: BlockNumber = 3;

    let origin = Origin::signed(REQUESTER); // Only sudo access to register new gateways for now

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&REQUESTER, INITIAL_BALANCE);
            let _ = Balances::deposit_creating(&BID_WINNER, INITIAL_BALANCE);
            let _ = Balances::deposit_creating(&BID_LOOSER, INITIAL_BALANCE);

            advance_to_block(BIDDING_BLOCK_NO);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(origin, side_effects, SpeedMode::Finalized, SecurityLvl::Optimistic));
            // Requester should have offered SFX::max_reward reserved
            assert_eq!(
                Balances::free_balance(&REQUESTER),
                INITIAL_BALANCE - MAX_REWARD
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_transfer_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 0,
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
                events,
                vec![
                    EventRecord { phase: Phase::Initialization, event: Event::Balances(
                        circuit_runtime_pallets::pallet_balances::Event::<Runtime>::Withdraw {
                            who: BID_WINNER, amount: 3
                        }),
                        topics: vec![]
                    },
                    EventRecord { phase: Phase::Initialization, event: Event::AccountManager(
                        circuit_runtime_pallets::pallet_account_manager::Event::<Runtime>::DepositReceived {
                            charge_id: H256::from_str("0x77b1278e47852c8787044159b32806a437ea9196772e8639ff677067a97b32bc").unwrap(),
                            payee: BID_WINNER,
                            recipient: Some(REQUESTER),
                            amount: 3
                        }),
                        topics: vec![] },
                    EventRecord { phase: Phase::Initialization, event: Event::Circuit(
                        circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::SFXNewBidReceived(
                            side_effect_a_id,
                            BID_WINNER,
                            BID_AMOUNT_C,
                        )
                    ),
                        topics: vec![] }]
            );

            // Reserve insurance + bid amounts of the current winner
            assert_eq!(
                Balances::free_balance(&BID_WINNER),
                INITIAL_BALANCE - REQUESTED_INSURANCE_AMOUNT
            );

            // Charlie bids better offer
            assert_ok!(Circuit::bid_sfx(
                Origin::signed(BID_LOOSER),
                side_effect_a_id,
                BID_AMOUNT_B,
            ));

            // Reserve insurance + bid amounts of the current winner
            assert_eq!(
                Balances::free_balance(&BID_LOOSER),
                INITIAL_BALANCE - REQUESTED_INSURANCE_AMOUNT
            );
            // Unreserve insurance + bid amounts of the previous bidder
            assert_eq!(Balances::free_balance(&BID_WINNER), INITIAL_BALANCE);
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
                Balances::free_balance(&BID_WINNER),
                INITIAL_BALANCE - REQUESTED_INSURANCE_AMOUNT
            );
            // Unreserve insurance + bid amounts of the previous bidder
            assert_eq!(Balances::free_balance(&BID_LOOSER), INITIAL_BALANCE);

            let expected_bonded_sfx_bid = SFXBid::<AccountId32, Balance, AssetId> {
                amount: BID_AMOUNT_A,
                requester: REQUESTER,
                executor: BID_WINNER,
                reserved_bond: None,
                insurance: REQUESTED_INSURANCE_AMOUNT,
                reward_asset_id: None,
            };

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
                Some(expected_bonded_sfx_bid)
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id).unwrap(),
                XExecSignal {
                    requester: REQUESTER,
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::InBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                }
            );

            advance_to_block(BIDDING_BLOCK_NO + BIDDING_TIMEOUT);

            <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(
                BIDDING_BLOCK_NO + BIDDING_TIMEOUT,
            );

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: REQUESTER,
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                })
            );
        });
}

#[test]
fn circuit_handles_swap_with_insurance() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let ext = ExtBuilder::default();

    let valid_swap_side_effect = produce_and_validate_side_effect(
        *b"swap",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_swap_side_effect.clone()];

    ext.with_default_xdns_records()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2 + ED); // Alice should have at least: fee (1) + insurance reward (2) + ED (for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1 + 1 + ED); // Bob should have at least: insurance deposit (1) + ED (for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_swap_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 0,
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );
        });
}

#[test]
fn circuit_handles_add_liquidity_without_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();

    let valid_add_liquidity_side_effect = produce_and_validate_side_effect(
        *b"aliq",
        0, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_add_liquidity_side_effect.clone()];

    ext.with_default_xdns_records()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2);
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_add_liquidity_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            assert_eq!(
                Circuit::get_pending_sfx_bids(xtx_id, side_effect_a_id).unwrap(),
                None
            );
        });
}

#[test]
fn circuit_handles_add_liquidity_with_insurance() {
    let origin = Origin::signed(ALICE);

    let ext = ExtBuilder::default();

    let valid_add_liquidity_side_effect = produce_and_validate_side_effect(
        *b"aliq",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_add_liquidity_side_effect.clone()];

    ext.with_default_xdns_records()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1 + 2 + ED); // Alice should have at least: fee (1) + insurance reward (2) + ED (for VariantA)
            let _ = Balances::deposit_creating(&BOB_RELAYER, 1 + 1 + ED); // Bob should have at least: insurance deposit (1) + ED (for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );

            assert_eq!(
                Circuit::get_full_side_effects(xtx_id).unwrap(),
                vec![vec![FullSideEffect {
                    input: valid_add_liquidity_side_effect.clone(),
                    confirmed: None,
                    best_bid: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 0,
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
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::Ready,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1),
                    speed_mode: SpeedMode::Finalized,
                }
            );
        });
}

#[test]
#[cfg(feature = "test-skip-verification")]
fn successfully_confirm_optimistic_transfer() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];
    const REQUESTED_INSURANCE_AMOUNT: Balance = 1;
    const INITIAL_BALANCE: Balance = 3;
    const BID_AMOUNT: Balance = 1;
    const MAX_REWARD: Balance = 1;

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
            let _ =
                Balances::deposit_creating(&BOB_RELAYER, REQUESTED_INSURANCE_AMOUNT + BID_AMOUNT); // Bob should have at least: insurance deposit (1)(for VariantA)

            System::set_block_number(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
            ));

            let (xtx_id, side_effect_a_id) = set_ids(
                valid_transfer_side_effect.clone(),
                ALICE,
                FIRST_REQUESTER_NONCE,
                FIRST_SFX_INDEX,
            );

            place_winning_bid_and_advance_3_blocks(
                BOB_RELAYER,
                xtx_id,
                side_effect_a_id,
                1 as Balance,
            );

            let mut scale_encoded_transfer_event = pallet_balances::Event::<Runtime>::Transfer {
                from: BOB,
                to: AccountId32::new([9u8; 32]),
                amount: 1,
            }
            .encode();
            // append an extra pallet event index byte as the second byte
            scale_encoded_transfer_event.insert(1, 4u8);

            let confirmation_transfer_1 = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                inclusion_data: scale_encoded_transfer_event,
                executioner: ALICE,
                received_at: System::block_number(),
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                Origin::signed(BOB_RELAYER),
                side_effect_a_id,
                confirmation_transfer_1
            ));
        })
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
//
// fn successfully_bond_optimistic(
//     side_effect: SideEffect<AccountId32, Balance>,
//     sfx_index: u32,
//     xtx_id: XtxId<Runtime>,
//     relayer: AccountId32,
//     submitter: AccountId32,
// ) {
//     let optional_insurance = side_effect.encoded_args[3].clone();
//
//     assert!(
//         optional_insurance.len() == 32,
//         "Wrong test value - optimistic transfer assumes optimistic arguments"
//     );
//
//     assert_ok!(Circuit::bid_sfx(
//         Origin::signed(relayer.clone()),
//         side_effect.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
//             &xtx_id.0, sfx_index
//         ),
//         2 as Balance,
//     ));
//
//     let [insurance, reward]: [u128; 2] = Decode::decode(&mut &optional_insurance[..]).unwrap();
//
//     let created_sfx_bid = Circuit::get_pending_sfx_bids(
//         xtx_id,
//         side_effect.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
//             &xtx_id.0, sfx_index,
//         ),
//     )
//     .unwrap()
//     .unwrap();
//
//     assert_eq!(created_sfx_bid.insurance, insurance as Balance);
//     // assert_eq!(created_sfx_bid.reserved_bond, Some(insurance as Balance));
//     assert_eq!(created_sfx_bid.amount, reward as Balance);
//     assert_eq!(
//         created_sfx_bid.requester,
//         Decode::decode(&mut &submitter.encode()[..]).unwrap()
//     );
//     assert_eq!(created_sfx_bid.executor, relayer);
// }

#[test]
fn two_dirty_transfers_are_allocated_to_2_steps_and_can_be_submitted() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::B,
    );

    let side_effects = vec![valid_transfer_side_effect_1, valid_transfer_side_effect_2];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 10);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            let events = System::events();
            assert_eq!(events.len(), 9);
        });
}

#[test]
fn two_dirty_transfers_are_allocated_to_2_steps_and_can_be_confirmed() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::B,
    );

    let side_effects = vec![valid_transfer_side_effect_1, valid_transfer_side_effect_2];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));
        });
}

// ToDo: Order for multiple should now be fixed - verify t3rn#261 is solved
#[test]
#[ignore]
fn circuit_handles_transfer_dirty_and_optimistic_and_swap() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _local_state = LocalState::new();
    let valid_transfer_side_effect_1 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let valid_transfer_side_effect_2 = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::B,
    );

    let valid_swap_side_effect = produce_and_validate_side_effect(
        *b"swap",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![
        valid_transfer_side_effect_1,
        valid_transfer_side_effect_2,
        valid_swap_side_effect,
    ];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));
        });
}

const ADAPTIVE_TIMEOUT_A: AdaptiveTimeout<u32, TargetId> = AdaptiveTimeout {
    estimated_height_here: 801,
    estimated_height_there: 824,
    submit_by_height_here: 401,
    submit_by_height_there: 424,
    emergency_timeout_here: 401,
    there: [0, 0, 0, 0],
    dlq: None,
};

#[test]
fn circuit_cancels_xtx_with_bids_after_timeout() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    let side_effects = vec![valid_transfer_side_effect.clone()];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);

            // The tiemout links that will be checked at on_initialize are there
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(ADAPTIVE_TIMEOUT_A)); // 100 offset + current block height 1 = 101

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                })
            );

            let sfx_id = valid_transfer_side_effect
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                    &xtx_id.0,
                    FIRST_SFX_INDEX,
                );

            place_winning_bid_and_advance_3_blocks(ALICE, xtx_id, sfx_id, 1);

            advance_to_block(410);

            <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(410);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::Reverted(Cause::Timeout),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                })
            );

            assert_eq!(Circuit::get_active_timing_links(xtx_id), None);

            // Emits event notifying about cancellation
            let events = System::events();

            // assert_eq!(events.len(), 9);

            assert_eq!(
                events.iter().any(|record| {
                    if let Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::XTransactionXtxRevertedAfterTimeOut(xtx_id_emit)) = record.event {
                        assert_eq!(xtx_id_emit, xtx_id);
                        true
                    } else {
                        false
                    } }),
                true
            );
            // Voids all associated side effects with Xtx by setting their confirmation to Err
        });
}

#[test]
fn circuit_cancels_xtx_with_incomplete_bid_after_timeout() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let _local_state = LocalState::new();
    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
    );

    const MAX_FEE: Balance = 1;

    let side_effects = vec![valid_transfer_side_effect.clone()];

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects,
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            let _events = System::events();
            // assert_eq!(events.len(), 8);

            let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);

            // The tiemout links that will be checked at on_initialize are there
            assert_eq!(Circuit::get_active_timing_links(xtx_id), Some(ADAPTIVE_TIMEOUT_A)); // 100 offset + current block height 1 = 101

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A, // 100 offset + current block height 1 = 101
                    delay_steps_at: None,
                    status: CircuitStatus::PendingBidding,
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                })
            );

            place_winning_bid_and_advance_3_blocks(
                ALICE,
                xtx_id,
                valid_transfer_side_effect
                    .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                        &xtx_id.0,
                        FIRST_SFX_INDEX,
                    ),
                MAX_FEE,
            );

            advance_to_block(410);

            <Clock as frame_support::traits::OnInitialize<BlockNumber>>::on_initialize(410);

            assert_eq!(
                Circuit::get_x_exec_signals(xtx_id),
                Some(XExecSignal {
                    requester: AccountId32::new(hex!(
                        "0101010101010101010101010101010101010101010101010101010101010101"
                    )),
                    timeouts_at: ADAPTIVE_TIMEOUT_A,
                    delay_steps_at: None,
                    status: CircuitStatus::Reverted(Cause::Timeout),
                    requester_nonce: FIRST_REQUESTER_NONCE,
                    steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
                })
            );

            assert_eq!(Circuit::get_active_timing_links(xtx_id), None);

            // Emits event notifying about cancellation
            let events = System::events();

            assert!(
                events.iter().any(|record| {
                    if let Event::Circuit(circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::XTransactionXtxRevertedAfterTimeOut(xtx_id_emit)) = record.event {
                        assert_eq!(xtx_id_emit, hex!(
                                "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
                            ).into());
                        true
                    } else {
                        false
                    } }),
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
            hex!("c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537").into();

        assert_eq!(res.xtx_id, xtx_id_new);
        assert_eq!(res.local_state, LocalState::new());
        assert_eq!(res.steps_cnt, (0, 1));
    });
}

#[test]
fn sdk_basic_success() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_sfx_abi()
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
                SpeedMode::Finalized,
                Some(res.xtx_id),
            );

            advance_to_block(1);
            brute_seed_block_1(*b"pdot");

            // then it submits to circuit
            assert_ok!(
                <Circuit as OnLocalTrigger<Runtime, BalanceOf>>::on_local_trigger(&origin, trigger)
            );

            advance_to_block(10);

            // submits a signal
            let signal =
                ExecutionSignal::new(&res.xtx_id, Some(res.steps_cnt.0), SignalKind::Complete);
            assert_ok!(Circuit::on_signal(&origin, signal.clone()));

            // validate the state
            check_queue(QueueValidator::Elements(vec![(ALICE, signal)]));

            // async process the signal
            advance_to_block(100);

            // no signal left
            check_queue(QueueValidator::Length(0));
        });
}

#[test]
#[ignore]
fn sdk_can_send_multiple_states() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            advance_to_block(1);
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
                        SpeedMode::Finalized,
                        Some(res.xtx_id),
                    )
                )
            );

            advance_to_block(10);
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
                        SpeedMode::Finalized,
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
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            advance_to_block(1);
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
                        SpeedMode::Finalized,
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
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            advance_to_block(1);
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
                        SpeedMode::Finalized,
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
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 50);

            let res = setup_fresh_state(&origin);

            advance_to_block(1);
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
                        SpeedMode::Finalized,
                        Some(res.xtx_id),
                    )
                )
            );
        });
}

use t3rn_primitives::SpeedMode;
use t3rn_sdk_primitives::{
    storage::BoundedVec,
    xc::{Call as CallVM, Operation},
};
use t3rn_types::migrations::v13::{FullSideEffectV13, SideEffectV13};

// TODO: this fails because the side effect doesnt work for the gateway, will be fixed in the future
#[ignore]
#[test]
fn call_to_vm_is_validated_correctly() {
    let origin = Origin::signed(ALICE);

    ExtBuilder::default()
        .with_standard_sfx_abi()
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
                        SpeedMode::Finalized,
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
            action: [116, 114, 97, 110],
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
            reward_asset_id: None,
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

// FIXME[https://github.com/t3rn/xbi-portal/issues/44]: the api has changed here, will be exposed in a better way
//
// /// XBI
// const INITIAL_BALANCE: Balance = 100;
// const MAX_EXECUTION_COST: Balance = 1;
// const MAX_NOTIFICATION_COST: Balance = 2;
//
// #[test]
// fn execute_side_effects_with_xbi_works_for_transfers() {
//     let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
//
//     let transfer_protocol_box =
//         Box::new(t3rn_types::standard::get_transfer_interface());
//
//     let mut local_state = LocalState::new();
//     let mut valid_transfer_side_effect = produce_and_validate_side_effect(
//         vec![
//             (Type::Address(32), ArgVariant::A),
//             (Type::Address(32), ArgVariant::B),
//             (Type::Uint(128), ArgVariant::A),
//             (Type::OptionalInsurance, ArgVariant::A), // insurance = 1, max_fee = 1
//         ],
//         &mut local_state,
//         transfer_protocol_box,
//         ALICE,
//         FIRST_REQUESTER_NONCE,
//         FIRST_SFX_INDEX,
//     );
//
//     valid_transfer_side_effect.target = [3, 3, 3, 3];
//
//     let side_effects = vec![valid_transfer_side_effect.clone()];
//
//
//
//     const MAX_FEE: Balance = 1;
//     const INSURANCE: Balance = 1;
//
//     ExtBuilder::default()
//         .with_standard_sfx_abi()
//         .with_default_xdns_records()
//         .build()
//         .execute_with(|| {
//             // XTX SETUP
//
//             let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE);
//
//             advance_to_block(1);
//             brute_seed_block_1([3, 3, 3, 3]);
//
//             let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
//
//             let sfx_id_a = valid_transfer_side_effect
//                 .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
//                 &xtx_id.0,
//                 FIRST_SFX_INDEX,
//             );
//
//             assert_ok!(Circuit::on_extrinsic_trigger(
//                 origin.clone(),
//                 side_effects,
//                 SpeedMode::Finalized,
//             ));
//
//             assert_eq!(
//                 Circuit::get_x_exec_signals(xtx_id).unwrap(),
//                 XExecSignal {
//                     requester: ALICE,
//                     timeouts_at: ADAPTIVE_TIMEOUT_A,
//                     delay_steps_at: None,
//                     status: CircuitStatus::PendingBidding,
//                     requester_nonce: FIRST_REQUESTER_NONCE,
//                     steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
//                 }
//             );
//
//             assert_eq!(
//                 Circuit::get_full_side_effects(xtx_id).unwrap(),
//                 vec![vec![FullSideEffect {
//                     input: valid_transfer_side_effect.clone(),
//                     confirmed: None,
//                     best_bid: None,
//                     security_lvl: SecurityLvl::Escrow,
//                     submission_target_height: 0,
//                     index: FIRST_SFX_INDEX,
//                 }]]
//             );
//
//             place_winning_bid_and_advance_3_blocks(ALICE, xtx_id, sfx_id_a, MAX_FEE);
//
//             assert_ok!(Circuit::execute_side_effects_with_xbi(
//                 origin,
//                 xtx_id,
//                 valid_transfer_side_effect,
//                 MAX_EXECUTION_COST as Balance,
//                 MAX_NOTIFICATION_COST as Balance,
//             ));
//
//             assert_eq!(
//                 Balances::free_balance(&ALICE),
//                 INITIAL_BALANCE
//                     - MAX_EXECUTION_COST
//                     - MAX_NOTIFICATION_COST
//                     - 2 * MAX_FEE
//                     - INSURANCE
//             );
//         });
// }
//
// #[test]
// fn execute_side_effects_with_xbi_works_for_call_evm() {
//     let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now
//
//     let xbi_evm = XBIFormat {
//         instr: XBIInstr::CallEvm {
//             source: AccountId20::repeat_byte(3),
//             target: AccountId20::repeat_byte(2),
//             value: sp_core::U256([1, 0, 0, 0]),
//             input: vec![8, 9],
//             gas_limit: 2,
//             max_fee_per_gas: sp_core::U256([4, 5, 6, 7]),
//             max_priority_fee_per_gas: None,
//             nonce: Some(sp_core::U256([3, 4, 6, 7])),
//             access_list: vec![],
//         },
//         metadata: XBIMetadata {
//             id: sp_core::H256::repeat_byte(2),
//             dest_para_id: 3333u32,
//             src_para_id: 4u32,
//             sent: ActionNotificationTimeouts {
//                 action: 1u32,
//                 notification: 2u32,
//             },
//             delivered: ActionNotificationTimeouts {
//                 action: 3u32,
//                 notification: 4u32,
//             },
//             executed: ActionNotificationTimeouts {
//                 action: 4u32,
//                 notification: 5u32,
//             },
//             max_exec_cost: 6u128,
//             max_notifications_cost: 8u128,
//             maybe_known_origin: None,
//             actual_aggregated_cost: None,
//         },
//     };
//
//     let mut valid_evm_sfx = xbi_2_sfx::<
//         Runtime,
//         <Runtime as circuit_runtime_pallets::pallet_circuit::Config>::Escrowed,
//     >(
//         xbi_evm,
//         vec![
//             1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 0,
//         ],
//         1,
//         1,
//     )
//     .unwrap();
//
//     // assert target
//     valid_evm_sfx.target = [1u8, 1u8, 1u8, 1u8];
//     let side_effects = vec![valid_evm_sfx.clone()];
//
//
//
//     const BID_AMOUNT: Balance = 1;
//     const INSURANCE: Balance = 1;
//     const MAX_REWARD: Balance = 1;
//
//     ExtBuilder::default()
//         .with_standard_sfx_abi()
//         .with_default_xdns_records()
//         .build()
//         .execute_with(|| {
//             // XTX SETUP
//             let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
//
//             advance_to_block(1);
//             brute_seed_block_1([3, 3, 3, 3]);
//             brute_seed_block_1([1, 1, 1, 1]);
//
//             let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
//
//             assert_ok!(Circuit::on_extrinsic_trigger(
//                 origin.clone(),
//                 side_effects,
//                 SpeedMode::Finalized,
//             ));
//
//             assert_eq!(
//                 Circuit::get_x_exec_signals(xtx_id).unwrap(),
//                 XExecSignal {
//                     requester: ALICE,
//                     timeouts_at: ADAPTIVE_TIMEOUT_A,
//                     delay_steps_at: None,
//                     status: CircuitStatus::PendingBidding,
//                     requester_nonce: FIRST_REQUESTER_NONCE,
//                     steps_cnt: (0, 1), speed_mode: SpeedMode::Finalized,
//                 }
//             );
//
//             assert_eq!(
//                 Circuit::get_full_side_effects(xtx_id).unwrap(),
//                 vec![vec![FullSideEffect {
//                     input: valid_evm_sfx.clone(),
//                     confirmed: None,
//                     best_bid: None,
//                     security_lvl: SecurityLvl::Escrow,
//                     submission_target_height: 0,
//                     index: FIRST_SFX_INDEX,
//                 }]]
//             );
//
//             place_winning_bid_and_advance_3_blocks(
//                 ALICE,
//                 xtx_id,
//                 valid_evm_sfx
//                     .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
//                         &xtx_id.0, 0,
//                     ),
//                 BID_AMOUNT,
//             );
//
//             assert_ok!(Circuit::execute_side_effects_with_xbi(
//                 origin,
//                 xtx_id,
//                 valid_evm_sfx,
//                 MAX_EXECUTION_COST as Balance,
//                 MAX_NOTIFICATION_COST as Balance,
//             ));
//
//             assert_eq!(
//                 Balances::free_balance(&ALICE),
//                 INITIAL_BALANCE
//                     - BID_AMOUNT
//                     - MAX_REWARD
//                     - INSURANCE
//                     - MAX_EXECUTION_COST
//                     - MAX_NOTIFICATION_COST
//             );
//         });
// }

#[test]
fn no_duplicate_xtx_and_sfx_ids() {
    let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

    let valid_transfer_side_effect = produce_and_validate_side_effect(
        *b"tran",
        1, // insurance
        1, // max_reward
        t3rn_abi::Codec::Scale,
        ArgVariant::A,
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

    let side_effects = vec![valid_transfer_side_effect];

    const BID_AMOUNT: Balance = 1;
    const INSURANCE: Balance = 1;
    const ENDOWMENT: Balance = 6;

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, ENDOWMENT + ED); // Alice should have at least: fee (1) + insurance reward (2) + ED (for VariantA)

            advance_to_block(1);
            brute_seed_block_1([0, 0, 0, 0]);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin.clone(),
                side_effects.clone(),
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));

            // manually increment nonce to simulate production environment
            frame_system::Pallet::<Runtime>::inc_account_nonce(ALICE);

            assert_ok!(Circuit::on_extrinsic_trigger(
                origin,
                side_effects.clone(),
                SpeedMode::Finalized,
                SecurityLvl::Optimistic,
            ));


            let next_events = System::events();

            assert_eq!(next_events, vec![
                EventRecord { phase: Phase::Initialization, event: Event::Balances(
                    circuit_runtime_pallets::pallet_balances::Event::<Runtime>::Deposit { who: ALICE, amount: ENDOWMENT + ED }), topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::System(
                    circuit_runtime_pallets::frame_system::Event::<Runtime>::NewAccount { account: ALICE }), topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::Balances(
                    circuit_runtime_pallets::pallet_balances::Event::<Runtime>::Endowed { account: ALICE, free_balance: ENDOWMENT  + ED }), topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::Balances(
                    circuit_runtime_pallets::pallet_balances::Event::<Runtime>::Withdraw { who: ALICE, amount: BID_AMOUNT }), topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::AccountManager(
                    circuit_runtime_pallets::pallet_account_manager::Event::<Runtime>::DepositReceived {
                        charge_id: expected_sfx_id_1,
                        payee: ALICE, recipient: None, amount: BID_AMOUNT
                    }), topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::Circuit(
                    circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::NewSideEffectsAvailable(
                        ALICE,
                        expected_xtx_id_1,
                        vec![
                            SideEffect {
                                target: [0, 0, 0, 0],
                                max_reward: BID_AMOUNT,
                                insurance: INSURANCE,
                                action: [116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                ],
                                signature: vec![],
                                enforce_executor: None,
                                reward_asset_id: None
                            }
                        ],
                        vec![expected_sfx_id_1]
                    )
                ), topics: vec![] },
                EventRecord { phase: Phase::Initialization, event: Event::Circuit(
                    circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::XTransactionReceivedForExec(
                        expected_xtx_id_1)),
                    topics: vec![]
                },
                EventRecord { phase: Phase::Initialization, event: Event::Balances(
                    circuit_runtime_pallets::pallet_balances::Event::<Runtime>::Withdraw { who: ALICE, amount: BID_AMOUNT }), topics: vec![] },
                EventRecord { phase: Phase::Initialization, event: Event::AccountManager(
                    circuit_runtime_pallets::pallet_account_manager::Event::<Runtime>::DepositReceived {
                        charge_id: expected_sfx_id_2,
                        payee: ALICE, recipient: None, amount: BID_AMOUNT }), topics: vec![] },
                EventRecord { phase: Phase::Initialization, event: Event::Circuit(
                    circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::NewSideEffectsAvailable(
                        ALICE,
                        expected_xtx_id_2,
                        vec![
                            SideEffect {
                                target: [0, 0, 0, 0],
                                max_reward: BID_AMOUNT,
                                insurance: INSURANCE,
                                action: [116, 114, 97, 110],
                                encoded_args: vec![
                                    vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                ],
                                signature: vec![],
                                enforce_executor: None,
                                reward_asset_id: None
                            }
                        ],
                        vec![
                            expected_sfx_id_2
                        ])), topics: vec![] },
                EventRecord { phase: Phase::Initialization, event: Event::Circuit(
                    circuit_runtime_pallets::pallet_circuit::Event::<Runtime>::XTransactionReceivedForExec(
                        expected_xtx_id_2
                    )
                ), topics: vec![] }
            ]);

            assert_ne!(expected_xtx_id_1, expected_xtx_id_2);
            assert_ne!(expected_sfx_id_1, expected_sfx_id_2);
        });
}

#[test]
fn test_storage_migration_v130_to_v140_for_fsx_map_with_updated_encoded_action_field_to_action_4b()
{
    fn create_v13_side_effect() -> SideEffectV13<AccountId32, Balance> {
        SideEffectV13 {
            target: [1, 1, 1, 1],
            max_reward: 100,
            insurance: 50,
            encoded_action: vec![1, 2, 3, 4],
            encoded_args: vec![vec![5, 6], vec![7, 8]],
            signature: vec![9, 10, 11],
            enforce_executor: None,
            reward_asset_id: Some(3),
        }
    }

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // Insert old SideEffect into storage.
            let sfx_v13 = create_v13_side_effect();
            let fsx_v13 = FullSideEffectV13::<AccountId32, BlockNumber, Balance> {
                input: sfx_v13.clone(),
                confirmed: None,
                security_lvl: SecurityLvl::Optimistic,
                submission_target_height: vec![12, 13, 14, 0, 0, 0, 0, 0],
                best_bid: None,
                index: 0,
            };

            let fsx_v14 = FullSideEffect::from(fsx_v13.clone());

            assert_eq!(
                fsx_v14,
                FullSideEffect {
                    input: SideEffect {
                        target: [1, 1, 1, 1],
                        max_reward: 100,
                        insurance: 50,
                        action: [1, 2, 3, 4],
                        encoded_args: vec![vec![5, 6], vec![7, 8]],
                        signature: vec![9, 10, 11],
                        enforce_executor: None,
                        reward_asset_id: Some(3),
                    },
                    confirmed: None,
                    security_lvl: SecurityLvl::Optimistic,
                    submission_target_height: 920844,
                    best_bid: None,
                    index: 0,
                }
            );

            // In this case encoded form of SFX v1.3.0 and v1.4.0 are not the same with the difference to the `action` field encoding.
            // v14: `[... 1, 2, 3, 4, ...]`,
            // v13: `[... 16, 1, 2, 3, 4, ...]`',
            assert_ne!(fsx_v14.encode(), fsx_v13.encode());

            pallet_circuit::FullSideEffects::<Runtime>::insert(
                H256::repeat_byte(1),
                // No clue how to insert old FSX into storage, checking migration mechanics only.
                vec![vec![FullSideEffect::from(fsx_v13)]],
            );

            // Check that storage version has not yet been updated.
            assert_eq!(pallet_circuit::StorageMigrations::<Runtime>::get(), 0);

            // Perform the runtime upgrade (call the `on_runtime_upgrade` function)
            let consumed_weight =
                <Circuit as frame_support::traits::OnRuntimeUpgrade>::on_runtime_upgrade();
            let max_weight =
                <Runtime as frame_system::Config>::DbWeight::get().reads_writes(10, 10);
            assert_eq!(consumed_weight, max_weight);

            // Check that storage version has been updated.
            assert_eq!(pallet_circuit::StorageMigrations::<Runtime>::get(), 1);

            // Check if the migrated data is as expected.
            let fsx =
                pallet_circuit::FullSideEffects::<Runtime>::get(H256::repeat_byte(1)).unwrap();
            for sub_list in fsx.iter() {
                for full_side_effect in sub_list.iter() {
                    let new_side_effect = &full_side_effect.input;
                    assert_eq!(new_side_effect.target, sfx_v13.target);
                    assert_eq!(new_side_effect.max_reward, sfx_v13.max_reward);
                    assert_eq!(new_side_effect.insurance, sfx_v13.insurance);
                    assert_eq!(new_side_effect.action, [1, 2, 3, 4]);
                    assert_eq!(new_side_effect.encoded_args, sfx_v13.encoded_args);
                    assert_eq!(new_side_effect.signature, sfx_v13.signature);
                    assert_eq!(new_side_effect.enforce_executor, sfx_v13.enforce_executor);
                    assert_eq!(new_side_effect.reward_asset_id, sfx_v13.reward_asset_id);
                }
            }
        });
}
