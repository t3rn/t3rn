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

//! Runtimes for pallet-xdns.

use super::*;
use circuit_mock_runtime::{ExtBuilder, Portal, *};
use codec::Decode;

use frame_support::{assert_err, assert_noop, assert_ok, traits::OnInitialize};
use frame_system::Origin;
use sp_core::crypto::AccountId32;
use sp_runtime::DispatchError;
use t3rn_primitives::{
    circuit::SecurityLvl::{Escrow, Optimistic},
    portal::Portal as PortalT,
    xdns::{FullGatewayRecord, GatewayRecord, PalletAssetsOverlay, Xdns},
    EthereumToken, ExecutionVendor,
    ExecutionVendor::{Substrate, EVM},
    FinalityVerifierActivity, GatewayActivity, GatewayVendor,
    GatewayVendor::{Ethereum, Kusama, Polkadot, Rococo},
    SpeedMode, SubstrateToken, TokenInfo,
};

use t3rn_abi::Codec::{Rlp, Scale};
use t3rn_primitives::xdns::EpochEstimate;

use t3rn_types::fsx::SecurityLvl;

const DEFAULT_GATEWAYS_IN_STORAGE_COUNT: usize = 8;
const STANDARD_SFX_ABI_COUNT: usize = 6;

#[test]
fn reboot_self_gateway_populates_entry_if_does_not_exist_with_all_sfx() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);
            assert_ok!(XDNS::reboot_self_gateway(
                circuit_mock_runtime::Origin::root(),
                GatewayVendor::Rococo
            ));
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3])
                    .unwrap()
                    .allowed_side_effects
                    .len(),
                7
            );
        });
}

#[test]
fn reboot_self_gateway_populates_entry_if_does_not_exist_with_no_sfx() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);
        assert_ok!(XDNS::reboot_self_gateway(
            circuit_mock_runtime::Origin::root(),
            GatewayVendor::Rococo
        ));
        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
        assert_eq!(
            pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3])
                .unwrap()
                .allowed_side_effects
                .len(),
            0
        );
    });
}

#[test]
fn genesis_should_seed_circuit_gateway_polkadot_and_kusama_nodes() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );
            assert!(pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3]).is_some());
            assert!(pallet_xdns::Gateways::<Runtime>::get(b"gate").is_some());
            assert!(pallet_xdns::Gateways::<Runtime>::get(b"pdot").is_some());
            assert!(pallet_xdns::Gateways::<Runtime>::get(b"ksma").is_some());
        });
}

#[test]
fn should_add_a_new_xdns_record_if_it_doesnt_exist() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(XDNS::add_new_gateway(
            *b"test",
            GatewayVendor::Rococo,
            ExecutionVendor::Substrate,
            t3rn_abi::Codec::Scale,
            None,   // registrant
            None,   // escrow_account
            vec![], // allowed_side_effects
        ));
        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
        assert!(pallet_xdns::Gateways::<Runtime>::get(b"test").is_some());
    });
}

fn add_self_as_base_gateway() {
    assert_ok!(XDNS::add_new_gateway(
        [3, 3, 3, 3],
        GatewayVendor::Rococo,
        ExecutionVendor::Substrate,
        t3rn_abi::Codec::Scale,
        None,   // registrant
        None,   // escrow_account
        vec![], // allowed_side_effects
    ));
}

#[test]
fn should_add_a_new_xdns_and_record_and_token_if_it_doesnt_exist() {
    ExtBuilder::default().build().execute_with(|| {
        // Add the self-gateway
        add_self_as_base_gateway();

        assert_ok!(XDNS::add_new_gateway(
            *b"test",
            GatewayVendor::Rococo,
            ExecutionVendor::Substrate,
            t3rn_abi::Codec::Scale,
            None,   // registrant
            None,   // escrow_account
            vec![], // allowed_side_effects
        ));

        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 2);
        assert!(pallet_xdns::Gateways::<Runtime>::get(b"test").is_some());

        assert_ok!(XDNS::register_new_token(
            &circuit_mock_runtime::Origin::root(),
            u32::from_le_bytes(*b"test"),
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            })
        ));

        assert_ok!(XDNS::link_token_to_gateway(
            u32::from_le_bytes(*b"test"),
            *b"test",
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            })
        ));

        // no duplicates
        assert_noop!(
            XDNS::link_token_to_gateway(
                u32::from_le_bytes(*b"test"),
                *b"test",
                TokenInfo::Substrate(SubstrateToken {
                    decimals: 18,
                    symbol: b"test".to_vec(),
                    id: 5
                })
            ),
            pallet_xdns::pallet::Error::<Runtime>::TokenRecordAlreadyExists
        );

        // no mismatched execution vendor
        assert_noop!(
            XDNS::link_token_to_gateway(
                u32::from_le_bytes(*b"test"),
                *b"test",
                TokenInfo::Ethereum(EthereumToken {
                    decimals: 18,
                    symbol: b"test".to_vec(),
                    address: Some([1; 20])
                })
            ),
            pallet_xdns::pallet::Error::<Runtime>::TokenRecordAlreadyExists
        );

        assert_eq!(pallet_xdns::Tokens::<Runtime>::iter().count(), 2);
    });
}

