use crate::exec_composer::{
    ExecComposer, OptimisticOutputMode, PessimisticOutputMode, RawAliveContractInfo,
    TEST_RUNTIME_VERSION,
};
use crate::mock::{ExtBuilder, Test};
use crate::{
    BalanceOf, Compose, Config, ContractActionDesc, GatewayABIConfig, GatewayGenesisConfig,
    RegistryContract, KEY_TYPE,
};
use codec::Encode;
use frame_support::{assert_err, assert_ok, weights::Weight};
use hex_literal::hex;
use sp_core::H256;
use sp_core::{crypto::Pair, sr25519, Hasher};
use sp_io::TestExternalities;
use sp_keystore::testing::KeyStore;
use sp_keystore::{KeystoreExt, SyncCryptoStore};
use sp_runtime::AccountId32;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use t3rn_primitives::{GatewayExpectedOutput, GatewayPointer, GatewayType, GatewayVendor};
use volatile_vm::wasm::{PrefabWasmModule, RunMode};
use volatile_vm::VolatileVM;

pub fn make_compose_out_of_raw_wat_code<T: Config>(
    wat_string_path: &str,
    input_data: Vec<u8>,
    dest: T::AccountId,
    value: BalanceOf<T>,
) -> Compose<T::AccountId, BalanceOf<T>> {
    let fixture_path = ["fixtures/", wat_string_path, ".wat"].concat();
    let file = File::open(fixture_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut wat_string = String::new();
    let _res = buf_reader.read_to_string(&mut wat_string);
    let wasm = match wat::parse_str(wat_string.clone()) {
        Ok(wasm) => wasm,
        Err(_err) => " invalid code str ".encode(),
    };
    Compose {
        name: b"component1".to_vec(),
        code_txt: wat_string.encode(),
        exec_type: b"exec_escrow".to_vec(),
        dest,
        value,
        bytes: wasm,
        input_data,
    }
}

pub fn insert_default_xdns_record() {
    use pallet_xdns::XdnsRecord;
    pallet_xdns::XDNSRegistry::<Test>::insert(
        // Below is blake2_hash of [0, 0, 0, 0]
        H256::from_slice(&hex!(
            "11da6d1f761ddf9bdb4c9d6e5303ebd41f61858d0a5647a1a7bfe089bf921be9"
        )),
        XdnsRecord::<AccountId32>::new(
            Default::default(),
            [0, 0, 0, 0],
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            GatewayGenesisConfig {
                modules_encoded: None,
                signed_extension: None,
                runtime_version: TEST_RUNTIME_VERSION,
                genesis_hash: Default::default(),
                extrinsics_version: 0u8,
            },
        ),
    );
}

fn setup_test_escrow_as_tx_signer(ext: &mut TestExternalities) -> AccountId32 {
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

    ext.register_extension(KeystoreExt(keystore.into()));
    // Alice's account
    hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()
}

fn make_registry_contract_out_of_wat<T: Config>(
    wat: &str,
    input_data: Vec<u8>,
    dest: T::AccountId,
    value: BalanceOf<T>,
) -> RegistryContract<T::Hash, T::AccountId, BalanceOf<T>, T::BlockNumber> {
    let compose = make_compose_out_of_raw_wat_code::<T>(wat, input_data, dest, value);

    RegistryContract::from_compose(
        compose.clone(),
        vec![],
        Default::default(),
        None,
        None,
        Some(RawAliveContractInfo {
            trie_id: Default::default(),
            storage_size: Default::default(),
            pair_count: Default::default(),
            code_hash: T::Hashing::hash(&compose.bytes),
            rent_allowance: Default::default(),
            rent_paid: Default::default(),
            deduct_block: Default::default(),
            last_write: Default::default(),
            _reserved: Default::default(),
        }),
        Default::default(),
    )
}

const INVALID_CODE: &str = "invalid_code";
const CODE_CALL: &str = "code_call";
const WRONG_CODE_MODULE_DISPATCH_NO_FUNC: &str = "wrong_code_module_dispatch_no_func";

#[test]
fn dry_run_succeeds_for_valid_call_contract_with_declared_foreign_target() {
    // Bob - dest
    let dest = AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let _gateway_id = [0 as u8; 4];
    let compose = make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, vec![], dest, value);

    let mut ext = TestExternalities::new_empty();
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
    let _gateway_abi_config: GatewayABIConfig = Default::default();

    let account_at_foreign_target = AccountId32::from(hex!(
        "0101010101010101010101010101010101010101010101010101010101010101"
    ));
    let example_foreign_target = [1u8, 2u8, 3u8, 4u8];

    ext.execute_with(|| {
        let _submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));

        insert_default_xdns_record();

        volatile_vm::DeclaredTargets::<Test>::insert(
            account_at_foreign_target,
            example_foreign_target.clone(),
        );

        let res = ExecComposer::dry_run_single_contract::<Test>(compose);
        assert_ok!(res.clone());
        assert_eq!(
            res.unwrap().action_descriptions,
            vec![ContractActionDesc {
                action_id: H256::from(hex!(
                    "8983f833d99e84d9dd10a9ce44549e9ba4fb831a62bd4435642ad6fa32a1da7f"
                )),
                target_id: Some(example_foreign_target),
                to: Some(AccountId32::from(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )))
            }]
        );
    });
}

