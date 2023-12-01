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
use circuit_mock_runtime::{ExtBuilder, Portal, RuntimeOrigin as Origin, *};
use codec::Decode;

use frame_support::pallet_prelude::Weight;

use frame_support::{assert_err, assert_noop, assert_ok, traits::OnInitialize};
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::DispatchError;
use t3rn_primitives::{
    circuit::SecurityLvl::{Escrow, Optimistic},
    clock::OnHookQueues,
    portal::Portal as PortalT,
    xdns::{FullGatewayRecord, GatewayRecord, PalletAssetsOverlay, Xdns},
    EthereumToken, ExecutionVendor,
    ExecutionVendor::{Substrate, EVM},
    FinalityVerifierActivity, GatewayActivity, GatewayVendor,
    GatewayVendor::{Ethereum, Kusama, Polkadot, Rococo},
    SpeedMode, SubstrateToken, TokenInfo, XDNSTopology,
};

use t3rn_abi::Codec::{Rlp, Scale};
use t3rn_primitives::{
    xdns::EpochEstimate,
    GatewayVendor::{Attesters, Sepolia, XBI},
};

use t3rn_types::fsx::SecurityLvl;
use xcm::latest::{prelude::MultiLocation, Junctions::Here};

const DEFAULT_GATEWAYS_IN_STORAGE_COUNT: usize = 8;
const STANDARD_SFX_ABI_COUNT: usize = 6;
const DEFAULT_MULTI_LOCATION: MultiLocation = MultiLocation {
    parents: 1,
    interior: Here,
};

#[test]
fn reboot_self_gateway_populates_entry_if_does_not_exist_with_all_sfx() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);
            assert_ok!(XDNS::reboot_self_gateway(
                Origin::root(),
                GatewayVendor::Rococo
            ));
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3])
                    .unwrap()
                    .allowed_side_effects
                    .len(),
                4
            );
        });
}

#[test]
fn reboot_self_gateway_populates_entry_all_gateway_ids_entry_only_once() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);
            assert_ok!(XDNS::reboot_self_gateway(
                Origin::root(),
                GatewayVendor::Rococo
            ));
            assert_ok!(XDNS::reboot_self_gateway(
                Origin::root(),
                GatewayVendor::Rococo
            ));
            assert_ok!(XDNS::reboot_self_gateway(
                Origin::root(),
                GatewayVendor::Rococo
            ));

            assert_eq!(XDNS::all_gateway_ids(), vec![[3, 3, 3, 3]]);
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3])
                    .unwrap()
                    .allowed_side_effects
                    .len(),
                4
            );
        });
}

#[test]
fn reboot_self_gateway_refreshes_entries_of_std_abi_to_other_gateways() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 8);
            // Manually add and set "tass" to dummy value for "test" gateway
            let swap_abi = pallet_xdns::StandardSFXABIs::<Runtime>::get(b"swap").unwrap();
            pallet_xdns::SFXABIRegistry::<Runtime>::insert(*b"gate", *b"tass", swap_abi);

            // set "tass" from standard to variable
            let tass_abi = pallet_xdns::StandardSFXABIs::<Runtime>::get(*b"tass").unwrap();
            assert_ok!(XDNS::reboot_self_gateway(
                Origin::root(),
                GatewayVendor::Rococo
            ));

            assert_eq!(
                pallet_xdns::SFXABIRegistry::<Runtime>::get(*b"gate", *b"tass"),
                Some(tass_abi)
            );
        });
}

