// Tests to be written here

// Loading .wasm files deps
use std::path::Path;
use std::{fs, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use codec::Encode;
use contracts::{escrow_exec::TransferEntry, BalanceOf, Gas};
use escrow_gateway_primitives::Phase;
use frame_support::{
    assert_err, assert_err_ignore_postinfo, assert_noop, assert_ok,
    traits::{Currency, Get, ReservableCurrency},
    weights::Weight,
};
use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

use crate::{mock::*, CallStamp, Error, ExecutionProofs, ExecutionStamp};

/***
    Multistep Call - puts_code, instantiates, calls and terminates wasm contract codes on the fly.
    Such a wasm code is called package.
    Consists of 3 execution phases:
        - Execute: Code results are stored on escrow account under corresponding to the call storage key.
        - Revert:  Code results are removed out of escrow account.
        - Commit:  Code results are moved from escrow account to target accounts.
***/

const EXECUTE_PHASE: u8 = 0;
const COMMIT_PHASE: u8 = 1;
const REVERT_PHASE: u8 = 2;

const ZERO_ACCOUNT: u64 = 0;
const ESCROW_ACCOUNT: u64 = 1;
const TEMP_EXEC_CONTRACT: u64 = 2;
const REQUESTER: u64 = 3;
const TARGET_DEST: u64 = 4;
const OTHER_ACCOUNT: u64 = 5;

/**
 BASE GAS COSTS:
  - INSTANTIATE = 175 * 500_000
  - CALL = 135 * 500_000
  - total = 310 * 500_000 = 155_000_000
**/
fn default_multistep_call_args() -> (u8, Vec<u8>, Vec<u8>, BalanceOf<Test>, Gas) {
    let phase = 0 as u8;
    let code: Vec<u8> = Vec::new();
    let input_data: Vec<u8> = Vec::new();
    let value = BalanceOf::<Test>::from(500_000 as u64);
    let gas_limit: Gas = 155_000_000 + 187_500_000 + 107_500_000 + 210_000; // Actual gas costs of "return_from_start_fn" instantiation cost
    return (phase, code, input_data, value, gas_limit);
}

#[test]
fn should_only_allow_to_be_called_by_escrow_account_being_sudo() {
    let (phase, code, input_data, value, gas_limit) = default_multistep_call_args();

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(&REQUESTER, 10_000_000_000);

        let err_rec = EscrowGateway::multistep_call(
            Origin::signed(OTHER_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            phase,
            code,
            value,
            gas_limit,
            input_data,
        );
        assert_noop!(err_rec, Error::<Test>::UnauthorizedCallAttempt);
    });
}

#[test]
fn during_execution_phase_when_given_empty_wasm_code_multistep_call_only_deferrs_transfer() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(&REQUESTER, 10_000_000_000);

        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            phase,
            Vec::new(),
            value,
            gas_limit,
            input_data,
        ));

        assert_eq!(
            EscrowGateway::deferred_transfers(&REQUESTER, &TARGET_DEST),
            [TransferEntry {
                to: [4, 0, 0, 0, 0, 0, 0, 0].to_vec(),
                value: 500000,
                data: [].to_vec(),
            },]
        );
    });
}

#[test]
fn during_execution_phase_when_given_correct_wasm_code_but_too_little_gas_limit_multistep_call_gives_initiate_error()
 {
    let (phase, _, input_data, value, mut gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/return_from_start_fn.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    // Make the gas limit too little
    gas_limit = 1000;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(&REQUESTER, 10_000_000_000);
        assert_err!(
            EscrowGateway::multistep_call(
                Origin::signed(ESCROW_ACCOUNT),
                REQUESTER,
                TARGET_DEST,
                phase,
                correct_wasm_code,
                value,
                gas_limit,
                input_data
            ),
            Error::<Test>::InitializationFailure
        );
    });
}

