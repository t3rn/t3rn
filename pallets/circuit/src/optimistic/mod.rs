use crate::{pallet::Error, *};
use sp_runtime::traits::Zero;

use crate::square_up::SquareUp;
use sp_std::marker::PhantomData;
use t3rn_primitives::{side_effect::SFXBid, transfers::EscrowedBalanceOf};

pub struct Optimistic<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Optimistic<T> {
    pub fn try_bid_4_sfx(
        current_fsx: &Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >,
        bidder: &T::AccountId,
        requester: &T::AccountId,
        bid: EscrowedBalanceOf<T, T::Escrowed>,
        sfx_id: SideEffectId<T>,
        xtx_id: T::Hash,
        current_accepted_bid: Option<SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>, u32>>,
    ) -> Result<SFXBid<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>, u32>, Error<T>> {
        let fsx = crate::Pallet::<T>::find_fsx_by_id(current_fsx, sfx_id, xtx_id)?;
        let (sfx_max_reward, sfx_security_lvl) = (fsx.input.max_reward, fsx.security_lvl.clone());
        // Check if bid doesn't go below dust
        if bid < <T::Escrowed as EscrowTrait<T>>::Currency::minimum_balance() {
            return Err(Error::<T>::BiddingRejectedBidBelowDust)
        }
        // Check if bid is attractive enough for requester
        if bid > sfx_max_reward {
            return Err(Error::<T>::BiddingRejectedBidTooHigh)
        }

        let mut sfx_bid =
            SFXBid::<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>, u32>::new_none_optimistic(
                bid,
                fsx.input.insurance,
                bidder.clone(),
                requester.clone(),
                fsx.input.reward_asset_id,
            );
        // Is the current bid for type SFX::Optimistic? If yes reserve the bond lock requirements
        if sfx_security_lvl == SecurityLvl::Optimistic {
            let total_xtx_step_optimistic_rewards_of_others =
                crate::Pallet::<T>::get_fsx_total_rewards(
                    &crate::Pallet::<T>::filter_fsx_by_security_lvl(
                        current_fsx,
                        SecurityLvl::Optimistic,
                    )
                    .into_iter()
                    .filter(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) != sfx_id)
                    .collect::<Vec<
                        FullSideEffect<
                            <T as frame_system::Config>::AccountId,
                            <T as frame_system::Config>::BlockNumber,
                            EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                        >,
                    >>(),
                );

            if total_xtx_step_optimistic_rewards_of_others > Zero::zero() {
                sfx_bid.reserved_bond = Some(total_xtx_step_optimistic_rewards_of_others);
            }
        }

        SquareUp::<T>::try_bid(sfx_id, requester, bidder, &sfx_bid, current_accepted_bid).map_err(
            |e| {
                log::error!("Error while trying to SquareUp::try_bid: {:?}", e);
                println!("Error while trying to SquareUp::try_bid: {:?}", e);
                Error::<T>::BiddingRejectedBetterBidFound
            },
        )?;

        Ok(sfx_bid)
    }

    pub fn try_unbond(local_ctx: &LocalXtxCtx<T>) -> Result<(), Error<T>> {
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

    pub fn try_slash(local_ctx: &LocalXtxCtx<T>) {
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
                let slashed_reserve: EscrowedBalanceOf<T, T::Escrowed> =
                    if let Some(v) = insurance.checked_add(&reserved_bond) {
                        v
                    } else {
                        log::error!("Could not compute slashed reserve");
                        return
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
                            insurance_plus_bond.checked_add(&sfx_bid.amount)
                        {
                            insurance_plus_bond_plus_bid
                        } else {
                            log::error!("Could not compute honest reward");
                            return
                        }
                    } else {
                        log::error!("Could not compute honest reward");
                        return
                    };
                <T as Config>::AccountManager::deposit_immediately(
                    &sfx_bid.executor,
                    checked_reward,
                    sfx_bid.reward_asset_id,
                )
            }
        }
    }

    pub fn try_dropped_at_bidding_refund(local_ctx: &LocalXtxCtx<T>) {
        for phase in local_ctx.full_side_effects.clone() {
            for fsx in phase {
                if fsx.is_bid_resolved() {
                    let sfx_bid = fsx.expect_sfx_bid();
                    let (insurance, reserved_bond) =
                        (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                    <T as Config>::AccountManager::deposit_immediately(
                        &sfx_bid.executor,
                        insurance + reserved_bond + sfx_bid.amount,
                        sfx_bid.reward_asset_id,
                    )
                }
            }
        }
    }
}
