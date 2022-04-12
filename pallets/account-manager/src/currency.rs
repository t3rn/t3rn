use crate::{Config, Pallet};
use frame_support::{
    dispatch::DispatchResult,
    traits::{
        BalanceStatus, Currency, ExistenceRequirement, LockIdentifier, LockableCurrency,
        NamedReservableCurrency, ReservableCurrency, SignedImbalance, WithdrawReasons,
    },
};
use pallet_balances::{NegativeImbalance, PositiveImbalance};
use sp_runtime::{traits::MaybeSerializeDeserialize, DispatchError};
use std::fmt::Debug;

impl<T: pallet_balances::Config<I>, I: 'static> Currency<T::AccountId> for Pallet<T, I>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type Balance = T::Balance;
    type NegativeImbalance = NegativeImbalance<T, I>;
    type PositiveImbalance = PositiveImbalance<T, I>;

    fn total_balance(who: &T::AccountId) -> Self::Balance {
        Self::total_balance(who)
    }

    fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
        Self::can_slash(who, value)
    }

    fn total_issuance() -> Self::Balance {
        Self::total_issuance()
    }

    fn minimum_balance() -> Self::Balance {
        Self::minimum_balance()
    }

    fn burn(mut amount: Self::Balance) -> Self::PositiveImbalance {
        Self::burn(amount)
    }

    fn issue(mut amount: Self::Balance) -> Self::NegativeImbalance {
        Self::issue(amount)
    }

    fn free_balance(who: &T::AccountId) -> Self::Balance {
        Self::free_balance(who)
    }

    fn ensure_can_withdraw(
        who: &T::AccountId,
        amount: T::Balance,
        reasons: WithdrawReasons,
        new_balance: T::Balance,
    ) -> DispatchResult {
        Self::ensure_can_withdraw(who, amount, reasons, new_balance)
    }

    fn transfer(
        transactor: &T::AccountId,
        dest: &T::AccountId,
        value: Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> DispatchResult {
        Self::transfer(transactor, dest, value, existence_requirement)
    }

    fn slash(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        Self::slash(who, value)
    }

    /// Deposit some `value` into the free balance of an existing target account `who`.
    ///
    /// Is a no-op if the `value` to be deposited is zero.
    fn deposit_into_existing(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Result<Self::PositiveImbalance, DispatchError> {
        Self::deposit_into_existing(who, value)
    }

    /// Deposit some `value` into the free balance of `who`, possibly creating a new account.
    ///
    /// This function is a no-op if:
    /// - the `value` to be deposited is zero; or
    /// - the `value` to be deposited is less than the required ED and the account does not yet
    ///   exist; or
    /// - the deposit would necessitate the account to exist and there are no provider references;
    ///   or
    /// - `value` is so large it would cause the balance of `who` to overflow.
    fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
        Self::deposit_creating(who, value)
    }

    /// Withdraw some free balance from an account, respecting existence requirements.
    ///
    /// Is a no-op if value to be withdrawn is zero.
    fn withdraw(
        who: &T::AccountId,
        value: Self::Balance,
        reasons: WithdrawReasons,
        liveness: ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, DispatchError> {
        Self::withdraw(who, value, reasons, liveness)
    }

    /// Force the new free balance of a target account `who` to some new value `balance`.
    fn make_free_balance_be(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        Self::make_free_balance_be(who, value)
    }
}

impl<T: pallet_balances::Config<I>, I: 'static> ReservableCurrency<T::AccountId> for Pallet<T, I>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
        Self::can_reserve(who, value)
    }

    fn slash_reserved(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        Self::slash_reserved(who, value)
    }

    fn reserved_balance(who: &T::AccountId) -> Self::Balance {
        Self::reserved_balance(who)
    }

    fn reserve(who: &T::AccountId, value: Self::Balance) -> DispatchResult {
        Self::reserve(who, value)
    }

    fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        Self::unreserve(who, value)
    }

    fn repatriate_reserved(
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> Result<Self::Balance, DispatchError> {
        Self::repatriate_reserved(slashed, beneficiary, value, status)
    }
}

impl<T: pallet_balances::Config<I>, I: 'static> NamedReservableCurrency<T::AccountId>
    for Pallet<T, I>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type ReserveIdentifier = T::ReserveIdentifier;

    fn slash_reserved_named(
        id: &Self::ReserveIdentifier,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        Self::slash_reserved_named(id, who, value)
    }

    fn reserved_balance_named(id: &Self::ReserveIdentifier, who: &T::AccountId) -> Self::Balance {
        Self::reserved_balance_named(id, who)
    }

    fn reserve_named(
        id: &Self::ReserveIdentifier,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> DispatchResult {
        Self::reserve_named(id, who, value)
    }

    fn unreserve_named(
        id: &Self::ReserveIdentifier,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Self::Balance {
        Self::unreserve_named(id, who, value)
    }

    fn repatriate_reserved_named(
        id: &Self::ReserveIdentifier,
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> Result<Self::Balance, DispatchError> {
        Self::repatriate_reserved_named(id, slashed, beneficiary, value, status)
    }
}

impl<T: pallet_balances::Config<I>, I: 'static> LockableCurrency<T::AccountId> for Pallet<T, I>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type MaxLocks = T::MaxLocks;
    type Moment = T::BlockNumber;

    fn set_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        reasons: WithdrawReasons,
    ) {
        Self::set_lock(id, who, amount, reasons)
    }

    // Extend a lock on the balance of `who`.
    // Is a no-op if lock amount is zero or `reasons` `is_none()`.
    fn extend_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        reasons: WithdrawReasons,
    ) {
        Self::extend_lock(id, who, amount, reasons)
    }

    fn remove_lock(id: LockIdentifier, who: &T::AccountId) {
        Self::remove_lock(id, who)
    }
}
