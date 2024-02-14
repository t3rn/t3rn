#![cfg_attr(not(feature = "std"), no_std)]

use circuit_runtime_types::{AssetId, EvmAddress, TokenId};
use fp_evm::{
    ExitError, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileOutput, PrecompileResult,
};
use sp_core::{H160, U256};
use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::threevm::{Erc20Mapping, Precompile, H160_POSITION_ASSET_ID_TYPE};
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

pub struct TokensPrecompile<T>(PhantomData<T>);

impl<T> Erc20Mapping for TokensPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config,
{
    fn encode_evm_address(v: TokenId) -> Option<EvmAddress> {
        let mut address = [9u8; 20];
        let mut asset_id_bytes: Vec<u8> = 0u32.to_be_bytes().to_vec();
        match v {
            TokenId::Asset(id) => asset_id_bytes = id.to_be_bytes().to_vec(),
            _ => {},
        }

        for byte_index in 0..asset_id_bytes.len() {
            address[byte_index + H160_POSITION_ASSET_ID_TYPE] =
                asset_id_bytes.as_slice()[byte_index];
        }

        Some(EvmAddress::from_slice(&asset_id_bytes.as_slice()))
    }

    fn decode_evm_address(v: EvmAddress) -> Option<TokenId> {
        let address = v.as_bytes();
        let mut asset_id_bytes = [0u8; 4];
        for byte_index in H160_POSITION_ASSET_ID_TYPE..20 {
            asset_id_bytes[byte_index - H160_POSITION_ASSET_ID_TYPE] = address[byte_index];
        }
        let asset_id = u32::from_be_bytes(asset_id_bytes);
        if asset_id == 0 {
            return Some(TokenId::Native)
        }
        Some(TokenId::Asset(asset_id))
    }
}

impl<T> EvmPrecompile for TokensPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let address = handle.code_address();

        if let Some(asset_id) = <TokensPrecompile<T> as Erc20Mapping>::decode_evm_address(address) {
            let _target_gas = handle.gas_limit();
            let _context = handle.context();
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

            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Not Implemented".into()),
            })
        }
        Err(PrecompileFailure::Error {
            exit_status: ExitError::Other("Invalid Asset Id".into()),
        })
    }
}

impl<T> TokensPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config,
{
    fn not_supported(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Supported".into()),
        })
    }

    fn total_supply(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn name(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn symbol(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn decimals(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn balance_of(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();

        // Convert EVM address to Substrate address
        // T::AddressMapping::
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }

    fn transfer(token_id: AssetId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();
        // Convert EVM Address to Substrate
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Implemented".into()),
        })
    }
}
