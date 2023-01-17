use crate::{pallet::Error, *};
use sp_runtime::traits::Zero;

use sp_std::marker::PhantomData;
use t3rn_primitives::side_effect::SFXBid;

pub struct Optimistic<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Optimistic<T> {
    pub fn try_bid_4_sfx(
        local_ctx: &mut LocalXtxCtx<T>,
        executor: &T::AccountId,
        bid: BalanceOf<T>,
        sfx_id: SideEffectId<T>,
        current_accepted_bid: Option<SFXBid<T::AccountId, BalanceOf<T>, u32>>,
    ) -> Result<SFXBid<T::AccountId, BalanceOf<T>, u32>, Error<T>> {
        // Check if Xtx is in the bidding state
        if local_ctx.xtx.status != CircuitStatus::PendingBidding {
            return Err(Error::<T>::BiddingInactive)
        }
        let fsx = crate::Pallet::<T>::recover_fsx_by_id(sfx_id, local_ctx)?;
        let (sfx_max_reward, sfx_security_lvl) = (fsx.input.max_reward, fsx.security_lvl.clone());
        // Check if bid doesn't go below dust
        if bid < T::Currency::minimum_balance() {
            return Err(Error::<T>::BiddingRejectedBidBelowDust)
        }
        // Check if bid is attractive enough for requester
        if bid > sfx_max_reward {
            return Err(Error::<T>::BiddingRejectedBidTooHigh)
        }
        // Check if bid beats the previous ones
        if let Some(current_best_bid) = &current_accepted_bid {
            if current_best_bid.bid <= bid {
                return Err(Error::<T>::BiddingRejectedBetterBidFound)
            }
        }
        // Check if bid candidate has enough balance and reserve
        let checked_bid = if let Some(v) = bid.checked_add(&fsx.input.insurance) {
            v
        } else {
            return Err(Error::<T>::ArithmeticErrorOverflow)
        };
        <T as Config>::AccountManager::withdraw_immediately(
            executor,
            checked_bid,
            fsx.input.reward_asset_id,
        )
        .map_err(|_e| Error::<T>::BiddingRejectedExecutorNotEnoughBalance)?;

        let mut sfx_bid = SFXBid::<T::AccountId, BalanceOf<T>, u32>::new_none_optimistic(
            bid,
            fsx.input.insurance,
            executor.clone(),
            local_ctx.xtx.requester.clone(),
            fsx.input.reward_asset_id,
        );
        // Is the current bid for type SFX::Optimistic? If yes reserve the bond lock requirements
        if sfx_security_lvl == SecurityLvl::Optimistic {
            sfx_bid = Self::bond_4_sfx(executor, local_ctx, &mut sfx_bid, sfx_id)?;
        }

        // Un-reserve the funds of discarded bidder.
        // Warning: From this point on all of the next operations must be infallible.
        if let Some(current_best_bid) = &current_accepted_bid {
            let mut total_unreserve = if let Some(v) = current_best_bid
                .insurance
                .checked_add(&current_best_bid.bid)
            {
                v
            } else {
                return Err(Error::<T>::ArithmeticErrorOverflow)
            };
            if let Some(bond) = current_best_bid.reserved_bond {
                if let Some(v) = total_unreserve.checked_add(&bond) {
                    total_unreserve = v
                } else {
                    return Err(Error::<T>::ArithmeticErrorOverflow)
                }
            }
            <T as Config>::AccountManager::deposit_immediately(
                &current_best_bid.executor,
                total_unreserve,
                current_best_bid.reward_asset_id,
            )
        }

        Ok(sfx_bid)
    }

