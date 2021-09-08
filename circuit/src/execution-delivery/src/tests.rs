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
use crate::{self as pallet_execution_delivery};

use frame_support::assert_err;

use sp_io;

use sp_core::{crypto::Pair, sr25519};
use sp_io::TestExternalities;
use sp_keystore::testing::KeyStore;
use sp_keystore::{KeystoreExt, SyncCryptoStore};

use pallet_execution_delivery::Compose;

use t3rn_primitives::{ExecPhase, ExecStep, InterExecSchedule};

use crate::mock::*;

use crate::mock::AccountId;

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