#[test]
fn reboot_self_gateway_populates_entry_if_does_not_exist_with_std_sfx() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);
        assert_ok!(XDNS::reboot_self_gateway(
            Origin::root(),
            GatewayVendor::Rococo
        ));
        assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 1);
        assert_eq!(
            pallet_xdns::Gateways::<Runtime>::get([3, 3, 3, 3])
                .unwrap()
                .allowed_side_effects
                .len(),
            4
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

use hex_literal::hex;
#[test]
fn should_zip_xdns_topology_for_default_records_via_event() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_ok!(XDNS::zip_topology(Origin::signed(ALICE)));
            // Retrieve the event and check the zip
            let all_events = System::events();
            let last_system_event = all_events.last();
            assert!(last_system_event.clone().is_some());
            let xdns_topology_zip = match last_system_event {
                Some(event) => match &event.event {
                    RuntimeEvent::XDNS(pallet_xdns::Event::XDNSTopologyZip(topology)) =>
                        topology.clone(),
                    _ => panic!(
                        "expected last event to be pallet_xdns::Event::XDNSTopologyZip: no different event emitted"
                    ),
                },
                None => panic!(
                    "expected last event to be pallet_xdns::Event::XDNSTopologyZip: no last event emitted"
                ),
            };

            assert_eq!(xdns_topology_zip.encode(), hex!("20000000000200000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00010101010000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00030303030000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00050505050000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00657468320301010000047472616e010200676174650200000000047472616e0102006b736d610100000000087472616e01027461737301040070646f740000000000087472616e01027461737301040000").to_vec())
        });
}

#[test]
fn should_unzip_xdns_topology_from_decoded_form_with_default_records() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {

            // check that the topology is empty via all assets and all gateways ids
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);

            let encoded_xdns_topology = hex!("20000000000200000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00010101010000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00030303030000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00050505050000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00657468320301010000047472616e010200676174650200000000047472616e0102006b736d610100000000087472616e01027461737301040070646f740000000000087472616e01027461737301040000").to_vec();
            // Decode the topology
            let decoded_xdns_topology = XDNSTopology::<AccountId32>::decode(&mut &encoded_xdns_topology[..]).unwrap();

            // Unzip the topology
            assert_ok!(XDNS::unzip_topology(Origin::root(), Some(decoded_xdns_topology), None));

            // check that the topology is not empty via all assets and all gateways ids
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), DEFAULT_GATEWAYS_IN_STORAGE_COUNT);
        });
}

