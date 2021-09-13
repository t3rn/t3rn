//! Benchmarking setup for pallet-circuit-execution-delivery

use super::*;
use bp_test_utils::test_header;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_system::RawOrigin;

use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::Compose;

use sp_runtime::{create_runtime_str, AccountId32};
use sp_version::RuntimeVersion;

use sp_keystore::testing::KeyStore;
use sp_keystore::{KeystoreExt, SyncCryptoStore};

use crate::{
    CurrentHeader, DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway,
    EthLikeKeccak256ValU64Gateway, Pallet as ExecDelivery, PolkadotLikeValU64Gateway,
};

pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
};

const USER_SEED: u32 = 999666;

const CODE: &str = r#"(module
    (func (export "call"))
    (func (export "deploy"))
)"#;

benchmarks! {
    decompose_io_schedule{
        let io_schedule = b"component1;".to_vec();
        let components: Vec<Compose<T::AccountId, BalanceOf<T>>> = vec![Compose {
            name: b"component1".to_vec(),
            code_txt: CODE.as_bytes().to_vec(),
            exec_type: b"exec_escrow".to_vec(),
            dest: account("TEST", 1_u32, USER_SEED),
            value: 0u32.into(),
            bytes: vec![],
            input_data: vec![],
        }];

        let expected = InterExecSchedule {
            phases: vec![ExecPhase {
                steps: vec![ExecStep {
                    compose: Compose {
                        name: b"component1".to_vec(),
                        code_txt: CODE.as_bytes().to_vec(),
                        exec_type: b"exec_escrow".to_vec(),
                        dest: account("TEST", 1_u32, USER_SEED),
                        value: 0u32.into(),
                        bytes: vec![],
                        input_data: vec![],
                    },
                }],
            }],
        };
    }: {
        Pallet::<T>::decompose_io_schedule(components.clone(), io_schedule.clone()).unwrap();
    } verify {
        assert_eq!(
            Pallet::<T>::decompose_io_schedule(components, io_schedule).unwrap(),
            expected
        );
    }

    register_gateway_default_polka {
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
            signed_extension: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
        };

        let first_header: CurrentHeader<T, DefaultPolkadotLikeGateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities).unwrap()}
    verify{}

    register_gateway_polka_u64 {
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
            signed_extension: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
        };

        let first_header: CurrentHeader<T, PolkadotLikeValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities).unwrap()}
    verify{}

    register_gateway_default_eth {
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
            signed_extension: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU32Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities).unwrap()}
    verify{}

    register_gateway_eth_u64 {
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
            signed_extension: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities).unwrap()}
    verify{}

    // Need help!

    // submit_composable_exec_order {
    //     let components: Vec<Compose<T::AccountId, BalanceOf<T>>> = vec![Compose {
    //         name: b"component1".to_vec(),
    //         code_txt: CODE.as_bytes().to_vec(),
    //         exec_type: b"exec_escrow".to_vec(),
    //         dest: account("TEST", 1_u32, USER_SEED),
    //         value: 0u32.into(),
    //         bytes: vec![0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 7, 17, 2, 4, 99, 97, 108, 108, 0, 0, 6, 100, 101, 112, 108, 111, 121, 0, 1, 10, 7, 2, 2, 0, 11, 2, 0, 11],
    //         input_data: vec![],
    //     }];
    //     let io_schedule = b"component1;".to_vec();
    //     let keystore = KeyStore::new();

    //     Insert Alice's keys
    //     const SURI_ALICE: &str = "//Alice";
    //     let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    //     SyncCryptoStore::insert_unknown(
    //         &keystore,
    //         KEY_TYPE,
    //         SURI_ALICE,
    //         key_pair_alice.public().as_ref(),
    //     )
    //     .expect("Inserts unknown key");
    //    insert_default_xdns_record();

    // }: _(RawOrigin::Signed(Default::default()), io_schedule, components)
    // verify{}

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::Test;
    use crate::tests::new_test_ext;
    use frame_support::assert_ok;

    #[test]
    fn benchmark_decompose_io_schedule() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_decompose_io_schedule::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_default_polka() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_default_polka::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_polka_u64() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_polka_u64::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_default_eth() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_default_eth::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_eth_u64() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_eth_u64::<Test>());
        })
    }

    // #[test]
    // fn benchmark_submit_composable_exec_order() {
    //     new_test_ext().execute_with(|| {
    //         assert_ok!(test_benchmark_submit_composable_exec_order::<Test>());
    //     })
    // }
}

impl_benchmark_test_suite!(
    ExecDelivery,
    crate::tests::new_test_ext(),
    crate::mock::Test
);