#[test]
fn should_not_link_token_without_gateway_record() {
    ExtBuilder::default().build().execute_with(|| {
        // no duplicates
        assert_noop!(
            XDNS::link_token_to_gateway(
                u32::from_le_bytes(*b"test"),
                *b"test",
                TokenInfo::Substrate(SubstrateToken {
                    decimals: 18,
                    symbol: b"test".to_vec(),
                    id: 5
                })
            ),
            pallet_xdns::pallet::Error::<Runtime>::GatewayRecordNotFound
        );
    });
}

#[test]
fn should_add_standard_sfx_abi() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::StandardSFXABIs::<Runtime>::iter().count(), 9);
        });
}

#[test]
fn should_not_add_a_new_xdns_record_if_it_already_exists() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::add_new_gateway(
                    [3, 3, 3, 3],
                    GatewayVendor::Rococo,
                    ExecutionVendor::Substrate,
                    t3rn_abi::Codec::Scale,
                    None,   // registrant
                    None,   // escrow_account
                    vec![], // allowed_side_effects
                ),
                pallet_xdns::pallet::Error::<Runtime>::GatewayRecordAlreadyExists
            );
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );
        });
}

#[test]
fn should_register_token_and_populate_assets_storage_successfully() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );

            assert!(!Runtime::contains_asset(&u32::from_le_bytes(*b"test")));

            assert_ok!(XDNS::register_new_token(
                &circuit_mock_runtime::Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                })
            ));

            assert!(Runtime::contains_asset(&u32::from_le_bytes(*b"test")));
        });
}

#[test]
fn should_purge_token_and_destroy_asset_storage_successfully() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );

            assert!(!Runtime::contains_asset(&u32::from_le_bytes(*b"test")));

            assert_ok!(XDNS::register_new_token(
                &circuit_mock_runtime::Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                })
            ));

            assert!(Runtime::contains_asset(&u32::from_le_bytes(*b"test")));

            assert_ok!(XDNS::purge_token_record(
                circuit_mock_runtime::Origin::root(),
                u32::from_le_bytes(*b"test"),
            ));

            assert!(!Runtime::contains_asset(&u32::from_le_bytes(*b"test")));
        });
}

#[test]
fn should_purge_a_gateway_record_successfully() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );
            assert_ok!(XDNS::register_new_token(
                &circuit_mock_runtime::Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                })
            ));

            assert_ok!(XDNS::link_token_to_gateway(
                u32::from_le_bytes(*b"test"),
                *b"gate",
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                })
            ));

            assert_eq!(
                pallet_xdns::Tokens::<Runtime>::iter_values()
                    .filter(|token| token.gateway_id == *b"gate")
                    .count(),
                1
            );

            assert_eq!(
                pallet_xdns::GatewayTokens::<Runtime>::get(*b"gate"),
                vec![u32::from_le_bytes(*b"test")]
            );

            assert!(
                pallet_xdns::Tokens::<Runtime>::get(u32::from_le_bytes(*b"test"), *b"gate")
                    .is_some(),
            );

            assert_ok!(XDNS::purge_gateway_record(
                Origin::<Runtime>::Root.into(),
                ALICE,
                *b"gate"
            ));

            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT - 1
            );
            assert!(pallet_xdns::Gateways::<Runtime>::get(b"gate").is_none());
            // should leave the token record intact registered on the base
            assert!(pallet_xdns::Tokens::<Runtime>::get(
                u32::from_le_bytes(*b"test"),
                [3, 3, 3, 3]
            )
            .is_some());

            assert!(
                pallet_xdns::Tokens::<Runtime>::get(u32::from_le_bytes(*b"test"), *b"gate")
                    .is_none(),
            );
        });
}

