// Tests to be written here

// Loading .wasm files deps
use crate::{mock::*, CallStamp, Compose, Error, ExecutionProofs, ExecutionStamp, InterExecReq};
use anyhow::{Context, Result};
use codec::Encode;
use frame_support::dispatch::DispatchError;
use frame_support::{assert_noop, assert_ok, storage::child, traits::Currency};
use gateway_escrow_engine::transfers::{BalanceOf, TransferEntry};
use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;
use std::path::Path;
use std::{fs, io::Read};
use versatile_wasm::{gas::Gas, runtime::get_child_storage_for_current_execution};
///
/// Multistep Call - puts_code, instantiates, calls and terminates wasm contract codes on the fly.
/// Such a wasm code is called package.
/// Consists of 3 execution phases:
/// * Execute: Code results are stored on escrow account under corresponding to the call storage key.
/// * Revert:  Code results are removed out of escrow account.
/// * Commit:  Code results are moved from escrow account to target accounts.

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
fn successfully_decodes_execution_schedule() {
    let (_phase, _, _input_data, value, _gas_limit) = default_multistep_call_args();
    let component_1_code_path = Path::new("./fixtures/transfer_return_code.wasm");
    let component_1_wasm_code = load_contract_code(&component_1_code_path).unwrap();
    let component_2_code_path = Path::new("./fixtures/storage_runtime_calls.wasm");
    let component_2_wasm_code = load_contract_code(&component_2_code_path).unwrap();
    let component_3_code_path = Path::new("./fixtures/storage_runtime_demo.wasm");
    let component_3_wasm_code = load_contract_code(&component_3_code_path).unwrap();
    let exact_gas_cost = 359_029_940 as u64;
    let gas_excess = 100 as u64;
    let _endowment = 100_000_000;
    let subsistence_threshold = 1;

    new_test_ext_builder(subsistence_threshold, ESCROW_ACCOUNT).execute_with(|| {
        let _ = Balances::deposit_creating(
            &REQUESTER,
            exact_gas_cost + gas_excess + subsistence_threshold,
        );
        assert_ok!(Circuit::composable_execution(
            Origin::signed(REQUESTER),
            ESCROW_ACCOUNT,
            vec![], // code components
            InterExecReq {
                components: vec![
                    Compose {
                        name: "component1".encode(),
                        gateway: "gatewayA".encode(),
                        exec_type: "tx-only".encode(),
                        dest: TARGET_DEST.encode(),
                        // bytes: component_1_wasm_code,
                        bytes: vec![],
                        code_txt: "var s = 1".encode(),
                    },
                    Compose {
                        name: "component2".encode(),
                        gateway: "gatewayA".encode(),
                        exec_type: "call-static".encode(),
                        dest: TARGET_DEST.encode(),
                        // bytes: component_2_wasm_code,
                        bytes: vec![],
                        code_txt: "var s = 2".encode(),
                    },
                    Compose {
                        name: "component3".encode(),
                        gateway: "gatewayA".encode(),
                        exec_type: "exec-side-effects".encode(),
                        dest: TARGET_DEST.encode(),
                        // bytes: component_3_wasm_code,
                        bytes: vec![],
                        code_txt: "var s = 3".encode(),
                    },
                ],
                io: "component1 || component2 > component3;".encode()
            },
            // r#"
            // {
            //     "component": "component1",
            //     "gateway": "gatewayA",
            //     "exec_type": "transferonly",
            //     "dest": "0xABCDEF0123456789",
            // }
            // "#.encode(), // exec request
            value,
            exact_gas_cost + gas_excess, //gas limit
            Encode::encode(&17),         // input
        ));
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