#[test]
fn should_unzip_xdns_topology_from_encoded_form_with_default_records() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .build()
        .execute_with(|| {

            // check that the topology is empty via all assets and all gateways ids
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), 0);

            let encoded_xdns_topology = hex!("20000000000200000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00010101010000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00030303030000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00050505050000000000187472616e0102746173730104737761700103616c697101036365766d010a7761736d010a00657468320301010000047472616e010200676174650200000000047472616e0102006b736d610100000000087472616e01027461737301040070646f740000000000087472616e01027461737301040000").to_vec();
            // Unzip the topology
            assert_ok!(XDNS::unzip_topology(Origin::root(), None, Some(encoded_xdns_topology)));

            // check that the topology is not empty via all assets and all gateways ids
            assert_eq!(pallet_xdns::Gateways::<Runtime>::iter().count(), DEFAULT_GATEWAYS_IN_STORAGE_COUNT);
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
fn should_allow_to_link_token_via_extrinsic_on_sudo_permission() {
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
            &Origin::root(),
            u32::from_le_bytes(*b"test"),
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            }),
            None,
        ));

        assert_ok!(XDNS::link_token(
            Origin::root(),
            *b"test",
            u32::from_le_bytes(*b"test"),
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            }),
            Some(DEFAULT_MULTI_LOCATION)
        ));

        // no duplicates
        assert_noop!(
            XDNS::link_token(
                Origin::root(),
                *b"test",
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    decimals: 18,
                    symbol: b"test".to_vec(),
                    id: 5
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ),
            pallet_xdns::pallet::Error::<Runtime>::TokenRecordAlreadyExists
        );

        // no mismatched execution vendor
        assert_noop!(
            XDNS::link_token(
                Origin::root(),
                *b"test",
                u32::from_le_bytes(*b"test"),
                TokenInfo::Ethereum(EthereumToken {
                    decimals: 18,
                    symbol: b"test".to_vec(),
                    address: Some([1; 20])
                }),
                None
            ),
            pallet_xdns::pallet::Error::<Runtime>::TokenRecordAlreadyExists
        );

        assert_eq!(pallet_xdns::Tokens::<Runtime>::iter().count(), 2);
    });
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
            &Origin::root(),
            u32::from_le_bytes(*b"test"),
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            }),
            None
        ));

        assert_ok!(XDNS::link_token_to_gateway(
            u32::from_le_bytes(*b"test"),
            *b"test",
            TokenInfo::Substrate(SubstrateToken {
                id: 1,
                symbol: b"test".to_vec(),
                decimals: 1,
            }),
            Some(DEFAULT_MULTI_LOCATION)
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
                }),
                Some(DEFAULT_MULTI_LOCATION)
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
                }),
                None
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
                }),
                Some(DEFAULT_MULTI_LOCATION)
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
fn should_enroll_and_unroll_new_abi_to_selected_gateway() {
    use t3rn_abi::SFXAbi;
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(pallet_xdns::StandardSFXABIs::<Runtime>::iter().count(), 9);

            let mut tran_sfx_abi = pallet_xdns::StandardSFXABIs::<Runtime>::get(b"tran").unwrap();
            tran_sfx_abi.maybe_prefix_memo = Some(2);
            let initial_target_abi: Vec<([u8; 4], SFXAbi)> =
                pallet_xdns::SFXABIRegistry::<Runtime>::iter_prefix(b"gate").collect();

            assert_eq!(initial_target_abi, vec![(*b"tran", tran_sfx_abi.clone())]);

            let mut tass_sfx_abi = pallet_xdns::StandardSFXABIs::<Runtime>::get(b"tass").unwrap();
            tass_sfx_abi.maybe_prefix_memo = Some(2);

            assert_ok!(XDNS::enroll_new_abi_to_selected_gateway(
                Origin::root(),
                *b"gate",
                *b"tass",
                None,
                Some(2)
            ));

            let updated_target_abi: Vec<([u8; 4], SFXAbi)> =
                pallet_xdns::SFXABIRegistry::<Runtime>::iter_prefix(b"gate").collect();

            assert_eq!(
                updated_target_abi,
                vec![(*b"tass", tass_sfx_abi), (*b"tran", tran_sfx_abi.clone())]
            );

            assert_ok!(XDNS::unroll_abi_of_selected_gateway(
                Origin::root(),
                *b"gate",
                *b"tass",
            ));

            assert_eq!(initial_target_abi, vec![(*b"tran", tran_sfx_abi.clone())]);
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
                &Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            assert!(Runtime::contains_asset(&u32::from_le_bytes(*b"test")));
        });
}

#[test]
fn mints_and_burns_registered_token_on_ownership_permissions() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_eq!(
                pallet_xdns::Gateways::<Runtime>::iter().count(),
                DEFAULT_GATEWAYS_IN_STORAGE_COUNT
            );

            let authorize_asset = 9999u32;
            // Enroll asset as mintable
            assert_ok!(XDNS::enroll_bridge_asset(
                Origin::root(),
                authorize_asset,
                [3, 3, 3, 3],
                TokenInfo::Substrate(SubstrateToken {
                    id: 9999,
                    symbol: b"9999".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            let beneficiary = AccountId::from([5; 32]);

            assert_ok!(XDNS::mint(authorize_asset, beneficiary.clone(), 999));
            // Check that the balance was minted
            assert_eq!(Assets::balance(authorize_asset, &beneficiary), 999);

            // Check that the balance can be burned
            assert_ok!(XDNS::burn(authorize_asset, beneficiary.clone(), 888));
            assert_eq!(Assets::balance(authorize_asset, &beneficiary), 111);
        });
}

#[test]
fn adds_remote_order_addresses_on_sudo_permission() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            assert_err!(
                XDNS::get_remote_order_contract_address([3, 3, 3, 3]),
                pallet_xdns::Error::<Runtime>::RemoteOrderAddressNotFound
            );

            assert_ok!(XDNS::add_remote_order_address(
                Origin::root(),
                [3, 3, 3, 3],
                H256::repeat_byte(1),
            ));

            assert_eq!(
                XDNS::get_remote_order_contract_address([3, 3, 3, 3]),
                Ok(H256::repeat_byte(1))
            );
        });
}

