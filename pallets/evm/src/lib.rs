// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # EVM Pallet
//!
//! The EVM pallet allows unmodified EVM code to be executed in a Substrate-based blockchain.
//! - [`evm::Config`]
//!
//! ## EVM Engine
//!
//! The EVM pallet uses [`SputnikVM`](https://github.com/rust-blockchain/evm) as the underlying EVM engine.
//! The engine is overhauled so that it's [`modular`](https://github.com/corepaper/evm).
//!
//! ## Execution Lifecycle
//!
//! There are a separate set of accounts managed by the EVM pallet. Substrate based accounts can call the EVM Pallet
//! to deposit or withdraw balance from the Substrate base-currency into a different balance managed and used by
//! the EVM pallet. Once a user has populated their balance, they can create and call smart contracts using this pallet.
//!
//! There's one-to-one mapping from Substrate accounts and EVM external accounts that is defined by a conversion function.
//!
//! ## EVM Pallet vs Ethereum Network
//!
//! The EVM pallet should be able to produce nearly identical results compared to the Ethereum mainnet,
//! including gas cost and balance changes.
//!
//! Observable differences include:
//!
//! - The available length of block hashes may not be 256 depending on the configuration of the System pallet
//! in the Substrate runtime.
//! - Difficulty and coinbase, which do not make sense in this pallet and is currently hard coded to zero.
//!
//! We currently do not aim to make unobservable behaviors, such as state root, to be the same. We also don't aim to follow
//! the exact same transaction / receipt format. However, given one Ethereum transaction and one Substrate account's
//! private key, one should be able to convert any Ethereum transaction into a transaction compatible with this pallet.
//!
//! The gas configurations are configurable. Right now, a pre-defined London hard fork configuration option is provided.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(more_qualified_paths)]
#![feature(associated_type_defaults)]

pub mod benchmarking;

#[cfg(test)]
pub mod mock;

pub mod runner;
#[cfg(test)]
pub mod tests;

use codec::{Decode, Encode};
pub use evm::{
    Config as EvmConfig, Context, ExitError, ExitFatal, ExitReason, ExitRevert, ExitSucceed,
};
pub use fp_evm::{
    Account, CallInfo, CreateInfo, ExecutionInfo, LinearCostPrecompile, Log, Precompile,
    PrecompileFailure, PrecompileOutput, PrecompileResult, PrecompileSet, Vicinity,
};
use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    pallet_prelude::IsType,
    traits::{
        tokens::fungible::Inspect, Currency, ExistenceRequirement, FindAuthor, Get, Imbalance,
        OnUnbalanced, SignedImbalance, WithdrawReasons,
    },
    weights::{Pays, PostDispatchInfo, Weight},
};
use frame_system::RawOrigin;
use scale_info::TypeInfo;
use sp_core::{H160, H256, U256};
use sp_runtime::{
    traits::{BadOrigin, Saturating, UniqueSaturatedInto, Zero},
    AccountId32,
};
use sp_std::vec::Vec;
use t3rn_primitives::{
    contract_metadata::ContractType, contracts_registry::AuthorInfo, threevm::ModuleOperations,
};

#[cfg(feature = "std")]
use fp_evm::GenesisAccount;

pub use self::{pallet::*, runner::Runner};

pub type CurrencyOf<T> = <T as pallet::Config>::Currency;

pub const REG_OPCODE_PREFIX: &[u8; 4] = b"reg:";

#[derive(Clone, Encode, Decode, TypeInfo, Default)]
#[scale_info(skip_type_params(T))]
pub struct ThreeVmInfo<T: Config> {
    author: AuthorInfo<T::AccountId, BalanceOf<T>>,
    kind: ContractType,
}

impl<T: Config> ModuleOperations<T, BalanceOf<T>> for ThreeVmInfo<T> {
    fn get_bytecode(&self) -> &Vec<u8> {
        todo!("noop")
    }

