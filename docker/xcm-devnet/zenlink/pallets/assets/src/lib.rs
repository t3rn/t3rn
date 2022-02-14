// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

//! # Assets Module
//!
//! A simple, secure module for dealing with fungible assets.
//!
//! ## Overview
//!
//! The Assets module provides functionality for asset management of fungible asset classes
//! with a fixed supply, including:
//!
//! * Asset Issuance
//! * Asset Transfer
//! * Asset Destruction
//!
//! To use it in your runtime, you need to implement the assets [`Config`](./trait.Config.html).
//!
//! The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.
//!
//! ### Terminology
//!
//! * **Asset issuance:** The creation of a new asset, whose total supply will belong to the account
//!   that issues the asset.
//! * **Asset transfer:** The action of transferring assets from one account to another.
//! * **Asset destruction:** The process of an account removing its entire holding of an asset.
//! * **Fungible asset:** An asset whose units are interchangeable.
//! * **Non-fungible asset:** An asset for which each unit has unique characteristics.
//!
//! ### Goals
//!
//! The assets system in Substrate is designed to make the following possible:
//!
//! * Issue a unique asset to its creator's account.
//! * Move assets between accounts.
//! * Remove an account's balance of an asset when requested by that account's owner and update the
//!   asset's total supply.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `issue` - Issues the total supply of a new fungible asset to the account of the caller of the
//!   function.
//! * `transfer` - Transfers an `amount` of units of fungible asset `id` from the balance of
//! the function caller's account (`origin`) to a `target` account.
//! * `destroy` - Destroys the entire holding of a fungible asset `id` associated with the account
//! that called the function.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for
//! documentation on each function.
//!
//! ### Public Functions
//! <!-- Original author of descriptions: @gavofyork -->
//!
//! * `balance` - Get the asset `id` balance of `who`.
//! * `total_supply` - Get the total supply of an asset `id`.
//!
//! Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available
//! functions.
//!
//! ## Usage
//!
//! The following example shows how to use the Assets module in your runtime by exposing public
//! functions to:
//!
//! * Issue a new fungible asset for a token distribution event (airdrop).
//! * Query the fungible asset holding balance of an account.
//! * Query the total supply of a fungible asset that has been issued.
//!
//! ### Prerequisites
//!
//! Import the Assets module and types and derive your runtime's configuration traits from the
//! Assets module trait.
//!
//! ### Simple Code Snippet
//!
//! ```rust,ignore
//! use pallet_assets as assets;
//! use frame_support::{decl_module, dispatch, ensure};
//! use frame_system::ensure_signed;
//!
//! pub trait Config: assets::Config { }
//!
//! decl_module! {
//! 	pub struct Module<T: Config> for enum Call where origin: T::Origin {
//! 		pub fn issue_token_airdrop(origin) -> dispatch::DispatchResult {
//! 			let sender = ensure_signed(origin).map_err(|e| e.as_str())?;
//!
//! 			const ACCOUNT_ALICE: u64 = 1;
//! 			const ACCOUNT_BOB: u64 = 2;
//! 			const COUNT_AIRDROP_RECIPIENTS: u64 = 2;
//! 			const TOKENS_FIXED_SUPPLY: u64 = 100;
//!
//! 			ensure!(!COUNT_AIRDROP_RECIPIENTS.is_zero(), "Divide by zero error.");
//!
//! 			let asset_id = Self::next_asset_id();
//!
//! 			<NextAssetId<T>>::mutate(|asset_id| *asset_id += 1);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_ALICE), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_BOB), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<TotalSupply<T>>::insert(asset_id, TOKENS_FIXED_SUPPLY);
//!
//! 			Self::deposit_event(RawEvent::Issued(asset_id, sender, TOKENS_FIXED_SUPPLY));
//! 			Ok(())
//! 		}
//! 	}
//! }
//! ```
//!
//! ## Assumptions
//!
//! Below are assumptions that must be held when using this module.  If any of
//! them are violated, the behavior of this module is undefined.
//!
//! * The total count of assets should be less than `Config::TokenId::max_value()`.
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::tabs_in_doc_comments)]
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter};
use frame_system::ensure_signed;
use sp_runtime::{
	traits::{AtLeast32Bit, AtLeast32BitUnsigned, Member, One, StaticLookup, Zero},
	DispatchResult,
};

