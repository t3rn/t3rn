use crate::{pallet::Error, *};
use sp_runtime::traits::Zero;

use crate::square_up::SquareUp;
use sp_std::marker::PhantomData;
use t3rn_primitives::{side_effect::SFXBid, transfers::EscrowedBalanceOf};

pub struct Bids<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> Bids<T> {
    pub fn try_bid(
        step_fsx: &mut Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >,
        bid_amount: EscrowedBalanceOf<T, T::Escrowed>,
        bidder: &T::AccountId,
        requester: &T::AccountId,
        sfx_id: SideEffectId<T>,
        xtx_id: XExecSignalId<T>,
    ) -> Result<
        Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>,
        Error<T>,
    > {
        // Check for the previous bids for SFX.
        let fsx = step_fsx
            .iter()
            .filter(|&fsx| fsx.confirmed.is_none())
            .find(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
            .ok_or(Error::<T>::FSXNotFoundById)?;

        let mut bid =
            SFXBid::<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>, u32>::new_none_optimistic(
                bid_amount,
                fsx.input.insurance,
                bidder.clone(),
                requester.clone(),
                fsx.input.reward_asset_id,
            );

        let current_accepted_bid = fsx.best_bid.clone();

        let (sfx_max_reward, sfx_security_lvl, sfx_insurance) = (
            fsx.input.max_reward,
            fsx.security_lvl.clone(),
            fsx.input.insurance,
        );
        // Check if bid doesn't go below dust
        if bid.amount < <T::Escrowed as EscrowTrait<T>>::Currency::minimum_balance() {
            return Err(Error::<T>::BiddingRejectedBidBelowDust)
        }
        // Check if bid is attractive enough for requester
        if bid.amount > sfx_max_reward {
            return Err(Error::<T>::BiddingRejectedBidTooHigh)
        }
        // Check if bid insurance satisfies requested insurance amount
        if bid.insurance != sfx_insurance {
            return Err(Error::<T>::BiddingRejectedInsuranceTooLow)
        }
        // Check if bid is higher than current best bid
        match current_accepted_bid.clone() {
            Some(current_best) =>
                if bid.amount >= current_best.amount {
                    return Err(Error::<T>::BiddingRejectedBetterBidFound)
                },
            None => {},
        }
        // Is the current bid for type SFX::Optimistic? If yes reserve the bond lock requirements
        if sfx_security_lvl == SecurityLvl::Optimistic {
            let total_xtx_step_optimistic_rewards_of_others = step_fsx
                .iter()
                .filter(|&fsx| fsx.security_lvl == SecurityLvl::Optimistic)
                // All FSX but the current one
                .filter(|&fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) != sfx_id)
                .map(|fsx| fsx.get_bond_value(Zero::zero()))
                .reduce(|total_reserved, next_amount| {
                    total_reserved
                        .checked_add(&next_amount)
                        .unwrap_or_else(|| total_reserved)
                });

            bid.reserved_bond = match total_xtx_step_optimistic_rewards_of_others {
                Some(x) =>
                    if x > Zero::zero() {
                        Some(x)
                    } else {
                        None
                    },
                None => None,
            };
        }

        SquareUp::<T>::try_bid(sfx_id, requester, bidder, &bid, current_accepted_bid).map_err(
            |e| {
                log::error!("Error while trying to SquareUp::try_bid: {:?}", e);
                Error::<T>::BiddingRejectedFailedToDepositBidderBond
            },
        )?;

        // Replace the best bid for the FSX
        step_fsx
            .iter_mut()
            .find(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
            .map(|mut fsx| {
                fsx.best_bid = Some(bid.clone());
            });

        Ok(step_fsx.clone())
    }
}