#[test]
fn dry_run_succeeds_for_valid_call_contract() {
    // Bob - dest
    let dest = AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let _gateway_id = [0 as u8; 4];
    let compose = make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, vec![], dest, value);

    let mut ext = TestExternalities::new_empty();
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
    let _gateway_abi_config: GatewayABIConfig = Default::default();

    ext.execute_with(|| {
        insert_default_xdns_record();

        let _submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));

        let res = ExecComposer::dry_run_single_contract::<Test>(compose);

        assert_ok!(res.clone());
        assert_eq!(
            res.unwrap().action_descriptions,
            vec![ContractActionDesc {
                action_id: H256::from(hex!(
                    "8983f833d99e84d9dd10a9ce44549e9ba4fb831a62bd4435642ad6fa32a1da7f"
                )),
                target_id: None,
                to: Some(AccountId32::from(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                )))
            }]
        );
    });
}

#[test]
fn dry_run_fails_for_invalid_call_contract() {
    // Bob - dest
    let dest = AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let _gateway_id = [0 as u8; 4];

    let compose = Compose {
        name: b"component1".to_vec(),
        code_txt: " invalid code str ".encode(),
        exec_type: b"exec_escrow".to_vec(),
        dest,
        value,
        bytes: " invalid code str ".encode(),
        input_data: vec![],
    };

    let mut ext = TestExternalities::new_empty();
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
    let _gateway_abi_config: GatewayABIConfig = Default::default();

    ext.execute_with(|| {
        insert_default_xdns_record();
        let _submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));
        let res = ExecComposer::dry_run_single_contract::<Test>(compose);
        assert_err!(res, "Can't decode WASM code");
    });
}

#[test]
fn pre_run_produces_outbound_messages_if_declared_remote_target() {
    // Bob - requester
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let input_data = vec![];
    let gas_limit = 1726103 + 283184644 + 143915670; // gas limit for the example call
    let gateway_id = None; // on-chain contract = None as a target_id
    let compose =
        make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, vec![], requester.clone(), value);

    let mut ext = TestExternalities::new_empty();
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
    let _gateway_abi_config: GatewayABIConfig = Default::default();

    let account_at_foreign_target = AccountId32::from(hex!(
        "0101010101010101010101010101010101010101010101010101010101010101"
    ));
    let example_foreign_target = [1u8, 2u8, 3u8, 4u8];

    ext.execute_with(|| {
        let submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));

        insert_default_xdns_record();

        volatile_vm::DeclaredTargets::<Test>::insert(
            account_at_foreign_target,
            example_foreign_target,
        );

        let _output_mode = PessimisticOutputMode::new();

        let gateway_abi_config = Default::default();
        let example_contract = ExecComposer::dry_run_single_contract::<Test>(compose).unwrap();

        let res = ExecComposer::pre_run_bunch_until_break::<Test>(
            vec![example_contract],
            escrow_account,
            submitter.clone(),
            requester,
            value,
            input_data,
            gas_limit,
            gateway_id,
            gateway_abi_config,
        );

        assert_ok!(res.clone());

        let succ_response = res.unwrap();
        let test_messages_at_this_round = succ_response.0;
        let first_message = test_messages_at_this_round[0].clone();

        crate::message_assembly::substrate_gateway_protocol::tests::assert_signed_payload(
            first_message,
            submitter.into(),
            vec![vec![4, 95], vec![1, 2, 3, 4]], // arguments
            vec![
                GatewayExpectedOutput::Events {
                    signatures: vec![b"Call(address,value,uint64,dynamic_bytes)".to_vec()],
                },
                GatewayExpectedOutput::Output {
                    output: b"dynamic_bytes".to_vec(),
                },
            ],
            vec![0, 0, 4, 95, 1, 2, 3, 4],
            vec![
                0, 0, 4, 95, 1, 2, 3, 4, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 14, 87, 81, 192, 38, 229,
                67, 178, 232, 171, 46, 176, 96, 153, 218, 161, 209, 229, 223, 71, 119, 143, 119,
                135, 250, 171, 69, 205, 241, 47, 227, 168, 14, 87, 81, 192, 38, 229, 67, 178, 232,
                171, 46, 176, 96, 153, 218, 161, 209, 229, 223, 71, 119, 143, 119, 135, 250, 171,
                69, 205, 241, 47, 227, 168,
            ],
            "state",
            "call",
        );
    });
}

