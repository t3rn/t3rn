use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, Error, ExecutionId, ExecutionRegistry,
    ExecutionRegistryItem, Pallet, Reason,
};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get},
};
use sp_runtime::{
    traits::{Bounded, CheckedDiv, CheckedMul, Zero},
    Perbill, Percent,
};

// TODO: remove unwraps from this
impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>> for Pallet<T> {
    fn deposit(
        execution_id: ExecutionId,
        payee: T::AccountId,
        recipient: T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        /// Reserve the funds from the payee account
        T::Currency::transfer(
            &payee,
            &T::EscrowAccount::get(),
            amount,
            ExistenceRequirement::KeepAlive,
        )?;

        if ExecutionRegistry::<T>::contains_key(execution_id) {
            return Err(Error::<T>::ExecutionAlreadyRegistered.into())
        }

        ExecutionRegistry::<T>::insert(
            execution_id,
            ExecutionRegistryItem::new(payee, recipient, amount),
        );

        Ok(())
    }

    fn finalize(execution_id: ExecutionId, reason: Option<Reason>) -> DispatchResult {
        let item = Pallet::<T>::execution_registry(execution_id)
            .ok_or(Error::<T>::ExecutionNotRegistered)?;
        Self::split(item, reason)?;
        // TODO: remove execution from registry
        Ok(ExecutionRegistry::<T>::remove(execution_id))
    }

    fn issue(recipient: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        T::Currency::transfer(
            &T::EscrowAccount::get(),
            recipient,
            amount,
            ExistenceRequirement::KeepAlive,
        )
    }

    fn split(
        item: ExecutionRegistryItem<T::AccountId, BalanceOf<T>>,
        reason: Option<Reason>,
    ) -> DispatchResult {
        // Simple rules for splitting, for now
        let (payee_split, recipient_split): (u8, u8) = match reason {
            None => (0, 100),
            Some(Reason::ContractReverted) => (90, 10),
            Some(Reason::UnexpectedFailure) => (50, 50),
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
    const EXECUTION_ID: u64 = 1;

    #[test]
    fn test_deposit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            AccountManager::deposit(EXECUTION_ID, ALICE, BOB, DEFAULT_BALANCE / 10).unwrap();

            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE / 10);

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

            AccountManager::deposit(EXECUTION_ID, ALICE, BOB, DEFAULT_BALANCE / 10);

            assert_err!(
                AccountManager::deposit(EXECUTION_ID, ALICE, BOB, DEFAULT_BALANCE / 10),
                Error::<Test>::ExecutionAlreadyRegistered
            );
        });
    }

    #[test]
    fn test_finalize_no_reason_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );

            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(AccountManager::deposit(EXECUTION_ID, ALICE, BOB, tx_amt));
            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE + tx_amt);

            assert_ok!(AccountManager::finalize(EXECUTION_ID, None));

            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE);

            let bob_balance = Balances::free_balance(&BOB);
            assert_eq!(bob_balance, tx_amt);

            let alice_balance = Balances::free_balance(&ALICE);
            assert_eq!(alice_balance, DEFAULT_BALANCE - tx_amt);

            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }

    #[test]
    fn test_finalize_revert_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );

            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(AccountManager::deposit(EXECUTION_ID, ALICE, BOB, tx_amt));
            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE + tx_amt);

            assert_ok!(AccountManager::finalize(
                EXECUTION_ID,
                Some(Reason::ContractReverted)
            ));

            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE);

            let bob_balance = Balances::free_balance(&BOB);
            assert_eq!(bob_balance, 10_000); // 10% of the original balance

            let alice_balance = Balances::free_balance(&ALICE);
            assert_eq!(alice_balance, (DEFAULT_BALANCE - tx_amt) + 90_000); // (DEFAULT - tx_amt) + 90% of tx_amt

            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }

    #[test]
    fn test_finalize_unexpected_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );

            let tx_amt = DEFAULT_BALANCE / 10;

            assert_ok!(AccountManager::deposit(EXECUTION_ID, ALICE, BOB, tx_amt));
            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE + tx_amt);

            assert_ok!(AccountManager::finalize(
                EXECUTION_ID,
                Some(Reason::UnexpectedFailure)
            ));

            let escrow_balance = Balances::free_balance(&<Test as Config>::EscrowAccount::get());
            assert_eq!(escrow_balance, DEFAULT_BALANCE);

            let bob_balance = Balances::free_balance(&BOB);
            assert_eq!(bob_balance, 50_000); // 50% of the original balance

            let alice_balance = Balances::free_balance(&ALICE);
            assert_eq!(alice_balance, (DEFAULT_BALANCE - tx_amt) + 50_000); // (DEFAULT - tx_amt) + 50% of tx_amt

            assert_eq!(AccountManager::execution_registry(EXECUTION_ID), None);
        });
    }
}