    fn get_author(&self) -> Option<&AuthorInfo<T::AccountId, BalanceOf<T>>> {
        Some(&self.author)
    }

    fn set_author(&mut self, author: AuthorInfo<T::AccountId, BalanceOf<T>>) {
        self.author = author;
    }

    fn get_type(&self) -> &ContractType {
        &self.kind
    }

    fn set_type(&mut self, kind: ContractType) {
        self.kind = kind;
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use fp_evm::FeeCalculator;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::DispatchErrorWithPostInfo;
    use t3rn_primitives::threevm::ThreeVm;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Calculator for current gas price.
        type FeeCalculator: FeeCalculator;

        /// Maps Ethereum gas to Substrate weight.
        type GasWeightMapping: GasWeightMapping;

        /// Block number to block hash.
        type BlockHashMapping: BlockHashMapping;

        /// Allow the origin to call on behalf of given address.
        type CallOrigin: EnsureAddressOrigin<Self::Origin>;
        /// Allow the origin to withdraw on behalf of given address.
        type WithdrawOrigin: EnsureAddressOrigin<Self::Origin, Success = Self::AccountId>;

        /// Mapping from address to account id.
        type AddressMapping: AddressMapping<Self::AccountId>;
        /// Currency type for withdraw and balance storage.
        type Currency: Currency<Self::AccountId> + Inspect<Self::AccountId>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Precompiles associated with this EVM engine.
        type PrecompilesType: PrecompileSet;
        type PrecompilesValue: Get<Self::PrecompilesType>;
        /// Chain ID of EVM.
        type ChainId: Get<u64>;
        /// The block gas limit. Can be a simple constant, or an adjustment algorithm in another pallet.
        type BlockGasLimit: Get<U256>;
        /// EVM execution runner.
        type Runner: Runner<Self>;

        /// To handle fee deduction for EVM transactions. An example is this pallet being used by `pallet_ethereum`
        /// where the chain implementing `pallet_ethereum` should be able to configure what happens to the fees
        /// Similar to `OnChargeTransaction` of `pallet_transaction_payment`
        type OnChargeTransaction: OnChargeEVMTransaction<Self>;

        /// Find author for the current block.
        type FindAuthor: FindAuthor<H160>;

        /// Make this pallet 3VM enabled
        type ThreeVm: ThreeVm<Self, BalanceOf<Self>>;

        /// EVM config used in the module.
        fn config() -> &'static EvmConfig {
            &LONDON_CONFIG
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Withdraw balance from EVM into currency/balances pallet.
        #[pallet::weight(0)]
        pub fn withdraw(
            origin: OriginFor<T>,
            address: H160,
            value: BalanceOf<T>,
        ) -> DispatchResult {
            let dest = ensure_signed(origin)?;
            let address_account_id = T::AddressMapping::get_or_into_account_id(&address);

            T::Currency::transfer(
                &address_account_id,
                &dest,
                value,
                ExistenceRequirement::AllowDeath,
            )?;

            Ok(())
        }

        /// Issue an EVM call operation. This is similar to a message call transaction in Ethereum.
        #[pallet::weight(T::GasWeightMapping::gas_to_weight(*gas_limit))]
        pub fn call(
            origin: OriginFor<T>,
            target: H160,
            input: Vec<u8>,
            value: U256,
            gas_limit: u64,
            max_fee_per_gas: U256,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            access_list: Vec<(H160, Vec<H256>)>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let source = T::AddressMapping::get_or_create_evm_address(&who);

            let is_transactional = true;
            let info = T::Runner::call(
                source,
                target,
                input,
                value,
                gas_limit,
                Some(max_fee_per_gas),
                max_priority_fee_per_gas,
                nonce,
                access_list,
                is_transactional,
                T::config(),
            )?;

            match info.exit_reason {
                ExitReason::Succeed(_) => {
                    Pallet::<T>::deposit_event(Event::<T>::Executed(target));
                },
                _ => {
                    Pallet::<T>::deposit_event(Event::<T>::ExecutedFailed(target));
                },
            };

            Ok(PostDispatchInfo {
                actual_weight: Some(T::GasWeightMapping::gas_to_weight(
                    info.used_gas.unique_saturated_into(),
                )),
                pays_fee: Pays::No,
            })
        }

        /// Issue an EVM create operation. This is similar to a contract creation transaction in
        /// Ethereum.
        #[pallet::weight(T::GasWeightMapping::gas_to_weight(*gas_limit))]
        pub fn create(
            origin: OriginFor<T>,
            init: Vec<u8>,
            value: U256,
            gas_limit: u64,
            max_fee_per_gas: U256,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            access_list: Vec<(H160, Vec<H256>)>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let source = T::AddressMapping::get_or_create_evm_address(&who);

            let is_transactional = true;
            let info = T::Runner::create(
                source,
                init,
                value,
                gas_limit,
                Some(max_fee_per_gas),
                max_priority_fee_per_gas,
                nonce,
                access_list,
                is_transactional,
                T::config(),
            )?;

            match info {
                CreateInfo {
                    exit_reason: ExitReason::Succeed(_),
                    value: create_address,
                    ..
                } => {
                    Pallet::<T>::deposit_event(Event::<T>::Created(create_address));
                },
                CreateInfo {
                    exit_reason: _,
                    value: create_address,
                    ..
                } => {
                    Pallet::<T>::deposit_event(Event::<T>::CreatedFailed(create_address));
                },
            }

            Ok(PostDispatchInfo {
                actual_weight: Some(T::GasWeightMapping::gas_to_weight(
                    info.used_gas.unique_saturated_into(),
                )),
                pays_fee: Pays::No,
            })
        }

        /// Issue an EVM create2 operation.
        #[pallet::weight(T::GasWeightMapping::gas_to_weight(*gas_limit))]
        pub fn create2(
            origin: OriginFor<T>,
            init: Vec<u8>,
            salt: H256,
            value: U256,
            gas_limit: u64,
            max_fee_per_gas: U256,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            access_list: Vec<(H160, Vec<H256>)>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let source = T::AddressMapping::get_or_create_evm_address(&who);

            let is_transactional = true;
            let info = T::Runner::create2(
                source,
                init,
                salt,
                value,
                gas_limit,
                Some(max_fee_per_gas),
                max_priority_fee_per_gas,
                nonce,
                access_list,
                is_transactional,
                T::config(),
            )?;

            match info {
                CreateInfo {
                    exit_reason: ExitReason::Succeed(_),
                    value: create_address,
                    ..
                } => {
                    Pallet::<T>::deposit_event(Event::<T>::Created(create_address));
                    // TODO: check how this relates to pre-post dispatch gas
                    Ok(PostDispatchInfo {
                        actual_weight: Some(T::GasWeightMapping::gas_to_weight(
                            info.used_gas.unique_saturated_into(),
                        )),
                        pays_fee: Pays::No,
                    })
                },
                CreateInfo {
                    exit_reason: _,
                    value: create_address,
                    ..
                } => {
                    Pallet::<T>::deposit_event(Event::<T>::CreatedFailed(create_address));
                    // TODO: check how this relates to pre-post dispatch gas
                    let post_info = PostDispatchInfo {
                        actual_weight: Some(T::GasWeightMapping::gas_to_weight(
                            info.used_gas.unique_saturated_into(),
                        )),
                        pays_fee: Pays::No,
                    };
                    Err(DispatchErrorWithPostInfo {
                        post_info,
                        error: Error::<T>::CreatedFailed.into(),
                    })
                },
            }
        }

        /// Claim an evm address, used to claim an evm address that is compatible with EVM.
        #[pallet::weight(T::DbWeight::get().reads_writes(1 as Weight, 2 as Weight))]
        pub fn claim(origin: OriginFor<T>) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            T::AddressMapping::get_or_create_evm_address(&origin);
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Ethereum events from contracts.
        Log(Log),
        /// A contract has been created at given \[address\].
        Created(H160),
        /// A \[contract\] was attempted to be created, but the execution failed.
        CreatedFailed(H160),
        /// A \[contract\] has been executed successfully with states applied.
        Executed(H160),
        /// A \[contract\] has been executed with errors. States are reverted with only gas fees applied.
        ExecutedFailed(H160),
        /// A deposit has been made at a given address. \[sender, address, value\]
        BalanceDeposit(T::AccountId, H160, U256),
        /// A withdrawal has been made from a given address. \[sender, address, value\]
        BalanceWithdraw(T::AccountId, H160, U256),
        ClaimAccount {
            account_id: T::AccountId,
            evm_address: H160,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not enough balance to perform action
        BalanceLow,
        /// Calculating total fee overflowed
        FeeOverflow,
        /// Calculating total payment overflowed
        PaymentOverflow,
        /// Withdraw fee failed
        WithdrawFailed,
        /// Gas price is too low.
        GasPriceTooLow,
        /// Nonce is invalid
        InvalidNonce,
        /// Tried to instantiate a contract with an invalid hash
        InvalidRegistryHash,
        /// 3VM failed to remunerate author
        RemunerateAuthor,
        ExecutedFailed,
        CreatedFailed,
    }

    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {
        pub accounts: std::collections::BTreeMap<H160, GenesisAccount>,
    }

    #[cfg(feature = "std")]
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            for (address, account) in &self.accounts {
                let account_id = T::AddressMapping::get_or_into_account_id(address);

                // ASSUME: in one single EVM transaction, the nonce will not increase more than
                // `u128::max_value()`.
                for _ in 0..account.nonce.low_u128() {
                    frame_system::Pallet::<T>::inc_account_nonce(&account_id);
                }

                T::Currency::deposit_creating(
                    &account_id,
                    account.balance.low_u128().unique_saturated_into(),
                );

                Pallet::<T>::create_account(*address, account.code.clone());

                for (index, value) in &account.storage {
                    <AccountStorages<T>>::insert(address, index, value);
                }
            }
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn account_codes)]
    pub type AccountCodes<T: Config> = StorageMap<_, Blake2_128Concat, H160, Vec<u8>, ValueQuery>;

    /// The storages for EVM contracts.
    ///
    /// AccountStorages: double_map EvmAddress, H256 => H256
    #[pallet::storage]
    #[pallet::getter(fn account_storages)]
    pub type AccountStorages<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, H160, Blake2_128Concat, H256, H256, ValueQuery>;

    // TODO: genesis
    // TODO: ensure removal
    #[pallet::storage]
    #[pallet::getter(fn account_3vm_info)]
    pub type Account3vmInfo<T: Config> =
        StorageMap<_, Blake2_128Concat, H160, ThreeVmInfo<T>, OptionQuery>;

    #[pallet::storage]
    pub type EvmAccountAddressMapping<T: Config> =
        StorageMap<_, Blake2_128Concat, H160, T::AccountId, OptionQuery>;

    #[pallet::storage]
    pub type AccountEvmAddressMapping<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, H160, OptionQuery>;
}

