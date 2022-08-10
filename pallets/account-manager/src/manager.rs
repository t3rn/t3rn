use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsExecutionRegistry,
    ContractsRegistryExecutionNonce, Error, Event, ExecutionRegistryItem, Outcome, Pallet,
    PendingChargesPerRound, SettlementsPerRound,
};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
};
use sp_runtime::{traits::Zero, DispatchError, Percent};
use sp_std::borrow::ToOwned;
use t3rn_primitives::{
    account_manager::{ExecutionId, RequestCharge, Settlement},
    circuit_clock::{BenefitSource, CircuitClock, CircuitRole, ClaimableArtifacts},
    common::RoundInfo,
    executors::Executors,
};

use pallet_xbi_portal::sabi::Sabi;
use t3rn_primitives::bridges::polkadot_core::Balance;

pub struct ActiveSetClaimablePerRound<Account, Balance> {
    pub executor: Account,
    pub claimable: Balance,
}

fn percent_ratio<T: Config>(amt: BalanceOf<T>, percent: u8) -> BalanceOf<T> {
    amt * (BalanceOf::<T>::from(percent) / BalanceOf::<T>::from(100u8))
}

impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>, T::Hash, T::BlockNumber>
    for Pallet<T>
{
    fn get_charge_or_fail(
        charge_id: T::Hash,
    ) -> Result<RequestCharge<T::AccountId, BalanceOf<T>>, DispatchError> {
        return if let Some(pending_charge) =
            PendingChargesPerRound::<T>::get(T::CircuitClock::current_round(), charge_id)
        {
            Ok(pending_charge)
        } else {
            Err(Error::<T>::ChargeAlreadyRegistered.into())
        }
    }

    fn no_charge_or_fail(charge_id: T::Hash) -> Result<(), DispatchError> {
        return if let Some(_pending_charge) =
            PendingChargesPerRound::<T>::get(T::CircuitClock::current_round(), charge_id)
        {
            Err(Error::<T>::ChargeAlreadyRegistered.into())
        } else {
            Ok(())
        }
    }

    fn bump_contracts_registry_nonce() -> Result<T::Hash, DispatchError> {
        let execution_id = ContractsRegistryExecutionNonce::<T>::get();
        ContractsRegistryExecutionNonce::<T>::mutate(|nonce| *nonce += 1);

        let charge_id = Decode::decode(&mut &Sabi::value_64_2_value_256(execution_id).encode()[..])
            .map_err(|_e| Error::<T>::DecodingExecutionIDFailed)?;

        Self::no_charge_or_fail(charge_id)?;
        Ok(charge_id)
    }

    /// If Called by 3VM as a execution deposit, expect:
    ///     - charge = gas_fees
    ///     - reward = 0
    /// If Called by Circuit as charge deposit, expect:
    ///     - charge = std SFX execution + delivery charge
    ///     - reward = Open Market based offered by requester
    fn deposit(
        charge_id: T::Hash,
        payee: &T::AccountId,
        charge_fee: BalanceOf<T>,
        offered_reward: BalanceOf<T>,
        source: BenefitSource,
        role: CircuitRole,
        maybe_recipient: Option<T::AccountId>,
    ) -> DispatchResult {
        Self::no_charge_or_fail(charge_id).map_err(|_e| Error::<T>::ExecutionAlreadyRegistered)?;

        T::Currency::reserve(payee, charge_fee + offered_reward)?;

        let recipient = if let Some(recipient) = maybe_recipient {
            recipient.clone()
        } else {
            // todo: Inspect if that's a good idea
            T::EscrowAccount::get()
        };

        PendingChargesPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            charge_id,
            RequestCharge {
                payee: payee.clone(),
                offered_reward,
                charge_fee,
                recipient,
                source,
                role,
            },
        );

        Ok(())
    }

    fn finalize(
        charge_id: T::Hash,
        outcome: Outcome,
        maybe_recipient: Option<T::AccountId>,
        maybe_actual_fees: Option<BalanceOf<T>>,
    ) -> DispatchResult {
        let charge = Self::get_charge_or_fail(charge_id)?;

        // Decide on charges split
        // Simple rules for splitting, for now, we take 1% to keep the account manager alive
        let (payee_split, recipient_split): (u8, u8) = match outcome {
            Outcome::Commit => (0, 99),
            Outcome::Revert => (89, 10),
            Outcome::UnexpectedFailure => (49, 50),
        };

        let payee_refund: BalanceOf<T> = if let Some(actual_fees) = maybe_actual_fees {
            percent_ratio::<T>(charge.charge_fee - actual_fees, payee_split)
        } else {
            percent_ratio::<T>(charge.charge_fee, payee_split)
        };
        T::Currency::slash_reserved(&charge.payee, charge.charge_fee + charge.offered_reward);
        T::Currency::deposit_creating(&charge.payee, payee_refund.clone());

        // Check if recipient has been updated
        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            charge.recipient
        };

        let recipient_payout_async = percent_ratio::<T>(
            charge.charge_fee - payee_refund.clone() + charge.offered_reward,
            recipient_split,
        );

        // Create Settlement for the future async claim
        SettlementsPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            charge_id,
            Settlement::<T::AccountId, BalanceOf<T>> {
                requester: charge.payee,
                recipient,
                settlement_amount: recipient_payout_async,
                outcome,
                source: charge.source,
                role: charge.role,
            },
        );

        // Take what's left - 1% to keep the account manager alive
        T::Currency::deposit_creating(
            &T::EscrowAccount::get(),
            charge.charge_fee - recipient_payout_async - payee_refund,
        );

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

        for settlement in SettlementsPerRound::<T>::iter_prefix_values(r) {
            // fixme: test that actually updates active_set_claimables or are the references wrong
            for active_set_claimable in active_set_claimables.iter_mut() {
                if active_set_claimable.executor == settlement.recipient {
                    active_set_claimable.claimable += settlement.settlement_amount;
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
        //         beneficiary: active_set_claimable.executor.clone(),
        //         role: CircuitRole::Executor,
        //         total_round_claim: collateral_bond_power * active_set_claimable.claimable,
        //         benefit_source: BenefitSource::TrafficRewards,
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
        //             role: CircuitRole::Staker,
        //             total_round_claim: staker_power * claimable_by_all_stakers_of_executor,
        //             benefit_source: BenefitSource::TrafficRewards,
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
            ContractsExecutionRegistry::<Test>::insert(
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
                EXECUTION_ID, Some(Outcome::ContractReverted)
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
                EXECUTION_ID, Some(Outcome::UnexpectedFailure)
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
