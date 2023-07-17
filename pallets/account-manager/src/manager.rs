use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsRegistryExecutionNonce, Error,
    Outcome, Pallet, PendingCharges, SettlementsPerRound,
};

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{fungibles::Inspect, Get},
};
use sp_runtime::{
    traits::{CheckedAdd, CheckedDiv, CheckedMul, Convert, Zero},
    ArithmeticError, DispatchError,
};
use sp_std::prelude::*;

use t3rn_primitives::{
    account_manager::{RequestCharge, Settlement},
    claimable::CircuitRole,
    clock::Clock,
};

use crate::monetary::Monetary;
use substrate_abi::{SubstrateAbiConverter as Sabi, Value256};

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
        if let Some(pending_charge) = PendingCharges::<T>::get(charge_id) {
            Ok(pending_charge)
        } else {
            Err(Error::<T>::NoChargeOfGivenIdRegistered.into())
        }
    }

    fn no_charge_or_fail(charge_id: T::Hash) -> Result<(), DispatchError> {
        if let Some(_pending_charge) = PendingCharges::<T>::get(charge_id) {
            Err(Error::<T>::ChargeAlreadyRegistered.into())
        } else {
            Ok(())
        }
    }

    fn get_settlement(
        settlement_id: T::Hash,
    ) -> Option<Settlement<T::AccountId, BalanceOf<T>, <T::Assets as Inspect<T::AccountId>>::AssetId>>
    {
        SettlementsPerRound::<T>::get(T::Clock::current_round(), settlement_id)
    }

    fn get_settlements_by_role(
        role: CircuitRole,
    ) -> Vec<(
        T::AccountId,
        Settlement<T::AccountId, BalanceOf<T>, <T::Assets as Inspect<T::AccountId>>::AssetId>,
    )> {
        let settlements = SettlementsPerRound::<T>::iter_prefix_values(T::Clock::current_round());
        settlements
            .filter(|settlement| settlement.role == role)
            .map(|settlement| (settlement.recipient.clone(), settlement))
            .collect()
    }

    fn bump_contracts_registry_nonce() -> Result<T::Hash, DispatchError> {
        let execution_id = ContractsRegistryExecutionNonce::<T>::get();
        ContractsRegistryExecutionNonce::<T>::mutate(|nonce| *nonce += 1);

        let xtx_id: Value256 = Sabi::convert(execution_id);
        let charge_id = Decode::decode(&mut &xtx_id.encode()[..])
            .map_err(|_e| Error::<T>::DecodingExecutionIDFailed)?;

        Self::no_charge_or_fail(charge_id)?;
        Ok(charge_id)
    }

    fn validate_deposit(
        charge_id: T::Hash,
        request_charge: RequestCharge<
            T::AccountId,
            BalanceOf<T>,
            <T::Assets as Inspect<T::AccountId>>::AssetId,
        >,
    ) -> Result<BalanceOf<T>, DispatchError> {
        Self::no_charge_or_fail(charge_id).map_err(|_e| Error::<T>::ExecutionAlreadyRegistered)?;

        let total_reserve_deposit = if let Some(checked_reserve) = request_charge
            .charge_fee
            .checked_add(&request_charge.offered_reward)
        {
            checked_reserve
        } else {
            log::error!("Could nor compute collateral bond power, arithmetic overflow");
            return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
        };

        if total_reserve_deposit == Zero::zero() {
            return Err(Error::<T>::SkippingEmptyCharges.into())
        }

        Ok(total_reserve_deposit)
    }

    fn deposit_batch(
        batch: &[(
            T::Hash,
            RequestCharge<
                T::AccountId,
                BalanceOf<T>,
                <T::Assets as Inspect<T::AccountId>>::AssetId,
            >,
        )],
    ) -> DispatchResult {
        let mut validated_requests: Vec<(
            T::Hash,
            &RequestCharge<
                T::AccountId,
                BalanceOf<T>,
                <T::Assets as Inspect<T::AccountId>>::AssetId,
            >,
            BalanceOf<T>,
        )> = Vec::with_capacity(batch.len());

        for (charge_id, request_charge) in batch {
            validated_requests.push((
                *charge_id,
                request_charge,
                Self::validate_deposit(*charge_id, request_charge.clone())?,
            ));
        }

        for (charge_id, request_charge, total_deposit) in validated_requests {
            Self::withdraw_immediately(
                &request_charge.payee,
                total_deposit,
                request_charge.maybe_asset_id,
            )?;
            PendingCharges::<T>::insert(charge_id, request_charge.clone());
            Self::deposit_event(crate::Event::DepositReceived {
                charge_id,
                payee: request_charge.payee.clone(),
                recipient: request_charge.recipient.clone(),
                amount: total_deposit,
            });
        }

        Ok(())
    }

    /// If Called by 3VM as a execution deposit, expect:
    ///     - charge = gas_fees
    ///     - reward = 0
    /// If Called by Circuit as charge deposit, expect:
    ///     - charge = std SFX execution + delivery charge
    ///     - reward = Open Market based offered by requester
    fn deposit(
        charge_id: T::Hash,
        request_charge: RequestCharge<
            T::AccountId,
            BalanceOf<T>,
            <T::Assets as Inspect<T::AccountId>>::AssetId,
        >,
    ) -> DispatchResult {
        Self::deposit_batch(&[(charge_id, request_charge)])
    }

    fn finalize(
        charge_id: T::Hash,
        outcome: Outcome,
        _maybe_recipient: Option<T::AccountId>,
        _maybe_actual_fees: Option<BalanceOf<T>>,
    ) -> DispatchResult {
        let _ = Self::finalize_infallible(charge_id, outcome);
        Ok(())
    }

    fn finalize_infallible(charge_id: T::Hash, outcome: Outcome) -> bool {
        if let Some(charge) = PendingCharges::<T>::get(charge_id) {
            // Infallible recipient assignment to Escrow
            let recipient = match charge.recipient {
                Some(recipient) => recipient,
                None => T::EscrowAccount::get(),
            };
            if charge.offered_reward > Zero::zero() {
                match outcome {
                    Outcome::Commit => {
                        SettlementsPerRound::<T>::insert(
                            T::Clock::current_round(),
                            charge_id,
                            Settlement::<
                                T::AccountId,
                                BalanceOf<T>,
                                <T::Assets as Inspect<T::AccountId>>::AssetId,
                            > {
                                requester: charge.payee,
                                recipient,
                                settlement_amount: charge.offered_reward,
                                outcome,
                                source: charge.source,
                                role: charge.role,
                                maybe_asset_id: charge.maybe_asset_id,
                            },
                        );
                    },
                    Outcome::Slash => {
                        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
                            &T::EscrowAccount::get(),
                            charge.maybe_asset_id,
                            charge.offered_reward,
                        );
                    },
                    Outcome::UnexpectedFailure | Outcome::Revert => {
                        Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
                            &charge.payee,
                            charge.maybe_asset_id,
                            charge.offered_reward,
                        );
                    },
                };
            }

            // Take charge fee to treasury
            if charge.charge_fee > Zero::zero() {
                Monetary::<T::AccountId, T::Assets, T::Currency, T::AssetBalanceOf>::deposit(
                    &T::EscrowAccount::get(),
                    charge.maybe_asset_id,
                    charge.charge_fee,
                );
            }
            PendingCharges::<T>::remove(charge_id);
            true
        } else {
            false
        }
    }

    fn cancel_deposit(charge_id: T::Hash) -> bool {
        match PendingCharges::<T>::get(charge_id) {
            Some(charge) => {
                Self::deposit_immediately(
                    &charge.payee,
                    charge.offered_reward,
                    charge.maybe_asset_id,
                );
                PendingCharges::<T>::remove(charge_id);
                true
            },
            None => false,
        }
    }

    fn assign_deposit(charge_id: T::Hash, recipient: &T::AccountId) -> bool {
        PendingCharges::<T>::mutate(charge_id, |maybe_charge| match maybe_charge {
            Some(charge) => {
                charge.recipient = Some(recipient.clone());
                true
            },
            None => false,
        })
    }

    fn transfer_deposit(
        charge_id: T::Hash,
        new_charge_id: T::Hash,
        new_reward: Option<BalanceOf<T>>,
        new_payee: Option<&T::AccountId>,
        new_recipient: Option<&T::AccountId>,
    ) -> DispatchResult {
        match PendingCharges::<T>::get(charge_id) {
            Some(charge) => {
                let offered_reward = if let Some(reward) = new_reward {
                    reward
                } else {
                    charge.offered_reward
                };

                let payee = if let Some(payee) = new_payee {
                    payee.clone()
                } else {
                    charge.payee
                };

                let recipient = if let Some(recipient) = new_recipient {
                    Some(recipient.clone())
                } else {
                    charge.recipient
                };

                // Release previous payee
                if !Self::cancel_deposit(charge_id) {
                    return Err(Error::<T>::TransferDepositFailedToReleasePreviousCharge.into())
                }

                <Self as AccountManagerExt<
                    T::AccountId,
                    BalanceOf<T>,
                    T::Hash,
                    T::BlockNumber,
                    <T::Assets as Inspect<T::AccountId>>::AssetId,
                >>::deposit(
                    new_charge_id,
                    RequestCharge {
                        payee,
                        offered_reward,
                        charge_fee: charge.charge_fee,
                        role: charge.role,
                        source: charge.source,
                        recipient,
                        maybe_asset_id: charge.maybe_asset_id,
                    },
                )
            },
            None => Err(Error::<T>::TransferDepositFailedOldChargeNotFound.into()),
        }
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
    use t3rn_primitives::{
        claimable::{BenefitSource, CircuitRole},
        common::RoundInfo,
        Balance,
    };

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
                RequestCharge {
                    payee: ALICE,
                    offered_reward: 0,
                    charge_fee: DEPOSIT_AMOUNT,
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(BOB),
                    maybe_asset_id: None
                }
            ));

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - DEPOSIT_AMOUNT
            );

            let charge_item =
                AccountManager::pending_charges_per_round::<H256>(execution_id).unwrap();
            assert_eq!(charge_item.payee, ALICE);
            assert_eq!(charge_item.recipient, Some(BOB));
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
                RequestCharge {
                    payee: ALICE,
                    offered_reward: DEPOSIT_AMOUNT,
                    charge_fee: 0,
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(BOB),
                    maybe_asset_id: None
                }
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
                    RequestCharge {
                        payee: ALICE,
                        offered_reward: DEPOSIT_AMOUNT,
                        charge_fee: 0,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::ContractAuthor,
                        recipient: Some(BOB),
                        maybe_asset_id: None
                    }
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
                RequestCharge {
                    payee: ALICE,
                    offered_reward: charge_amt,
                    charge_fee: 0,
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(BOB),
                    maybe_asset_id: None
                }
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

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);

            assert_eq!(
                AccountManager::pending_charges_per_round::<H256>(execution_id,),
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
                RequestCharge {
                    payee: ALICE,
                    offered_reward: charge_amt,
                    charge_fee: 0,
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(BOB),
                    maybe_asset_id: None
                }
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

            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_eq!(
                AccountManager::pending_charges_per_round::<H256>(execution_id,),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            )
            .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(settlement.settlement_amount, charge_amt);
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
                RequestCharge {
                    payee: ALICE,
                    offered_reward: charge_amt,
                    charge_fee: 0,
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::ContractAuthor,
                    recipient: Some(BOB),
                    maybe_asset_id: None
                }
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

            let _one_percent_charge_amt = charge_amt / 100;
            let _fifty_percent_charge_amt = charge_amt / 100 * 50;

            // User gets 100% of their reward back if the circuit fails unexpectedly
            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE);

            assert_eq!(
                AccountManager::pending_charges_per_round::<H256>(execution_id,),
                None
            );

            // Expect no settlement at revert
            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            );

            assert_eq!(settlement, None);
        });
    }
}
