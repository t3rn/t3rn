use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsRegistryExecutionNonce, Error,
    Outcome, Pallet, PendingChargesPerRound, SettlementsPerRound,
};

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{fungibles::Inspect, Get},
};
use sp_runtime::{
    traits::{CheckedAdd, CheckedDiv, CheckedMul, Zero},
    ArithmeticError, DispatchError,
};
use sp_std::{prelude::*, vec};

use t3rn_primitives::{
    account_manager::{RequestCharge, Settlement},
    claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
    clock::Clock,
    common::RoundInfo,
    executors::Executors,
};

use crate::monetary::Monetary;
use pallet_xbi_portal::sabi::Sabi;

pub struct ActiveSetClaimablePerRound<Account, Balance> {
    pub executor: Account,
    pub claimable: Balance,
}

pub fn percent_ratio<BalanceOf: Zero + CheckedDiv + CheckedMul + From<u8>>(
    amt: BalanceOf,
    percent: u8,
) -> Result<BalanceOf, DispatchError> {
    amt.checked_mul(&BalanceOf::from(percent))
        .ok_or::<DispatchError>("PercentRatio::ChargeOrSettlementCalculationOverflow".into())?
        .checked_div(&BalanceOf::from(100u8))
        .ok_or::<DispatchError>("PercentRatio::ChargeOrSettlementCalculationOverflow".into())
}