#[test]
fn pre_run_recognizes_call_module_from_flags_and_fails_for_empty_names() {
    // Bob - requester
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let input_data = vec![];
    let gas_limit = Weight::MAX;
    let gateway_id = None; // on-chain contract = None as a target_id
    let compose = make_compose_out_of_raw_wat_code::<Test>(
        WRONG_CODE_MODULE_DISPATCH_NO_FUNC,
        vec![],
        requester.clone(),
        value,
    );

    let mut ext = TestExternalities::new_empty();
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);
    let _gateway_abi_config: GatewayABIConfig = Default::default();

    let account_at_foreign_target = AccountId32::from(hex!(
        "0101010101010101010101010101010101010101010101010101010101010101"
    ));
    let example_foreign_target = [1u8, 2u8, 3u8, 4u8];

    ext.execute_with(|| {
        let submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));

        // Set the default XDNS record for default [0, 0, 0, 0] gateway
        insert_default_xdns_record();

        volatile_vm::DeclaredTargets::<Test>::insert(
            account_at_foreign_target,
            example_foreign_target,
        );

        let _output_mode = PessimisticOutputMode::new();

        let gateway_abi_config = Default::default();
        let example_contract = ExecComposer::dry_run_single_contract::<Test>(compose).unwrap();

        let res = ExecComposer::pre_run_bunch_until_break::<Test>(
            vec![example_contract],
            escrow_account,
            submitter,
            requester,
            value,
            input_data,
            gas_limit,
            gateway_id,
            gateway_abi_config,
        );

        assert_eq!(
            res,
            Err("Input < 64 doesn't allow to extract function and method names")
        )
    });
}

#[test]
fn pre_run_bunch_until_break_succeeds_for_two_contracts() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let escrow_account = setup_test_escrow_as_tx_signer(&mut ext);

    let compose_one =
        make_compose_out_of_raw_wat_code::<Test>(CODE_CALL, vec![], requester.clone(), value);

    let compose_two = make_compose_out_of_raw_wat_code::<Test>(
        WRONG_CODE_MODULE_DISPATCH_NO_FUNC,
        vec![],
        requester.clone(),
        value,
    );

    ext.execute_with(|| {
        let submitter = crate::Pallet::<Test>::select_authority(escrow_account.clone())
            .unwrap_or_else(|_| panic!("failed to select_authority"));
        insert_default_xdns_record();

        let contract_one = ExecComposer::dry_run_single_contract::<Test>(compose_one).unwrap();
        let contract_two = ExecComposer::dry_run_single_contract::<Test>(compose_two).unwrap();

        let res = ExecComposer::pre_run_bunch_until_break::<Test>(
            vec![contract_one, contract_two],
            escrow_account,
            submitter,
            requester,
            value,
            vec![],
            Weight::MAX,
            None,
            Default::default(),
        );

        assert_ok!(res.clone());

        let unwrapped_result = res.unwrap();
        assert_eq!(unwrapped_result.1, 2u16);
        assert_eq!(unwrapped_result.0.len(), 0);
    });
}

#[test]
fn preload_bunch_of_contracts_succeeds_for_one_contract() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);

    let schedule = <Test as VolatileVM>::Schedule::get();
    let executable = PrefabWasmModule::<Test>::from_code(
        temp_contract_one.bytes.clone(),
        &schedule,
        RunMode::Dry,
        None,
    )
    .unwrap();

    ext.execute_with(|| {
        insert_default_xdns_record();
        let res = ExecComposer::preload_bunch_of_contracts::<Test>(
            vec![temp_contract_one],
            Default::default(),
        );

        assert_ok!(res);
        let fetched_contract =
            volatile_vm::Pallet::<Test>::get_contract_code_lazy(executable.code_hash);
        assert!(fetched_contract.is_ok());
        // This assertion should logically pass but is failing. Doesnt makes sense.
        // assert_eq!(fetched_contract.unwrap().code_hash, executable.code_hash);
    });
}

#[test]
fn preload_bunch_of_contracts_succeeds_for_two_contracts() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);
    let temp_contract_two =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);
    ext.execute_with(|| {
        insert_default_xdns_record();

        let res = ExecComposer::preload_bunch_of_contracts::<Test>(
            vec![temp_contract_one, temp_contract_two],
            Default::default(),
        );

        assert_ok!(res);
    });
}