/// Type alias for currency balance.
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Type alias for negative imbalance during fees
type NegativeImbalanceOf<C, T> =
    <C as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub trait EnsureAddressOrigin<OuterOrigin> {
    /// Success return type.
    type Success;

    /// Perform the origin check.
    fn ensure_address_origin(
        address: &H160,
        origin: OuterOrigin,
    ) -> Result<Self::Success, BadOrigin> {
        Self::try_address_origin(address, origin).map_err(|_| BadOrigin)
    }

    /// Try with origin.
    fn try_address_origin(
        address: &H160,
        origin: OuterOrigin,
    ) -> Result<Self::Success, OuterOrigin>;
}

/// Ensure that the origin is root.
pub struct EnsureAddressRoot<AccountId>(sp_std::marker::PhantomData<AccountId>);

impl<OuterOrigin, AccountId> EnsureAddressOrigin<OuterOrigin> for EnsureAddressRoot<AccountId>
where
    OuterOrigin: Into<Result<RawOrigin<AccountId>, OuterOrigin>> + From<RawOrigin<AccountId>>,
{
    type Success = ();

    fn try_address_origin(_address: &H160, origin: OuterOrigin) -> Result<(), OuterOrigin> {
        origin.into().and_then(|o| match o {
            RawOrigin::Root => Ok(()),
            r => Err(OuterOrigin::from(r)),
        })
    }
}

