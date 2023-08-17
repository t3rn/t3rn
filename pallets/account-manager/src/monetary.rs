use sp_std::marker::PhantomData;

use frame_support::{
    pallet_prelude::*,
    traits::{
        tokens::{fungibles::Unbalanced, WithdrawConsequence},
        ExistenceRequirement, ReservableCurrency, WithdrawReasons,
    },
};

use frame_support::traits::tokens::{
    Fortitude::{Force, Polite},
    Precision::{BestEffort, Exact},
    Preservation::{Expendable, Preserve, Protect},
    Restriction::Free,
};
use sp_runtime::traits::Convert;

pub struct Monetary<AccountId, Assets, NativeCurrency, AssetBalanceOf>(
    PhantomData<(AccountId, Assets, NativeCurrency, AssetBalanceOf)>,
);
impl<
        AccountId,
        Assets: Unbalanced<AccountId>,
        NativeCurrency: ReservableCurrency<AccountId>,
        AssetBalanceOf: Convert<NativeCurrency::Balance, Assets::Balance>,
    > Monetary<AccountId, Assets, NativeCurrency, AssetBalanceOf>
{
    pub fn deposit(
        beneficiary: &AccountId,
        asset_id: Option<Assets::AssetId>,
        amount: NativeCurrency::Balance,
    ) {
        match asset_id {
            None => {
                NativeCurrency::deposit_creating(beneficiary, amount);
            },
            Some(asset_id) => {
                Assets::increase_balance(
                    asset_id,
                    beneficiary,
                    AssetBalanceOf::convert(amount),
                    Exact,
                );
            },
        }
    }

    pub fn can_withdraw(
        beneficiary: &AccountId,
        asset_id: Option<Assets::AssetId>,
        amount: NativeCurrency::Balance,
    ) -> bool {
        match asset_id {
            None =>
                NativeCurrency::free_balance(beneficiary) + NativeCurrency::minimum_balance()
                    >= amount,
            Some(asset_id) => {
                match Assets::can_withdraw(asset_id, beneficiary, AssetBalanceOf::convert(amount)) {
                    WithdrawConsequence::Success => true,
                    _ => false,
                }
            },
        }
    }

    pub fn withdraw(
        source: &AccountId,
        amount: NativeCurrency::Balance,
        maybe_asset_id: Option<Assets::AssetId>,
    ) -> DispatchResult {
        match maybe_asset_id {
            None => match NativeCurrency::withdraw(
                source,
                amount,
                WithdrawReasons::RESERVE,
                ExistenceRequirement::KeepAlive,
            ) {
                Err(e) => Err(e),
                Ok(_imbalance) => Ok(()),
            },
            Some(asset_id) => {
                match Assets::decrease_balance(
                    asset_id,
                    source,
                    AssetBalanceOf::convert(amount),
                    Exact,
                    Expendable,
                    Polite,
                ) {
                    Err(e) => Err(e),
                    Ok(_imbalance) => Ok(()),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_mock_runtime::*;
    use frame_support::traits::fungibles::Mutate;

    use frame_support::{assert_noop, assert_ok, traits::Currency};
    use sp_runtime::{traits::ConvertInto, ModuleError};

    use circuit_runtime_types::{AccountId, Balance};

    const DEFAULT_BALANCE: Balance = 1_000_000;

    #[test]
    fn given_native_assets_monetary_deposits_correctly() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            const DEPOSIT_AMOUNT: Balance = DEFAULT_BALANCE / 10;
            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);

            Monetary::<AccountId, Assets, Balances, ConvertInto>::deposit(
                &ALICE,
                None,
                DEFAULT_BALANCE / 10,
            );
            assert_eq!(
                Balances::free_balance(ALICE),
                DEFAULT_BALANCE + DEPOSIT_AMOUNT
            );
        });
    }

    #[test]
    fn given_foreign_assets_monetary_deposits_correctly() {
        ExtBuilder::default().build().execute_with(|| {
            const FOREIGN_ASSET_A: AssetId = 1;
            const MIN_BALANCE_ASSET_A: Balance = 1;
            assert_ok!(Assets::force_create(
                RuntimeOrigin::root(),
                FOREIGN_ASSET_A,
                sp_runtime::MultiAddress::Id(BOB), /* owner */
                true,                              /* is_sufficient */
                MIN_BALANCE_ASSET_A
            ));

            const DEPOSIT_AMOUNT: Balance = DEFAULT_BALANCE / 10;

            assert_ok!(Assets::mint_into(FOREIGN_ASSET_A, &ALICE, DEFAULT_BALANCE));
            assert_eq!(Assets::balance(FOREIGN_ASSET_A, ALICE), DEFAULT_BALANCE);

            Monetary::<AccountId, Assets, Balances, ConvertInto>::deposit(
                &ALICE,
                Some(FOREIGN_ASSET_A),
                DEPOSIT_AMOUNT,
            );
            assert_eq!(
                Assets::balance(FOREIGN_ASSET_A, ALICE),
                DEFAULT_BALANCE + DEPOSIT_AMOUNT
            );
        });
    }

    #[test]
    fn given_foreign_assets_monetary_withdraws_correctly() {
        ExtBuilder::default().build().execute_with(|| {
            const FOREIGN_ASSET_A: AssetId = 1;
            const MIN_BALANCE_ASSET_A: Balance = 1;
            assert_ok!(Assets::force_create(
                RuntimeOrigin::root(),
                FOREIGN_ASSET_A,
                sp_runtime::MultiAddress::Id(BOB), /* owner */
                true,                              /* is_sufficient */
                MIN_BALANCE_ASSET_A
            ));

            const DEPOSIT_AMOUNT: Balance = DEFAULT_BALANCE / 10;

            assert_ok!(Assets::mint_into(FOREIGN_ASSET_A, &ALICE, DEFAULT_BALANCE));
            assert_eq!(Assets::balance(FOREIGN_ASSET_A, ALICE), DEFAULT_BALANCE);

            assert_ok!(
                Monetary::<AccountId, Assets, Balances, ConvertInto>::withdraw(
                    &ALICE,
                    DEPOSIT_AMOUNT,
                    Some(FOREIGN_ASSET_A),
                )
            );

            assert_eq!(
                Assets::balance(FOREIGN_ASSET_A, ALICE),
                DEFAULT_BALANCE - DEPOSIT_AMOUNT
            );

            // withdraw again works
            assert_ok!(
                Monetary::<AccountId, Assets, Balances, ConvertInto>::withdraw(
                    &ALICE,
                    DEPOSIT_AMOUNT,
                    Some(FOREIGN_ASSET_A),
                )
            );

            assert_eq!(
                Assets::balance(FOREIGN_ASSET_A, ALICE),
                DEFAULT_BALANCE - 2 * DEPOSIT_AMOUNT
            );

            // withdraw too much fails
            assert_noop!(
                Monetary::<AccountId, Assets, Balances, ConvertInto>::withdraw(
                    &ALICE,
                    DEFAULT_BALANCE,
                    Some(FOREIGN_ASSET_A),
                ),
                DispatchError::Module(ModuleError {
                    index: 12,
                    error: [0, 0, 0, 0],
                    message: Some("BalanceLow")
                })
            );
        });
    }
}
