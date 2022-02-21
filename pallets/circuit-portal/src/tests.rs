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
use bp_test_utils::test_header;
use codec::Encode;
use frame_support::assert_ok;
use t3rn_primitives::bridges::test_utils as bp_test_utils;

use sp_core::Hasher;
use sp_io::TestExternalities;
use sp_version::{create_runtime_str, RuntimeVersion};

use t3rn_primitives::{abi::GatewayABIConfig, *};

use crate::{
    mock::*, AllowedSideEffect, CurrentHeader, DefaultPolkadotLikeGateway,
    EthLikeKeccak256ValU32Gateway, EthLikeKeccak256ValU64Gateway, PolkadotLikeValU64Gateway,
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
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let gateway_sys_props = GatewaySysProps {
        ss58_format: 0,
        token_symbol: Encode::encode(""),
        token_decimals: 0,
    };

    let first_header: CurrentHeader<Test, DefaultPolkadotLikeGateway> = test_header(0);

    let authorities = Some(vec![]);
    let allowed_side_effects = vec![];

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(Portal::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            first_header.encode(),
            authorities,
            allowed_side_effects,
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
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let gateway_sys_props = GatewaySysProps {
        ss58_format: 0,
        token_symbol: Encode::encode(""),
        token_decimals: 0,
    };

    let first_header: CurrentHeader<Test, PolkadotLikeValU64Gateway> = test_header(0);

    let authorities = Some(vec![]);
    let allowed_side_effects = vec![];

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(Portal::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            first_header.encode(),
            authorities,
            allowed_side_effects,
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
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let gateway_sys_props = GatewaySysProps {
        ss58_format: 0,
        token_symbol: Encode::encode(""),
        token_decimals: 0,
    };

    let first_header: CurrentHeader<Test, EthLikeKeccak256ValU32Gateway> = test_header(0);

    let authorities = Some(vec![]);
    let allowed_side_effects = vec![];

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(Portal::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            first_header.encode(),
            authorities,
            allowed_side_effects,
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
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let gateway_sys_props = GatewaySysProps {
        ss58_format: 0,
        token_symbol: Encode::encode(""),
        token_decimals: 0,
    };

    let first_header: CurrentHeader<Test, EthLikeKeccak256ValU64Gateway> = test_header(0);

    let authorities = Some(vec![]);
    let allowed_side_effects = vec![];

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| {
        assert_ok!(Portal::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            first_header.encode(),
            authorities,
            allowed_side_effects,
        ));
    });
}

#[test]
fn test_register_gateway_with_u64_substrate_header_and_allowed_side_effects() {
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
        runtime_version: TEST_RUNTIME_VERSION,
        genesis_hash: Default::default(),
        extrinsics_version: 0u8,
    };

    let gateway_sys_props = GatewaySysProps {
        ss58_format: 0,
        token_symbol: Encode::encode(""),
        token_decimals: 0,
    };

    let first_header: CurrentHeader<Test, PolkadotLikeValU64Gateway> = test_header(0);

    let authorities = Some(vec![]);

    let allowed_side_effects: Vec<AllowedSideEffect> = vec!["swap".into()];

    let mut ext = TestExternalities::new_empty();
    ext.execute_with(|| System::set_block_number(1));
    ext.execute_with(|| {
        assert_ok!(Portal::register_gateway(
            origin,
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor.clone(),
            gateway_type.clone(),
            gateway_genesis,
            gateway_sys_props.clone(),
            first_header.encode(),
            authorities,
            allowed_side_effects.clone(),
        ));

        // Assert the stored xdns record

        let xdns_id =
            <Test as frame_system::Config>::Hashing::hash(Encode::encode(&gateway_id).as_ref());
        let result = pallet_xdns::XDNSRegistry::<Test>::get(xdns_id);

        assert!(result.is_some());
        let xdns_record = result.unwrap();
        let stored_side_effects = xdns_record.allowed_side_effects;

        assert_eq!(stored_side_effects.len(), 1);
        assert_eq!(stored_side_effects, allowed_side_effects);

        // Assert events emitted

        System::assert_last_event(Event::Portal(crate::Event::NewGatewayRegistered(
            gateway_id,
            gateway_type,
            gateway_vendor,
            gateway_sys_props,
            allowed_side_effects,
        )));
        // XdnsRecordStored and NewGatewayRegistered
        assert_eq!(System::events().len(), 2);
    });
}