/// Ensure that the origin never happens.
pub struct EnsureAddressNever<AccountId>(sp_std::marker::PhantomData<AccountId>);

impl<OuterOrigin, AccountId> EnsureAddressOrigin<OuterOrigin> for EnsureAddressNever<AccountId> {
    type Success = AccountId;

    fn try_address_origin(_address: &H160, origin: OuterOrigin) -> Result<AccountId, OuterOrigin> {
        Err(origin)
    }
}

/// Ensure that the address is truncated hash of the origin. Only works if the account id is
/// `AccountId32`.
pub struct EnsureAddressTruncated;

impl<OuterOrigin> EnsureAddressOrigin<OuterOrigin> for EnsureAddressTruncated
where
    OuterOrigin: Into<Result<RawOrigin<AccountId32>, OuterOrigin>> + From<RawOrigin<AccountId32>>,
{
    type Success = AccountId32;

    fn try_address_origin(address: &H160, origin: OuterOrigin) -> Result<AccountId32, OuterOrigin> {
        origin.into().and_then(|o| match o {
            RawOrigin::Signed(who) if AsRef::<[u8; 32]>::as_ref(&who)[0..20] == address[0..20] =>
                Ok(who),
            r => Err(OuterOrigin::from(r)),
        })
    }
}

/// Just ensure that the origin is signed or root, only useful for testing
#[cfg(test)]
pub struct EnsureSigned<T: frame_system::Config>(sp_std::marker::PhantomData<T::AccountId>);

