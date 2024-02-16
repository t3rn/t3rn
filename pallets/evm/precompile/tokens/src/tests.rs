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
    ExtBuilder::default().build().execute_with(|| {
        precompiles()
            .prepare_test(
                sender_evm_addr(),
                trn_evm_address(),
                EvmDataWriter::new_with_selector(Action::TransferFrom).build(),
            )
            .expect_cost(0)
            .expect_no_logs()
            .execute_error(pallet_evm::ExitError::Other("Not Supported".into()))
    });
}