#[test]
fn during_execution_phase_when_given_correct_wasm_code_multistep_call_succeeds() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/return_from_start_fn.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(&REQUESTER, 10_000_000_000);

        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            phase,
            correct_wasm_code,
            value,
            gas_limit,
            input_data
        ));
    });
}

#[test]
fn during_execution_phase_when_given_correct_wasm_code_multistep_call_vm_succeeds() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/return_from_start_fn.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(&REQUESTER, 10_000_000_000);

        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            phase,
            correct_wasm_code,
            value,
            gas_limit,
            input_data
        ));
    });
}

/**
    TRANSFERS
**/
#[test]
fn transfer_during_execution_phase_succeeds_and_consumes_costs_correctly_and_deferrs_transfers() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/transfer_return_code.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    /// Set fees
    let sufficient_gas_limit = (170_000_000 + 17_500_000) as u64; // base (exact init costs) + exec_cost = 187_500_000
    let endowment = 100_000_000;
    let subsistence_threshold = 66;
    let inner_contract_transfer_value = 100;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            sufficient_gas_limit
                + endowment
                + subsistence_threshold
                + (value)
                + inner_contract_transfer_value,
        );
        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            phase,
            correct_wasm_code,
            value,
            sufficient_gas_limit,
            input_data
        ));

        // Escrow Account is now pre-charged by requester to cover:
        // 187_500_000 gas_fees + 500_000 requested balance transfer to &target_dest + 100 requested by contract value transfer to &0
        assert_eq!(Balances::total_balance(&ESCROW_ACCOUNT), 188_000_100);

        // Requester is only left with subsistence threshold
        assert_eq!(Balances::total_balance(&REQUESTER), subsistence_threshold);

        // Account of temporary execution contract is now charged with endowment
        assert_eq!(Balances::total_balance(&TEMP_EXEC_CONTRACT), endowment);

        // Still nothing on target_dest account as it is only the execution phase.
        assert_eq!(Balances::total_balance(&TARGET_DEST), 0);

        // There should be an entry with deferred transfer to the target dest though as well as the requested by contract value transfer of 100 to &0
        assert_eq!(
            EscrowGateway::deferred_transfers(&REQUESTER, &TARGET_DEST),
            [
                TransferEntry {
                    to: [4, 0, 0, 0, 0, 0, 0, 0].to_vec(),
                    value: 500000,
                    data: [].to_vec(),
                },
                TransferEntry {
                    to: [0, 0, 0, 0, 0, 0, 0, 0].to_vec(),
                    value: 100,
                    data: [].to_vec(),
                }
            ]
        );
    });
}

#[test]
fn commit_phase_cannot_be_triggered_without_preceeding_execution() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/transfer_return_code.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    /// Set fees
    let sufficient_gas_limit = (170_000_000 + 17_500_000) as u64; // base (exact init costs) + exec_cost = 187_500_000
    let endowment = 100_000_000;
    let subsistence_threshold = 66;
    let inner_contract_transfer_value = 100;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            sufficient_gas_limit
                + endowment
                + subsistence_threshold
                + (value)
                + inner_contract_transfer_value,
        );

        assert_noop!(
            EscrowGateway::multistep_call(
                Origin::signed(ESCROW_ACCOUNT),
                REQUESTER,
                TARGET_DEST,
                COMMIT_PHASE,
                correct_wasm_code.clone(),
                value,
                sufficient_gas_limit,
                input_data.clone()
            ),
            Error::<Test>::CommitOnlyPossibleAfterSuccessfulExecutionPhase
        );
    });
}

