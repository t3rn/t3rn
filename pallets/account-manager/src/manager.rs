use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, Error, Event, ExecutionNonce,
    ExecutionRegistry, ExecutionRegistryItem, Pallet, Reason, SfxPendingChargesPerRound,
    SfxSettlementsPerRound,
};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
};
use sp_runtime::{traits::Zero, Percent};
use sp_std::borrow::ToOwned;
use t3rn_primitives::{
    account_manager::{ExecutionId, SfxRequestCharge, SfxSettlement},
    circuit_clock::{BenefitSource, CircuitClock, CircuitRole, ClaimableArtifacts},
    common::RoundInfo,
    executors::Executors,
};

pub struct ActiveSetClaimablePerRound<Account, Balance> {
    pub executor: Account,
    pub claimable: Balance,
}

impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>, T::Hash, T::BlockNumber>
    for Pallet<T>
{
    fn deposit(
        payee: &T::AccountId,
        recipient: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let execution_id = ExecutionNonce::<T>::get();
        ExecutionNonce::<T>::mutate(|nonce| *nonce += 1);

        T::Currency::transfer(
            payee,
            &T::EscrowAccount::get(),
            amount,
            ExistenceRequirement::KeepAlive,
        )?;
        T::Currency::reserve(&T::EscrowAccount::get(), amount)?;

        if ExecutionRegistry::<T>::contains_key(execution_id) {
            return Err(Error::<T>::ExecutionAlreadyRegistered.into())
        }

        ExecutionRegistry::<T>::insert(
            execution_id,
            ExecutionRegistryItem::new(payee.clone(), recipient.clone(), amount),
        );

        Self::deposit_event(Event::DepositReceived {
            execution_id,
            payee: payee.clone(),
            recipient: recipient.clone(),
            amount,
        });

        Ok(())
    }

    /// Reward executor for successful sfx execution - accounted after successful xtx resolution
    fn commit_charge(executor: T::AccountId, sfx_id: T::Hash) -> DispatchResult {
        let pending_charge: SfxRequestCharge<T::AccountId, BalanceOf<T>> =
            if let Some(pending_charge) =
                SfxPendingChargesPerRound::<T>::get(T::CircuitClock::current_round(), sfx_id)
            {
                pending_charge
            } else {
                return Err(Error::<T>::PendingChargeNotFoundAtCommit.into())
            };

        SfxPendingChargesPerRound::<T>::remove(T::CircuitClock::current_round(), sfx_id);

        // todo: charge requester now or at the end of the round?
        T::Currency::unreserve(
            &pending_charge.requester,
            pending_charge.charge_fee + pending_charge.offered_reward,
        );
        T::Currency::transfer(
            &pending_charge.requester,
            &T::EscrowAccount::get(),
            // todo: does escrow_account hold the offered_reward until the claim?
            pending_charge.charge_fee + pending_charge.offered_reward,
            ExistenceRequirement::KeepAlive,
        )?;

        SfxSettlementsPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            sfx_id,
            SfxSettlement::<T::AccountId, BalanceOf<T>> {
                reward: pending_charge.offered_reward,
                fee: pending_charge.charge_fee,
                payer: pending_charge.requester,
                executor,
            },
        );

        Ok(().into())
    }

    /// Charge requester for SFX submission: reward + fees.
    fn charge_requester(
        charge: SfxRequestCharge<T::AccountId, BalanceOf<T>>,
        sfx_id: T::Hash,
    ) -> DispatchResult {
        T::Currency::reserve(&charge.requester, charge.charge_fee + charge.offered_reward)?;

        SfxPendingChargesPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            sfx_id,
            charge.clone(),
        );

        Ok(())
    }

    /// Refund the reward back to the requester. Keep the fees.
    fn refund_charge(sfx_id: T::Hash) -> DispatchResult {
        let pending_charge: SfxRequestCharge<T::AccountId, BalanceOf<T>> =
            if let Some(pending_charge) =
                SfxPendingChargesPerRound::<T>::get(T::CircuitClock::current_round(), sfx_id)
            {
                pending_charge
            } else {
                return Err(Error::<T>::PendingChargeNotFoundAtRefund.into())
            };

        T::Currency::unreserve(
            &pending_charge.requester,
            pending_charge.charge_fee + pending_charge.offered_reward,
        );
        T::Currency::transfer(
            &pending_charge.requester,
            &T::EscrowAccount::get(),
            pending_charge.charge_fee,
            ExistenceRequirement::KeepAlive,
        )?;

        SfxPendingChargesPerRound::<T>::remove(T::CircuitClock::current_round(), sfx_id);

        Ok(().into())
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

    /// Collect claimable (only SFX execution rewards) for Executors and Stakers submitted by Circuit at the duration of the current Round
    fn on_collect_claimable(
        _n: T::BlockNumber,
        r: RoundInfo<T::BlockNumber>,
    ) -> Result<Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>, &'static str> {
        let mut claimable_artifacts: Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>> = vec![];
        let mut active_set_claimables: Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>> =
            T::Executors::active_set()
                .into_iter()
                .map(|executor: T::AccountId| ActiveSetClaimablePerRound {
                    executor,
                    claimable: Zero::zero(),
                })
                .collect::<Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>>>();

        for sfx_settlement in SfxSettlementsPerRound::<T>::iter_prefix_values(r) {
            // fixme: test that actually updates active_set_claimables or are the references wrong
            for mut active_set_claimable in active_set_claimables.iter_mut() {
                if active_set_claimable.executor == sfx_settlement.executor {
                    active_set_claimable.claimable += sfx_settlement.reward;
                }
            }
        }

        // fixme: Below is good for review after aligning the Currency traits we use :<
        //error[E0308]: mismatched types
        //    --> pallets/account-manager/src/manager.rs:253:40
        // 253 |                     total_round_claim: staker_power * claimable_by_all_stakers_of_executor,
        //     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected pallet::Config::Currency, found t3rn_primitives::EscrowTrait::Currency

        // for active_set_claimable in active_set_claimables {
        //     let collateral_bond = T::Executors::collateral_bond(&active_set_claimable.executor);
        //     let nominated_stake =
        //         T::Executors::total_nominated_stake(&active_set_claimable.executor);
        //     // calculate % ratio of rewards proportionally to Executor's own Collateral to Nominated Stake
        //     let total_stake_power = collateral_bond + nominated_stake;
        //
        //     // todo: ensure it's in range (0,1>
        //     let collateral_bond_power = collateral_bond / total_stake_power.clone();
        //
        //     claimable_artifacts.push(ClaimableArtifacts {
        //         beneficiary: active_set_claimable.executor,
        //         role: CircuitRole::Relayer,
        //         total_round_claim: collateral_bond_power * active_set_claimable.claimable,
        //         benefit_source: BenefitSource::ExecutorRewards,
        //     });
        //
        //     // todo: ensure it's in range <0,1)
        //     let nominated_stake_power = nominated_stake / total_stake_power;
        //
        //     let claimable_by_all_stakers_of_executor =
        //         nominated_stake_power.clone() * active_set_claimable.claimable;
        //
        //     for nominated_stake in T::Executors::stakes_per_executor(&active_set_claimable.executor)
        //     {
        //         let staker_power = nominated_stake.nominated_stake / nominated_stake_power.clone();
        //         claimable_artifacts.push(ClaimableArtifacts {
        //             beneficiary: nominated_stake.staker,
        //             role: CircuitRole::Relayer,
        //             total_round_claim: staker_power * claimable_by_all_stakers_of_executor,
        //             benefit_source: BenefitSource::ExecutorStakingRewards,
        //         });
        //     }
        // }

        Ok(claimable_artifacts)
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
            <AccountManager as AccountManagerExt<AccountId, BalanceOf<Test>, Hash, BlockNumber>>::deposit(
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
                <AccountManager as AccountManagerExt<
                    AccountId,
                    BalanceOf<Test>,
                    Hash,
                    BlockNumber,
                >>::deposit(&ALICE, &BOB, DEFAULT_BALANCE / 10),
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
                Hash,
                BlockNumber,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
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
                Hash,
                BlockNumber,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
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
                Hash,
                BlockNumber,
            >>::deposit(&ALICE, &BOB, tx_amt));
            assert_eq!(
                Balances::reserved_balance(&<Test as Config>::EscrowAccount::get()),
                tx_amt
            );
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
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
