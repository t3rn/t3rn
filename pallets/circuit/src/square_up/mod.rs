use crate::*;
use frame_support::{ensure, traits::ExistenceRequirement};
use sp_runtime::DispatchResult;

#[cfg(test)]
pub mod test;

use sp_std::marker::PhantomData;
use t3rn_primitives::account_manager::RequestCharge;

pub struct SquareUp<T: Config> {
    _phantom: PhantomData<T>,
}

// A) fallible lock requester for all SFX max rewards @CircuitStatus::Requested
//
// B) fallible lock executor at bidding for SFX @CircuitStatus::PendingBidding | @CircuitStatus::InBidding
//
// C) fallible executor's attempt to execute SFX via XBI @CircuitStatus::PendingBidding
//
// D) (expected to be infallible) unreserve requester's max rewards and reserve requester's rewards for bid amounts @CircuitStatus::Ready
//
// E) infallible unreserve requester's max rewards and executor's bid amounts @CircuitStatus::Killed(_)
//
// F) infallible unreserve requester's max rewards and slash dishonest executors @CircuitStatus::Revert(_)
//
// G) infallible rewards payouts via AccountManager::finalize and infallible unlock executor's bonds @CircuitStatus::Finalize
impl<T: Config> SquareUp<T> {
    /// Fallible lock requester' max rewards for Xtx.
    pub fn try_request(local_ctx: &LocalXtxCtx<T, BalanceOf<T>>) -> DispatchResult {
        let fsx_array = Machine::<T>::read_current_step_fsx(local_ctx);
        let requester = local_ctx.xtx.requester.clone();

        if !fsx_array.iter().all(|fsx| {
            <T as Config>::AccountManager::can_withdraw(
                &requester,
                fsx.input.max_reward,
                fsx.input.reward_asset_id,
            )
        }) {
            log::error!(
                "AssetsFailedToWithdraw for asset id {:?} and max reward {:?} ",
                fsx_array[0].input.reward_asset_id,
                fsx_array[0].input.max_reward
            );
            return Err(Error::<T>::AssetsFailedToWithdraw.into())
        }

        let request_charges = fsx_array
            .iter()
            .map(|fsx| {
                (
                    fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id),
                    RequestCharge {
                        payee: requester.clone(),
                        offered_reward: fsx.input.max_reward,
                        charge_fee: Zero::zero(),
                        source: BenefitSource::TrafficFees,
                        // Assign the role as offset reward to executor.
                        role: CircuitRole::Executor,
                        recipient: None,
                        maybe_asset_id: fsx.input.reward_asset_id,
                    },
                )
            })
            .collect::<Vec<(T::Hash, RequestCharge<T::AccountId, BalanceOf<T>, u32>)>>();

        <T as Config>::AccountManager::deposit_batch(request_charges.as_slice())?;

