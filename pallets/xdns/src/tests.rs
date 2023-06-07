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
use circuit_mock_runtime::{ExtBuilder, *};
use codec::Decode;
use frame_support::{assert_err, assert_noop, assert_ok};
use frame_system::Origin;
use sp_runtime::DispatchError;
use t3rn_primitives::{
    xdns::{PalletAssetsOverlay, Xdns},
    EthereumToken, ExecutionVendor, GatewayVendor, SubstrateToken, TokenInfo,
};
const DEFAULT_GATEWAYS_IN_STORAGE_COUNT: usize = 8;
const STANDARD_SFX_ABI_COUNT: usize = 7;

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
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let actual = XDNS::get_verification_vendor(b"pdot");
            assert_ok!(actual, GatewayVendor::Polkadot);
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
