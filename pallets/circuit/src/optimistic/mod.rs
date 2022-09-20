use crate::{pallet::Error, *};

use frame_support::traits::ReservableCurrency;
use sp_std::marker::PhantomData;
use t3rn_primitives::transfers::EscrowedBalanceOf;

pub struct Optimistic<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Optimistic<T> {
    pub fn bond_4_sfx(
        executor: &T::AccountId,
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_id: SideEffectId<T>,
    ) -> Result<
        InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        Error<T>,
    > {
        let total_xtx_step_optimistic_rewards_of_others = crate::Pallet::<T>::get_fsx_total_rewards(
            &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
                local_ctx,
                SecurityLvl::Optimistic,
            )
            .into_iter()
            .filter(|fsx| fsx.input.generate_id::<SystemHashing<T>>() != sfx_id)
            .collect::<Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                >,
            >>(),
        );
        let mut insurance_deposit = Self::get_insurance_deposit_mutable_ref(local_ctx, sfx_id)?;

        <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(
            executor,
            insurance_deposit.insurance,
        )
        .map_err(|_e| Error::<T>::InsuranceBondTooLow)?;

        <T as Config>::Executors::reserve_bond(
            executor,
            total_xtx_step_optimistic_rewards_of_others,
        )
        .map_err(|_e| Error::<T>::InsuranceBondTooLow)?;

        insurance_deposit.bonded_relayer = Some(executor.clone());
        insurance_deposit.reserved_bond = total_xtx_step_optimistic_rewards_of_others;
        // ToDo: Consider removing status from insurance_deposit since redundant with relayer: Option<Relayer>
        insurance_deposit.status = CircuitStatus::Bonded;

        Ok(insurance_deposit.clone())
    }

    pub fn try_unbond(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let optimistic_fsx_in_step = crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        for fsx in &optimistic_fsx_in_step {
            let side_effect_id = fsx.input.generate_id::<SystemHashing<T>>();
            if let Some((_id, insurance_request)) = local_ctx
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
                } else {
                    return Err(Error::<T>::RefundTransferFailed)
                }
            } else {
                // This is a forbidden state which should have not happened -
                //  at this point all of the insurances should have a bonded relayer assigned
                return Err(Error::<T>::RefundTransferFailed)
            }
        }
        Ok(())
    }

    pub(self) fn get_insurance_deposit_mutable_ref(
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_id: SideEffectId<T>,
    ) -> Result<
        &mut InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        Error<T>,
    > {
        let mut maybe_insurance_deposit: Option<
            &mut InsuranceDeposit<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        > = None;

        for (id, x) in local_ctx.insurance_deposits.iter_mut() {
            if *id == sfx_id {
                maybe_insurance_deposit = Some(x);
            }
        }
        if let Some(insurance_deposit) = maybe_insurance_deposit {
            Ok(insurance_deposit)
        } else {
            Err(Error::<T>::InsuranceBondNotRequired)
        }
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

        for fsx in &optimistic_fsx_in_step {
            let side_effect_id = fsx.input.generate_id::<SystemHashing<T>>();
            if let Some((_id, insurance_request)) = local_ctx
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
                    slash_insurance_request.reserved_bond,
                );
            }
        }
    }
}
