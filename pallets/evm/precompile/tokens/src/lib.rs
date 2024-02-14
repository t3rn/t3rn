#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use pallet_3vm_account_mapping::EvmAddressMapping;
use sp_core::{H160, U256};
use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::threevm::{Erc20Mapping, Precompile};
//use precompile_util::FunctionModifier;

#[precompile_util_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
    TotalSupply = "totalSupply()",
    BalanceOf = "balanceOf(address)",
    Allowance = "allowance(address,address)",
    Transfer = "transfer(address,uint256)",
    Approve = "approve(address,uint256)",
    TransferFrom = "transferFrom(address,address,uint256)",
    Name = "name()",
    Symbol = "symbol()",
    Decimals = "decimals()",
}

pub struct TokensPrecompile<T: pallet_evm::Config>(PhantomData<T>);

impl<T: pallet_evm::Config> EvmPrecompile for TokensPrecompile<T> {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let address = handle.code_address();
        let input = handle.input();
        let _target_gas = handle.gas_limit();
        let _context = handle.context();
        // TODO: Read asset_id from precompile address using the Erc20Mapping function provieded in runtime
        // TODO: Read function selector
        // TODO: Implement Actions
        /*
        let result = {
            let selector = match handle.read_selector() {
                Ok(selector) => selector,
                Err(e) => return Err(e),
            };

            if let Err(err) = handle.check_function_modifier(match selector {
                Action::Approve | Action::Transfer | Action::TransferFrom => FunctionModifier::Payable,
                _ => FunctionModifier::View,
            }) {
                return Err(err);
            }

            match selector {
                // Local and Foreign common
                Action::TotalSupply => Self::total_supply(handle),
                Action::BalanceOf => Self::balance_of(handle),
                Action::Allowance => Self::not_supported(handle),
                Action::Transfer => Self::transfer(handle),
                Action::Approve => Self::not_supported(handle),
                Action::TransferFrom => Self::not_supported(handle),
                Action::Name => Self::name(handle),
                Action::Symbol => Self::symbol(handle),
                Action::Decimals => Self::decimals(handle),
            }
        };
        return result;
        */
        Err(PrecompileFailure::Error {
            exit_status: ExitError::Other("Not Implemented".into()),
        })
    }
}

impl<T: pallet_evm::Config> TokensPrecompile<T> {
    fn not_supported(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Supported".into()),
        })
    }

    fn total_supply(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn name(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn symbol(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn decimals(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn balance_of(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn transfer(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }
}