    pub(self) fn bond_4_sfx(
        executor: &T::AccountId,
        local_ctx: &mut LocalXtxCtx<T>,
        sfx_bid: &mut SFXBid<T::AccountId, BalanceOf<T>, u32>,
        sfx_id: SideEffectId<T>,
    ) -> Result<SFXBid<T::AccountId, BalanceOf<T>, u32>, Error<T>> {
        let total_xtx_step_optimistic_rewards_of_others = crate::Pallet::<T>::get_fsx_total_rewards(
            &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
                local_ctx,
                SecurityLvl::Optimistic,
            )
            .into_iter()
            .filter(|fsx| fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id) != sfx_id)
            .collect::<Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >>(),
        );

        if total_xtx_step_optimistic_rewards_of_others > Zero::zero() {
            <T as Config>::AccountManager::withdraw_immediately(
                executor,
                total_xtx_step_optimistic_rewards_of_others,
                sfx_bid.reward_asset_id,
            )
            .map_err(|_e| Error::<T>::BiddingRejectedExecutorNotEnoughBalance)?;
            sfx_bid.reserved_bond = Some(total_xtx_step_optimistic_rewards_of_others);
        }

        Ok(sfx_bid.clone())
    }

    pub fn try_unbond(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let optimistic_fsx_in_step = crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        for fsx in optimistic_fsx_in_step {
            if fsx.is_bid_resolved() {
                let sfx_bid = fsx.expect_sfx_bid();
                let (insurance, reserved_bond) =
                    (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                let checked_insurance = if let Some(v) = insurance.checked_add(&reserved_bond) {
                    v
                } else {
                    return Err(Error::<T>::ArithmeticErrorOverflow)
                };
                <T as Config>::AccountManager::deposit_immediately(
                    &sfx_bid.executor,
                    checked_insurance,
                    sfx_bid.reward_asset_id,
                )
            }
        }

        Ok(())
    }

    pub fn try_slash(local_ctx: &mut LocalXtxCtx<T>) -> Result<(), Error<T>> {
        let optimistic_fsx_in_step = &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );

        // Slash loop
        for fsx in optimistic_fsx_in_step {
            // Look for invalid FSX cases to slash
            if !fsx.is_successfully_confirmed() && fsx.is_bid_resolved() {
                let sfx_bid = fsx.expect_sfx_bid();
                let insurance = *sfx_bid.get_insurance();
                let reserved_bond = if let Some(bond) = sfx_bid.get_reserved_bond() {
                    *bond
                } else {
                    Zero::zero()
                };

                // ToDo: Introduce more sophisticated slashed rewards split between
                //  treasury, users, honest executors
                let slashed_reserve: BalanceOf<T> =
                    if let Some(v) = insurance.checked_add(&reserved_bond) {
                        v
                    } else {
                        return Err(Error::<T>::ArithmeticErrorOverflow)
                    };
                <T as Config>::AccountManager::deposit_immediately(
                    &T::SelfAccountId::get(),
                    slashed_reserve,
                    sfx_bid.reward_asset_id,
                )
            }
        }

        // Single reserved_bond consists out of Sum(N) sfxN.max_rewards, where N isn't executors' SFX index.
        // Repatriation therefore should always suffice to cover up the losses on targets by getting
        //  the bid amounts back to the honest executors
        // Repatriate loop
        for fsx in optimistic_fsx_in_step {
            // Look for valid FSX cases to repatriate
            if fsx.is_successfully_confirmed() && fsx.is_bid_resolved() {
                let sfx_bid = fsx.expect_sfx_bid();
                let (insurance, reserved_bond) =
                    (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                // First unlock honest executor  and the reward to honest executors
                // since the reserved bond was slashed and should always suffice.
                let checked_reward =
                    if let Some(insurance_plus_bond) = insurance.checked_add(&reserved_bond) {
                        if let Some(insurance_plus_bond_plus_bid) =
                            insurance_plus_bond.checked_add(&sfx_bid.bid)
                        {
                            insurance_plus_bond_plus_bid
                        } else {
                            log::error!("Could not compute honest reward");
                            return Err(Error::<T>::ArithmeticErrorOverflow)
                        }
                    } else {
                        log::error!("Could not compute honest reward");
                        return Err(Error::<T>::ArithmeticErrorOverflow)
                    };
                <T as Config>::AccountManager::deposit_immediately(
                    &sfx_bid.executor,
                    checked_reward,
                    sfx_bid.reward_asset_id,
                )
            }
        }
        Ok(())
    }

    pub fn try_dropped_at_bidding_refund(local_ctx: &mut LocalXtxCtx<T>) {
        for phase in local_ctx.full_side_effects.clone() {
            for fsx in phase {
                if fsx.is_bid_resolved() {
                    let sfx_bid = fsx.expect_sfx_bid();
                    let (insurance, reserved_bond) =
                        (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                    <T as Config>::AccountManager::deposit_immediately(
                        &sfx_bid.executor,
                        insurance + reserved_bond + sfx_bid.bid,
                        sfx_bid.reward_asset_id,
                    )
                }
            }
        }
    }
}
