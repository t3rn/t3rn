use crate::*;
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
    pub fn try_request(local_ctx: &LocalXtxCtx<T>) -> DispatchResult {
        let fsx_array = Machine::<T>::read_current_step_fsx(local_ctx);
        let requester = local_ctx.xtx.requester.clone();

        if !fsx_array.iter().all(|fsx| {
            <T as Config>::AccountManager::can_withdraw(
                &requester,
                fsx.input.max_reward,
                fsx.input.reward_asset_id,
            )
        }) {
            return Err(Error::<T>::RequesterNotEnoughBalance.into())
        }

        let request_charges = fsx_array
            .iter()
            .map(|fsx| {
                (
                    fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id),
                    RequestCharge {
                        payee: requester.clone(),
                        offered_reward: fsx.input.max_reward,
                        charge_fee: Zero::zero(),
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor,
                        recipient: None,
                        maybe_asset_id: fsx.input.reward_asset_id,
                    },
                )
            })
            .collect::<Vec<(T::Hash, RequestCharge<T::AccountId, BalanceOf<T>, u32>)>>();

        <T as Config>::AccountManager::deposit_batch(request_charges)?;

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
            .amount
            .checked_add(&bid.reserved_bond.unwrap_or(Zero::zero()))
            .ok_or(Error::<T>::ArithmeticErrorOverflow)?
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
    pub fn bind_bidders(local_ctx: &LocalXtxCtx<T>) {
        for fsx in Machine::<T>::read_current_step_fsx(local_ctx).iter() {
            let sfx_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
            if let Some(bid) = &fsx.best_bid {
                if !<T as Config>::AccountManager::assign_deposit(sfx_id, &bid.executor) {
                    log::error!(
                        "assign_deposit: expect assign_deposit to succeed for sfx_id: {:?}",
                        sfx_id
                    );
                }
            } else {
                log::error!(
                    "bind_bidders: expect best_bid to be Some for sfx_id: {:?}",
                    sfx_id
                );
            }
        }
    }

    /// Drop Xtx and unlock requester and all executors that posted bids - without penalties.
    pub fn kill(local_ctx: &LocalXtxCtx<T>) {
        for fsx in Machine::<T>::read_current_step_fsx(local_ctx).iter() {
            let sfx_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
            if !<T as Config>::AccountManager::cancel_deposit(sfx_id) {
                log::error!(
                    "kill: expect cancel_deposit to succeed for sfx_id: {:?}",
                    sfx_id
                );
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
    }

    /// Finalize Xtx after successful run.
    pub fn finalize(local_ctx: &LocalXtxCtx<T>) {
        let optimistic_fsx_in_step: Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        > = Machine::<T>::read_current_step_fsx(local_ctx)
            .iter()
            .filter(|&fsx| fsx.security_lvl == SecurityLvl::Optimistic)
            .cloned()
            .collect();

        for fsx in optimistic_fsx_in_step.iter() {
            let sfx_id = fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id);

            match &fsx.best_bid {
                Some(bid) => {
                    if !<T as Config>::AccountManager::finalize_infallible(
                        bid.generate_id::<SystemHashing<T>, T>(sfx_id),
                        match &fsx.confirmed {
                            // Revert deposits for honest SFX resolution
                            Some(_confirmed) => Outcome::Revert,
                            // Slash dishonest SFX resolution to Escrow Account
                            None => Outcome::Slash,
                        },
                    ) {
                        log::error!(
                            "revert: expect finalize_infallible to succeed for bid_id: {:?}",
                            bid.generate_id::<SystemHashing<T>, T>(sfx_id)
                        );
                    }
                    if !<T as Config>::AccountManager::finalize_infallible(
                        sfx_id,
                        match &fsx.confirmed {
                            // Commit deposits from requesters to executors for honest SFX resolution
                            Some(_confirmed) => Outcome::Commit,
                            // Refund requester for dishonest SFX resolution
                            None => Outcome::Revert,
                        },
                    ) {
                        log::error!(
                            "revert: expect finalize_infallible to succeed for sfx_id: {:?}",
                            sfx_id
                        );
                    }
                },
                None => {
                    log::error!(
                        "revert: disallowed state: reverting without fsx.best_bid assigned {:?}",
                        sfx_id
                    );
                },
            }
        }
    }

    /// Finalize Xtx after successful run - reward Escrow executors.
    pub fn commit(_local_ctx: &LocalXtxCtx<T>) {}
}
