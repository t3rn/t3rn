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

use crate::mock::{ContractsRegistry, ExtBuilder, Test};
use crate::{Error, RegistryContract};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use t3rn_primitives::contract_metadata::ContractMetadata;

#[test]
fn fetch_contract_by_id_should_return_single_contract() {
    let test_contract = RegistryContract {
        code_txt: vec![],
        bytes: vec![],
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: Default::default(),
    };
    ExtBuilder::default()
        .with_contracts(vec![test_contract.clone()])
        .build()
        .execute_with(|| {
            crate::ContractsRegistry::<Test>::insert(
                test_contract.generate_id::<Test>(),
                test_contract.clone(),
            );
            let actual =
                ContractsRegistry::fetch_contract_by_id(test_contract.generate_id::<Test>());
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"some contract".to_vec(),
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
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
        crate::ContractsRegistry::<Test>::insert(
            test_contract_name.generate_id::<Test>(),
            test_contract_name.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_desc.generate_id::<Test>(),
            test_contract_desc.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_wrong.generate_id::<Test>(),
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(vec![], vec![], vec![], vec![], None, None, None, None, None),
    };
    let test_contract_author2 = RegistryContract {
        code_txt: b"some_code_2".to_vec(),
        bytes: vec![],
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(vec![], vec![], vec![], vec![], None, None, None, None, None),
    };
    let test_contract_author3 = RegistryContract {
        code_txt: b"some_code_3".to_vec(),
        bytes: vec![],
        author: 2_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(vec![], vec![], vec![], vec![], None, None, None, None, None),
    };
    ExtBuilder::default().build().execute_with(|| {
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author1.generate_id::<Test>(),
            test_contract_author1.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author2.generate_id::<Test>(),
            test_contract_author2.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author3.generate_id::<Test>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(Some(1), None);
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
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
        author: 2_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(vec![], vec![], vec![], vec![], None, None, None, None, None),
    };
    ExtBuilder::default().build().execute_with(|| {
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author1.generate_id::<Test>(),
            test_contract_author1.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author2.generate_id::<Test>(),
            test_contract_author2.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author3.generate_id::<Test>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(Some(1), Some(b"contract".to_vec()));
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            b"contract 1".to_vec(),
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
        author: 1_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(
            vec![],
            vec![],
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
        author: 2_u64,
        author_fees_per_single_use: None,
        abi: None,
        action_descriptions: vec![],
        info: None,
        meta: ContractMetadata::new(vec![], vec![], vec![], vec![], None, None, None, None, None),
    };
    ExtBuilder::default().build().execute_with(|| {
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author1.generate_id::<Test>(),
            test_contract_author1.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author2.generate_id::<Test>(),
            test_contract_author2.clone(),
        );
        crate::ContractsRegistry::<Test>::insert(
            test_contract_author3.generate_id::<Test>(),
            test_contract_author3.clone(),
        );
        let actual = ContractsRegistry::fetch_contracts(None, None);
        assert_err!(actual, Error::UnknownContract);
    })
}
