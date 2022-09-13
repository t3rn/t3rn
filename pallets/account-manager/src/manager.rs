use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, Error, Event, ExecutionNonce,
    ExecutionRegistry, ExecutionRegistryItem, Pallet, Reason,
};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
};
use sp_runtime::{traits::Zero, Percent};
use sp_std::borrow::ToOwned;
use t3rn_primitives::account_manager::ExecutionId;

impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>> for Pallet<T> {
    fn deposit(
        payee: &T::AccountId,
        recipient: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> Result<u64, sp_runtime::DispatchError> {
        let nonce = ExecutionNonce::<T>::get();
        if ExecutionRegistry::<T>::contains_key(nonce) {
            return Err(Error::<T>::ExecutionAlreadyRegistered.into())
        }

        ExecutionNonce::<T>::mutate(|nonce| *nonce += 1);

        T::Currency::transfer(
            payee,
            &T::EscrowAccount::get(),
            amount,
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::reserve(&T::EscrowAccount::get(), amount)?;

        ExecutionRegistry::<T>::insert(
            nonce,
            ExecutionRegistryItem::new(payee.clone(), recipient.clone(), amount),
        );

        Self::deposit_event(Event::DepositReceived {
            execution_id: nonce,
            payee: payee.clone(),
            recipient: recipient.clone(),
            amount,
        });

        Ok(nonce)
    }

    fn finalize(execution_id: ExecutionId, reason: Option<Reason>) -> DispatchResult {
        let item = Pallet::<T>::execution_registry(execution_id)
            .ok_or(Error::<T>::ExecutionNotRegistered)?;
        Self::split(item, reason)?;
        ExecutionRegistry::<T>::remove(execution_id);

        Self::deposit_event(Event::ExecutionFinalized { execution_id });
        Ok(())
    }

    fn issue(recipient: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        let (slashed, actionable_unslashed) =
            T::Currency::slash_reserved(&T::EscrowAccount::get(), amount);
        assert!(
            actionable_unslashed == Zero::zero(),
            "The account manager didn't have enough funds to issue the requested amount"
        );
        T::Currency::resolve_creating(recipient, slashed);

        Self::deposit_event(Event::Issued {
            recipient: recipient.to_owned(),
            amount,
        });

        Ok(())
    }

    fn split(
        item: ExecutionRegistryItem<T::AccountId, BalanceOf<T>>,
        reason: Option<Reason>,
    ) -> DispatchResult {
        // Simple rules for splitting, for now, we take 1% to keep the account manager alive
        let (payee_split, recipient_split): (u8, u8) = match reason {
            None => (0, 99),
            Some(Reason::ContractReverted) => (89, 10),
            Some(Reason::UnexpectedFailure) => (49, 49),
        };

        let pay_split = |split: u8, recipient: &T::AccountId| -> DispatchResult {
            if !split.is_zero() {
                let percent = Percent::from_percent(split);
                let amt = percent * *item.balance();
                Self::issue(recipient, amt)
            } else {
                Ok(())
            }
        };

        // TODO: these need to be joined or handle failure, maybe on_initialize retry a queue of failures after reserving
        pay_split(payee_split, item.payee())?;
        pay_split(recipient_split, item.recipient())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock::*, Error};
    use frame_support::{assert_err, assert_ok};

    const DEFAULT_BALANCE: u64 = 1_000_000;
    const EXECUTION_ID: u64 = 0;

    #[test]
    fn test_deposit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            <AccountManager as AccountManagerExt<AccountId, BalanceOf<Test>>>::deposit(
                &ALICE,
                &BOB,
                DEFAULT_BALANCE / 10,
            )
            .unwrap();

            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                DEFAULT_BALANCE / 10
            );

            let registry_item = AccountManager::execution_registry(EXECUTION_ID).unwrap();
            assert_eq!(*registry_item.payee(), ALICE);
            assert_eq!(*registry_item.recipient(), BOB);
            assert_eq!(*registry_item.balance(), DEFAULT_BALANCE / 10);
        });
    }

    #[test]
    fn test_deposit_when_already_exist_fails() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            ExecutionRegistry::<Test>::insert(
                EXECUTION_ID,
                ExecutionRegistryItem::new(ALICE.clone(), BOB.clone(), DEFAULT_BALANCE),
            );
            assert_err!(
                <AccountManager as AccountManagerExt<AccountId, BalanceOf<Test>>>::deposit(
                    &ALICE,
                    &BOB,
                    DEFAULT_BALANCE / 10
                ),
                Error::<Test>::ExecutionAlreadyRegistered
            );
        });
    }

    #[test]
    fn test_finalize_no_reason_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::finalize(EXECUTION_ID, None));
            let one_percent_tx_amt = DEFAULT_BALANCE / 1000;
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                one_percent_tx_amt // 1% left now
            );
            assert_eq!(
                Balances::free_balance(&BOB),
                DEFAULT_BALANCE + (tx_amt - one_percent_tx_amt)
            );
            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - tx_amt);
            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }

    #[test]
    fn test_finalize_revert_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::finalize(
                EXECUTION_ID, Some(Reason::ContractReverted)
            ));
            assert_eq!(
                Balances::free_balance(&<Test as Config>::EscrowAccount::get()),
                DEFAULT_BALANCE
            );
            assert_eq!(Balances::free_balance(&BOB), DEFAULT_BALANCE + 10_000); // 10% of the original balance
            assert_eq!(
                Balances::free_balance(&ALICE),
                (DEFAULT_BALANCE - tx_amt) + 89_000
            ); // (DEFAULT - tx_amt) + 89% of tx_amt
            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }

    #[test]
    fn test_finalize_unexpected_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
            >>::finalize(
                EXECUTION_ID, Some(Reason::UnexpectedFailure)
            ));
            assert_eq!(
                Balances::free_balance(&<Test as Config>::EscrowAccount::get()),
                DEFAULT_BALANCE
            );
            assert_eq!(Balances::free_balance(&BOB), DEFAULT_BALANCE + 49_000); // 49% of the original balance
            assert_eq!(
                Balances::free_balance(&ALICE),
                (DEFAULT_BALANCE - tx_amt) + 49_000
            ); // (DEFAULT - tx_amt) + 49% of tx_amt
            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }
}