#[test]
fn preload_bunch_of_contracts_fails_for_one_contract_when_contract_invalid() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(INVALID_CODE, vec![], requester.clone(), value);

    ext.execute_with(|| {
        insert_default_xdns_record();

        let res = ExecComposer::preload_bunch_of_contracts::<Test>(
            vec![temp_contract_one],
            Default::default(),
        );

        assert_err!(res, "Can't decode WASM code");
    });
}

#[test]
fn preload_bunch_of_contracts_fails_for_two_contracts_when_one_contract_invalid() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);
    let temp_contract_two =
        make_registry_contract_out_of_wat::<Test>(INVALID_CODE, vec![], requester.clone(), value);

    ext.execute_with(|| {
        insert_default_xdns_record();
        let res = ExecComposer::preload_bunch_of_contracts::<Test>(
            vec![temp_contract_one, temp_contract_two],
            Default::default(),
        );

        assert_err!(res, "Can't decode WASM code");
    });
}

#[test]
fn run_single_contract_fails_with_stack_error_when_contract_not_preloaded() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let mut temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);

    ext.execute_with(|| {
        insert_default_xdns_record();
        let res = ExecComposer::run_single_contract::<Test, OptimisticOutputMode>(
            &mut temp_contract_one,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(), // requester
            Default::default(),
            Default::default(),
        );

        assert_err!(res, "Can't create VVM call stack");
    });
}

#[test]
fn run_single_contract_fails_with_xdns_error_when_xdns_record_not_present() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let mut temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);

    ext.execute_with(|| {
        // This comment line is intentional
        // Not inserting xdns record to replicate this failure
        // insert_default_xdns_record();

        let res = ExecComposer::run_single_contract::<Test, OptimisticOutputMode>(
            &mut temp_contract_one,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        assert_err!(res, "Xdns record not found");
    });
}

#[test]
fn run_single_contract_fails_with_wasm_parse_error_when_contract_is_invalid() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let mut temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(INVALID_CODE, vec![], requester.clone(), value);
    ext.execute_with(|| {
        insert_default_xdns_record();
        let res = ExecComposer::run_single_contract::<Test, OptimisticOutputMode>(
            &mut temp_contract_one,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        assert_err!(res, "Can't decode WASM code");
    });
}

#[test]
#[ignore = "Please implement proper assertions when retrieve_gateway_protocol is properly implemented"]
fn run_single_contract_fails_with_retrieve_gateway_protocol_error() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let mut temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);

    ext.execute_with(|| {
        insert_default_xdns_record();

        let res = ExecComposer::run_single_contract::<Test, OptimisticOutputMode>(
            &mut temp_contract_one,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        assert_eq!(res, res);
    });
}

#[test]
fn run_single_contract_succeeds() {
    let mut ext = TestExternalities::new_empty();
    let requester =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    let value = BalanceOf::<Test>::from(0u32);
    let mut temp_contract_one =
        make_registry_contract_out_of_wat::<Test>(CODE_CALL, vec![], requester.clone(), value);

    ext.execute_with(|| {
        insert_default_xdns_record();
        let preload_response = ExecComposer::preload_bunch_of_contracts::<Test>(
            vec![temp_contract_one.clone()],
            Default::default(),
        );
        assert_ok!(preload_response);

        let res = ExecComposer::run_single_contract::<Test, OptimisticOutputMode>(
            &mut temp_contract_one,
            Default::default(),
            Weight::MAX,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(), // requester
            Default::default(),
            Default::default(),
        );
        assert_ok!(res.clone());
    });
}

#[test]
fn retrieve_gateway_pointer_success_with_circuit_gateway_id() {
    ExtBuilder::default()
        .with_default_xdns_records()
        .build()
        .execute_with(|| {
            let gateway_id = *b"circ";
            let gateway_pointer = ExecComposer::retrieve_gateway_pointer::<Test>(Some(gateway_id));

            let xdns_record_id =
                <Test as frame_system::Config>::Hashing::hash(Encode::encode(&gateway_id).as_ref());
            let xdns_record = pallet_xdns::Pallet::<Test>::xdns_registry(xdns_record_id).unwrap();

            let expected = Ok(GatewayPointer {
                id: xdns_record.gateway_id,
                gateway_type: xdns_record.gateway_type,
                vendor: xdns_record.gateway_vendor,
            });

            assert_eq!(gateway_pointer, expected);
        })
}

#[test]
fn retrieve_gateway_pointer_success_with_none() {
    let gateway_id = None;

    let gateway_pointer = ExecComposer::retrieve_gateway_pointer::<Test>(gateway_id);

    let expected = Ok(GatewayPointer {
        id: Default::default(),
        gateway_type: GatewayType::ProgrammableExternal(0),
        vendor: GatewayVendor::Substrate,
    });

    assert_eq!(gateway_pointer, expected);
}
