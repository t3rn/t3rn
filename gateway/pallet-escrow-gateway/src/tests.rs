// Tests to be written here

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, weights::Weight,
                    assert_err_ignore_postinfo, assert_err,
                    traits::{Currency, Get, ReservableCurrency},
};
use escrow_gateway_primitives::{Phase};
use sp_std::vec::Vec;
use contracts::{BalanceOf, Gas, GasMeter};

// Loading .wasm files deps
use std::{fs, io::Read, path::PathBuf};
use anyhow::{Context, Result};
use sp_core::H256;
use sp_runtime::{traits::Hash};
use std::path::Path;


/***
    Multistep Call - puts_code, instantiates, calls and terminates wasm contract codes on the fly.
    Such a wasm code is called package.
    Consists of 3 execution phases:
        - Execute: Code results are stored on escrow account under corresponding to the call storage key.
        - Revert:  Code results are removed out of escrow account.
        - Commit:  Code results are moved from escrow account to target accounts.
***/

const EXECUTE_PHASE: u8 = 0;
const REVERT_PHASE: u8  = 1;
const COMMIT_PHASE: u8  = 2;

const ALICE: u64 = 1;
const BOB: u64 = 2;
const CHARLIE: u64 = 3;

fn default_multistep_call_args () -> (u8, Vec<u8>, Vec<u8>, BalanceOf<Test>, Gas) {
    let phase = 0 as u8;
    let code: Vec<u8> = Vec::new();
    let input_data: Vec<u8> = Vec::new();
    let value = BalanceOf::<Test>::from(500_000 as u64);
    let gas_limit: Gas = 201_000_000; // Actual gas costs of "return_from_start_fn" instantiation cost.
    return (phase, code, input_data, value, gas_limit);
}

#[test]
fn during_execution_phase_when_given_empty_wasm_code_multistep_call_gives_put_code_error() {
    let (phase, code, input_data, value, gas_limit) = default_multistep_call_args();

    new_test_ext().execute_with(|| {
        assert_noop!(
            EscrowGateway::multistep_call(Origin::signed(ALICE), phase, code, value, gas_limit, input_data),
            Error::<Test>::PutCodeFailure
        );
    });
}

#[test]
fn during_execution_phase_when_given_correct_wasm_code_but_too_little_gas_limit_multistep_call_gives_initiate_error() {
    let (phase, _, input_data, value, mut gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/return_from_start_fn.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();
    // Make the gas limit too little
    gas_limit = gas_limit - 1;

    new_test_ext_builder(50).execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 10_000_000_000);
        assert_err!(
            EscrowGateway::multistep_call(Origin::signed(ALICE), phase, correct_wasm_code, value, gas_limit, input_data),
            Error::<Test>::InitializationFailure
        );
    });
}

#[test]
fn during_execution_phase_when_given_correct_wasm_code_multistep_call_succeeds() {
    let (phase, _, input_data, value, gas_limit) = default_multistep_call_args();
    let correct_wasm_path = Path::new("src/fixtures/return_from_start_fn.wasm");
    let correct_wasm_code = load_contract_code(&correct_wasm_path).unwrap();

    new_test_ext_builder(50).execute_with(|| {

        let _ = Balances::deposit_creating(&ALICE, 10_000_000_000);

        assert_ok!(
            EscrowGateway::multistep_call(Origin::signed(ALICE), phase, correct_wasm_code, value, gas_limit, input_data)
        );
    });
}

/// Load the wasm blob from the specified path.
///
/// Defaults to the target contract wasm in the current project, inferred via the crate metadata.
fn load_contract_code(path: &Path) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut file = fs::File::open(path)
        .context(format!("Failed to open {}", path.display()))?;
    file.read_to_end(&mut data)?;

    Ok(data)
}

/// Load a given wasm module represented by a .wat file and returns a wasm binary contents along
/// with it's hash.
///
/// The fixture files are located under the `fixtures/` directory.
fn compile_module<T>(
    fixture_name: &str,
) -> wat::Result<(Vec<u8>, <T::Hashing as Hash>::Output)>
    where
        T: frame_system::Trait,
{
    let fixture_path = ["fixtures/", fixture_name, ".wat"].concat();
    let wasm_binary = wat::parse_file(fixture_path)?;
    let code_hash = T::Hashing::hash(&wasm_binary);
    Ok((wasm_binary, code_hash))
}
