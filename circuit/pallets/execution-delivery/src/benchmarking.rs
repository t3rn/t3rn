//! Benchmarking setup for pallet-circuit-execution-delivery

use super::*;
use bp_test_utils::test_header;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};

use frame_system::{Origin, RawOrigin};

use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::Compose;

use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

use crate::{
    CurrentHeader, DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway,
    EthLikeKeccak256ValU64Gateway, PolkadotLikeValU64Gateway,
};

pub use crate::Pallet as ExecDelivery;

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

const CODE1: &str = r#"(module
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

const CODE2: &str = r#"(module
	(import "seal0" "seal_get_storage" (func $seal_get_storage (param i32 i32 i32) (result i32)))
	(import "seal0" "seal_set_storage" (func $seal_set_storage (param i32 i32 i32)))
	(import "seal0" "seal_input" (func $seal_input (param i32 i32)))
	(import "env" "memory" (memory 16 16))

	;; [0, 32) storage key
	(data (i32.const 0) "\01")

	;; [32, 36) buffer where input is copied (expected size of storage item)

	;; [36, 40) size of the input buffer
	(data (i32.const 36) "\04")

	;; [40, 44) size of buffer for seal_get_storage set to max
	(data (i32.const 40) "\FF\FF\FF\FF")

	;; [44, inf) seal_get_storage buffer

	(func $assert (param i32)
		(block $ok
			(br_if $ok
				(get_local 0)
			)
			(unreachable)
		)
	)

	(func (export "call")
		(call $seal_input (i32.const 32) (i32.const 36))

		;; assert input size == 4
		(call $assert
			(i32.eq
				(i32.load (i32.const 36))
				(i32.const 4)
			)
		)

		;; place a garbage value in storage, the size of which is specified by the call input.
		(call $seal_set_storage
			(i32.const 0)		;; Pointer to storage key
			(i32.const 0)		;; Pointer to value
			(i32.load (i32.const 32))	;; Size of value
		)

		(call $assert
			(i32.eq
				(call $seal_get_storage
					(i32.const 0)		;; Pointer to storage key
					(i32.const 44)		;; buffer where to copy result
					(i32.const 40)		;; pointer to size of buffer
				)
				(i32.const 0)
			)
		)

		(call $assert
			(i32.eq
				(i32.load (i32.const 40))
				(i32.load (i32.const 32))
			)
		)
	)

	(func (export "deploy"))

)
"#;