#[test]
fn finds_correct_amount_of_allowed_side_effects() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                XDNS::allowed_side_effects(&[3, 3, 3, 3]).len(),
                STANDARD_SFX_ABI_COUNT
            )
        });
}

#[test]
fn should_error_trying_to_purge_a_missing_xdns_record() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::purge_gateway_record(Origin::<Runtime>::Root.into(), ALICE, *b"miss"),
                pallet_xdns::pallet::Error::<Runtime>::XdnsRecordNotFound
            );
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );
        });
}

#[test]
fn should_error_trying_to_purge_an_xdns_record_if_not_root() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::purge_gateway_record(
                    Origin::<Runtime>::Signed(ALICE).into(),
                    ALICE,
                    *b"gate"
                ),
                DispatchError::BadOrigin
            );
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );
            assert!(pallet_xdns::Gateways::<Runtime>::get(b"gate").is_some());
        });
}

#[test]
fn gate_gateway_vendor_returns_error_for_unknown_record() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let actual = XDNS::get_verification_vendor(b"rand");
            assert_err!(actual, pallet_xdns::Error::<Runtime>::XdnsRecordNotFound);
        });
}

#[test]
fn gate_gateway_vendor_returns_vendor_for_known_record() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let actual = XDNS::get_verification_vendor(b"pdot");
            assert_ok!(actual, GatewayVendor::Polkadot);
        });
}

#[test]
fn xdns_returns_full_gateway_record() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                XDNS::fetch_full_gateway_records(),
                vec![
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [0, 0, 0, 0],
                            verification_vendor: Rococo,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4)),
                                ([115, 119, 97, 112], Some(3)),
                                ([97, 108, 105, 113], Some(3)),
                                ([99, 101, 118, 109], Some(10)),
                                ([119, 97, 115, 109], Some(10)),
                            ]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [1, 1, 1, 1],
                            verification_vendor: Polkadot,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4)),
                                ([115, 119, 97, 112], Some(3)),
                                ([97, 108, 105, 113], Some(3)),
                                ([99, 101, 118, 109], Some(10)),
                                ([119, 97, 115, 109], Some(10)),
                            ]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [3, 3, 3, 3],
                            verification_vendor: Polkadot,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4)),
                                ([115, 119, 97, 112], Some(3)),
                                ([97, 108, 105, 113], Some(3)),
                                ([99, 101, 118, 109], Some(10)),
                                ([119, 97, 115, 109], Some(10)),
                            ]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [5, 5, 5, 5],
                            verification_vendor: Polkadot,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4)),
                                ([115, 119, 97, 112], Some(3)),
                                ([97, 108, 105, 113], Some(3)),
                                ([99, 101, 118, 109], Some(10)),
                                ([119, 97, 115, 109], Some(10)),
                            ]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [101, 116, 104, 50],
                            verification_vendor: Ethereum,
                            execution_vendor: EVM,
                            codec: Rlp,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![([116, 114, 97, 110], Some(2))]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [103, 97, 116, 101],
                            verification_vendor: Rococo,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![([116, 114, 97, 110], Some(2))]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [107, 115, 109, 97],
                            verification_vendor: Kusama,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4))
                            ]
                        },
                        tokens: vec![]
                    },
                    FullGatewayRecord {
                        gateway_record: GatewayRecord {
                            gateway_id: [112, 100, 111, 116],
                            verification_vendor: Polkadot,
                            execution_vendor: Substrate,
                            codec: Scale,
                            registrant: None,
                            escrow_account: None,
                            allowed_side_effects: vec![
                                ([116, 114, 97, 110], Some(2)),
                                ([116, 97, 115, 115], Some(4))
                            ]
                        },
                        tokens: vec![]
                    }
                ]
            );
        });
}

#[test]
fn xdns_returns_error_for_inactive_gateway() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let is_active_res = XDNS::verify_active(b"pdot", 0u32, &SecurityLvl::Optimistic);
            assert!(is_active_res.is_err());
        });
}

#[test]
fn xdns_overview_returns_activity_for_all_registered_targets_after_turning_on_via_portal() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            for gateway in XDNS::fetch_full_gateway_records().iter() {
                // ToDo: Uncomment when eth2::turn_on implemented
                if gateway.gateway_record.verification_vendor == Ethereum {
                    continue
                }
                Portal::turn_on(
                    circuit_mock_runtime::Origin::root(),
                    gateway.gateway_record.gateway_id,
                )
                .unwrap();
            }

            assert_eq!(XDNS::process_all_verifier_overviews(10), ());
            assert_eq!(XDNS::process_overview(10), ());
            let overview = XDNS::gateways_overview();

            assert_eq!(
                overview,
                vec![
                    GatewayActivity {
                        gateway_id: [0, 0, 0, 0],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [1, 1, 1, 1],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [3, 3, 3, 3],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [5, 5, 5, 5],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [103, 97, 116, 101],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [107, 115, 109, 97],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [112, 100, 111, 116],
                        reported_at: 10,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    }
                ]
            );
        });
}