        // Ensure that all deposits were successful and left associated under the SFX id.
        // This is a sanity check, as the next step during status transition to "Ready"
        //  will associate the deposits by SFX id with bidders and set the .enforce_execution field.
        ensure!(
            fsx_array.iter().all(|fsx| {
                <T as Config>::AccountManager::get_charge_or_fail(
                    fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id),
                )
                .is_ok()
            }),
            Error::<T>::SanityAfterCreatingSFXDepositsFailed
        );

        // Sum all finality fee estimates to all escrow targets, that would require attestations.
        let all_escrow_targets = fsx_array
            .iter()
            .filter(|fsx| fsx.security_lvl == SecurityLvl::Escrow)
            .map(|fsx| fsx.input.target)
            .collect::<Vec<TargetId>>();

        // FIXME: cannot sum
        let finality_fees_sum = all_escrow_targets
            .iter()
            .map(|target| T::Attesters::estimate_finality_fee(target))
            .collect::<Vec<BalanceOf<T>>>()
            .iter()
            .sum();

        // FIXME: cannot find the treasury account
        T::Currency::transfer(
            &requester,
            &T::TreasuryAccounts::get_treasury_account(TreasuryAccount::Fee),
            finality_fees_sum,
            ExistenceRequirement::KeepAlive,
        )
        .map_err(|_| Error::<T>::RequesterNotEnoughBalance.into())?;

        Ok(())
    }

    /// Fallible bidding attempt by executors.
    /// Input: LocalXtxCtx, bidder, bid_amount, bid_asset_id
    /// Output: Result<(), Error<T>>
    pub fn try_bid(
        sfx_id: T::Hash,
        requester: &T::AccountId,
        bidder: &T::AccountId,
        bid: &SFXBid<T::AccountId, BalanceOf<T>, u32>,
        current_best_bid: Option<SFXBid<T::AccountId, BalanceOf<T>, u32>>,
    ) -> DispatchResult {
        let total_bid_deposit = bid
            .reserved_bond
            .unwrap_or_else(Zero::zero)
            .checked_add(&bid.insurance)
            .ok_or(Error::<T>::ArithmeticErrorOverflow)?;

        match current_best_bid {
            Some(current_best_bid) => {
                if bid.amount >= current_best_bid.amount {
                    return Err(Error::<T>::BiddingRejectedBetterBidFound.into())
                }
                <T as Config>::AccountManager::transfer_deposit(
                    current_best_bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                    bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                    Some(total_bid_deposit),
                    Some(&bid.executor),
                    None,
                )
            },
            None => <T as Config>::AccountManager::deposit(
                bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                RequestCharge {
                    payee: bidder.clone(),
                    offered_reward: total_bid_deposit,
                    charge_fee: Zero::zero(),
                    source: BenefitSource::TrafficRewards,
                    role: CircuitRole::Executor,
                    recipient: Some(requester.clone()),
                    maybe_asset_id: bid.reward_asset_id,
                },
            ),
        }
    }

    /// Infallible re-balance requesters locked rewards after possibly lower bids are posted.
    pub fn bind_bidders(local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>) -> bool {
        let mut res: bool = false;

        let (current_step, _) = local_ctx.xtx.steps_cnt;

        let step_fsx = match local_ctx.full_side_effects.get_mut(current_step as usize) {
            Some(step_fsx) => step_fsx,
            None => local_ctx
                .full_side_effects
                .last_mut()
                .expect("read_current_step_fsx to have at least one step in FSX steps"),
        };
        for mut fsx in step_fsx.iter_mut() {
            let sfx_id = fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
            if let Some(bid) = &fsx.best_bid {
                if !<T as Config>::AccountManager::assign_deposit(sfx_id, &bid.executor) {
                    log::error!(
                        "assign_deposit: expect assign_deposit to succeed for sfx_id: {:?}",
                        sfx_id
                    );
                } else {
                    fsx.input.enforce_executor = Some(bid.executor.clone());
                    res = true;
                }
            } else {
                log::error!(
                    "bind_bidders: expect best_bid to be Some for sfx_id: {:?}",
                    sfx_id
                );
            }
        }

        res
    }

    /// Drop Xtx and unlock requester and all executors that posted bids - without penalties.
    pub fn kill(local_ctx: &LocalXtxCtx<T, BalanceOf<T>>) -> bool {
        let mut killed = false;
        for fsx in Machine::<T>::read_current_step_fsx(local_ctx).iter() {
            let sfx_id = fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
            if !<T as Config>::AccountManager::cancel_deposit(sfx_id) {
                log::error!(
                    "kill: expect cancel_deposit to succeed for sfx_id: {:?}",
                    sfx_id
                );
            } else {
                killed = true;
            }
            if let Some(bid) = &fsx.best_bid {
                if !<T as Config>::AccountManager::cancel_deposit(
                    bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                ) {
                    log::error!(
                        "kill: expect cancel_deposit to succeed for bid_id: {:?}",
                        bid.generate_id::<SystemHashing<T>, T>(sfx_id)
                    );
                }
            }
        }
        killed
    }

    /// Finalize Xtx after successful run.
    pub fn finalize(local_ctx: &LocalXtxCtx<T, BalanceOf<T>>) -> bool {
        let optimistic_fsx_in_step: Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
        > = Machine::<T>::read_current_step_fsx(local_ctx)
            .iter()
            .filter(|&fsx| fsx.security_lvl == SecurityLvl::Optimistic)
            .cloned()
            .collect();

        let mut finalized = true;

        let mut step_outcome = Outcome::Commit;

        for fsx in optimistic_fsx_in_step.iter() {
            let sfx_id = fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
            match &fsx.best_bid {
                Some(bid) => {
                    let outcome = match &fsx.confirmed {
                        // Revert deposits for honest SFX resolution
                        Some(_confirmed) => Outcome::Revert,
                        // Slash dishonest SFX resolution to Escrow Account
                        None => Outcome::Slash,
                    };
                    // If at least one SFX is not confirmed, then the whole XTX is reverted for requester
                    if outcome == Outcome::Slash {
                        step_outcome = Outcome::Revert;
                    }
                    if !<T as Config>::AccountManager::finalize_infallible(
                        bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                        outcome,
                    ) {
                        log::error!(
                            "revert: expect finalize_infallible to succeed for bid_id: {:?}",
                            bid.generate_id::<SystemHashing<T>, T>(sfx_id)
                        );
                        finalized = false;
                    }
                },
                None => {
                    log::error!(
                        "revert: disallowed state: reverting without fsx.best_bid assigned {:?}",
                        sfx_id
                    );
                    finalized = false;
                },
            }
        }
        // Finalize XTX for requester - charge all deposits or return all max_reward deposits back to requester.
        Machine::<T>::read_current_step_fsx(local_ctx)
            .iter()
            .for_each(|fsx| {
                let sfx_id = fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                if !<T as Config>::AccountManager::finalize_infallible(sfx_id, step_outcome.clone())
                {
                    log::error!(
                        "revert: expect finalize_infallible to succeed for sfx_id: {:?}",
                        sfx_id
                    );
                    finalized = false;
                }
            });
        finalized
    }

    /// Finalize Xtx after successful run - reward Escrow executors.
    pub fn commit(_local_ctx: &LocalXtxCtx<T, BalanceOf<T>>) {}
}