#[cfg(test)]
impl<OuterOrigin, T: frame_system::Config> EnsureAddressOrigin<OuterOrigin> for EnsureSigned<T>
where
    OuterOrigin: Into<Result<RawOrigin<T::AccountId>, OuterOrigin>> + From<RawOrigin<T::AccountId>>,
{
    type Success = Option<T::AccountId>;

    fn try_address_origin(
        _address: &H160,
        origin: OuterOrigin,
    ) -> Result<Self::Success, OuterOrigin> {
        origin.into().and_then(|o| match o {
            RawOrigin::Signed(who) => Ok(Some(who)),
            RawOrigin::Root => Ok(None),
            r => Err(OuterOrigin::from(r)),
        })
    }
}

pub trait AddressMapping<AccountId> {
    // Returns the AccountId used to generate the given EvmAddress.
    fn get_or_into_account_id(address: &H160) -> AccountId;
    /// Returns the EvmAddress associated with a given AccountId or the
    /// underlying EvmAddress of the AccountId.
    /// Returns None if there is no EvmAddress associated with the AccountId
    /// and there is no underlying EvmAddress in the AccountId.
    fn get_evm_address(account_id: &AccountId) -> Option<H160>;
    /// Returns the EVM address associated with an account ID and generates an
    /// account mapping if no association exists.
    fn get_or_create_evm_address(account_id: &AccountId) -> H160;
}

// Creates a an EvmAddress from an AccountId by appending the bytes "evm:" to
// the account_id and hashing it.
fn account_to_default_evm_address(account_id: &impl Encode) -> H160 {
    let payload = (b"evm:", account_id);
    H160::from_slice(&payload.using_encoded(sp_io::hashing::blake2_256)[0..20])
}