#[test]
fn xdns_overview_returns_activity_for_all_registered_targets_after_turning_on_via_portal_and_adding_attestation_target(
) {
    use circuit_mock_runtime::Attesters;
    use sp_core::H256;
    use t3rn_primitives::attesters::AttestersWriteApi;

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            assert_ok!(XDNS::add_escrow_account(
                circuit_mock_runtime::Origin::root(),
                [1, 1, 1, 1],
                AccountId32::new([1; 32])
            ));

            Attesters::force_activate_target(circuit_mock_runtime::Origin::root(), [1, 1, 1, 1])
                .unwrap();
            Attesters::request_sfx_attestation_commit([1, 1, 1, 1], H256::repeat_byte(1));
            Attesters::on_initialize(System::block_number());

            assert_eq!(XDNS::process_overview(System::block_number()), ());
            let overview = XDNS::gateways_overview();

            assert_eq!(
                overview,
                vec![
                    GatewayActivity {
                        gateway_id: [0, 0, 0, 0],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [1, 1, 1, 1],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Escrow,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [3, 3, 3, 3],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [5, 5, 5, 5],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [103, 97, 116, 101],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [107, 115, 109, 97],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    },
                    GatewayActivity {
                        gateway_id: [112, 100, 111, 116],
                        reported_at: 17,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: true
                    }
                ]
            );
        });
}

#[test]
fn on_initialize_should_update_update_verifiers_overview_no_more_often_than_each_50_blocks() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            let expected_verifier_overview_all_off = vec![
                FinalityVerifierActivity {
                    verifier: Polkadot,
                    reported_at: 74,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: false,
                },
                FinalityVerifierActivity {
                    verifier: Kusama,
                    reported_at: 74,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: false,
                },
                FinalityVerifierActivity {
                    verifier: Rococo,
                    reported_at: 74,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: false,
                },
                FinalityVerifierActivity {
                    verifier: Ethereum,
                    reported_at: 74,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: false,
                },
            ];

            let expected_verifier_overview_all_on = vec![
                FinalityVerifierActivity {
                    verifier: Polkadot,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: Kusama,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: Rococo,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: Ethereum,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
            ];

            assert_eq!(XDNS::verifier_overview(), expected_verifier_overview_all_on);

            // Turn all the gateways off at the beginning. expect that the verifiers overview will be updated only after 50 blocks
            for gateway in XDNS::fetch_full_gateway_records().iter() {
                Portal::turn_off(
                    circuit_mock_runtime::Origin::root(),
                    gateway.gateway_record.gateway_id,
                )
                .unwrap();
            }

            let last_reported_block = expected_verifier_overview_all_on[0].reported_at;

            System::set_block_number(last_reported_block + 1);
            XDNS::on_initialize(System::block_number());
            assert_eq!(XDNS::verifier_overview(), expected_verifier_overview_all_on);

            System::set_block_number(last_reported_block + 5);
            XDNS::on_initialize(System::block_number());
            assert_eq!(XDNS::verifier_overview(), expected_verifier_overview_all_on);

            System::set_block_number(System::block_number() + 52);
            XDNS::on_initialize(System::block_number());

            assert_eq!(
                XDNS::verifier_overview(),
                expected_verifier_overview_all_off
            );
        });
}

#[test]
fn get_slowest_verifier_target_applies_emergency_offset_without_epochs_history() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let emergency_offset: BlockNumber = 100;
            let speed_mode = SpeedMode::Fast;

            // Get all targets
            let all_targets = XDNS::all_gateway_ids();

            // Test the function
            let result =
                XDNS::get_slowest_verifier_target(all_targets, &speed_mode, emergency_offset);

            println!("result: {result:?}");

            // Write asserts based on the expected output
            match result {
                Some((verifier, target, local_offset, remote_offset)) => {
                    // Check that the verifier and target are expected values
                    // You may need to implement PartialEq for GatewayVendor and TargetId
                    assert_eq!(verifier, GatewayVendor::Ethereum);
                    assert_eq!(target, *b"eth2");

                    // Check that the offsets are correct
                    assert_eq!(local_offset, emergency_offset);
                    assert_eq!(remote_offset, emergency_offset);
                },
                None => panic!("Expected Some, got None"),
            }
        });
}

