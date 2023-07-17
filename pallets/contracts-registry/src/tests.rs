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

//! Unit tests for pallet contracts-registry.

use circuit_mock_runtime::{pallet_contracts_registry::pallet::Error, *};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use sp_runtime::DispatchError;
use t3rn_primitives::{
    contract_metadata::{ContractMetadata, ContractType},
    contracts_registry::{
        AuthorInfo, ContractsRegistry as ContractsRegistryExt, KindValidator, RegistryContract,
    },
};

#[test]
fn fetch_contract_by_id_should_return_single_contract() {
    let test_contract = RegistryContract {
        code_txt: vec![],
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: Default::default(),
    };
    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
                test_contract.generate_id::<Runtime>(),
                test_contract.clone(),
            );
            let actual =
                ContractsRegistry::fetch_contract_by_id(test_contract.generate_id::<Runtime>());
            assert_ok!(actual, test_contract);
        })
}

#[test]
fn fetch_contract_by_id_should_error_if_contract_doesnt_exist() {
    ExtBuilder::default().build().execute_with(|| {
        let actual = ContractsRegistry::fetch_contract_by_id(H256([1; 32]));
        assert_err!(actual, Error::UnknownContract);
    })
}

#[test]
fn fetch_contracts_by_metadata_should_return_all_matching_contracts() {
    let test_contract_name = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"some contract".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_desc = RegistryContract {
        code_txt: b"some_code_2".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            Some(b"contract description".to_vec()),
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_wrong = RegistryContract {
        code_txt: b"some_code_3".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            Some(b"other description".to_vec()),
            None,
            None,
            None,
            None,
        ),
    };
    ExtBuilder::default().build().execute_with(|| {
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_name.generate_id::<Runtime>(),
            test_contract_name.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_desc.generate_id::<Runtime>(),
            test_contract_desc.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_wrong.generate_id::<Runtime>(),
            test_contract_wrong.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(None, Some(b"contract".to_vec()));
        assert_ok!(
            actual,
            vec![test_contract_name.clone(), test_contract_desc.clone()]
        );
    })
}

#[test]
fn fetch_contracts_by_author_should_return_all_matching_contracts() {
    let test_contract_author1 = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author2 = RegistryContract {
        code_txt: b"some_code_2".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),

        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author3 = RegistryContract {
        code_txt: b"some_code_3".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(BOB, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    ExtBuilder::default().build().execute_with(|| {
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author1.generate_id::<Runtime>(),
            test_contract_author1.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author2.generate_id::<Runtime>(),
            test_contract_author2.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author3.generate_id::<Runtime>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(Some(ALICE), None);
        assert_ok!(
            actual,
            vec![test_contract_author1.clone(), test_contract_author2.clone()]
        );
    })
}

#[test]
fn fetch_contracts_by_author_and_metadata_should_return_all_matching_contracts() {
    let test_contract_author1 = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author2 = RegistryContract {
        code_txt: b"some_code_2".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            Some(b"contract 2".to_vec()),
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author3 = RegistryContract {
        code_txt: b"some_code_3".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(BOB, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    ExtBuilder::default().build().execute_with(|| {
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author1.generate_id::<Runtime>(),
            test_contract_author1.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author2.generate_id::<Runtime>(),
            test_contract_author2.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author3.generate_id::<Runtime>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(Some(ALICE), Some(b"contract".to_vec()));
        assert_ok!(
            actual,
            vec![test_contract_author1.clone(), test_contract_author2.clone()]
        );
    })
}

#[test]
fn fetch_contracts_with_no_parameters_should_error() {
    let test_contract_author1 = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author2 = RegistryContract {
        code_txt: b"some_code_2".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            Some(b"contract 2".to_vec()),
            None,
            None,
            None,
            None,
        ),
    };
    let test_contract_author3 = RegistryContract {
        code_txt: b"some_code_3".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(BOB, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };
    ExtBuilder::default().build().execute_with(|| {
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author1.generate_id::<Runtime>(),
            test_contract_author1.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author2.generate_id::<Runtime>(),
            test_contract_author2.clone(),
        );
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract_author3.generate_id::<Runtime>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(None, None);
        assert_err!(actual, Error::UnknownContract);
    })
}

#[test]
fn add_new_contract_succeeds_for_default() {
    let origin = Origin::root();
    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let requester = ALICE;

    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            assert_ok!(ContractsRegistry::add_new_contract(
                origin,
                requester,
                test_contract.clone()
            ));
        })
}

#[test]
fn add_new_contract_fails_for_no_sudo_origin() {
    let origin = Origin::signed(ALICE);
    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let requester = ALICE;

    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            assert_err!(
                ContractsRegistry::add_new_contract(origin, requester, test_contract.clone()),
                DispatchError::BadOrigin
            );
        })
}

#[test]
fn add_new_contract_fails_if_contract_already_exists() {
    let origin = Origin::root();
    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let requester = ALICE;

    ExtBuilder::default().build().execute_with(|| {
        pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
            test_contract.generate_id::<Runtime>(),
            test_contract.clone(),
        );
        assert_err!(
            ContractsRegistry::add_new_contract(origin, requester, test_contract.clone()),
            Error::<Runtime>::ContractAlreadyExists
        )
    })
}

#[test]
fn purge_succeeds_for_default_contract() {
    let origin = Origin::root();
    let requester = ALICE;

    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let contract_id = test_contract.generate_id::<Runtime>();

    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
                test_contract.generate_id::<Runtime>(),
                test_contract.clone(),
            );
            assert_ok!(ContractsRegistry::purge(origin, requester, contract_id));
            assert_eq!(
                pallet_contracts_registry::ContractsRegistry::<Runtime>::get(contract_id),
                None
            );
        });
}

#[test]
fn purge_fails_if_contract_does_not_exist() {
    let origin = Origin::root();
    let requester = ALICE;

    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let contract_id = test_contract.generate_id::<Runtime>();

    ExtBuilder::default()
        .with_contracts(vec![test_contract])
        .build()
        .execute_with(|| {
            assert_err!(
                ContractsRegistry::purge(origin, requester, contract_id),
                Error::<Runtime>::UnknownContract
            );
        })
}

#[test]
fn purge_fails_if_origin_not_root() {
    let origin = Origin::signed(ALICE);
    let requester = ALICE;

    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::VanillaWasm,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let contract_id = test_contract.generate_id::<Runtime>();

    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            pallet_contracts_registry::ContractsRegistry::<Runtime>::insert(
                test_contract.generate_id::<Runtime>(),
                test_contract.clone(),
            );
            assert_err!(
                ContractsRegistry::purge(origin, requester, contract_id),
                DispatchError::BadOrigin
            );
        })
}

#[test]
fn test_kind_validator() {
    let test_contract = RegistryContract {
        code_txt: b"some_code".to_vec(),
        bytes: vec![],
        author: AuthorInfo::new(ALICE, None),
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
            ContractType::System,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
            None,
        ),
    };

    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            assert_eq!(
                test_contract.meta.get_contract_type().can_instantiate(),
                false
            );
            assert_eq!(
                test_contract
                    .meta
                    .get_contract_type()
                    .can_generate_side_effects(),
                true
            );
            assert_eq!(
                test_contract.meta.get_contract_type().can_remunerate(),
                true
            );
            assert_eq!(test_contract.meta.get_contract_type().has_storage(), false);
        });
}
