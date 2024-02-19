use crate::Action;
use circuit_mock_runtime::{
    contracts_config::PrecompilesValue, evm_precompile_util::Precompiles, *,
};
use frame_support::{
    dispatch::RawOrigin,
    traits::fungibles::approvals::{Inspect, Mutate},
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
fn name_native_works() {
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
fn symbol_native_works() {
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
fn decimals_native_works() {
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
fn total_supply_native_works() {
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

#[test]
fn allowance_native_not_supported() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Allowance).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_error(pallet_evm::ExitError::Other("Not Supported".into()))
    });
}

#[test]
fn balance_of_native_works() {
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

#[test]
fn approve_native_not_supported() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Approve).build(),
            )
            .expect_cost(1756)
            .expect_no_logs()
            .execute_error(pallet_evm::ExitError::Other("Not Supported".into()))
    });
}

#[test]
fn transfer_from_native_not_supported() {
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
fn transfer_native_works() {
    let (pairs, mut ext) = new_test_ext(2);
    let sender = &pairs[0];
    let receiver = &pairs[1];
    ext.execute_with(|| {
        assert_eq!(
            circuit_mock_runtime::Balances::free_balance(&receiver.account_id),
            10_000_000
        );
        assert_eq!(
            circuit_mock_runtime::Balances::free_balance(&sender.account_id),
            10_000_000
        );

        precompiles()
            .prepare_test(
                sender.address,
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::Transfer)
                    .write(Address::from(receiver.address))
                    .write(U256::from(1_000_000u64))
                    .build(),
            )
            .expect_cost(1756)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(1u64).build());

        assert_eq!(
            circuit_mock_runtime::Balances::free_balance(&receiver.account_id),
            11_000_000
        );
        assert_eq!(
            circuit_mock_runtime::Balances::free_balance(&sender.account_id),
            9_000_000
        );
    });
}

#[test]
fn name_asset_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Name).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(
                EvmDataWriter::new()
                    .write(Bytes::from("TST".as_bytes()))
                    .build(),
            );
    });
}

#[test]
fn symbol_asset_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Symbol).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(
                EvmDataWriter::new()
                    .write(Bytes::from("TST".as_bytes()))
                    .build(),
            );
    });
}

#[test]
fn decimals_asset_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Decimals).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(18)).build());
    });
}

#[test]
fn total_supply_asset_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::TotalSupply).build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(10_000)).build());
    });
}

#[test]
fn allowance_asset_works() {
    let (pairs, mut ext) = new_test_ext(2);
    let owner = &pairs[0];
    let spender = &pairs[1];
    ext.execute_with(|| {
        circuit_mock_runtime::Assets::approve_transfer(
            RawOrigin::Signed(owner.account_id.clone()).into(),
            1u32.into(),
            spender.account_id.clone().into(),
            1000u128.into(),
        );

        precompiles()
            .prepare_test(
                owner.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Allowance)
                    .write(Address::from(owner.address))
                    .write(Address::from(spender.address))
                    .build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(1000)).build());
    });
}

#[test]
fn balance_of_asset_works() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::BalanceOf)
                    .write(Address::from(sender.address))
                    .build(),
            )
            .expect_cost(1250)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(U256::from(10_000)).build());
    });
}

#[test]
fn approve_asset_works() {
    let (pairs, mut ext) = new_test_ext(2);
    let owner = &pairs[0];
    let spender = &pairs[1];
    ext.execute_with(|| {
        assert_eq!(
            circuit_mock_runtime::Assets::allowance(1u32, &owner.account_id, &spender.account_id),
            0
        );
        precompiles()
            .prepare_test(
                owner.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Approve)
                    .write(Address::from(spender.address))
                    .write(U256::from(2000u64))
                    .build(),
            )
            .expect_cost(3006)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(1u64).build());

        assert_eq!(
            circuit_mock_runtime::Assets::allowance(1u32, &owner.account_id, &spender.account_id),
            2000
        );
    });
}

#[test]
fn transfer_from_asset_not_supported() {
    let (pairs, mut ext) = new_test_ext(1);
    let sender = &pairs[0];
    ext.execute_with(|| {
        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::TransferFrom).build(),
            )
            .expect_cost(0)
            .expect_no_logs()
            .execute_error(pallet_evm::ExitError::Other("Not Supported".into()))
    });
}

#[test]
fn transfer_asset_works() {
    let (pairs, mut ext) = new_test_ext(2);
    let sender = &pairs[0];
    let receiver = &pairs[1];
    ext.execute_with(|| {
        assert_eq!(
            circuit_mock_runtime::Assets::balance(1u32, &receiver.account_id),
            10000
        );
        assert_eq!(
            circuit_mock_runtime::Assets::balance(1u32, &sender.account_id),
            10000
        );

        precompiles()
            .prepare_test(
                sender.address,
                tst_evm_address(),
                EvmDataWriter::new_with_selector(Action::Transfer)
                    .write(Address::from(receiver.address))
                    .write(U256::from(1000u64))
                    .build(),
            )
            .expect_cost(1756)
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(1u64).build());

        assert_eq!(
            circuit_mock_runtime::Assets::balance(1u32, &receiver.account_id),
            11000
        );
        assert_eq!(
            circuit_mock_runtime::Assets::balance(1u32, &sender.account_id),
            9000
        );
    });
}