/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	/// The arithmetic type of asset identifier.
	type TokenId: Parameter + AtLeast32Bit + Default + Copy;
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		/// Issue a new class of fungible assets. There are, and will only ever be, `total`
		/// such assets and they'll all belong to the `origin` initially. It will have an
		/// identifier `TokenId` instance: this will be specified in the `Issued` event.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 2 storage writes (condec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn issue(origin, #[compact] total: T::Balance) {
			let origin = ensure_signed(origin)?;

			let id = Self::next_asset_id();
			<NextAssetId<T>>::mutate(|id| *id += One::one());

			<Balances<T>>::insert((id, &origin), total);
			<TotalSupply<T>>::insert(id, total);

			Self::deposit_event(RawEvent::Issued(id, origin, total));
		}

		/// Move some assets from one holder to another.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 static lookup
		/// - 2 storage mutations (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn transfer(origin,
			#[compact] id: T::TokenId,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: T::Balance
		) {
			let origin = ensure_signed(origin)?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

			Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate((id, target), |balance| *balance += amount);
		}

		/// Destroy any assets of `id` owned by `origin`.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 1 storage deletion (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn destroy(origin, #[compact] id: T::TokenId) {
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, &origin));
			ensure!(!balance.is_zero(), Error::<T>::BalanceZero);

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
		}
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Config>::AccountId,
		<T as Config>::Balance,
		<T as Config>::TokenId,
	{
		/// Some assets were issued. \[asset_id, owner, total_supply\]
		Issued(TokenId, AccountId, Balance),
		/// Some assets were transferred. \[asset_id, from, to, amount\]
		Transferred(TokenId, AccountId, AccountId, Balance),
		/// Some assets were destroyed. \[asset_id, owner, balance\]
		Destroyed(TokenId, AccountId, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Transfer amount should be non-zero
		AmountZero,
		/// Account balance must be greater than or equal to the transfer amount
		BalanceLow,
		/// Balance should be non-zero
		BalanceZero,
	}
}

decl_storage! {
	trait Store for Module<T: Config> as Assets {
		/// The number of units of assets held by any given account.
		Balances: map hasher(blake2_128_concat) (T::TokenId, T::AccountId) => T::Balance;
		/// The next asset identifier up for grabs.
		NextAssetId get(fn next_asset_id): T::TokenId;
		/// The total unit supply of an asset.
		///
		/// TWOX-NOTE: `TokenId` is trusted, so this is safe.
		TotalSupply: map hasher(twox_64_concat) T::TokenId => T::Balance;
	}
}

impl<T: Config> Module<T> {
	pub fn balance(id: T::TokenId, who: T::AccountId) -> T::Balance {
		<Balances<T>>::get((id, who))
	}

	/// Get the total supply of an asset `id`.
	pub fn total_supply(id: T::TokenId) -> T::Balance {
		<TotalSupply<T>>::get(id)
	}

	pub fn inner_transfer(
		id: T::TokenId,
		origin: T::AccountId,
		target: T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		let origin_account = (id, origin.clone());
		let origin_balance = <Balances<T>>::get(&origin_account);
		ensure!(!amount.is_zero(), Error::<T>::AmountZero);
		ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

		<Balances<T>>::mutate((id, origin), |balance| *balance -= amount);
		<Balances<T>>::mutate((id, target), |balance| *balance += amount);
		Ok(())
	}

	pub fn inner_deposit(id: T::TokenId, origin: T::AccountId, amount: T::Balance) -> DispatchResult {
		ensure!(!amount.is_zero(), Error::<T>::AmountZero);

		<TotalSupply<T>>::mutate(id, |total_supply| *total_supply += amount);
		<Balances<T>>::mutate((id, origin), |balance| *balance += amount);
		Ok(())
	}

	pub fn inner_withdraw(id: T::TokenId, origin: T::AccountId, amount: T::Balance) -> DispatchResult {
		let origin_account = (id, origin.clone());
		let origin_balance = <Balances<T>>::get(&origin_account);
		ensure!(!amount.is_zero(), Error::<T>::AmountZero);
		ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

		<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= amount);
		<Balances<T>>::mutate((id, origin), |balance| *balance -= amount);
		Ok(())
	}
}

