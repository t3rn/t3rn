use crate::Action;
use circuit_mock_runtime::{
    contracts_config::PrecompilesValue, evm_precompile_util::Precompiles, *,
};
use hex_literal::hex;
use precompile_util_solidity::{
    data::{Address, Bytes, EvmDataWriter},
    testing::*,
};
use sp_core::{H160, U256};
use sp_runtime::traits::Zero;
use sp_std::boxed::Box;

fn precompiles() -> Precompiles<circuit_mock_runtime::Runtime> {
    PrecompilesValue::get()
}

#[test]
fn handles_non_supported() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::TransferFrom).build(),
            )
            .expect_cost(0)
            .expect_no_logs()
            .execute_error(pallet_evm::ExitError::Other("Not Supported".into()))
    });
}

#[test]
fn name_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Name).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(
                EvmDataWriter::new()
                    .write(Bytes::from("TRN".as_bytes()))
                    .build(),
            );
    });
}

#[test]
fn symbol_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Symbol).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(
                EvmDataWriter::new()
                    .write(Bytes::from("TRN".as_bytes()))
                    .build(),
            );
    });
}

#[test]
fn decimals_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Decimals).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(12)).build());
    });
}

#[test]
fn total_supply_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::TotalSupply).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(10_000_000)).build());
    });
}

#[ignore]
#[test]
fn balance_of_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::BalanceOf)
                    .write(Address::from(sender.address))
                    .build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(10_000_000)).build());
    });
}

#[ignore]
#[test]
fn transfer_works() {
    let (pairs, mut ext) = new_test_ext(2);
    let sender = &pairs[0];
    let receiver = &pairs[1];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Transfer)
                    .write(Address::from(receiver.address))
                    .write(U256::from(1000u64))
                    .build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(1u64).build());
    });
}