#[test]
fn should_purge_token_and_destroy_asset_as_root_successfully() {
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
                &Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            assert!(Runtime::contains_asset(&u32::from_le_bytes(*b"test")));

            let res = XDNS::purge_token_record(Origin::root(), u32::from_le_bytes(*b"test"));

            assert_ok!(res);

            assert!(!Runtime::contains_asset(&u32::from_le_bytes(*b"test")));
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
                &Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            assert!(Runtime::contains_asset(&u32::from_le_bytes(*b"test")));

            let admin_origin = AccountId32::from(hex_literal::hex!(
                "6d6f646c657363726f7772790000000000000000000000000000000000000000"
            ));
            let res = XDNS::purge_token_record(
                Origin::signed(admin_origin),
                u32::from_le_bytes(*b"test"),
            );

            println!("{:?}", res);
            assert_ok!(res);

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
                &Origin::root(),
                u32::from_le_bytes(*b"test"),
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                }),
                None
            ));

            assert_ok!(XDNS::link_token_to_gateway(
                u32::from_le_bytes(*b"test"),
                *b"gate",
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"test".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
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

            assert_ok!(XDNS::purge_gateway_record(Origin::root(), ALICE, *b"gate"));

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
                XDNS::purge_gateway_record(Origin::root(), ALICE, *b"miss"),
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
                XDNS::purge_gateway_record(Origin::signed(ALICE), ALICE, *b"gate"),
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
                Portal::turn_on(Origin::root(), gateway.gateway_record.gateway_id).unwrap();
            }

            assert_eq!(
                XDNS::process_all_verifier_overviews(10),
                Weight::from_parts(25000000u64, 0)
            );
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
                        gateway_id: [101, 116, 104, 50],
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
                Origin::root(),
                [1, 1, 1, 1],
                AccountId32::new([1; 32])
            ));

            Attesters::force_activate_target(Origin::root(), [1, 1, 1, 1]).unwrap();
            Attesters::request_sfx_attestation_commit([1, 1, 1, 1], H256::repeat_byte(1), None);
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
                        gateway_id: [101, 116, 104, 50],
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
                FinalityVerifierActivity {
                    verifier: Sepolia,
                    reported_at: 74,
                    justified_height: 0,
                    finalized_height: 0,
                    updated_height: 0,
                    epoch: 0,
                    is_active: false,
                },
                FinalityVerifierActivity {
                    verifier: XBI,
                    reported_at: 74,
                    justified_height: 74,
                    finalized_height: 74,
                    updated_height: 74,
                    epoch: 0,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: Attesters,
                    reported_at: 74,
                    justified_height: 0,
                    finalized_height: 0,
                    updated_height: 0,
                    epoch: 0,
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
                FinalityVerifierActivity {
                    verifier: Sepolia,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: XBI,
                    reported_at: 17,
                    justified_height: 24,
                    finalized_height: 24,
                    updated_height: 24,
                    epoch: 26,
                    is_active: true,
                },
                FinalityVerifierActivity {
                    verifier: Attesters,
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
                Portal::turn_off(Origin::root(), gateway.gateway_record.gateway_id).unwrap();
            }

            let last_reported_block = expected_verifier_overview_all_on[0].reported_at;

            System::set_block_number(last_reported_block + 1);
            <GlobalOnInitQueues as OnHookQueues<Runtime>>::process_hourly(
                System::block_number(),
                Weight::from_parts(u64::MAX, 0),
            );
            assert_eq!(XDNS::verifier_overview(), expected_verifier_overview_all_on);

            System::set_block_number(last_reported_block + 5);
            <GlobalOnInitQueues as OnHookQueues<Runtime>>::process_hourly(
                System::block_number(),
                Weight::from_parts(u64::MAX, 0),
            );
            assert_eq!(XDNS::verifier_overview(), expected_verifier_overview_all_on);

            System::set_block_number(System::block_number() + 52);
            <GlobalOnInitQueues as OnHookQueues<Runtime>>::process_hourly(
                System::block_number(),
                Weight::from_parts(u64::MAX, 0),
            );

            assert_eq!(
                XDNS::verifier_overview(),
                expected_verifier_overview_all_off
            );
        });
}