pub trait ExternalAssetHandler<TokenId, AccountId, TokenBalance> {
	fn contains(id: TokenId) -> bool;

	fn balance(id: TokenId, who: AccountId) -> TokenBalance;

	fn total_supply(id: TokenId) -> TokenBalance;

	fn transfer(id: TokenId, origin: AccountId, target: AccountId, amount: TokenBalance) -> DispatchResult;

	fn deposit(id: TokenId, who: AccountId, amount: TokenBalance) -> DispatchResult;

	fn withdraw(id: TokenId, who: AccountId, amount: TokenBalance) -> DispatchResult;
}

impl<T: Config> ExternalAssetHandler<T::TokenId, T::AccountId, T::Balance> for Module<T> {
	fn contains(id: T::TokenId) -> bool {
		Self::next_asset_id() > id
	}

	fn balance(id: T::TokenId, who: T::AccountId) -> T::Balance {
		Self::balance(id, who)
	}

	fn total_supply(id: T::TokenId) -> T::Balance {
		Self::total_supply(id)
	}

	fn transfer(id: T::TokenId, origin: T::AccountId, target: T::AccountId, amount: T::Balance) -> DispatchResult {
		Self::inner_transfer(id, origin, target, amount)
	}

	fn deposit(id: T::TokenId, who: T::AccountId, amount: T::Balance) -> DispatchResult {
		Self::inner_deposit(id, who, amount)
	}

	fn withdraw(id: T::TokenId, who: T::AccountId, amount: T::Balance) -> DispatchResult {
		Self::inner_withdraw(id, who, amount)
	}
}

#[cfg(test)]
mod tests {
	use frame_support::{assert_noop, assert_ok, parameter_types};
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

	use crate as pallet_assets;

	use super::*;

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
	}
	impl frame_system::Config for Test {
		type BaseCallFilter = ();
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type Call = Call;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
	}

	impl Config for Test {
		type Event = Event;
		type Balance = u64;
		type TokenId = u32;
	}

	fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap()
			.into()
	}

	#[test]
	fn issuing_asset_units_to_issuer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
		});
	}

	#[test]
	fn querying_total_supply_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
			assert_ok!(Assets::transfer(Origin::signed(2), 0, 3, 31));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 19);
			assert_eq!(Assets::balance(0, 3), 31);
			assert_ok!(Assets::destroy(Origin::signed(3), 0));
			assert_eq!(Assets::total_supply(0), 69);
		});
	}

	#[test]
	fn transferring_amount_above_available_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
		});
	}

	#[test]
	fn transferring_amount_more_than_available_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
			assert_ok!(Assets::destroy(Origin::signed(1), 0));
			assert_eq!(Assets::balance(0, 1), 0);
			assert_noop!(Assets::transfer(Origin::signed(1), 0, 1, 50), Error::<Test>::BalanceLow);
		});
	}

	#[test]
	fn transferring_less_than_one_unit_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_noop!(Assets::transfer(Origin::signed(1), 0, 2, 0), Error::<Test>::AmountZero);
		});
	}

	#[test]
	fn transferring_more_units_than_total_supply_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_noop!(
				Assets::transfer(Origin::signed(1), 0, 2, 101),
				Error::<Test>::BalanceLow
			);
		});
	}

	#[test]
	fn destroying_asset_balance_with_positive_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::destroy(Origin::signed(1), 0));
		});
	}

	#[test]
	fn destroying_asset_balance_with_zero_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 2), 0);
			assert_noop!(Assets::destroy(Origin::signed(2), 0), Error::<Test>::BalanceZero);
		});
	}

	#[test]
	fn inner_transfer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::inner_transfer(0, 1, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
		});
	}

	#[test]
	fn inner_withdraw_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::inner_withdraw(0, 1, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::total_supply(0), 50);
		});
	}

	#[test]
	fn inner_deposit_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_eq!(Assets::total_supply(0), 100);
			assert_ok!(Assets::inner_deposit(0, 2, 50));
			assert_eq!(Assets::total_supply(0), 150);
			assert_eq!(Assets::balance(0, 2), 50);
		});
	}
}
