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
use crate::ExecComposer;
use crate::{self as pallet_execution_delivery};
use codec::Encode;

use frame_support::{assert_err, assert_ok};

use crate::{Xtx, XtxSchedule};
use sp_io;

use sp_core::{crypto::Pair, sr25519, Hasher};
use sp_runtime::traits::Zero;

use crate::exec_composer::tests::{insert_default_xdns_record, make_compose_out_of_raw_wat_code};
use crate::exec_composer::*;

use sp_io::TestExternalities;
use sp_keystore::testing::KeyStore;
use sp_keystore::{KeystoreExt, SyncCryptoStore};

use pallet_execution_delivery::Compose;

use t3rn_primitives::{abi::GatewayABIConfig, transfers::BalanceOf, *};

use crate::mock::*;

use crate::mock::AccountId;

pub fn new_test_ext() -> TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    TestExternalities::new(t)
}

#[test]
fn it_submits_empty_composable_exec_request() {
    sp_io::TestExternalities::default().execute_with(|| {
        assert_err!(
            ExecDelivery::submit_composable_exec_order(
                Origin::signed(Default::default()),
                vec![],
                vec![]
            ),
            "empty parameters submitted for execution order"
        );
    });
}

#[test]
fn it_should_correctly_parse_a_minimal_valid_io_schedule() {
    let expected = InterExecSchedule {
        phases: vec![ExecPhase {
            steps: vec![ExecStep {
                compose: Compose {
                    name: b"component1".to_vec(),
                    code_txt: r#""#.as_bytes().to_vec(),
                    exec_type: b"exec_escrow".to_vec(),
                    dest: AccountId::new([1 as u8; 32]),
                    value: 0,
                    bytes: vec![],
                    input_data: vec![],
                },
            }],
        }],
    };

    let io_schedule = b"component1;".to_vec();
    let components = vec![Compose {
        name: b"component1".to_vec(),
        code_txt: r#""#.as_bytes().to_vec(),
        exec_type: b"exec_escrow".to_vec(),
        dest: AccountId::new([1 as u8; 32]),
        value: 0,
        bytes: vec![],
        input_data: vec![],
    }];

    assert_eq!(
        ExecDelivery::decompose_io_schedule(components, io_schedule).unwrap(),
        expected
    )
}