use t3rn_primitives::xdns::TokenRecord;
#[test]
fn adds_and_lists_supported_bridging_assets_when_authorized_by_root() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let authorize_asset: u32 = 1111;
            let mintable_gateway: [u8; 4] = [1, 1, 1, 1];

            assert!(!XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));

            assert_ok!(XDNS::register_new_token(
                &Origin::root(),
                authorize_asset,
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"mint".to_vec(),
                    decimals: 1,
                }),
                None
            ));

            assert_ok!(XDNS::link_token_to_gateway(
                authorize_asset,
                mintable_gateway,
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"mint".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            assert_ok!(XDNS::add_supported_bridging_asset(
                Origin::root(),
                authorize_asset,
                mintable_gateway,
            ));

            // Check that the asset is added
            assert_eq!(
                XDNS::list_available_mint_assets(mintable_gateway),
                vec![TokenRecord {
                    token_id: 1111,
                    gateway_id: [1, 1, 1, 1],
                    token_props: TokenInfo::Substrate(SubstrateToken {
                        id: 1,
                        symbol: b"mint".to_vec(),
                        decimals: 1
                    })
                    token_location: Some(DEFAULT_MULTI_LOCATION),
                }]
            );

            assert!(XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));
        });
}

#[test]
fn enrolls_supported_bridging_assets_when_authorized_by_root() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let authorize_asset: u32 = 1111;
            let mintable_gateway: [u8; 4] = [1, 1, 1, 1];

            assert!(!XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));

            assert_ok!(XDNS::enroll_bridge_asset(
                Origin::root(),
                authorize_asset,
                mintable_gateway,
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"mint".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            // Check that the asset is added
            assert_eq!(
                XDNS::list_available_mint_assets(mintable_gateway),
                vec![TokenRecord {
                    token_id: 1111,
                    gateway_id: [1, 1, 1, 1],
                    token_props: TokenInfo::Substrate(SubstrateToken {
                        id: 1,
                        symbol: b"mint".to_vec(),
                        decimals: 1
                    }),
                    token_location: Some(DEFAULT_MULTI_LOCATION),
                }]
            );

            assert!(XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));
        });
}

#[test]
fn purges_previously_added_supported_bridging_assets_when_authorized_by_root() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .with_default_attestation_targets()
        .build()
        .execute_with(|| {
            // Define emergency_offset and SpeedMode
            let authorize_asset: u32 = 1111;
            let mintable_gateway: [u8; 4] = [1, 1, 1, 1];

            assert!(!XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));

            assert_ok!(XDNS::register_new_token(
                &Origin::root(),
                authorize_asset,
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"mint".to_vec(),
                    decimals: 1,
                }),
                None
            ));

            assert_ok!(XDNS::link_token_to_gateway(
                authorize_asset,
                mintable_gateway,
                TokenInfo::Substrate(SubstrateToken {
                    id: 1,
                    symbol: b"mint".to_vec(),
                    decimals: 1,
                }),
                Some(DEFAULT_MULTI_LOCATION)
            ));

            assert_ok!(XDNS::add_supported_bridging_asset(
                Origin::root(),
                authorize_asset,
                mintable_gateway,
            ));

            assert!(XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));

            assert_ok!(XDNS::purge_supported_bridging_asset(
                Origin::root(),
                authorize_asset,
                mintable_gateway,
            ));

            assert!(!XDNS::check_asset_is_mintable(
                mintable_gateway,
                authorize_asset
            ));

            assert_eq!(XDNS::list_available_mint_assets(mintable_gateway), vec![]);
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
                Portal::turn_off(Origin::root(), gateway.gateway_record.gateway_id).unwrap();
            }
            assert_eq!(
                XDNS::process_all_verifier_overviews(100),
                Weight::from_parts(2125000000u64, 0)
            );
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
                        gateway_id: [101, 116, 104, 50],
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

