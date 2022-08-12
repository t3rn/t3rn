use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsRegistryExecutionNonce, Error,
    Outcome, Pallet, PendingChargesPerRound, SettlementsPerRound,
};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, Get, ReservableCurrency},
};
use sp_runtime::{traits::Zero, DispatchError};

use sp_runtime::traits::{CheckedDiv, CheckedMul};

use t3rn_primitives::{
    account_manager::{RequestCharge, Settlement},
    claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
    clock::Clock,
    common::RoundInfo,
    executors::Executors,
};

use pallet_xbi_portal::sabi::Sabi;

pub struct ActiveSetClaimablePerRound<Account, Balance> {
    pub executor: Account,
    pub claimable: Balance,
}

fn percent_ratio<T: Config>(amt: BalanceOf<T>, percent: u8) -> Result<BalanceOf<T>, DispatchError> {
    amt.checked_mul(&BalanceOf::<T>::from(percent))
        .ok_or::<DispatchError>(Error::<T>::ChargeOrSettlementCalculationOverflow.into())?
        .checked_div(&BalanceOf::<T>::from(100u8))
        .ok_or::<DispatchError>(Error::<T>::ChargeOrSettlementCalculationOverflow.into())
}

impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>, T::Hash, T::BlockNumber>
for Pallet<T>
{
    fn get_charge_or_fail(
        charge_id: T::Hash,
    ) -> Result<RequestCharge<T::AccountId, BalanceOf<T>>, DispatchError> {
        return if let Some(pending_charge) =
        PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id)
        {
            Ok(pending_charge)
        } else {
            Err(Error::<T>::ChargeAlreadyRegistered.into())
        }
    }

    fn no_charge_or_fail(charge_id: T::Hash) -> Result<(), DispatchError> {
        return if let Some(_pending_charge) =
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
            T::Clock::current_round(),
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
        let (payee_split, recipient_split, recipient_bonus): (u8, u8, BalanceOf<T>) = match outcome
        {
            Outcome::Commit => (0, 99, charge.offered_reward),
            Outcome::Revert => (89, 10, Zero::zero()),
            Outcome::UnexpectedFailure => (49, 50, Zero::zero()),
        };

        let payee_refund: BalanceOf<T> = if let Some(actual_fees) = maybe_actual_fees {
            percent_ratio::<T>(charge.charge_fee - actual_fees, payee_split)?
        } else {
            percent_ratio::<T>(charge.charge_fee, payee_split)?
        };

        T::Currency::slash_reserved(&charge.payee, charge.charge_fee + charge.offered_reward);
        T::Currency::deposit_creating(&charge.payee, payee_refund.clone());

        // Check if recipient has been updated
        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            charge.recipient
        };

        let recipient_fee_rewards = percent_ratio::<T>(charge.charge_fee, recipient_split)?;

        // Create Settlement for the future async claim
        SettlementsPerRound::<T>::insert(
            T::Clock::current_round(),
            charge_id,
            Settlement::<T::AccountId, BalanceOf<T>> {
                requester: charge.payee,
                recipient,
                settlement_amount: recipient_fee_rewards + recipient_bonus,
                outcome,
                source: charge.source,
                role: charge.role,
            },
        );

        PendingChargesPerRound::<T>::remove(T::Clock::current_round(), charge_id);

        // Take what's left - 1% to keep the account manager alive
        T::Currency::deposit_creating(
            &T::EscrowAccount::get(),
            charge.charge_fee - recipient_fee_rewards - payee_refund,
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::{assert_err, assert_ok};

    use sp_core::H256;
    use t3rn_primitives::common::RoundInfo;

    const DEFAULT_BALANCE: u64 = 1_000_000;

    #[test]
    fn test_deposit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                DEFAULT_BALANCE / 10,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), DEFAULT_BALANCE / 10);

            let charge_item = AccountManager::pending_charges_per_round::<RoundInfo<u64>, H256>(
                Default::default(),
                execution_id,
            )
                .unwrap();
            assert_eq!(charge_item.payee, ALICE);
            assert_eq!(charge_item.recipient, BOB);
            assert_eq!(charge_item.charge_fee, DEFAULT_BALANCE / 10);
        });
    }

    #[test]
    fn test_deposit_when_already_exist_fails() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                DEFAULT_BALANCE / 10,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_err!(
                <AccountManager as AccountManagerExt<
                    AccountId,
                    BalanceOf<Test>,
                    Hash,
                    BlockNumber,
                >>::deposit(
                    execution_id,
                    &ALICE,
                    DEFAULT_BALANCE / 10,
                    0,
                    BenefitSource::TrafficRewards,
                    CircuitRole::ContractAuthor,
                    Some(BOB),
                ),
                Error::<Test>::ExecutionAlreadyRegistered
            );
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
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::Revert, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            let ten_percent_charge_amt = charge_amt / 10;

            assert_eq!(
                Balances::free_balance(&<Test as Config>::EscrowAccount::get()),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - ten_percent_charge_amt - one_percent_charge_amt
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<u64>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<u64>, H256>(
                Default::default(),
                execution_id,
            )
                .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(settlement.settlement_amount, ten_percent_charge_amt);
        });
    }

    #[test]
    fn test_finalize_commit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::Commit, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            assert_eq!(
                Balances::free_balance(&<Test as Config>::EscrowAccount::get()),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );
            assert_eq!(Balances::free_balance(&ALICE), DEFAULT_BALANCE - charge_amt);

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<u64>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<u64>, H256>(
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
                &<Test as Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                BalanceOf<Test>,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::UnexpectedFailure, None, None,
            ));

            let one_percent_charge_amt = charge_amt / 100;
            let fifty_percent_charge_amt = charge_amt / 100 * 50;

            assert_eq!(
                Balances::free_balance(&<Test as Config>::EscrowAccount::get()),
                one_percent_charge_amt + DEFAULT_BALANCE // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE - fifty_percent_charge_amt - one_percent_charge_amt
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<u64>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<u64>, H256>(
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