/// Hashed address mapping.
pub struct StoredHashAddressMapping<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> AddressMapping<T::AccountId> for StoredHashAddressMapping<T>
where
    T::AccountId: IsType<AccountId32>,
{
    fn get_or_into_account_id(address: &H160) -> T::AccountId {
        if let Some(acc) = EvmAccountAddressMapping::<T>::get(address) {
            acc
        } else {
            let mut data: [u8; 32] = [0u8; 32];
            data[0..4].copy_from_slice(b"evm:");
            data[4..24].copy_from_slice(&address[..]);
            AccountId32::from(data).into()
        }
    }

    fn get_evm_address(account_id: &T::AccountId) -> Option<H160> {
        // Return the EvmAddress if a mapping to account_id exists
        AccountEvmAddressMapping::<T>::get(account_id).or_else(|| {
            let data: &[u8] = account_id.into_ref().as_ref();
            // Return the underlying EVM address if it exists otherwise return None
            if data.starts_with(b"evm:") {
                Some(H160::from_slice(&data[4..24]))
            } else {
                None
            }
        })
    }

    fn get_or_create_evm_address(account_id: &T::AccountId) -> H160 {
        Self::get_evm_address(account_id).unwrap_or_else(|| {
            let addr = account_to_default_evm_address(account_id);

            // create reverse mapping
            EvmAccountAddressMapping::<T>::insert(addr, account_id);
            AccountEvmAddressMapping::<T>::insert(account_id, addr);

            Pallet::<T>::deposit_event(Event::ClaimAccount {
                account_id: account_id.clone(),
                evm_address: addr,
            });

            addr
        })
    }
}

/// A trait for getting a block hash by number.
pub trait BlockHashMapping {
    fn block_hash(number: u32) -> H256;
}

/// Returns the Substrate block hash by number.
pub struct SubstrateBlockHashMapping<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> BlockHashMapping for SubstrateBlockHashMapping<T> {
    fn block_hash(number: u32) -> H256 {
        let number = T::BlockNumber::from(number);
        H256::from_slice(frame_system::Pallet::<T>::block_hash(number).as_ref())
    }
}

/// A mapping function that converts Ethereum gas to Substrate weight
pub trait GasWeightMapping {
    fn gas_to_weight(gas: u64) -> Weight;
    fn weight_to_gas(weight: Weight) -> u64;
}

impl GasWeightMapping for () {
    fn gas_to_weight(gas: u64) -> Weight {
        gas as Weight
    }

    fn weight_to_gas(weight: Weight) -> u64 {
        weight
    }
}

static LONDON_CONFIG: EvmConfig = EvmConfig::london();

impl<T: Config> Pallet<T> {
    /// Check whether an account is empty.
    pub fn is_account_empty(address: &H160) -> bool {
        let account = Self::account_basic(address);
        let code_len = <AccountCodes<T>>::decode_len(address).unwrap_or(0);

        account.nonce == U256::zero() && account.balance == U256::zero() && code_len == 0
    }

    /// Remove an account if its empty.
    pub fn remove_account_if_empty(address: &H160) {
        if Self::is_account_empty(address) {
            Self::remove_account(address);
        }
    }

    /// Remove an account.
    pub fn remove_account(address: &H160) {
        if <AccountCodes<T>>::contains_key(address) {
            let account_id = T::AddressMapping::get_or_into_account_id(address);
            let _ = frame_system::Pallet::<T>::dec_sufficients(&account_id);
        }

        <AccountCodes<T>>::remove(address);
        <AccountStorages<T>>::clear_prefix(address, 0, None);
    }

    /// Create an account.
    pub fn create_account(address: H160, code: Vec<u8>) {
        if code.is_empty() {
            return
        }

        if !<AccountCodes<T>>::contains_key(address) {
            let account_id = T::AddressMapping::get_or_into_account_id(&address);
            let _ = frame_system::Pallet::<T>::inc_sufficients(&account_id);
        }

        <AccountCodes<T>>::insert(address, code);
    }