impl<T: Config>
    AccountManagerExt<
        T::AccountId,
        BalanceOf<T>,
        T::Hash,
        T::BlockNumber,
        <T::Assets as Inspect<T::AccountId>>::AssetId,
    > for Pallet<T>
{
    fn get_charge_or_fail(
        charge_id: T::Hash,
    ) -> Result<
        RequestCharge<T::AccountId, BalanceOf<T>, <T::Assets as Inspect<T::AccountId>>::AssetId>,
        DispatchError,
    > {
        if let Some(pending_charge) =
            PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id)
        {
            Ok(pending_charge)
        } else {
            Err(Error::<T>::NoChargeOfGivenIdRegistered.into())
        }
    }

    fn no_charge_or_fail(charge_id: T::Hash) -> Result<(), DispatchError> {
        if let Some(_pending_charge) =
            PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id)
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
        maybe_asset_id: Option<<T::Assets as Inspect<T::AccountId>>::AssetId>,
    ) -> DispatchResult {
        Self::no_charge_or_fail(charge_id).map_err(|_e| Error::<T>::ExecutionAlreadyRegistered)?;

        let total_reserve_deposit =
            if let Some(checked_reserve) = charge_fee.checked_add(&offered_reward) {
                checked_reserve
            } else {
                log::error!("Could nor compute collateral bond power, arithmetic overflow");
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };

        if total_reserve_deposit == Zero::zero() {
            return Err(Error::<T>::SkippingEmptyCharges.into())
        }

        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::withdraw(
            payee,
            total_reserve_deposit,
            maybe_asset_id,
        )?;

        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            // todo: Inspect if that's a good idea
            T::EscrowAccount::get()
        };

        PendingChargesPerRound::<T>::insert(
            T::Clock::current_round(),
            charge_id,
            RequestCharge {
                payee: payee.clone(),
                offered_reward,
                charge_fee,
                recipient: recipient.clone(),
                source,
                role,
                maybe_asset_id,
            },
        );

        Self::deposit_event(crate::Event::DepositReceived {
            charge_id,
            payee: payee.clone(),
            recipient,
            amount: total_reserve_deposit,
        });

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
        let (payee_split, recipient_split, recipient_bonus): (u8, u8, BalanceOf<T>) = match outcome
        {
            Outcome::Commit => (0, 99, Zero::zero()),
            Outcome::Revert => (99, 0, Zero::zero()),
            Outcome::UnexpectedFailure => (49, 50, Zero::zero()),
        };

        let total_reserved = charge.charge_fee + charge.offered_reward;

        let payee_refund: BalanceOf<T> = if let Some(actual_fees) = maybe_actual_fees {
            // ToDo: Better handle case when actual fees outgrow total_reserved
            if actual_fees > total_reserved {
                return Err(Error::<T>::ChargeOrSettlementActualFeesOutgrowReserved.into())
            }
            percent_ratio::<BalanceOf<T>>(total_reserved - actual_fees, payee_split)?
        } else {
            percent_ratio::<BalanceOf<T>>(total_reserved, payee_split)?
        };

        if payee_refund > Zero::zero() {
            Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
                &charge.payee,
                charge.maybe_asset_id,
                payee_refund,
            );
        }

        // Check if recipient has been updated
        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            charge.recipient
        };

        let recipient_rewards = percent_ratio::<BalanceOf<T>>(total_reserved, recipient_split)?;

        // Create Settlement for the future async claim
        if recipient_rewards > Zero::zero() {
            SettlementsPerRound::<T>::insert(
                T::Clock::current_round(),
                charge_id,
                Settlement::<T::AccountId, BalanceOf<T>> {
                    requester: charge.payee,
                    recipient,
                    settlement_amount: recipient_rewards + recipient_bonus,
                    outcome,
                    source: charge.source,
                    role: charge.role,
                },
            );
        }
        PendingChargesPerRound::<T>::remove(T::Clock::current_round(), charge_id);

        // Take what's left to treasury
        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
            &T::EscrowAccount::get(),
            charge.maybe_asset_id,
            total_reserved - payee_refund - recipient_rewards,
        );

        Ok(())
    }

    fn finalize_infallible(
        charge_id: T::Hash,
        outcome: Outcome,
        maybe_recipient: Option<T::AccountId>,
        maybe_actual_fees: Option<BalanceOf<T>>,
    ) {
        if PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id).is_some() {
            <Self as AccountManagerExt<
                T::AccountId,
                BalanceOf<T>,
                T::Hash,
                T::BlockNumber,
                <T::Assets as Inspect<T::AccountId>>::AssetId,
            >>::finalize(charge_id, outcome, maybe_recipient, maybe_actual_fees)
            .expect("Expect try finalize to be infallible");
        }
    }

    /// Collect claimable (only SFX execution rewards) for Executors and Stakers submitted by Circuit at the duration of the current Round
    fn on_collect_claimable(
        _n: T::BlockNumber,
        r: RoundInfo<T::BlockNumber>,
    ) -> Result<Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>, DispatchError> {
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

        for active_set_claimable in active_set_claimables {
            let collateral_bond = T::Executors::collateral_bond(&active_set_claimable.executor);
            let nominated_stake =
                T::Executors::total_nominated_stake(&active_set_claimable.executor);
            // calculate % ratio of rewards proportionally to Executor's own Collateral to Nominated Stake
            let total_stake_power =
                if let Some(checked_stake) = collateral_bond.checked_add(&nominated_stake) {
                    checked_stake
                } else {
                    log::error!("Could nor compute collateral bond power, arithmetic overflow");
                    return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                };

            // TODO: ensure it's in range (0,1>
            let round_claim = if let Some(collateral_bond_power) =
                collateral_bond.checked_div(&total_stake_power)
            {
                if let Some(total_round_claim) =
                    collateral_bond_power.checked_mul(&active_set_claimable.claimable)
                {
                    total_round_claim
                } else {
                    log::error!("Could nor compute collateral bond power, division by zero");
                    return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                }
            } else {
                log::error!("Could not compute total round claim, arithmetic overflow");
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };

            claimable_artifacts.push(ClaimableArtifacts {
                beneficiary: active_set_claimable.executor.clone(),
                role: CircuitRole::Executor,
                total_round_claim: round_claim,
                benefit_source: BenefitSource::TrafficRewards,
            });

            // TODO: ensure it's in range <0,1)
            let nominated_stake_power =
                if let Some(checked_nominated) = nominated_stake.checked_div(&total_stake_power) {
                    checked_nominated
                } else {
                    log::error!("Could nor compute collateral bond power, division by zero");
                    return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                };

            let claimable_by_all_stakers_of_executor = if let Some(checked_claimable) =
                nominated_stake_power.checked_mul(&active_set_claimable.claimable)
            {
                checked_claimable
            } else {
                log::error!("Could nor compute collateral bond power, arithmetic overflow");
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };

            for nominated_stake in T::Executors::stakes_per_executor(&active_set_claimable.executor)
            {
                let total_claim = if let Some(checked_nominated) = nominated_stake
                    .nominated_stake
                    .checked_div(&nominated_stake_power)
                {
                    if let Some(checked_claim) =
                        checked_nominated.checked_mul(&claimable_by_all_stakers_of_executor)
                    {
                        checked_claim
                    } else {
                        log::error!("Could nor compute collateral bond power, arithmetic overflow");
                        return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                    }
                } else {
                    log::error!("Could nor compute collateral bond power, division by zero");
                    return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                };
                claimable_artifacts.push(ClaimableArtifacts {
                    beneficiary: nominated_stake.staker,
                    role: CircuitRole::Staker,
                    total_round_claim: total_claim,
                    benefit_source: BenefitSource::TrafficRewards,
                });
            }
        }

        Ok(claimable_artifacts)
    }

    fn can_withdraw(
        payee: &T::AccountId,
        amount: BalanceOf<T>,
        asset_id: Option<<T::Assets as Inspect<T::AccountId>>::AssetId>,
    ) -> bool {
        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::can_withdraw(
            payee, asset_id, amount,
        )
    }

    fn deposit_immediately(
        beneficiary: &T::AccountId,
        amount: BalanceOf<T>,
        asset_id: Option<<T::Assets as Inspect<T::AccountId>>::AssetId>,
    ) {
        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
            beneficiary,
            asset_id,
            amount,
        )
    }

    fn withdraw_immediately(
        payee: &T::AccountId,
        amount: BalanceOf<T>,
        asset_id: Option<<T::Assets as Inspect<T::AccountId>>::AssetId>,
    ) -> DispatchResult {
        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::withdraw(
            payee, amount, asset_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_mock_runtime::*;

    use frame_support::{assert_err, assert_ok};

    pub use frame_support::traits::Currency;

    use sp_core::H256;
    use t3rn_primitives::{common::RoundInfo, Balance};

    const DEFAULT_BALANCE: Balance = 1_000_000;

    #[test]
    fn test_deposit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            const DEPOSIT_AMOUNT: Balance = DEFAULT_BALANCE / 10;
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                DEPOSIT_AMOUNT,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
                None,
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - DEPOSIT_AMOUNT
            );

            let charge_item = AccountManager::pending_charges_per_round::<
                RoundInfo<BlockNumber>,
                H256,
            >(Default::default(), execution_id)
            .unwrap();
            assert_eq!(charge_item.payee, ALICE);
            assert_eq!(charge_item.recipient, BOB);
            assert_eq!(charge_item.charge_fee, DEPOSIT_AMOUNT);
        });
    }

    #[test]
    fn test_deposit_when_already_exist_fails() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            const DEPOSIT_AMOUNT: Balance = DEFAULT_BALANCE / 10;

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                DEPOSIT_AMOUNT,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
                None,
            ));

            assert_err!(
                <AccountManager as AccountManagerExt<
                    AccountId,
                    Balance,
                    Hash,
                    BlockNumber,
                    AssetId,
                >>::deposit(
                    execution_id,
                    &ALICE,
                    DEPOSIT_AMOUNT,
                    0,
                    BenefitSource::TrafficRewards,
                    CircuitRole::ContractAuthor,
                    Some(BOB),
                    None,
                ),
                pallet_account_manager::Error::<Runtime>::ExecutionAlreadyRegistered
            );
        });
    }

    #[test]
    fn test_finalize_revert_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
                None,
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::finalize(
                execution_id, Outcome::Revert, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            let _ten_percent_charge_amt = charge_amt / 10;

            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - one_percent_charge_amt
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            // Expect no settlement at revert
            assert_eq!(
                AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );
        });
    }

    #[test]
    fn test_overflow_err_after_actual_fees_exceed_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            const CHARGE: Balance = 100;
            const INSURANCE: Balance = 10;

            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber, AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                CHARGE,
                INSURANCE,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB), None,
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - (CHARGE + INSURANCE));

            assert_err!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber, AssetId,
            >>::finalize(
                execution_id, Outcome::Revert, None, Some(CHARGE + INSURANCE + 1),
            ),
                circuit_runtime_pallets::pallet_account_manager::Error::<Runtime>::ChargeOrSettlementActualFeesOutgrowReserved,
            );
        });
    }

    #[test]
    fn percent_ratio_works_for_zero() {
        ExtBuilder::default().build().execute_with(|| {
            assert_eq!(percent_ratio::<Balance>(0, 100).unwrap(), 0);
            assert_eq!(percent_ratio::<Balance>(100, 0).unwrap(), 0);
            assert_eq!(percent_ratio::<Balance>(10, 100).unwrap(), 10);
            assert_eq!(percent_ratio::<Balance>(100, 10).unwrap(), 10);
        });
    }

    #[test]
    fn test_finalize_commit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
                None,
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::finalize(
                execution_id, Outcome::Commit, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );
            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            )
            .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(
                settlement.settlement_amount,
                charge_amt - one_percent_charge_amt
            );
        });
    }

    #[test]
    fn test_finalize_unexpected_failure_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
                None,
            ));

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::finalize(
                execution_id, Outcome::UnexpectedFailure, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            let fifty_percent_charge_amt = charge_amt / 100 * 50;

            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - fifty_percent_charge_amt - one_percent_charge_amt
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            )
            .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(settlement.settlement_amount, fifty_percent_charge_amt);
        });
    }
}