#[test]
fn during_commit_phase_transfers_move_from_deferred_to_target_destinations() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/transfer_return_code.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    /// Set fees
    let sufficient_gas_limit = (170_000_000 + 17_500_000) as u64; // base (exact init costs) + exec_cost = 187_500_000
    let endowment = 100_000_000;
    let subsistence_threshold = 66;
    let inner_contract_transfer_value = 100;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            sufficient_gas_limit
                + endowment
                + subsistence_threshold
                + (value)
                + inner_contract_transfer_value,
        );
        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            EXECUTE_PHASE,
            correct_wasm_code.clone(),
            value,
            sufficient_gas_limit,
            input_data.clone()
        ));

        // There should be an entry with deferred transfer to the target dest though as well as the requested by contract value transfer of 100 to &0
        assert_eq!(Balances::total_balance(&TARGET_DEST), 0);
        assert_eq!(Balances::total_balance(&ZERO_ACCOUNT), 0);

        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            COMMIT_PHASE,
            correct_wasm_code.clone(),
            value,
            sufficient_gas_limit,
            input_data.clone()
        ));

        assert_eq!(Balances::total_balance(&TARGET_DEST), 500_000);
        assert_eq!(Balances::total_balance(&ZERO_ACCOUNT), 100);
    });
}

#[test]
fn successful_commit_phase_changes_phase_of_execution_stamp() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/transfer_return_code.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    /// Set fees
    let sufficient_gas_limit = (170_000_000 + 17_500_000) as u64; // base (exact init costs) + exec_cost = 187_500_000
    let endowment = 100_000_000;
    let subsistence_threshold = 66;
    let inner_contract_transfer_value = 100;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            sufficient_gas_limit
                + endowment
                + subsistence_threshold
                + (value)
                + inner_contract_transfer_value,
        );
        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            EXECUTE_PHASE,
            correct_wasm_code.clone(),
            value,
            sufficient_gas_limit,
            input_data.clone()
        ));

        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            COMMIT_PHASE,
            correct_wasm_code.clone(),
            value,
            sufficient_gas_limit,
            input_data.clone()
        ));
        assert_eq!(
            EscrowGateway::execution_stamps(
                &REQUESTER,
                &<Test as frame_system::Trait>::Hashing::hash(&correct_wasm_code.clone())
            ),
            ExecutionStamp {
                timestamp: 0,
                phase: 1,
                proofs: Some(ExecutionProofs {
                    result: Some(vec![
                        17, 218, 109, 31, 118, 29, 223, 155, 219, 76, 157, 110, 83, 3, 235, 212,
                        31, 97, 133, 141, 10, 86, 71, 161, 167, 191, 224, 137, 191, 146, 27, 233
                    ]),
                    storage: Some(vec![
                        251, 157, 122, 148, 72, 142, 85, 179, 78, 9, 191, 10, 233, 122, 212, 27,
                        172, 57, 71, 192, 40, 9, 217, 136, 38, 77, 99, 3, 206, 138, 53, 31
                    ]),
                    deferred_transfers: vec![
                        TransferEntry {
                            to: vec![4, 0, 0, 0, 0, 0, 0, 0],
                            value: 500000,
                            data: vec![]
                        },
                        TransferEntry {
                            to: vec![0, 0, 0, 0, 0, 0, 0, 0],
                            value: 100,
                            data: vec![]
                        }
                    ]
                }),
                call_stamps: vec![CallStamp {
                    // That's good - transfer contract doesn't touch the storage therefore pre and post state are equal.
                    pre_storage: vec![
                        3, 23, 10, 46, 117, 151, 183, 183, 227, 216, 76, 5, 57, 29, 19, 154, 98,
                        177, 87, 231, 135, 134, 216, 192, 130, 242, 157, 207, 76, 17, 19, 20
                    ],
                    post_storage: vec![
                        3, 23, 10, 46, 117, 151, 183, 183, 227, 216, 76, 5, 57, 29, 19, 154, 98,
                        177, 87, 231, 135, 134, 216, 192, 130, 242, 157, 207, 76, 17, 19, 20
                    ],
                    dest: vec![2, 0, 0, 0, 0, 0, 0, 0]
                }],
                failure: None
            }
        );
    });
}

