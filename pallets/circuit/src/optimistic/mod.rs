use crate::{pallet::Error, *};

use frame_support::traits::ReservableCurrency;
use sp_std::marker::PhantomData;
use t3rn_primitives::transfers::EscrowedBalanceOf;
pub struct Optimistic<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Optimistic<T> {
    pub fn bond_4_sfx(
        // xtx_id: T::Hash,
        // step_id: T::Hash,
        executor: &T::AccountId,
        local_ctx: &LocalXtxCtx<T>,
        insurance_deposit: &InsuranceDeposit<
            T::AccountId,
            T::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >, // bond_amount: EscrowedBalanceOf<T, <T as Config>::Escrowed>,
           // total_xtx_step_optimistic_rewards: EscrowedBalanceOf<T, <T as Config>::Escrowed>,
    ) -> Result<
        InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        Error<T>,
    > {
        let total_xtx_step_optimistic_rewards = crate::Pallet::<T>::get_fsx_total_rewards(
            &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
                local_ctx,
                SecurityLvl::Optimistic,
            ),
        );

        <T as Config>::Executors::reserve_bond(executor, total_xtx_step_optimistic_rewards)
            .map_err(|_e| Error::<T>::InsuranceBondTooLow)?;
        <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(
            executor,
            insurance_deposit.insurance,
        );

        let mut insurance_deposit_copy = insurance_deposit.clone();
        log::info!("bond insurance deposit -- charged");
        insurance_deposit_copy.bonded_relayer = Some(executor.clone());
        insurance_deposit_copy.reserved_bond = total_xtx_step_optimistic_rewards;
        // ToDo: Consider removing status from insurance_deposit since redundant with relayer: Option<Relayer>
        insurance_deposit_copy.status = CircuitStatus::Bonded;

        // T::Executors::bond_collateral();
        // Cover co-executors by reserving a chunk of bonded collateral
        // <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve from bond_collateral the amount of all of the other SFX Optimistic Rewards in current Xtx Step.
        // <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(sum_of_all_optimistic_rewards_for_this_step - current_reward)
        // Cover users by reserving the requested bond amount from executor account.
        // <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(executioner, bond_amount)?;

        Ok(insurance_deposit_copy)
    }

    pub fn try_unbond(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let optimistic_fsx_in_step = crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        for fsx in optimistic_fsx_in_step.iter() {
            let side_effect_id = fsx.input.generate_id::<SystemHashing<T>>();
            return if let Some((_id, insurance_request)) = local_ctx
                .insurance_deposits
                .iter()
                .find(|(id, _)| *id == side_effect_id)
            {
                if let Some(bonded_relayer) = &insurance_request.bonded_relayer {
                    <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                        bonded_relayer,
                        insurance_request.insurance,
                    );
                    <T as Config>::Executors::unreserve_bond(
                        bonded_relayer,
                        insurance_request.reserved_bond,
                    );
                    Ok(())
                } else {
                    Err(Error::<T>::RefundTransferFailed)
                }
            } else {
                // This is a forbidden state which should have not happened -
                //  at this point all of the insurances should have a bonded relayer assigned
                Err(Error::<T>::RefundTransferFailed)
            }
        }

        //         let side_effect_id = side_effect.generate_id::<SystemHashing<T>>();
        //         // Reward insurance
        //         // Check if the side effect was insured and if the relayer matches the bonded one
        //         return if let Some((_id, insurance_request)) = local_ctx
        //             .insurance_deposits
        //             .iter()
        //             .find(|(id, _)| *id == side_effect_id)
        //         {
        //             if let Some(bonded_relayer) = &insurance_request.bonded_relayer {
        //                 match enact_status {
        //                     InsuranceEnact::Reward => {
        //                         // Reward relayer with and give back his insurance from Vault
        //                         EscrowCurrencyOf::<T>::transfer(
        //                             &Self::account_id(),
        //                             bonded_relayer,
        //                             insurance_request.insurance + insurance_request.reward,
        //                             AllowDeath,
        //                         )
        //                         .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail
        //                     },
        //                     InsuranceEnact::RefundBoth => {
        //                         EscrowCurrencyOf::<T>::transfer(
        //                             &Self::account_id(),
        //                             &insurance_request.requester,
        //                             insurance_request.reward,
        //                             AllowDeath,
        //                         )
        //                         .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
        //
        //                         EscrowCurrencyOf::<T>::transfer(
        //                             &Self::account_id(),
        //                             bonded_relayer,
        //                             insurance_request.insurance,
        //                             AllowDeath,
        //                         )
        //                         .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
        //                     },
        //                     InsuranceEnact::RefundAndPunish => {
        //                         EscrowCurrencyOf::<T>::transfer(
        //                             &Self::account_id(),
        //                             &insurance_request.requester,
        //                             insurance_request.reward,
        //                             AllowDeath,
        //                         )
        //                         .map_err(|_| Error::<T>::RefundTransferFailed)?; // should not fail
        //                     },
        //                 }
        //             } else {
        //                 // This is a forbidden state which should have not happened -
        //                 //  at this point all of the insurances should have a bonded relayer assigned
        //                 return Err(Error::<T>::RefundTransferFailed)
        //             }
        //             Ok(true)
        //         } else {
        //             Ok(false)
        //         }

        Ok(())
    }

    pub fn try_slash(local_ctx: &mut LocalXtxCtx<T>) {
        let optimistic_fsx_in_step = crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );

        let mut slashed_executors: Vec<(
            &T::AccountId,
            &InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        )> = vec![];
        let mut repatriated_executors: Vec<&T::AccountId> = vec![];

        for fsx in optimistic_fsx_in_step.iter() {
            let side_effect_id = fsx.input.generate_id::<SystemHashing<T>>();
            return if let Some((_id, insurance_request)) = local_ctx
                .insurance_deposits
                .iter()
                .find(|(id, _)| *id == side_effect_id)
            {
                if let Some(bonded_relayer) = &insurance_request.bonded_relayer {
                    if fsx.confirmed.is_some()
                        && fsx
                            .confirmed
                            .as_ref()
                            .expect("ensured exists in the same check")
                            .err
                            .is_none()
                    {
                        repatriated_executors.push(bonded_relayer)
                    } else {
                        slashed_executors.push((bonded_relayer, insurance_request))
                    }
                }
            }
        }

        for (slash_executor, slash_insurance_request) in slashed_executors.iter() {
            <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::slash_reserved(
                slash_executor,
                slash_insurance_request.insurance,
            );
            <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::deposit_creating(
                &slash_insurance_request.requester,
                slash_insurance_request.insurance,
            );
            <T as Config>::Executors::slash_bond(
                slash_executor,
                slash_insurance_request.reserved_bond,
            );

            for repatriated_executor in &repatriated_executors {
                <T as Config>::Executors::increase_bond(
                    repatriated_executor,
                    slash_insurance_request.reserved_bond, // / EscrowedBalanceOf<T, T::Escrowed > ::from(repatriated_executors.len() as u128),
                );
            }
        }
    }
}