#[test]
fn get_slowest_verifier_target_selects_slowest_for_filled_epoch_history() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let emergency_offset: BlockNumber = 100;
            let speed_mode = SpeedMode::Finalized;

            // Get all targets
            let all_targets = XDNS::all_gateway_ids();

            // Set the epoch history for the Ethereum verifier
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Ethereum,
                vec![EpochEstimate::<u32> {
                    local: 48,
                    remote: 32,
                    moving_average_local: 46,
                    moving_average_remote: 32,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Rococo,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Kusama,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Polkadot,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );

            // Test the function
            let result =
                XDNS::get_slowest_verifier_target(all_targets, &speed_mode, emergency_offset);

            println!("result: {result:?}");

            // Write asserts based on the expected output
            match result {
                Some((verifier, target, local_offset, remote_offset)) => {
                    // Check that the verifier and target are expected values
                    // You may need to implement PartialEq for GatewayVendor and TargetId
                    assert_eq!(verifier, GatewayVendor::Ethereum);
                    assert_eq!(target, *b"eth2");

                    // Check that the offsets are correct
                    assert_eq!(local_offset, 3 * 46); // 3 x moving_average_local
                    assert_eq!(remote_offset, 3 * 32); // 3 x moving_average_remote
                },
                None => panic!("Expected Some, got None"),
            }
        });
}

#[test]
fn test_estimate_adaptive_timeout_on_slowest_target() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let emergency_offset: BlockNumber = 100;
            let speed_mode = SpeedMode::Finalized;

            // Get all targets
            let all_targets = XDNS::all_gateway_ids();

            // Set the epoch history for the Ethereum verifier
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Ethereum,
                vec![EpochEstimate::<u32> {
                    local: 48,
                    remote: 32,
                    moving_average_local: 46,
                    moving_average_remote: 32,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Rococo,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Kusama,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );
            pallet_xdns::EpochHistory::<Runtime>::insert(
                Polkadot,
                vec![EpochEstimate::<u32> {
                    local: 3,
                    remote: 4,
                    moving_average_local: 4,
                    moving_average_remote: 3,
                }],
            );

            // Test the function
            let result = XDNS::estimate_adaptive_timeout_on_slowest_target(
                all_targets,
                &speed_mode,
                emergency_offset,
            );

            println!("result: {result:?}");

            // Write asserts based on the expected output
            assert_eq!(result.there, *b"eth2"); // target.clone()
            assert_eq!(result.estimated_height_here, 293); // submit_by_height_here + submit_by_local_offset
            assert_eq!(result.estimated_height_there, 216); // submit_by_height_there + submit_by_remote_offset
            assert_eq!(result.submit_by_height_here, 155); // current_block + submit_by_local_offset
            assert_eq!(result.submit_by_height_there, 120); // latest_overview_of_verifier.finalized_height + submit_by_remote_offset
            assert_eq!(result.emergency_timeout_here, 117); // emergency_offset + current_block
            assert_eq!(result.dlq, None); // default value
        });
}

#[test]
fn xdns_overview_returns_activity_for_all_registered_but_not_active_after_turning_off() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            for gateway in XDNS::fetch_full_gateway_records().iter() {
                Portal::turn_off(
                    circuit_mock_runtime::Origin::root(),
                    gateway.gateway_record.gateway_id,
                )
                .unwrap();
            }
            assert_eq!(XDNS::process_all_verifier_overviews(100), ());
            assert_eq!(XDNS::process_overview(100), ());

            let overview = XDNS::gateways_overview();

            assert_eq!(
                overview,
                vec![
                    GatewayActivity {
                        gateway_id: [0, 0, 0, 0],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [1, 1, 1, 1],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [3, 3, 3, 3],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [5, 5, 5, 5],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [103, 97, 116, 101],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [107, 115, 109, 97],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    },
                    GatewayActivity {
                        gateway_id: [112, 100, 111, 116],
                        reported_at: 100,
                        justified_height: 24,
                        finalized_height: 24,
                        updated_height: 24,
                        attestation_latency: None,
                        security_lvl: Optimistic,
                        is_active: false
                    }
                ]
            );
        });
}