    /// Get the account basic in EVM format.
    pub fn account_basic(address: &H160) -> Account {
        let account_id = T::AddressMapping::get_or_into_account_id(address);

        let nonce = frame_system::Pallet::<T>::account_nonce(&account_id);
        // keepalive `true` takes into account ExistentialDeposit as part of what's considered liquid balance.
        let balance = T::Currency::reducible_balance(&account_id, true);

        Account {
            nonce: U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(nonce)),
            balance: U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(balance)),
        }
    }

    /// Get the author using the FindAuthor trait.
    pub fn find_author() -> H160 {
        let digest = <frame_system::Pallet<T>>::digest();
        let pre_runtime_digests = digest.logs.iter().filter_map(|d| d.as_pre_runtime());

        T::FindAuthor::find_author(pre_runtime_digests).unwrap_or_default()
    }

    pub fn get_threevm_info(address: &H160) -> Option<(T::AccountId, BalanceOf<T>, u8)> {
        Account3vmInfo::<T>::get(address).map(|info| {
            (
                info.author.account,
                info.author.fees_per_single_use.unwrap_or_default(),
                info.kind.into(),
            )
        })
    }

    pub fn get_account_code(address: &H160) -> Vec<u8> {
        AccountCodes::<T>::get(address)
    }
}

/// Handle withdrawing, refunding and depositing of transaction fees.
/// Similar to `OnChargeTransaction` of `pallet_transaction_payment`
pub trait OnChargeEVMTransaction<T: Config> {
    type LiquidityInfo: Default;

    /// Before the transaction is executed the payment of the transaction fees
    /// need to be secured.
    fn withdraw_fee(
        who: &H160,
        fee: U256,
        author: Option<&T::AccountId>,
    ) -> Result<Self::LiquidityInfo, Error<T>>;

    /// After the transaction was executed the actual fee can be calculated.
    /// This function should refund any overpaid fees and optionally deposit
    /// the corrected amount.
    fn correct_and_deposit_fee(
        who: &H160,
        corrected_fee: U256,
        already_withdrawn: Self::LiquidityInfo,
    );

    /// Introduced in EIP1559 to handle the priority tip payment to the block Author.
    fn pay_priority_fee(tip: U256);
}

/// Implements the transaction payment for a pallet implementing the `Currency`
/// trait (eg. the pallet_balances) using an unbalance handler (implementing
/// `OnUnbalanced`).
/// Similar to `CurrencyAdapter` of `pallet_transaction_payment`
/// /// TODO: move this to account manager
pub struct ThreeVMCurrencyAdapter<C, OU>(sp_std::marker::PhantomData<(C, OU)>);

type EvmAccountImbalanceLiquidityInfo<T, C> = (
    Option<<T as frame_system::Config>::AccountId>,
    Option<NegativeImbalanceOf<C, T>>,
);