#[test]
fn it_should_correctly_parse_a_valid_io_schedule_with_2_phases() {
    let expected = InterExecSchedule {
        phases: vec![
            ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component1".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),
                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                }],
            },
            ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component2".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),
                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                }],
            },
        ],
    };

    let io_schedule = b"component1 | component2;".to_vec();
    let components = vec![
        Compose {
            name: b"component1".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
        Compose {
            name: b"component2".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
    ];

    assert_eq!(
        ExecDelivery::decompose_io_schedule(components, io_schedule).unwrap(),
        expected
    )
}

#[test]
fn it_should_correctly_parse_a_valid_io_schedule_with_1_phase_and_2_steps() {
    let expected = InterExecSchedule {
        phases: vec![ExecPhase {
            steps: vec![
                ExecStep {
                    compose: Compose {
                        name: b"component1".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),

                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                },
                ExecStep {
                    compose: Compose {
                        name: b"component2".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),

                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                },
            ],
        }],
    };

    let io_schedule = b"component1 , component2;".to_vec();
    let components = vec![
        Compose {
            name: b"component1".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
        Compose {
            name: b"component2".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
    ];

    assert_eq!(
        ExecDelivery::decompose_io_schedule(components, io_schedule).unwrap(),
        expected
    )
}

#[test]
fn it_should_correctly_parse_a_valid_io_schedule_with_complex_structure() {
    let expected = InterExecSchedule {
        phases: vec![
            ExecPhase {
                steps: vec![
                    ExecStep {
                        compose: Compose {
                            name: b"component1".to_vec(),
                            code_txt: r#""#.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: AccountId::new([1 as u8; 32]),
                            value: 0,
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                    ExecStep {
                        compose: Compose {
                            name: b"component2".to_vec(),
                            code_txt: r#""#.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: AccountId::new([1 as u8; 32]),
                            value: 0,
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                ],
            },
            ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component2".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),

                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                }],
            },
            ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component1".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),

                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),
                        value: 0,
                        bytes: vec![],
                        input_data: vec![],
                    },
                }],
            },
            ExecPhase {
                steps: vec![
                    ExecStep {
                        compose: Compose {
                            name: b"component2".to_vec(),
                            code_txt: r#""#.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: AccountId::new([1 as u8; 32]),
                            value: 0,
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                    ExecStep {
                        compose: Compose {
                            name: b"component2".to_vec(),
                            code_txt: r#""#.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: AccountId::new([1 as u8; 32]),
                            value: 0,
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                    ExecStep {
                        compose: Compose {
                            name: b"component1".to_vec(),
                            code_txt: r#""#.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: AccountId::new([1 as u8; 32]),
                            value: 0,
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                ],
            },
        ],
    };

    let io_schedule = b"     component1 , component2 | component2 |     component1| component2, component2, component1;   ".to_vec();
    let components = vec![
        Compose {
            name: b"component1".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
        Compose {
            name: b"component2".to_vec(),
            code_txt: r#""#.as_bytes().to_vec(),

            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: 0,
            bytes: vec![],
            input_data: vec![],
        },
    ];

    assert_eq!(
        ExecDelivery::decompose_io_schedule(components, io_schedule).unwrap(),
        expected
    )
}

#[test]
fn it_should_throw_when_io_schedule_does_not_end_correctly() {
    let expected = "IOScheduleNoEndingSemicolon";

    let io_schedule = b"component1".to_vec();
    let components = vec![Compose {
        name: b"component1".to_vec(),
        code_txt: r#""#.as_bytes().to_vec(),

        exec_type: b"exec_escrow".to_vec(),
        dest: AccountId::new([1 as u8; 32]),
        value: 0,
        bytes: vec![],
        input_data: vec![],
    }];

    assert_err!(
        ExecDelivery::decompose_io_schedule(components, io_schedule),
        expected
    );
}

#[test]
fn it_should_throw_when_io_schedule_references_a_missing_component() {
    let expected = "IOScheduleUnknownCompose";

    let io_schedule = b"component1 | component2;".to_vec();
    let components = vec![Compose {
        name: b"component1".to_vec(),
        code_txt: r#""#.as_bytes().to_vec(),

        exec_type: b"exec_escrow".to_vec(),
        dest: AccountId::new([1 as u8; 32]),
        value: 0,
        bytes: vec![],
        input_data: vec![],
    }];

    assert_err!(
        ExecDelivery::decompose_io_schedule(components, io_schedule),
        expected
    );
}

#[test]
fn it_should_throw_with_empty_io_schedule() {
    let expected = "IOScheduleEmpty";

    let io_schedule = b"".to_vec();
    let components = vec![Compose {
        name: b"component1".to_vec(),
        code_txt: r#""#.as_bytes().to_vec(),

        exec_type: b"exec_escrow".to_vec(),
        dest: AccountId::new([1 as u8; 32]),
        value: 0,
        bytes: vec![],
        input_data: vec![],
    }];

    assert_err!(
        ExecDelivery::decompose_io_schedule(components, io_schedule),
        expected
    );
}

#[test]
fn test_authority_selection() {
    let keystore = KeyStore::new();

    // Insert Alice's keys
    const SURI_ALICE: &str = "//Alice";
    let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_ALICE,
        key_pair_alice.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Insert Bob's keys
    const SURI_BOB: &str = "//Bob";
    let key_pair_bob = sr25519::Pair::from_string(SURI_BOB, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_BOB,
        key_pair_bob.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Insert Charlie's keys
    const SURI_CHARLIE: &str = "//Charlie";
    let key_pair_charlie =
        sr25519::Pair::from_string(SURI_CHARLIE, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_CHARLIE,
        key_pair_charlie.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Alice's account
    // let escrow: AccountId = hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into();

    // Bob's account
    let escrow: AccountId =
        hex_literal::hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"]
            .into();
    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(keystore.into()));
    ext.execute_with(|| {
        let submitter = ExecDelivery::select_authority(escrow.clone());

        assert!(submitter.is_ok());
    });
}

#[test]
fn error_if_keystore_is_empty() {
    let keystore = KeyStore::new();

    // Alice's escrow account
    let escrow: AccountId =
        hex_literal::hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"]
            .into();

    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(keystore.into()));
    ext.execute_with(|| {
        let submitter = ExecDelivery::select_authority(escrow.clone());

        assert!(submitter.is_err());
    });
}

#[test]
fn error_if_incorrect_escrow_is_submitted() {
    let keystore = KeyStore::new();

    // Insert Alice's keys
    const SURI_ALICE: &str = "//Alice";
    let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_ALICE,
        key_pair_alice.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Insert Bob's keys
    const SURI_BOB: &str = "//Bob";
    let key_pair_bob = sr25519::Pair::from_string(SURI_BOB, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_BOB,
        key_pair_bob.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Insert Charlie's keys
    const SURI_CHARLIE: &str = "//Charlie";
    let key_pair_charlie =
        sr25519::Pair::from_string(SURI_CHARLIE, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_CHARLIE,
        key_pair_charlie.public().as_ref(),
    )
    .expect("Inserts unknown key");

    // Alice's original account => d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
    // Alice's tempered account => a51593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
    // The first 3 bytes are changed, thus making the account invalid
    let escrow: AccountId =
        hex_literal::hex!["a51593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]
            .into();

    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(keystore.into()));
    ext.execute_with(|| {
        let submitter = ExecDelivery::select_authority(escrow.clone());

        assert!(submitter.is_err());
    });
}

const CODE_CALL: &str = "code_call";

#[test]
fn dry_run_whole_xtx_unseen_contract_one_phase_and_one_step_success() {
    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        let mut contracts = vec![];
        let mut action_descriptions = vec![];
        let mut unseen_contracts = vec![];
        let mut contract_ids = vec![];

        let input_data = vec![];
        let dest = AccountId::new([1 as u8; 32]);
        let value = 0;

        let compose = make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, input_data, dest, value);

        let inter_schedule = InterExecSchedule {
            phases: vec![ExecPhase {
                steps: vec![ExecStep { compose: compose }],
            }],
        };

        let escrow_account: AccountId =
            hex_literal::hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"]
                .into();

        let requester: AccountId =
            hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]
                .into();

        let first_phase = inter_schedule
            .phases
            .get(0)
            .expect("At least one phase should be in inter schedule");

        let step = first_phase
            .steps
            .get(0)
            .expect("At least one step in a phase");

        insert_default_xdns_record();

        let unseen_contract =
            ExecComposer::dry_run_single_contract::<Test>(step.compose.clone()).unwrap();

        unseen_contracts.push(unseen_contract.clone());
        contracts.extend(unseen_contracts);

        action_descriptions.extend(unseen_contract.action_descriptions.clone());

        let mut protocol_part_of_contract = step.compose.code_txt.clone();
        protocol_part_of_contract.extend(step.compose.bytes.clone());

        let key = <Test as frame_system::Config>::Hashing::hash(
            Encode::encode(&mut protocol_part_of_contract).as_ref(),
        );

        contract_ids.push(key);

        let max_steps = contracts.len() as u32;

        let (current_block_no, block_zero) = (
            <frame_system::Pallet<Test>>::block_number(),
            <Test as frame_system::Config>::BlockNumber::zero(),
        );

        let xtx_schedule = XtxSchedule::new_sequential_from_contracts(
            contracts.clone(),
            contract_ids.clone(),
            vec![step.compose.clone()],
            action_descriptions.clone(),
            None,
            current_block_no.clone(),
        )
        .unwrap();

        
        let expected_xtx = Xtx {
            estimated_worth: Default::default(),
            current_worth: Default::default(),
            requester: requester.clone(),
            escrow_account: escrow_account.clone(),
            payload: vec![],
            current_step: 0,
            steps_no: max_steps,
            current_phase: 0,
            current_round: 0,
            result_status: vec![],
            phases_blockstamps: (current_block_no, block_zero),
            schedule: xtx_schedule,
        };

        assert_eq!(
            ExecDelivery::dry_run_whole_xtx(inter_schedule, escrow_account, requester),
            Ok((expected_xtx, contracts, contract_ids, action_descriptions))
        );
    });
}

#[test]
fn test_submit_composable_exec_order() {
    let dest = AccountId::new([1 as u8; 32]);
    let value = BalanceOf::<Test>::from(0u32);
    let input_data = vec![];
    let io_schedule = b"component1;".to_vec();

    let compose = make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, input_data, dest, value);

    let keystore = KeyStore::new();

    // Insert Alice's keys
    const SURI_ALICE: &str = "//Alice";
    let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    SyncCryptoStore::insert_unknown(
        &keystore,
        KEY_TYPE,
        SURI_ALICE,
        key_pair_alice.public().as_ref(),
    )
    .expect("Inserts unknown key");

    let mut ext = TestExternalities::new_empty();
    ext.register_extension(KeystoreExt(keystore.into()));
    ext.execute_with(|| {
        insert_default_xdns_record();
        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            io_schedule,
            vec![compose],
        ));
    });
}