#[test]
fn test_storage_migration_v140_to_v150_for_standard_side_effects_to_standard_sfx_abi() {
    type EventSignature = Vec<u8>;
    use t3rn_abi::SFXAbi;
    use t3rn_types::gateway::{CryptoAlgo, HasherAlgo};

    #[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub enum Type {
        Address(u16),
        DynamicAddress,
        Bool,
        Int(u16),
        Uint(u16),
        /// where u8 is bytes length
        Bytes(u8),
        DynamicBytes,
        String,
        Enum(u8),
        Struct(u8),
        Mapping(Box<Type>, Box<Type>),
        Contract,
        Ref(Box<Type>),
        Option(Box<Type>),
        OptionalInsurance,
        OptionalReward,
        StorageRef(Box<Type>),
        /// There is no way to declare value in Solidity (should there be?)
        Value,
        /// DynamicBytes and String are lowered to a vector.
        Slice,
        Hasher(HasherAlgo, u16),
        Crypto(CryptoAlgo),
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub struct SideEffectInterface {
        pub id: [u8; 4],
        pub name: SideEffectName,
        pub argument_abi: Vec<Type>,
        pub argument_to_state_mapper: Vec<EventSignature>,
        pub confirm_events: Vec<EventSignature>,
        pub escrowed_events: Vec<EventSignature>,
        pub commit_events: Vec<EventSignature>,
        pub revert_events: Vec<EventSignature>,
    }

    fn get_transfer_interface() -> SideEffectInterface {
        SideEffectInterface {
            id: *b"tran",
            name: b"transfer".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: from
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: value
                Type::OptionalInsurance, // argument_3: insurance
            ],
            argument_to_state_mapper: vec![
                b"from".to_vec(),
                b"to".to_vec(),
                b"value".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"Transfer(_executor,to,value)".to_vec()],
            escrowed_events: vec![b"Transfer(_source,_executor,to,value)".to_vec()],
            commit_events: vec![b"Transfer(_executor,to,value)".to_vec()],
            revert_events: vec![b"Transfer(_executor,from,value)".to_vec()],
        }
    }

    fn get_swap_interface() -> SideEffectInterface {
        SideEffectInterface {
            id: *b"swap",
            name: b"swap".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: caller
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: amount_from
                Type::Value,             // argument_3: amount_to
                Type::DynamicBytes,      // argument_4: asset_from
                Type::DynamicBytes,      // argument_5: asset_to
                Type::OptionalInsurance, // argument_6: insurance
            ],
            argument_to_state_mapper: vec![
                b"caller".to_vec(),
                b"to".to_vec(),
                b"amount_from".to_vec(),
                b"amount_to".to_vec(),
                b"asset_from".to_vec(),
                b"asset_to".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"MultiTransfer(_executor,to,asset_to,amount_to)".to_vec()],
            escrowed_events: vec![
                b"MultiTransfer(_source,_executor,to,asset_to,amount_to)".to_vec()
            ],
            commit_events: vec![b"MultiTransfer(_executor,to,asset_to,amount_to)".to_vec()],
            revert_events: vec![b"MultiTransfer(_executor,caller,asset_from,amount_from)".to_vec()],
        }
    }

    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // Insert some old storage entries
            let old_entries = vec![
                (*b"tran", get_transfer_interface()),
                (*b"swap", get_swap_interface()),
            ];

            for (key, value) in old_entries.clone() {
                // pallet_contracts_registry::ContractsRegistry::<Runtime>::
                // assume encoded form will be the same as the old storage
                pallet_xdns::StandardSideEffects::<Runtime>::insert(key, value.encode());
            }

            // Ensure the old storage entries are present
            for (key, value) in old_entries.iter() {
                assert_eq!(
                    pallet_xdns::StandardSideEffects::<Runtime>::get(key),
                    Some(value.encode())
                );
            }

            // Perform the runtime upgrade (call the `on_runtime_upgrade` function)
            let consumed_weight =
                <XDNS as frame_support::traits::OnRuntimeUpgrade>::on_runtime_upgrade();
            let max_weight =
                <Runtime as frame_system::Config>::DbWeight::get().reads_writes(10, 10);
            assert_eq!(consumed_weight, max_weight);

            // Ensure the old storage entries are removed
            for (key, _) in old_entries.iter() {
                assert!(pallet_xdns::StandardSideEffects::<Runtime>::get(key).is_none());
            }

            // Ensure the new storage entries are created
            for (key, _value) in old_entries.iter() {
                let sfx4b_id = *key;
                assert_eq!(
                    pallet_xdns::StandardSFXABIs::<Runtime>::get(sfx4b_id),
                    SFXAbi::get_standard_interface(sfx4b_id)
                );
            }
        });
}
