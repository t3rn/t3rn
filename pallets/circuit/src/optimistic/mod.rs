use crate::{pallet::Error, *};

use frame_support::traits::ReservableCurrency;

use sp_std::marker::PhantomData;
use t3rn_primitives::{side_effect::SFXBid, transfers::EscrowedBalanceOf};

pub struct Optimistic<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Optimistic<T> {
    pub fn try_bid_4_sfx(
        local_ctx: &mut LocalXtxCtx<T>,
        executor: &T::AccountId,
        bid: EscrowedBalanceOf<T, T::Escrowed>,
        sfx_id: SideEffectId<T>,
        current_accepted_bid: Option<SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>>,
    ) -> Result<SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>, Error<T>> {
        // Check if Xtx is in the bidding state
        if local_ctx.xtx.status != CircuitStatus::PendingBidding {
            return Err(Error::<T>::BiddingInactive)
        }
        let fsx = crate::Pallet::<T>::recover_fsx_by_id(sfx_id, local_ctx)?;
        let (sfx_max_fee, sfx_security_lvl) = (fsx.input.max_fee, fsx.security_lvl);

        if bid > sfx_max_fee {
            return Err(Error::<T>::BiddingRejectedBidTooHigh)
        }

        if let Some(current_best_bid) = current_accepted_bid {
            if current_best_bid.bid < bid {
                return Err(Error::<T>::BiddingRejectedBetterBidFound)
            }
        }

        let mut sfx_bid =
            SFXBid::<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>::new_none_optimistic(
                bid,
                fsx.input.insurance,
                executor.clone(),
                local_ctx.xtx.requester.clone(),
            );
        // Is the current bid for type SFX::Optimistic? If yes reserve the bond lock requirements
        if sfx_security_lvl == SecurityLvl::Optimistic {
            sfx_bid = Self::bond_4_sfx(executor, local_ctx, &mut sfx_bid, sfx_id)?;
        }

        Ok(sfx_bid)
    }

    pub(self) fn bond_4_sfx(
        executor: &T::AccountId,
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_bid: &mut SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>,
        sfx_id: SideEffectId<T>,
    ) -> Result<SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>, Error<T>> {
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

        // todo/fixme: add insurance field to fsx.input.insurance
        let insurance = Zero::zero();

        <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(
            executor,
            insurance + total_xtx_step_optimistic_rewards_of_others,
        )
        .map_err(|_e| Error::<T>::BiddingFailedExecutorsBalanceTooLowToReserve)?;

        sfx_bid.insurance = insurance;
        sfx_bid.reserved_bond = Some(insurance);

        Ok(sfx_bid.clone())
    }

    pub fn try_unbond(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let optimistic_fsx_in_step = crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        for fsx in optimistic_fsx_in_step {
            let sfx_bid = fsx.expect_sfx_bid();
            let (insurance, reserved_bond) =
                (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

            <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                &sfx_bid.executor,
                insurance + reserved_bond,
            );
        }

        Ok(())
    }

    pub fn try_slash(local_ctx: &mut LocalXtxCtx<T>) {
        let mut slashed_reserve: EscrowedBalanceOf<T, T::Escrowed> = Zero::zero();
        let mut slashed_counter: usize = 0;

        let optimistic_fsx_in_step = &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        // Slash loop
        for fsx in optimistic_fsx_in_step {
            // Look for invalid FSX cases to slash
            if !fsx.is_successfully_confirmed() {
                let sfx_bid = fsx.expect_sfx_bid();
                let (insurance, reserved_bond) =
                    (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                // First slash executor
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::slash_reserved(
                    &sfx_bid.executor,
                    insurance + reserved_bond,
                );
                slashed_reserve += reserved_bond;
                slashed_counter += 1;
            }
        }

        let honest_counter = optimistic_fsx_in_step.len() - slashed_counter;
        let repatriation_bonus_per_honest_fsx =
            slashed_reserve / EscrowedBalanceOf::<T, T::Escrowed>::from(honest_counter as u32);

        // Repatriate loop
        for fsx in optimistic_fsx_in_step {
            // Look for valid FSX cases to repatriate
            if fsx.is_successfully_confirmed() {
                let sfx_bid = fsx.expect_sfx_bid();
                let (insurance, reserved_bond) =
                    (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                // First unlock honest executor
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                    &sfx_bid.executor,
                    insurance + reserved_bond,
                );
                // Repatriate slashed bonus since honest executor won't get rewards for the SFX::max_fee
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::deposit_creating(
                    &sfx_bid.executor,
                    repatriation_bonus_per_honest_fsx,
                );
            }
        }
    }
}