use bp_test_utils::test_header;

use crate::{
    CurrentHeader, DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway,
    EthLikeKeccak256ValU64Gateway, PolkadotLikeValU64Gateway,
};

#[test]
fn test_register_gateway_with_default_polka_like_header() {
    let origin = Origin::root(); // only sudo access to register new gateways for now
    let url = b"ws://localhost:9944".to_vec();
    let gateway_id = [0; 4];
    let gateway_abi: GatewayABIConfig = Default::default();

    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::ProgrammableInternal(0);

    let _gateway_pointer = GatewayPointer {
        id: [0; 4],
        vendor: GatewayVendor::Substrate,
        gateway_type: GatewayType::ProgrammableInternal(0),
    };

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        signed_extension: None,
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let first_header: CurrentHeader<Test, DefaultPolkadotLikeGateway> = test_header(0);

    let authorities = Some(vec![]);

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(ExecDelivery::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            first_header.encode(),
            authorities
        ));
    });
}

#[test]
fn test_register_gateway_with_u64_substrate_header() {
    let origin = Origin::root(); // only sudo access to register new gateways for now
    let url = b"ws://localhost:9944".to_vec();
    let gateway_id = [0; 4];
    let gateway_abi: GatewayABIConfig = Default::default();

    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::ProgrammableInternal(0);

    let _gateway_pointer = GatewayPointer {
        id: [0; 4],
        vendor: GatewayVendor::Substrate,
        gateway_type: GatewayType::ProgrammableInternal(0),
    };

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        signed_extension: None,
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let first_header: CurrentHeader<Test, PolkadotLikeValU64Gateway> = test_header(0);

    let authorities = Some(vec![]);

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(ExecDelivery::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            first_header.encode(),
            authorities
        ));
    });
}