#[test]
fn test_storage_migration_v143_to_v144_that_kills_old_xdns_records_entry() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // Insert raw xdns records entry
            frame_support::storage::unhashed::put_raw(
                &[
                    225, 205, 72, 162, 242, 43, 101, 142, 192, 157, 178, 168, 200, 143, 21, 13,
                    175, 239, 182, 147, 135, 79, 226, 105, 210, 52, 22, 179, 228, 93, 185, 249,
                    114, 111, 99, 111,
                ],
                &[1, 2, 3],
            );

            pallet_xdns::StorageMigrations::<Runtime>::set(1);

            // Perform the runtime upgrade (call the `on_runtime_upgrade` function)
            let consumed_weight =
                <XDNS as frame_support::traits::OnRuntimeUpgrade>::on_runtime_upgrade();
            let max_weight = <Runtime as frame_system::Config>::DbWeight::get().reads_writes(0, 1);
            assert_eq!(consumed_weight, max_weight);

            assert_eq!(
                frame_support::storage::unhashed::get::<Vec<u8>>(&[
                    225, 205, 72, 162, 242, 43, 101, 142, 192, 157, 178, 168, 200, 143, 21, 13,
                    175, 239, 182, 147, 135, 79, 226, 105, 210, 52, 22, 179, 228, 93, 185, 249,
                    114, 111, 99, 111,
                ],),
                None
            );
        });
}

#[test]
fn test_storage_migration_v144_to_v145_that_kills_old_xdns_records_entry() {
    ExtBuilder::default()
        .with_standard_sfx_abi()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            // Insert raw xdns records entry
            frame_support::storage::unhashed::put_raw(
                &[
                    84, 10, 79, 135, 84, 170, 82, 152, 163, 214, 233, 170, 9, 233, 63, 151, 78, 11,
                    18, 119, 80, 58, 19, 112, 111, 133, 165, 20, 116, 96, 124, 88, 24, 172, 250,
                    191, 195, 140, 91, 41, 106, 32, 177, 28, 37, 248, 177, 35, 27, 230, 169, 204,
                    8, 192, 121, 163, 226, 24, 100, 166, 207, 36, 66, 173, 219, 150, 184, 250, 101,
                    171, 135, 85,
                ],
                &[3, 2, 1],
            );

            pallet_xdns::StorageMigrations::<Runtime>::set(2);

            // Perform the runtime upgrade (call the `on_runtime_upgrade` function)
            let consumed_weight =
                <XDNS as frame_support::traits::OnRuntimeUpgrade>::on_runtime_upgrade();
            let max_weight = <Runtime as frame_system::Config>::DbWeight::get().reads_writes(0, 1);
            assert_eq!(consumed_weight, max_weight);

            assert_eq!(
                frame_support::storage::unhashed::get::<Vec<u8>>(&[
                    84, 10, 79, 135, 84, 170, 82, 152, 163, 214, 233, 170, 9, 233, 63, 151, 78, 11,
                    18, 119, 80, 58, 19, 112, 111, 133, 165, 20, 116, 96, 124, 88, 24, 172, 250,
                    191, 195, 140, 91, 41, 106, 32, 177, 28, 37, 248, 177, 35, 27, 230, 169, 204,
                    8, 192, 121, 163, 226, 24, 100, 166, 207, 36, 66, 173, 219, 150, 184, 250, 101,
                    171, 135, 85,
                ],),
                None
            );
        });
}
