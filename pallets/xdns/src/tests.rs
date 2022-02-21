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

//! Tests for pallet-xdns.

use super::*;
use crate::mock::{ExtBuilder, Test, XDNS};
use codec::Decode;
use frame_support::{assert_err, assert_noop, assert_ok};
use frame_system::Origin;
use sp_runtime::DispatchError;
use t3rn_primitives::{GatewayType, GatewayVendor, abi::Type};

#[test]
fn genesis_should_seed_circuit_gateway_polkadot_and_kusama_nodes() {
    let circuit_hash = <Test as frame_system::Config>::Hashing::hash(b"circ");
    let gateway_hash = <Test as frame_system::Config>::Hashing::hash(b"gate");
    let polkadot_hash = <Test as frame_system::Config>::Hashing::hash(b"pdot");
    let kusama_hash = <Test as frame_system::Config>::Hashing::hash(b"ksma");

    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 4);
            assert!(XDNSRegistry::<Test>::get(circuit_hash).is_some());
            assert!(XDNSRegistry::<Test>::get(gateway_hash).is_some());
            assert!(XDNSRegistry::<Test>::get(polkadot_hash).is_some());
            assert!(XDNSRegistry::<Test>::get(kusama_hash).is_some());
        });
}

#[test]
fn should_add_a_new_xdns_record_if_it_doesnt_exist() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(XDNS::add_new_xdns_record(
            Origin::<Test>::Root.into(),
            b"some_url".to_vec(),
            *b"test",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::TxOnly(0),
            Default::default(),
            Default::default(),
            vec![],
        ));
        assert_eq!(XDNSRegistry::<Test>::iter().count(), 1);
        assert!(
            XDNSRegistry::<Test>::get(<Test as frame_system::Config>::Hashing::hash(b"test"))
                .is_some()
        );
    });
}

#[test]
fn should_not_add_a_new_side_effect_if_it_exist() {
    ExtBuilder::default()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::add_side_effect(
                    Origin::<Test>::Root.into(),
                    *b"aliq",
                    b"add_liquidity".to_vec(),
                    vec![
                        Type::DynamicAddress,    // argument_0: caller
                        Type::DynamicAddress,    // argument_1: to
                        Type::DynamicBytes,      // argument_2: asset_left
                        Type::DynamicBytes,      // argument_3: asset_right
                        Type::DynamicBytes,      // argument_4: liquidity_token
                        Type::Value,             // argument_5: amount_left
                        Type::Value,             // argument_6: amount_right
                        Type::Value,             // argument_7: amount_liquidity_token
                        Type::OptionalInsurance, // argument_8: insurance
                    ],
                    vec![
                        b"caller".to_vec(),
                        b"to".to_vec(),
                        b"asset_left".to_vec(),
                        b"assert_right".to_vec(),
                        b"liquidity_token".to_vec(),
                        b"amount_left".to_vec(),
                        b"amount_right".to_vec(),
                        b"amount_liquidity_token".to_vec(),
                        b"insurance".to_vec(),
                    ],
                    vec![b"ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![b"ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![b"MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![
                        b"MultiTransfer(executor,caller,asset_left,amount_left)".to_vec(),
                        b"MultiTransfer(executor,caller,asset_right,amount_right)".to_vec()
                    ]
                ),
                crate::pallet::Error::<Test>::SideEffectInterfaceAlreadyExists
            );
            assert_eq!(CustomSideEffects::<Test>::iter().count(), 0);
        });
}

#[test]
fn should_add_standard_side_effects() {
    ExtBuilder::default()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            assert_eq!(StandardSideEffects::<Test>::get().unwrap().len(), 5);
        });
}

#[test]
fn should_add_a_new_side_effect_if_it_doesnt_exist() {
    ExtBuilder::default()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            assert_ok!(
                XDNS::add_side_effect(
                    Origin::<Test>::Root.into(),
                    *b"cust",
                    b"custom_side_effect".to_vec(),
                    vec![
                        Type::DynamicAddress,    // argument_0: caller
                        Type::DynamicAddress,    // argument_1: to
                        Type::DynamicBytes,      // argument_2: asset_left
                        Type::DynamicBytes,      // argument_3: asset_right
                        Type::DynamicBytes,      // argument_4: liquidity_token
                        Type::Value,             // argument_5: amount_left
                        Type::Value,             // argument_6: amount_right
                        Type::Value,             // argument_7: amount_liquidity_token
                        Type::OptionalInsurance, // argument_8: insurance
                    ],
                    vec![
                        b"caller".to_vec(),
                        b"to".to_vec(),
                        b"asset_left".to_vec(),
                        b"assert_right".to_vec(),
                        b"liquidity_token".to_vec(),
                        b"amount_left".to_vec(),
                        b"amount_right".to_vec(),
                        b"amount_liquidity_token".to_vec(),
                        b"insurance".to_vec(),
                    ],
                    vec![b"ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![b"ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![b"MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)".to_vec()],
                    vec![
                        b"MultiTransfer(executor,caller,asset_left,amount_left)".to_vec(),
                        b"MultiTransfer(executor,caller,asset_right,amount_right)".to_vec()
                    ]
                )
            );
            assert_eq!(CustomSideEffects::<Test>::iter().count(), 1);
            let side_effect = CustomSideEffects::<Test>::get(<Test as frame_system::Config>::Hashing::hash(b"cust")).unwrap();
            assert_eq!(side_effect.get_id(), *b"cust");
            assert_eq!(side_effect.get_name(), *b"custom_side_effect");
        });
}