fn insert_default_xdns_record<T: pallet_xdns::Config>() {
    let url = b"some_url".to_vec();

    let gateway_abi: GatewayABIConfig = Default::default();
    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::TxOnly(0);

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        signed_extensions: None,
        runtime_version: TEST_RUNTIME_VERSION,
        extrinsics_version: 0u8,
        genesis_hash: Default::default(),
    };
    pallet_xdns::Pallet::<T>::add_new_xdns_record(
        Origin::<T>::Root.into(),
        url,
        [0, 0, 0, 0],
        gateway_abi,
        gateway_vendor,
        gateway_type,
        gateway_genesis,
        vec![],
    )
    .unwrap();
}

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
            signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, DefaultPolkadotLikeGateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
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
            signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, PolkadotLikeValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
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
            signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU32Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
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
            signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
    verify{}

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

    //     // Error here: failed to select authority / no keystore associated for the current context

    //     // let keystore = KeyStore::new();
    //     // // Insert Alice's keys
    //     // const SURI_ALICE: &str = "//Alice";
    //     // let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    //     // SyncCryptoStore::insert_unknown(
    //     //     &keystore,
    //     //     KEY_TYPE,
    //     //     SURI_ALICE,
    //     //     key_pair_alice.public().as_ref(),
    //     // )
    //     // .expect("Inserts unknown key");
    //     insert_default_xdns_record::<T>();

    // }: _(RawOrigin::Signed(Default::default()), io_schedule, components)
    // verify{}

    dry_run_whole_xtx_one_component {
        let inter_exec_schedule = InterExecSchedule {
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
        let requester: T::AccountId = account("TEST", 1_u32, USER_SEED);

    }: {Pallet::<T>::dry_run_whole_xtx(inter_exec_schedule, requester)}
    verify{}

    dry_run_whole_xtx_three_components {
        let inter_exec_schedule = InterExecSchedule {
            phases: vec![ExecPhase {
                steps: vec![
                    ExecStep {
                        compose: Compose {
                            name: b"component1".to_vec(),
                            code_txt: CODE.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: account("TEST", 1_u32, USER_SEED),
                            value: 0u32.into(),
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                    ExecStep {
                        compose: Compose {
                            name: b"component2".to_vec(),
                            code_txt: CODE1.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: account("TEST", 2_u32, USER_SEED),
                            value: 0u32.into(),
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                    ExecStep {
                        compose: Compose {
                            name: b"component2".to_vec(),
                            code_txt: CODE2.as_bytes().to_vec(),

                            exec_type: b"exec_escrow".to_vec(),
                            dest: account("TEST", 3_u32, USER_SEED),
                            value: 0u32.into(),
                            bytes: vec![],
                            input_data: vec![],
                        },
                    },
                ],
            }],
        };

        let requester: T::AccountId = account("TEST", 1_u32, USER_SEED);
    }: {Pallet::<T>::dry_run_whole_xtx(inter_exec_schedule, requester)}
    verify{}

    // pre_run_bunch_until_break {
    //     let compose = Compose {
    //         name: b"component1".to_vec(),
    //         code_txt: CODE1.as_bytes().to_vec(),
    //         exec_type: b"exec_escrow".to_vec(),
    //         dest: account("TEST", 1_u32, USER_SEED),
    //         value: 0u32.into(),
    //         bytes: vec![
    //                 0, 97, 115, 109, 1, 0, 0, 0, 1, 17, 2, 96, 9, 127, 127, 126, 127, 127, 127, 127,
    //                 127, 127, 1, 127, 96, 0, 0, 2, 34, 2, 5, 115, 101, 97, 108, 48, 9, 115, 101, 97,
    //                 108, 95, 99, 97, 108, 108, 0, 0, 3, 101, 110, 118, 6, 109, 101, 109, 111, 114, 121,
    //                 2, 1, 1, 1, 3, 3, 2, 1, 1, 7, 17, 2, 4, 99, 97, 108, 108, 0, 1, 6, 100, 101, 112,
    //                 108, 111, 121, 0, 2, 10, 28, 2, 23, 0, 65, 4, 65, 32, 66, 0, 65, 36, 65, 8, 65, 44,
    //                 65, 4, 65, 127, 65, 0, 16, 0, 26, 11, 2, 0, 11, 11, 60, 3, 0, 65, 4, 11, 32, 1, 1,
    //                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //                 1, 1, 0, 65, 36, 11, 8, 6, 0, 0, 0, 0, 0, 0, 0, 0, 65, 44, 11, 4, 1, 2, 3, 4,
    //         ],
    //         input_data: vec![],
    //     };
    //     let escrow_account: T::AccountId = account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1_u32, USER_SEED);

    //     let requester: T::AccountId = account("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv", 1_u32, USER_SEED);
    //     let gateway_id = None;
    //     let gateway_abi_config: GatewayABIConfig = Default::default();

    //     insert_default_xdns_record::<T>();

    //     // Error here: failed to select authority / no keystore associated for the current context

    //     // let keystore = KeyStore::new();
    //     // // Insert Alice's keys
    //     // const SURI_ALICE: &str = "//Alice";
    //     // let key_pair_alice = sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    //     // SyncCryptoStore::insert_unknown(
    //     //     &keystore,
    //     //     KEY_TYPE,
    //     //     SURI_ALICE,
    //     //     key_pair_alice.public().as_ref(),
    //     // )
    //     // .expect("Inserts unknown key");

    //     let example_contract = crate::ExecComposer::dry_run_single_contract::<T>(compose).unwrap();
    //     let submitter = crate::Pallet::<T>::select_authority(escrow_account.clone())
    //         .unwrap_or_else(|_| panic!("failed to select_authority"));

    // }: {crate::ExecComposer::pre_run_bunch_until_break::<T>(
    //         vec![example_contract],
    //         escrow_account,
    //         submitter,
    //         requester,
    //         0u32.into(),
    //         vec![],
    //         Weight::MAX,
    //         gateway_id,
    //         gateway_abi_config,
    //     )}
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
    //     let keystore = KeyStore::new();

    //     // Insert Alice's keys
    //     const SURI_ALICE: &str = "//Alice";
    //     let key_pair_alice =
    //         sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    //     SyncCryptoStore::insert_unknown(
    //         &keystore,
    //         KEY_TYPE,
    //         SURI_ALICE,
    //         key_pair_alice.public().as_ref(),
    //     )
    //     .expect("Inserts unknown key");
    //     new_test_ext().register_extension(KeystoreExt(keystore.into()));
    //     new_test_ext().execute_with(|| {
    //         assert_ok!(test_benchmark_submit_composable_exec_order::<Test>());
    //     })
    // }

    #[test]
    fn benchmark_dry_run_whole_xtx_one_component() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_dry_run_whole_xtx_one_component::<Test>());
        })
    }

    #[test]
    fn benchmark_dry_run_whole_xtx_three_components() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_dry_run_whole_xtx_three_components::<Test>());
        })
    }

    // #[test]
    // fn benchmark_pre_run_bunch_until_break() {
    //     let keystore = KeyStore::new();

    //     // Insert Alice's keys
    //     const SURI_ALICE: &str = "//Alice";
    //     let key_pair_alice =
    //         sr25519::Pair::from_string(SURI_ALICE, None).expect("Generates key pair");
    //     SyncCryptoStore::insert_unknown(
    //         &keystore,
    //         KEY_TYPE,
    //         SURI_ALICE,
    //         key_pair_alice.public().as_ref(),
    //     )
    //     .expect("Inserts unknown key");
    //     new_test_ext().register_extension(KeystoreExt(keystore.into()));
    //     new_test_ext().execute_with(|| {
    //         assert_ok!(test_benchmark_pre_run_bunch_until_break::<Test>());
    //     })
    // }
}

impl_benchmark_test_suite!(
    ExecDelivery,
    crate::tests::new_test_ext(),
    crate::mock::Test
);
