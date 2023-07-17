//! Benchmarking setup for pallet-contracts-registry

use super::*;
use crate::Pallet as ContractsRegistry;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use t3rn_primitives::contract_metadata::ContractMetadata;
const USER_SEED: u32 = 999666;
use t3rn_primitives::contracts_registry::{AuthorInfo, ContractsRegistry as ContractsRegistryExt};

const CODE_CALL: &str = r#"
(module
	;; seal_call(
	;;    callee_ptr: u32,
	;;    callee_len: u32,
	;;    gas: u64,
	;;    value_ptr: u32,
	;;    value_len: u32,
	;;    input_data_ptr: u32,
	;;    input_data_len: u32,
	;;    output_ptr: u32,
	;;    output_len_ptr: u32
	;;) -> u32
	(import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
	(import "env" "memory" (memory 1 1))
	(func (export "call")
		(drop
			(call $seal_call
				(i32.const 4)  ;; Pointer to "callee" address.
				(i32.const 32)  ;; Length of "callee" address.
				(i64.const 0)  ;; How much gas to devote for the execution. 0 = all.
				(i32.const 36) ;; Pointer to the buffer with value to transfer
				(i32.const 8)  ;; Length of the buffer with value to transfer.
				(i32.const 44) ;; Pointer to input data buffer address
				(i32.const 4)  ;; Length of input data buffer
				(i32.const 4294967295) ;; u32 max value is the sentinel value: do not copy output
				(i32.const 0) ;; Length is ignored in this case
			)
		)
	)
	(func (export "deploy"))

	;; Destination AccountId (ALICE)
	(data (i32.const 4)
		"\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
		"\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
	)

	;; Amount of value to transfer.
	;; Represented by u64 (8 bytes long) in little endian.
	(data (i32.const 36) "\06\00\00\00\00\00\00\00")

	(data (i32.const 44) "\01\02\03\04")
)
"#;

benchmarks! {
    add_new_contract {
        let test_contract: RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber> =
            RegistryContract {
                code_txt: CODE_CALL.as_bytes().to_vec(),
                // hadrcoded bytes
                bytes: vec![
                    0, 97, 115, 109, 1, 0, 0, 0, 1, 17, 2, 96, 9, 127, 127, 126, 127, 127, 127, 127,
                    127, 127, 1, 127, 96, 0, 0, 2, 34, 2, 5, 115, 101, 97, 108, 48, 9, 115, 101, 97,
                    108, 95, 99, 97, 108, 108, 0, 0, 3, 101, 110, 118, 6, 109, 101, 109, 111, 114, 121,
                    2, 1, 1, 1, 3, 3, 2, 1, 1, 7, 17, 2, 4, 99, 97, 108, 108, 0, 1, 6, 100, 101, 112,
                    108, 111, 121, 0, 2, 10, 28, 2, 23, 0, 65, 4, 65, 32, 66, 0, 65, 36, 65, 8, 65, 44,
                    65, 4, 65, 127, 65, 0, 16, 0, 26, 11, 2, 0, 11, 11, 60, 3, 0, 65, 4, 11, 32, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 0, 65, 36, 11, 8, 6, 0, 0, 0, 0, 0, 0, 0, 0, 65, 44, 11, 4, 1, 2, 3, 4,
                ],
                author: AuthorInfo::new(account("TEST", 1_u32, USER_SEED), None),
                abi: None,
                action_descriptions: vec![],
                info: None,
                meta: ContractMetadata::new(
                    vec![],
                    b"contract 1".to_vec(),
                    vec![],
                    vec![],
                    vec![],
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            };
        let contract_id = test_contract.generate_id::<T, Keccak256>();
        let requester: T::AccountId = account("TEST", 1_u32, USER_SEED);
    }: _(RawOrigin::Root, requester.clone(), test_contract.clone())
    verify {
        assert!(pallet::ContractsRegistry::<T>::contains_key(&contract_id));
    }

    purge {
        let requester_1: T::AccountId = account("TEST", 1_u32, USER_SEED);

        let test_contract_1: RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber> =
            RegistryContract {
                code_txt: b"some_code".to_vec(),
                bytes: vec![],
                author: AuthorInfo::new(account("TEST", 1_u32, USER_SEED), None),
                abi: None,
                action_descriptions: vec![],
                info: None,
                meta: ContractMetadata::new(
                    vec![],
                    b"contract 1".to_vec(),
                    vec![],
                    vec![],
                    vec![],
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            };

        let requester_2: T::AccountId = account("TEST", 2_u32, USER_SEED);

        let test_contract_2: RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber> =
            RegistryContract {
                code_txt: b"some_code_2".to_vec(),
                bytes: vec![],
                author: AuthorInfo::new(account("TEST", 2_u32, USER_SEED), None),
                abi: None,
                action_descriptions: vec![],
                info: None,
                meta: ContractMetadata::new(
                    vec![],
                    b"contract 2".to_vec(),
                    vec![],
                    vec![],
                    vec![],
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            };

        let contract_id_1 = test_contract_1.generate_id::<T, Keccak256>();
        crate::ContractsRegistry::<T>::insert(
            test_contract_1.generate_id::<T, Keccak256>(),
            test_contract_1.clone(),
        );

        let contract_id_2 = test_contract_2.generate_id::<T, Keccak256>();
        crate::ContractsRegistry::<T>::insert(
            test_contract_2.generate_id::<T, Keccak256>(),
            test_contract_2.clone(),
        );
    }: _(RawOrigin::Root, requester_1.clone(), contract_id_1.clone())
    verify {
        assert_ok!(ContractsRegistry::<T>::purge(
            RawOrigin::Root.into(),
            requester_2,
            contract_id_2
        ));
    }

    fetch_contracts {
        let test_contract_author_1: RegistryContract<
            T::Hash,
            T::AccountId,
            BalanceOf<T>,
            T::BlockNumber,
        > = RegistryContract {
            code_txt: b"some_code_1".to_vec(),
            bytes: vec![],
            author: AuthorInfo::new(account("TEST", 1_u32, USER_SEED), None),
            abi: None,
            action_descriptions: vec![],
            info: None,
            meta: ContractMetadata::new(
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                Some(b"contract 1".to_vec()),
                None,
                None,
                None,
                None,
            ),
        };
        let test_contract_author_2: RegistryContract<
            T::Hash,
            T::AccountId,
            BalanceOf<T>,
            T::BlockNumber,
        > = RegistryContract {
            code_txt: b"some_code_2".to_vec(),
            bytes: vec![],
            author: AuthorInfo::new(account("TEST", 1_u32, USER_SEED), None),
            abi: None,
            action_descriptions: vec![],
            info: None,
            meta: ContractMetadata::new(
                vec![],
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
        let test_contract_author_3: RegistryContract<
            T::Hash,
            T::AccountId,
            BalanceOf<T>,
            T::BlockNumber,
        > = RegistryContract {
            code_txt: b"some_code_3".to_vec(),
            bytes: vec![],
            author: AuthorInfo::new(account("TEST", 2_u32, USER_SEED), None),
            abi: None,
            action_descriptions: vec![],
            info: None,
            meta: ContractMetadata::new(
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                None,
                None,
                None,
                None,
                None
            ),
        };

        crate::ContractsRegistry::<T>::insert(
            test_contract_author_1.generate_id::<T, Keccak256>(),
            test_contract_author_1.clone(),
        );
        crate::ContractsRegistry::<T>::insert(
            test_contract_author_2.generate_id::<T, Keccak256>(),
            test_contract_author_2.clone(),
        );
        crate::ContractsRegistry::<T>::insert(
            test_contract_author_3.generate_id::<T, Keccak256>(),
            test_contract_author_3.clone(),
        );

        let author_1: T::AccountId = account("TEST", 1_u32, USER_SEED);

    }: {ContractsRegistry::<T>::fetch_contracts(Some(author_1.clone()), Some(b"contract".to_vec()))}
    verify {
        assert_eq!(
            ContractsRegistry::<T>::fetch_contracts(Some(author_1), Some(b"contract".to_vec())),
            Ok(vec![
                test_contract_author_1.clone(),
                test_contract_author_2.clone()
            ])
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::Test;
    use frame_support::assert_ok;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        TestExternalities::new(t)
    }

    #[test]
    fn benchmark_register_contract() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_add_new_contract::<Test>());
        })
    }

    #[test]
    fn benchmark_purge() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_purge::<Test>());
        })
    }

    #[test]
    fn benchmark_fetch_contracts() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_fetch_contracts::<Test>());
        })
    }
}

impl_benchmark_test_suite!(
    ContractsRegistry,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::Test
);
