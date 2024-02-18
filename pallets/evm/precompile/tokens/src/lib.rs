#![cfg_attr(not(feature = "std"), no_std)]
use circuit_runtime_types::{EvmAddress, TokenId};
use fp_evm::{
    ExitError, ExitRevert, Precompile as EvmPrecompile, PrecompileFailure, PrecompileHandle,
    PrecompileResult,
};

use frame_support::{
    sp_runtime::traits::StaticLookup,
    traits::{
        fungibles::{
            approvals::Inspect, metadata::Inspect as MetadataInspect, Inspect as IssuanceInspect,
        },
        tokens::currency::Currency,
        ExistenceRequirement, OriginTrait,
    },
};
use frame_system::RawOrigin;
use pallet_evm::AddressMapping;
use precompile_util_solidity::{
    data::{Address, Bytes, EvmData, EvmDataWriter},
    error,
    handle::PrecompileHandleExt,
    modifier::FunctionModifier,
    revert,
    substrate::RuntimeHelper,
    succeed, EvmResult,
};
use sp_core::{H160, U256};
use sp_std::{marker::PhantomData, str::*, vec::Vec};
use t3rn_primitives::threevm::{
    convert_decimals_from_evm, Erc20Mapping, Precompile, DECIMALS_VALUE,
    H160_POSITION_ASSET_ID_TYPE,
};

#[cfg(test)]
mod tests;

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
    T: pallet_evm::Config + pallet_assets::Config + frame_system::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
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
    T: pallet_evm::Config + pallet_assets::Config + frame_system::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let address = handle.code_address();

        if let Some(token_id) = <TokensPrecompile<T> as Erc20Mapping>::decode_evm_address(address) {
            let result = {
                let selector = match handle.read_selector() {
                    Ok(selector) => selector,
                    Err(e) => return Err(e),
                };

                if let Err(err) = handle.check_function_modifier(match selector {
                    Action::Approve | Action::Transfer | Action::TransferFrom =>
                        FunctionModifier::Payable,
                    _ => FunctionModifier::View,
                }) {
                    return Err(err)
                }

                match selector {
                    Action::TotalSupply => Self::total_supply(token_id, handle),
                    Action::BalanceOf => Self::balance_of(token_id, handle),
                    Action::Allowance => Self::allowance(token_id, handle),
                    Action::Transfer => Self::transfer(token_id, handle),
                    Action::Approve => Self::not_supported(handle),
                    Action::TransferFrom => Self::not_supported(handle),
                    Action::Name => Self::name(token_id, handle),
                    Action::Symbol => Self::symbol(token_id, handle),
                    Action::Decimals => Self::decimals(token_id, handle),
                }
            };
            return result
        }
        Err(PrecompileFailure::Error {
            exit_status: ExitError::Other("Invalid Asset Id".into()),
        })
    }
}