#[test]
fn should_not_add_a_new_xdns_record_if_it_already_exists() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::add_new_xdns_record(
                    Origin::<Test>::Root.into(),
                    b"some_url".to_vec(),
                    *b"circ",
                    Default::default(),
                    GatewayVendor::Substrate,
                    GatewayType::TxOnly(0),
                    Default::default(),
                    Default::default(),
                    vec![],
                ),
                crate::pallet::Error::<Test>::XdnsRecordAlreadyExists
            );
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 4);
        });
}

#[test]
fn should_purge_a_xdns_record_successfully() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let gateway_hash = <Test as frame_system::Config>::Hashing::hash(b"gate");

            assert_ok!(XDNS::purge_xdns_record(
                Origin::<Test>::Root.into(),
                1,
                gateway_hash
            ));
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 3);
            assert!(XDNSRegistry::<Test>::get(gateway_hash).is_none());
        });
}

#[test]
fn finds_correct_amount_of_allowed_side_effects() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .with_standard_side_effects()
        .build()
        .execute_with(|| {
            assert_eq!(
                XDNS::allowed_side_effects(
                    b"circ"
                ).iter().count(),
                2
            )
        });

}

#[test]
fn should_error_trying_to_purge_a_missing_xdns_record() {
    let missing_hash = <Test as frame_system::Config>::Hashing::hash(b"miss");

    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_noop!(
                XDNS::purge_xdns_record(Origin::<Test>::Root.into(), 1, missing_hash),
                crate::pallet::Error::<Test>::UnknownXdnsRecord
            );
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 4);
        });
}

#[test]
fn should_error_trying_to_purge_an_xdns_record_if_not_root() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let gateway_hash = <Test as frame_system::Config>::Hashing::hash(b"gate");

            assert_noop!(
                XDNS::purge_xdns_record(Origin::<Test>::Signed(1).into(), 1, gateway_hash),
                DispatchError::BadOrigin
            );
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 4);
            assert!(XDNSRegistry::<Test>::get(gateway_hash).is_some());
        });
}

#[test]
fn should_update_ttl_for_a_known_xdns_record() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let gateway_hash = <Test as frame_system::Config>::Hashing::hash(b"gate");

            assert_ok!(XDNS::update_ttl(Origin::<Test>::Root.into(), *b"gate", 2));
            assert_eq!(XDNSRegistry::<Test>::iter().count(), 4);
            assert_eq!(
                XDNSRegistry::<Test>::get(gateway_hash)
                    .unwrap()
                    .last_finalized,
                Some(2)
            );
        });
}

#[test]
fn should_error_when_trying_to_update_ttl_for_a_missing_xdns_record() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            XDNS::update_ttl(Origin::<Test>::Root.into(), *b"miss", 2),
            crate::pallet::Error::<Test>::XdnsRecordNotFound
        );
    });
}

#[test]
fn should_error_when_trying_to_update_ttl_as_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            XDNS::update_ttl(Origin::<Test>::Signed(1).into(), *b"gate", 2),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn should_contain_gateway_system_properties() {
    let polkadot_hash = <Test as frame_system::Config>::Hashing::hash(b"pdot");
    let kusama_hash = <Test as frame_system::Config>::Hashing::hash(b"ksma");

    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let polkadot_xdns_record = XDNSRegistry::<Test>::get(polkadot_hash).unwrap();
            let kusama_xdns_record = XDNSRegistry::<Test>::get(kusama_hash).unwrap();
            let polkadot_symbol: Vec<u8> =
                Decode::decode(&mut &polkadot_xdns_record.gateway_sys_props.token_symbol[..])
                    .unwrap();
            let kusama_symbol: Vec<u8> =
                Decode::decode(&mut &kusama_xdns_record.gateway_sys_props.token_symbol[..])
                    .unwrap();

            assert_eq!(polkadot_xdns_record.gateway_sys_props.ss58_format, 0u16);
            assert_eq!(kusama_xdns_record.gateway_sys_props.ss58_format, 2u16);
            assert_eq!(&String::from_utf8_lossy(&polkadot_symbol), "DOT");
            assert_eq!(&String::from_utf8_lossy(&kusama_symbol), "KSM");
            assert_eq!(polkadot_xdns_record.gateway_sys_props.token_decimals, 10u8);
            assert_eq!(kusama_xdns_record.gateway_sys_props.token_decimals, 12u8);
        });
}

#[test]
fn fetch_abi_should_return_abi_for_a_known_xdns_record() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let actual = XDNS::get_abi(*b"pdot");
            assert_ok!(actual);
        });
}

#[test]
fn fetch_abi_should_error_for_unknown_xdns_record() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let actual = XDNS::get_abi(*b"rand");
            assert_err!(actual, "Xdns record not found");
        });
}