#[test]
fn successful_commit_phase_applies_deferred_storage_writes() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/storage_size.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    /// Set fees
    let sufficient_gas_limit = (170_000_000 + 17_500_000) as u64; // base (exact init costs) + exec_cost = 187_500_000
    let endowment = 100_000_000;
    let subsistence_threshold = 66;
    let inner_contract_transfer_value = 100;

    new_test_ext_builder(50, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            sufficient_gas_limit
                + endowment
                + subsistence_threshold
                + (value)
                + inner_contract_transfer_value,
        );
        assert_ok!(EscrowGateway::multistep_call(
            Origin::signed(ESCROW_ACCOUNT),
            REQUESTER,
            TARGET_DEST,
            EXECUTE_PHASE,
            correct_wasm_code.clone(),
            value,
            sufficient_gas_limit,
            Encode::encode(&17),
        ));


        assert_eq!(
            EscrowGateway::execution_stamps(
                &REQUESTER,
                &<Test as frame_system::Trait>::Hashing::hash(&correct_wasm_code.clone())
            ),
            ExecutionStamp {
                timestamp: 0,
                phase: 0,
                proofs: Some(ExecutionProofs {
                    result: Some(vec![
                        14, 87, 81, 192, 38, 229, 67, 178, 232, 171, 46, 176, 96, 153, 218, 161,
                        209, 229, 223, 71, 119, 143, 119, 135, 250, 171, 69, 205, 241, 47, 227,
                        168
                    ]),
                    storage: Some(vec![
                        225, 105, 234, 201, 35, 192, 174, 73, 89, 201, 124, 236, 194, 183, 248,
                        252, 93, 242, 222, 216, 124, 186, 81, 105, 223, 249, 163, 32, 84, 85, 114,
                        217
                    ]),
                    deferred_transfers: vec![TransferEntry {
                        to: vec![4, 0, 0, 0, 0, 0, 0, 0],
                        value: 500000,
                        data: vec![]
                    }]
                }),
                call_stamps: vec![CallStamp {
                    // That's good - set storage contract touches the storage therefore pre and post state are different.
                    pre_storage: vec![
                        3, 23, 10, 46, 117, 151, 183, 183, 227, 216, 76, 5, 57, 29, 19, 154, 98,
                        177, 87, 231, 135, 134, 216, 192, 130, 242, 157, 207, 76, 17, 19, 20
                    ],
                    post_storage: vec![
                        70, 154, 173, 211, 104, 40, 179, 230, 168, 8, 155, 88, 138, 35, 147, 247,
                        218, 122, 13, 85, 84, 230, 21, 127, 1, 8, 128, 129, 211, 108, 6, 215
                    ],
                    dest: vec![2, 0, 0, 0, 0, 0, 0, 0]
                }],
                failure: None
            }
        );
        // assert_ok!(EscrowGateway::multistep_call(
        //     Origin::signed(ESCROW_ACCOUNT),
        //     REQUESTER,
        //     TARGET_DEST,
        //     COMMIT_PHASE,
        //     correct_wasm_code.clone(),
        //     value,
        //     sufficient_gas_limit,
        //     input_data.clone()
        // ));
    });
}

/// Load the wasm blob from the specified path.
///
/// Defaults to the target contract wasm in the current project, inferred via the crate metadata.
fn load_contract_code(path: &Path) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut file = fs::File::open(path).context(format!("Failed to open {}", path.display()))?;
    file.read_to_end(&mut data)?;

    Ok(data)
}

/// Load a given wasm module represented by a .wat file and returns a wasm binary contents along
/// with it's hash.
///
/// The fixture files are located under the `fixtures/` directory.
fn compile_module<T>(fixture_name: &str) -> wat::Result<(Vec<u8>, <T::Hashing as Hash>::Output)>
where
    T: frame_system::Trait,
{
    let fixture_path = ["fixtures/", fixture_name, ".wat"].concat();
    let wasm_binary = wat::parse_file(fixture_path)?;
    let code_hash = T::Hashing::hash(&wasm_binary);
    Ok((wasm_binary, code_hash))
}
