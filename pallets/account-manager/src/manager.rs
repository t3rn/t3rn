use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsExecutionRegistry,
    ContractsRegistryExecutionNonce, Error, Event, ExecutionRegistryItem, Pallet,
    PendingChargesPerRound, Reason, SettlementsPerRound,
};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
};
use sp_runtime::{traits::Zero, Percent};
use sp_std::borrow::ToOwned;
use t3rn_primitives::{
    account_manager::{ExecutionId, RequestCharge, SfxSettlement},
    circuit_clock::{CircuitClock, ClaimableArtifacts},
    common::RoundInfo,
    executors::Executors,
};

use pallet_xbi_portal::sabi::Sabi;

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
        let execution_id = ContractsRegistryExecutionNonce::<T>::get();
        ContractsRegistryExecutionNonce::<T>::mutate(|nonce| *nonce += 1);

        if ContractsExecutionRegistry::<T>::contains_key(execution_id) {
            return Err(Error::<T>::ExecutionAlreadyRegistered.into())
        }

        // ContractsExecutionRegistry::<T>::insert(
        //     execution_id,
        //     ExecutionRegistryItem::new(payee.clone(), recipient.clone(), amount),
        // );

        let execution_id_hash: T::Hash =
            Decode::decode(&mut &Sabi::value_64_2_value_256(execution_id).encode()[..])
                .map_err(|_e| Error::<T>::DecodingExecutionIDFailed)?;

        Self::charge_requester(
            RequestCharge {
                requester: payee.clone(),
                offered_reward: Zero::zero(),
                charge_fee: amount,
                recipient: recipient.clone(),
            },
            execution_id_hash,
        )?;

        Ok(())
    }

    /// Charge requester for SFX submission: reward + fees.
    fn charge_requester(
        charge: RequestCharge<T::AccountId, BalanceOf<T>>,
        sfx_id: T::Hash,
    ) -> DispatchResult {
        T::Currency::reserve(&charge.requester, charge.charge_fee + charge.offered_reward)?;

        PendingChargesPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            sfx_id,
            charge.clone(),
        );

        Ok(())
    }

    fn finalize(execution_id: ExecutionId, reason: Option<Reason>) -> DispatchResult {
        // let item = Pallet::<T>::execution_registry(execution_id)
        //     .ok_or(Error::<T>::ExecutionNotRegistered)?;

        let charge_id: T::Hash = Decode::decode(&mut &Sabi::value_64_2_value_256(execution_id).encode()[..])
            .map_err(|_e| Error::<T>::DecodingExecutionIDFailed)?;

        let charge_request: RequestCharge<T::AccountId, BalanceOf<T>> =
            if let Some(pending_charge) =
            PendingChargesPerRound::<T>::get(T::CircuitClock::current_round(), charge_id)
            {
                pending_charge
            } else {
                return Err(Error::<T>::PendingChargeNotFoundAtCommit.into())
            };


        // Simple rules for splitting, for now, we take 1% to keep the account manager alive
        let (payee_split, recipient_split): (u8, u8) = match reason {
            None => (0, 99),
            Some(Reason::ContractReverted) => (89, 10),
            Some(Reason::UnexpectedFailure) => (49, 50),
        };

        let pay_split = |split: u8| -> BalanceOf<T> {
            if !split.is_zero() {
                let percent = Percent::from_percent(split);
                percent * *charge_request.charge_fee;
            } else {
                Zero::zero()
            }
        };

        let requster_refund_amount = pay_split(payee_split)?;
        T::Currency::slash_reserved(&charge_request.requester, charge_request.charge_fee + charge_request.offered_reward);
        T::Currency::resolve_creating(&charge_request.requester, requster_refund_amount);

        let recipient_amount = pay_split(recipient_split)?;

        T::Currency::resolve_creating(&T::EscrowAccount::get(), recipient_amount + charge_request.offered_reward);
        SfxSettlementsPerRound::<T>::insert(
            T::CircuitClock::current_round(),
            sfx_id,
            RequestCharge::<T::AccountId, BalanceOf<T>> {
                requester: T::EscrowAccount::get(),
                offered_reward: recipient_amount + charge_request.offered_reward,
                charge_fee: Zero::zero(),
                recipient: charge_request.recipient
            },
        );

        PendingChargesPerRound::<T>::remove(T::CircuitClock::current_round(), charge_id);

        Self::deposit_event(Event::ContractsRegistryExecutionFinalized { execution_id });

        Ok(())
    }

    /// Collect claimable (only SFX execution rewards) for Executors and Stakers submitted by Circuit at the duration of the current Round
    fn on_collect_claimable(
        _n: T::BlockNumber,
        r: RoundInfo<T::BlockNumber>,
    ) -> Result<Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>, &'static str> {
        let claimable_artifacts: Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>> = vec![];
        let mut active_set_claimables: Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>> =
            T::Executors::active_set()
                .into_iter()
                .map(|executor: T::AccountId| ActiveSetClaimablePerRound {
                    executor,
                    claimable: Zero::zero(),
                })
                .collect::<Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>>>();

        for sfx_settlement in SettlementsPerRound::<T>::iter_prefix_values(r) {
            // fixme: test that actually updates active_set_claimables or are the references wrong
            for active_set_claimable in active_set_claimables.iter_mut() {
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
        //         role: CircuitRole::Executor,
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
        //             role: CircuitRole::Staker,
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