impl<T> TokensPrecompile<T>
where
    T: pallet_evm::Config + pallet_assets::Config + frame_system::Config,
    <T as pallet_assets::Config>::AssetId: From<u32>,
    <T as pallet_assets::Config>::AssetIdParameter: From<u32>,
    <T as pallet_assets::Config>::Balance: EvmData,
    <<T as pallet_evm::Config>::Currency as Currency<
        <T as frame_system::pallet::Config>::AccountId,
    >>::Balance: EvmData,
    sp_core::U256: From<<T as pallet_assets::Config>::Balance>,
    sp_core::U256: From<
        <<T as pallet_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance,
    >,
{
    fn not_supported(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Supported".into()),
        })
    }

    fn total_supply(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        match token_id {
            TokenId::Native => {
                let native_total_issuance = <T as pallet_evm::Config>::Currency::total_issuance();
                Ok(succeed(
                    EvmDataWriter::new()
                        .write(U256::from(native_total_issuance))
                        .build(),
                ))
            },
            TokenId::Asset(asset_id) => {
                let asset_total_issuance =
                    pallet_assets::Pallet::<T>::total_issuance(asset_id.into());
                Ok(succeed(
                    EvmDataWriter::new()
                        .write(U256::from(asset_total_issuance))
                        .build(),
                ))
            },
        }
    }

    fn name(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        match token_id {
            TokenId::Native => Ok(succeed(
                EvmDataWriter::new()
                    .write(Bytes::from("TRN".as_bytes()))
                    .build(),
            )),
            TokenId::Asset(asset_id) => {
                let asset_name = pallet_assets::Pallet::<T>::name(asset_id.into());
                Ok(succeed(
                    EvmDataWriter::new()
                        .write(Bytes::from(asset_name.as_slice()))
                        .build(),
                ))
            },
        }
    }

    fn symbol(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        match token_id {
            TokenId::Native => Ok(succeed(
                EvmDataWriter::new()
                    .write(Bytes::from("TRN".as_bytes()))
                    .build(),
            )),
            TokenId::Asset(asset_id) => {
                let asset_symbol = pallet_assets::Pallet::<T>::symbol(asset_id.into());
                Ok(succeed(
                    EvmDataWriter::new()
                        .write(Bytes::from(asset_symbol.as_slice()))
                        .build(),
                ))
            },
        }
    }

    fn decimals(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        match token_id {
            TokenId::Native => Ok(succeed(EvmDataWriter::new().write(U256::from(12)).build())),
            TokenId::Asset(asset_id) => {
                let asset_decimals = pallet_assets::Pallet::<T>::decimals(asset_id.into());
                Ok(succeed(
                    EvmDataWriter::new()
                        .write(U256::from(asset_decimals))
                        .build(),
                ))
            },
        }
    }

    fn allowance(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        if let TokenId::Asset(asset_id) = token_id {
            let input = handle.input();

            // Parse input
            let mut input = handle.read_input()?;
            input.expect_arguments(2)?;

            let owner: H160 = input.read::<Address>()?.into();
            let spender: H160 = input.read::<Address>()?.into();

            let amount: U256 = {
                let owner = <T as pallet_evm::Config>::AddressMapping::into_account_id(owner);
                let spender = <T as pallet_evm::Config>::AddressMapping::into_account_id(spender);

                pallet_assets::Pallet::<T>::allowance(asset_id.into(), &owner, &spender).into()
            };
            return Ok(succeed(EvmDataWriter::new().write(amount).build()))
        }
        Err(PrecompileFailure::Error {
            exit_status: pallet_evm::ExitError::Other("Not Supported".into()),
        })
    }

    fn balance_of(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult {
        handle.record_cost(RuntimeHelper::<T>::db_read_gas_cost())?;

        let input = handle.input();

        // Parse input
        let mut input = handle.read_input()?;
        input.expect_arguments(1)?;

        let owner: H160 = input.read::<Address>()?.into();

        // Convert EVM address to Substrate address
        let who = <T as pallet_evm::Config>::AddressMapping::into_account_id(owner);
        let mut who_balance: U256 = U256::zero();
        match token_id {
            TokenId::Native => {
                who_balance = U256::from(<T as pallet_evm::Config>::Currency::free_balance(&who));
                //if who_balance != U256::zero() {
                //    who_balance = who_balance.checked_div(U256::from(DECIMALS_VALUE))
                //}
            },
            TokenId::Asset(asset_id) => {
                who_balance =
                    U256::from(pallet_assets::Pallet::<T>::balance(asset_id.into(), &who));
            },
        };
        Ok(succeed(EvmDataWriter::new().write(who_balance).build()))
    }

    fn transfer(token_id: TokenId, handle: &mut impl PrecompileHandle) -> PrecompileResult
    where
        <T as pallet_assets::Config>::Balance: EvmData,
        <<T as pallet_evm::Config>::Currency as Currency<
            <T as frame_system::pallet::Config>::AccountId,
        >>::Balance: EvmData,
    {
        handle.record_log_costs_manual(3, 32)?;
        // Parse input
        let mut input = handle.read_input()?;
        input.expect_arguments(2)?;
        let to: H160 = input.read::<Address>()?.into();

        // Convert EVM address to Substrate address
        let origin =
            <T as pallet_evm::Config>::AddressMapping::into_account_id(handle.context().caller);
        let to = <T as pallet_evm::Config>::AddressMapping::into_account_id(to);

        // Get transfer amount value from input and transfer assets using
        // either pallet_evm::Config::Currnecy::transfer or pallet_assets::transfer
        match token_id {
            TokenId::Native => {
                let value: <<T as pallet_evm::Config>::Currency as Currency<
                    <T as frame_system::pallet::Config>::AccountId,
                >>::Balance = input
                    .read::<<<T as pallet_evm::Config>::Currency as Currency<
                        <T as frame_system::pallet::Config>::AccountId,
                    >>::Balance>()?
                    .into();

                <T as pallet_evm::Config>::Currency::transfer(
                    &origin,
                    &to,
                    value,
                    ExistenceRequirement::KeepAlive,
                )
                .map_err(|e| PrecompileFailure::Revert {
                    exit_status: ExitRevert::Reverted,
                    output: Into::<&str>::into(e).as_bytes().to_vec(),
                })?;
            },
            TokenId::Asset(asset_id) => {
                let value: <T as pallet_assets::Config>::Balance = input
                    .read::<<T as pallet_assets::Config>::Balance>()?
                    .into();

                pallet_assets::Pallet::<T>::transfer(
                    RawOrigin::Signed(origin).into(),
                    asset_id.into(),
                    <T as frame_system::Config>::Lookup::unlookup(to),
                    value,
                )
                .map_err(|e| PrecompileFailure::Revert {
                    exit_status: ExitRevert::Reverted,
                    output: Into::<&str>::into(e).as_bytes().to_vec(),
                })?;
            },
        };
        Ok(succeed(EvmDataWriter::new().write(true).build()))
    }
}
