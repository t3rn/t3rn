//! Benchmarking setup for pallet-circuit-execution-delivery

use super::*;

use crate::{Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
type AccountId = sp_runtime::AccountId32;
use frame_support::assert_ok;
//use std::str::FromStr;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::{ExecPhase, ExecStep, InterExecSchedule, Compose};
use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;
// use sp_keystore::testing::KeyStore;
// use sp_keystore::{KeystoreExt, SyncCryptoStore};
// use sp_core::{crypto::Pair, sr25519};
// pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");


pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
};

benchmarks! {
    sort_vector {
        let x in 0 .. 10000;
        let mut m = Vec::<u32>::new();
        for i in (0..x).rev() {
            m.push(i);
        }
    }: {
        m.sort();
    }

    decompose_io_schedule{
        let expected = InterExecSchedule {
            phases: vec![ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component1".to_vec(),
                        code_txt: r#""#.as_bytes().to_vec(),
                        exec_type: b"exec_escrow".to_vec(),
                        dest: AccountId::new([1 as u8; 32]),//T::AccountId::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap(),//AccountId::new([1 as u8; 32]),
                        value: BalanceOf::from(10u32),
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
            dest: AccountId::new([1 as u8; 32]),//T::AccountId::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap(),//AccountId::new([1 as u8; 32]),
            value: BalanceOf::from(10u32),
            bytes: vec![],
            input_data: vec![],
        }];
    }: {
        Pallet::<T>::decompose_io_schedule(components, io_schedule).unwrap();
    } verify {
        assert_eq!(
            Pallet::<T>::decompose_io_schedule(components, io_schedule).unwrap(),
            expected
        )
    }

    register_gateway {
        let caller: T::AccountId = account("caller", 0, 0);
        let url = b"ws://localhost:9944".to_vec();
        let gateway_id = [0; 4];
        let gateway_abi: GatewayABIConfig = Default::default();
        
        //     fn default() -> GatewayABIConfig {
        //         GatewayABIConfig {
        //             block_number_type_size: 32,
        //             hash_size: 32,
        //             hasher: HasherAlgo::Blake2,
        //             crypto: CryptoAlgo::Sr25519,
        //             address_length: 32,
        //             value_type_size: 64,
        //             decimals: 8,
        //             structs: vec![],
        //         }
        //     }
        
        let gateway_vendor = GatewayVendor::Substrate;
        let gateway_type = GatewayType::ProgrammableInternal;

        let _gateway_pointer = GatewayPointer {
            id: [0; 4],
            vendor: GatewayVendor::Substrate,
            gateway_type: GatewayType::ProgrammableInternal,
        };

        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded: None,
            signed_extension: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
        };

        let first_header = GenericPrimitivesHeader {
            parent_hash: None,
            number: 1,
            state_root: None,
            extrinsics_root: None,
            digest: None,
            // parent_hash: Default::default(),
            // number: 0,
            // state_root: Default::default(), //Some(H256::from_slice(&hex!("b2fc47904df5e355c6ab476d89fbc0733aeddbe302f0b94ba4eea9283f7e89e7"))),
            // extrinsics_root: Default::default(), //Some(H256::from_slice(&hex!("03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314"))),
            // digest: Default::default(),
        };

        let authorities = Some(vec![]);

    }: _(RawOrigin::Signed(caller), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header, authorities)
    verify {
        assert_eq!(1,1);
    }

    submit_composable_exec_order {
        let caller: T::AccountId = account("caller", 0, 0);
        let io_schedule = b"component1;".to_vec();

        const CONTRACT: &str = r#"
                (module
                    (func (export "call"))
                    (func (export "deploy"))
                )
                "#;

        let components = vec![Compose {
            name: b"component1".to_vec(),
            code_txt: CONTRACT.encode(),
            exec_type: b"exec_escrow".to_vec(),
            dest: AccountId::new([1 as u8; 32]),
            value: BalanceOf::from(0u32),
            bytes: vec![
                0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 7, 17, 2, 4, 99, 97,
                108, 108, 0, 0, 6, 100, 101, 112, 108, 111, 121, 0, 1, 10, 7, 2, 2, 0, 11, 2, 0, 11,
            ],
            input_data: vec![],
        }];

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

    }: _(RawOrigin::Signed(caller.clone()), io_schedule.clone(), components.clone())
    verify {
        assert_ok!(Pallet::<T>::submit_composable_exec_order(
            RawOrigin::Signed(caller),
            io_schedule,
            components
        ));
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn benchmark_sort_vector() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_sort_vector::<Test>());
        })
    }

    #[test]
    fn benchmark_decompose_io_schedule() {
        new_test_ext().execute_with(|| {
            assert_ok(test_benchmark_decompose_io_schedule::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway::<Test>());
        })
    }

    #[test]
    fn benchmark_submit_composable_exec_order() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_submit_composable_exec_order::<Test>());
        })
    }
}

impl_benchmark_test_suite!(
    ExecDelivery,
    crate::tests::new_test_ext(),
    crate::tests::Test
);