impl<T, C, OU> OnChargeEVMTransaction<T> for ThreeVMCurrencyAdapter<C, OU>
where
    T: Config,
    C: Currency<<T as frame_system::Config>::AccountId>,
    C::PositiveImbalance: Imbalance<
        <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
        Opposite = C::NegativeImbalance,
    >,
    C::NegativeImbalance: Imbalance<
        <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
        Opposite = C::PositiveImbalance,
    >,
    OU: OnUnbalanced<NegativeImbalanceOf<C, T>>,
{
    // Kept type as Option to satisfy bound of Default
    type LiquidityInfo = EvmAccountImbalanceLiquidityInfo<T, C>;

    fn withdraw_fee(
        who: &H160,
        fee: U256,
        author: Option<&T::AccountId>,
    ) -> Result<Self::LiquidityInfo, Error<T>> {
        if fee.is_zero() {
            return Ok((None, None))
        }
        log::info!("withdraw_fee: {:?} {:?}", who, fee);
        let account_id = T::AddressMapping::get_or_into_account_id(who);
        log::info!("withdraw_fee: {:?} {:?}", account_id, fee);
        let imbalance = C::withdraw(
            &account_id,
            fee.low_u128().unique_saturated_into(),
            WithdrawReasons::FEE,
            ExistenceRequirement::AllowDeath,
        )
        .map_err(|_| Error::<T>::BalanceLow)?;

        if let Some(author) = author {
            Ok((Some(author.clone()), Some(imbalance)))
        } else {
            Ok((None, Some(imbalance)))
        }
    }

    fn correct_and_deposit_fee(
        who: &H160,
        corrected_fee: U256,
        already_withdrawn: Self::LiquidityInfo,
    ) {
        if let (beneficiary, Some(paid)) = already_withdrawn {
            let account_id = T::AddressMapping::get_or_into_account_id(who);

            // Calculate how much refund we should return
            let refund_amount = paid
                .peek()
                .saturating_sub(corrected_fee.low_u128().unique_saturated_into());
            // refund to the account that paid the fees. If this fails, the
            // account might have dropped below the existential balance. In
            // that case we don't refund anything.
            let refund_imbalance = C::deposit_into_existing(&account_id, refund_amount)
                .unwrap_or_else(|_| C::PositiveImbalance::zero());

            // Make sure this works with 0 ExistentialDeposit
            // https://github.com/paritytech/substrate/issues/10117
            // If we tried to refund something, the account still empty and the ED is set to 0,
            // we call `make_free_balance_be` with the refunded amount.
            let refund_imbalance = if C::minimum_balance().is_zero()
                && refund_amount > C::Balance::zero()
                && C::total_balance(&account_id).is_zero()
            {
                // Known bug: Substrate tried to refund to a zeroed AccountData, but
                // interpreted the account to not exist.
                match C::make_free_balance_be(&account_id, refund_amount) {
                    SignedImbalance::Positive(p) => p,
                    _ => C::PositiveImbalance::zero(),
                }
            } else {
                refund_imbalance
            };

            // merge the imbalance caused by paying the fees and refunding parts of it again.
            let adjusted_paid = paid
                .offset(refund_imbalance)
                .same()
                .unwrap_or_else(|_| C::NegativeImbalance::zero());

            // Since there is a beneficiary, account-manager handles it
            if let Some(beneficiary) = beneficiary {
                <C as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::resolve_creating(&beneficiary, adjusted_paid);
            } else {
                OU::on_unbalanced(adjusted_paid);
            }
        }
    }

    fn pay_priority_fee(tip: U256) {
        let author = <Pallet<T>>::find_author();
        let account_id = T::AddressMapping::get_or_into_account_id(&author);
        if let Err(e) =
            C::deposit_into_existing(&account_id, tip.low_u128().unique_saturated_into())
        {
            log::error!("Failed to pay priority fee: {:?}", e);
        }
    }
}

impl<T: Config> fp_evm::traits::Evm<T::Origin> for Pallet<T> {
    type Outcome = Result<(CallInfo, Weight), sp_runtime::DispatchError>;

    fn call(
        origin: T::Origin,
        _source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
    ) -> Self::Outcome {
        let who = frame_system::ensure_signed(origin)?;
        let source = T::AddressMapping::get_or_create_evm_address(&who);

        let is_transactional = true;
        let info = T::Runner::call(
            source,
            target,
            input,
            value,
            gas_limit,
            Some(max_fee_per_gas),
            max_priority_fee_per_gas,
            nonce,
            access_list,
            is_transactional,
            T::config(),
        )
        .map_err(|e| e.into())?;

        match info.exit_reason {
            ExitReason::Succeed(_) => {
                Pallet::<T>::deposit_event(Event::<T>::Executed(target));
            },
            _ => {
                let error = Event::<T>::ExecutedFailed(target);
                Pallet::<T>::deposit_event(error);
            },
        };
        let gas = T::GasWeightMapping::gas_to_weight(info.used_gas.unique_saturated_into());
        Ok((
            // I really don't enjoy that this is here, does frontier evm even rollback?
            info, gas,
        ))
    }
}
