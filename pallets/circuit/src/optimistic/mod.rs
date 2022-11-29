use crate::{pallet::Error, *};
use frame_support::traits::fungible::Inspect;
use sp_runtime::traits::Zero;

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
        let (sfx_max_reward, sfx_security_lvl) = (fsx.input.max_reward, fsx.security_lvl.clone());
        // Check if bid doesn't go below dust
        if bid < <T::Escrowed as EscrowTrait<T>>::Currency::minimum_balance() {
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
        <T::Escrowed as EscrowTrait<T>>::Currency::reserve(executor, bid + fsx.input.insurance)
            .map_err(|_e| Error::<T>::BiddingRejectedExecutorNotEnoughBalance)?;

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

        // Un-reserve the funds of discarded bidder.
        // Warning: From this point on all of the next operations must be infallible.
        if let Some(current_best_bid) = &current_accepted_bid {
            let mut total_unreserve = current_best_bid.insurance + current_best_bid.bid;
            if let Some(bond) = current_best_bid.reserved_bond {
                total_unreserve += bond;
            }
            <T::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                &current_best_bid.executor,
                total_unreserve,
            );
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
            .filter(|fsx| fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id) != sfx_id)
            .collect::<Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, <T as Config>::Escrowed>,
                >,
            >>(),
        );

        if total_xtx_step_optimistic_rewards_of_others > Zero::zero() {
            <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::reserve(
                executor,
                total_xtx_step_optimistic_rewards_of_others,
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

                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                    &sfx_bid.executor,
                    insurance + reserved_bond,
                );
            }
        }

        Ok(())
    }

    pub fn try_slash(local_ctx: &mut LocalXtxCtx<T>) {
        let mut slashed_reserve: EscrowedBalanceOf<T, T::Escrowed> = Zero::zero();

        let optimistic_fsx_in_step = &crate::Pallet::<T>::get_current_step_fsx_by_security_lvl(
            local_ctx,
            SecurityLvl::Optimistic,
        );
        // Slash loop
        for fsx in optimistic_fsx_in_step {
            // Look for invalid FSX cases to slash
            if !fsx.is_successfully_confirmed() && fsx.is_bid_resolved() {
                let sfx_bid = fsx.expect_sfx_bid();
                let (insurance, reserved_bond) =
                    (*sfx_bid.get_insurance(), *sfx_bid.expect_reserved_bond());

                // First slash executor
                slashed_reserve += insurance + reserved_bond;
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::slash_reserved(
                    &sfx_bid.executor,
                    insurance + reserved_bond,
                );
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

                // First unlock honest executor
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::unreserve(
                    &sfx_bid.executor,
                    insurance + reserved_bond,
                );
                // Repatriate the reward to honest executors since the reserved bond was slashed and should always suffice
                slashed_reserve -= sfx_bid.bid;
                <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::deposit_creating(
                    &sfx_bid.executor,
                    sfx_bid.bid,
                );
            }
        }

        // ToDo: Introduce more sophisticated slashed rewards split between
        //  treasury, users, honest executors
        if slashed_reserve > Zero::zero() {
            <<T as Config>::Escrowed as EscrowTrait<T>>::Currency::deposit_creating(
                &T::SelfAccountId::get(),
                slashed_reserve,
            );
        }
    }
}