#[test]
fn test_register_gateway_with_default_eth_like_header() {
    let origin = Origin::root(); // only sudo access to register new gateways for now
    let url = b"ws://localhost:9944".to_vec();
    let gateway_id = [0; 4];
    let gateway_abi: GatewayABIConfig = Default::default();

    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::ProgrammableInternal(0);

    let _gateway_pointer = GatewayPointer {
        id: [0; 4],
        vendor: GatewayVendor::Substrate,
        gateway_type: GatewayType::ProgrammableInternal(0),
    };

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        signed_extension: None,
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let first_header: CurrentHeader<Test, EthLikeKeccak256ValU32Gateway> = test_header(0);

    let authorities = Some(vec![]);

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(ExecDelivery::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            first_header.encode(),
            authorities
        ));
    });
}

#[test]
fn test_register_gateway_with_u64_eth_like_header() {
    let origin = Origin::root(); // only sudo access to register new gateways for now
    let url = b"ws://localhost:9944".to_vec();
    let gateway_id = [0; 4];
    let gateway_abi: GatewayABIConfig = Default::default();

    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::ProgrammableInternal(0);

    let _gateway_pointer = GatewayPointer {
        id: [0; 4],
        vendor: GatewayVendor::Substrate,
        gateway_type: GatewayType::ProgrammableInternal(0),
    };

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        signed_extension: None,
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let first_header: CurrentHeader<Test, EthLikeKeccak256ValU64Gateway> = test_header(0);

    let authorities = Some(vec![]);

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(ExecDelivery::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            first_header.encode(),
            authorities
        ));
    });
}
