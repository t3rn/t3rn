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
use crate::{mock::*, Error};
use codec::{Decode, Encode};
use frame_support::{assert_err, assert_noop, assert_ok, dispatch::PostDispatchInfo};
use frame_system::pallet_prelude::OriginFor;
use hex_literal::hex;
use serde_json::Value;
use sp_core::crypto::AccountId32;
use sp_io::TestExternalities;
use sp_runtime::{DispatchError, DispatchErrorWithPostInfo};
use sp_version::{create_runtime_str, RuntimeVersion};
use std::fs;
use t3rn_primitives::{
    abi::GatewayABIConfig,
    portal::{RegistrationData, RococoBridge},
    xdns::{AllowedSideEffect, Xdns},
    ChainId, EscrowTrait, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};

pub fn new_test_ext() -> TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    TestExternalities::new(t)
}

pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
    state_version: 1,
};

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

// iterates sequentially though all test files in mock-data
fn run_mock_tests() -> Result<(), DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let mut paths: Vec<_> = fs::read_dir("src/mock-data/")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());

    for entry in paths {
        let path = entry.path();
        let file = fs::read_to_string(&path).unwrap();
        let json: Value = serde_json::from_str(file.as_str()).unwrap();
        for entry in json.as_array().unwrap() {
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
                _ => unimplemented!(),
            }
        }
    }
    Ok(().into())
}

#[test]
fn runs_mock_tests() {
    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        run_mock_tests();
    });
}

#[test]
fn register_rococo_successfully() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::root(); // only sudo access to register new gateways for now
    ext.execute_with(|| {
        assert_ok!(register_file(origin, "1-register-roco.json", true, 0));
    });
}

#[test]
fn fails_registration_with_invalid_signer() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::signed([0u8; 32].into()); // only sudo access to register new gateways for now
    ext.execute_with(|| {
        assert_noop!(
            register_file(origin, "1-register-roco.json", false, 0),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn gateway_can_only_be_registered_once() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::root(); // only sudo access to register new gateways for now
    ext.execute_with(|| {
        assert_ok!(register_file(
            origin.clone(),
            "1-register-roco.json",
            false,
            0
        ));
        assert_noop!(
            register_file(origin, "1-register-roco.json", false, 0),
            pallet_xdns::Error::<Test>::XdnsRecordAlreadyExists
        );
    });
}

#[test]
fn cant_submit_without_registering() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::root();
    ext.execute_with(|| {
        assert_noop!(
            submit_header_file(origin, "2-headers-roco.json", 0),
            Error::<Test>::GatewayVendorNotFound
        );
    });
}

#[test]
fn cant_submit_with_gap() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::signed([0u8; 32].into());
    let root = Origin::root();
    ext.execute_with(|| {
        assert_ok!(register_file(root, "1-register-roco.json", true, 0));
        assert_noop!(
            submit_header_file(origin, "5-headers-roco.json", 0),
            Error::<Test>::SubmitHeaderError
        );
    });
}

#[test]
fn can_submit_valid_header_data() {
    let mut ext = TestExternalities::new_empty();
    let root = Origin::root();
    let origin = Origin::signed([0u8; 32].into());
    ext.execute_with(|| {
        assert_ok!(register_file(root, "1-register-roco.json", true, 0));
        assert_ok!(submit_header_file(origin.clone(), "2-headers-roco.json", 0));
        assert_noop!(
            // can't submit twice
            submit_header_file(origin, "2-headers-roco.json", 0),
            Error::<Test>::SubmitHeaderError
        );
    });
}

#[test]
fn can_register_parachain_and_add_header() {
    let mut ext = TestExternalities::new_empty();
    let root = Origin::root();
    let origin = Origin::signed([0u8; 32].into());
    ext.execute_with(|| {
        // ToDo activate once xdns is refactored
        // assert_noop!( // can't register parachain before relaychain
        //     register_file(root.clone(), "3-register-pang.json", false),
        //     Error::<Test>::RegistrationError
        // );
        assert_ok!(register_file(root.clone(), "1-register-roco.json", true, 0));
        assert_ok!(submit_header_file(origin.clone(), "2-headers-roco.json", 0));
        assert_noop!(
            submit_header_file(origin.clone(), "7-headers-pang.json", 0),
            Error::<Test>::GatewayVendorNotFound
        );
        assert_ok!(register_file(root.clone(), "4-register-pang.json", true, 0));
        assert_noop!(
            // needs relaychain header first
            submit_header_file(origin.clone(), "7-headers-pang.json", 0),
            Error::<Test>::SubmitHeaderError
        );
        assert_ok!(submit_header_file(origin.clone(), "5-headers-roco.json", 0),);
        assert_ok!(submit_header_file(origin.clone(), "7-headers-pang.json", 0),);
    });
}

#[test]
fn can_update_owner() {
    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        let one = AccountId::new([1u8; 32]);
        let two = AccountId::new([2u8; 32]);
        assert_ok!(register_file(
            Origin::root(),
            "1-register-roco.json",
            true,
            0
        ));
        assert_ok!(Portal::set_owner(
            Origin::root(),
            *b"roco",
            Some(one.clone()).encode()
        ));
        assert_noop!(
            Portal::set_owner(
                Origin::signed(two.clone()),
                *b"roco",
                Some(one.clone()).encode()
            ),
            Error::<Test>::SetOwnerError
        );
        assert_ok!(Portal::set_owner(
            Origin::signed(one.clone()),
            *b"roco",
            Some(two.clone()).encode()
        ),);
        assert_ok!(Portal::set_owner(
            Origin::signed(two.clone()),
            *b"roco",
            vec![0] // encoded none
        ),);
        assert_noop!(
            Portal::set_owner(
                Origin::signed(two.clone()),
                *b"roco",
                Some(one.clone()).encode()
            ),
            Error::<Test>::SetOwnerError
        );
        assert_noop!(
            Portal::set_owner(
                Origin::signed(one.clone()),
                *b"roco",
                Some(two.clone()).encode()
            ),
            Error::<Test>::SetOwnerError
        );
        assert_ok!(
            // root can still override for now
            Portal::set_owner(Origin::root(), *b"roco", Some(one.clone()).encode()),
        );
    });
}

#[test]
fn can_be_set_operational() {
    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        let one = AccountId::new([1u8; 32]);
        // let two = AccountId::new([2u8; 32]);
        // let root = Origin::root();
        let origin = Origin::signed([0u8; 32].into());

        assert_ok!(register_file(
            Origin::root(),
            "1-register-roco.json",
            true,
            0
        ));
        assert_ok!(Portal::set_operational(Origin::root(), *b"roco", false));
        assert_noop!(
            submit_header_file(origin.clone(), "2-headers-roco.json", 0),
            Error::<Test>::SubmitHeaderError
        );
        assert_ok!(Portal::set_owner(
            Origin::root(),
            *b"roco",
            Some(one.clone()).encode()
        ));
        assert_ok!(Portal::set_operational(Origin::signed(one), *b"roco", true));
        assert_ok!(submit_header_file(origin.clone(), "2-headers-roco.json", 0));
    });
}
