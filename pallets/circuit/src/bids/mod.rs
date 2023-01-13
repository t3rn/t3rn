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
        let fsx = current_fsx
            .iter()
            .filter(|&fsx| fsx.confirmed.is_none())
            .find(|fsx| fsx.generate_id::<SystemHashing<T>, T>(xtx_id) == sfx_id)
            .ok_or(Error::<T>::FSXNotFoundById)?;

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
            let total_xtx_step_optimistic_rewards_of_others = &current_fsx
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

            sfx_bid.reserved_bond = match total_xtx_step_optimistic_rewards_of_others {
                Some(x) =>
                    if x > &Zero::zero() {
                        Some(x.clone())
                    } else {
                        None
                    },
                None => None,
            };
        }

        SquareUp::<T>::try_bid(sfx_id, requester, bidder, &sfx_bid, current_accepted_bid).map_err(
            |e| {
                log::error!("Error while trying to SquareUp::try_bid: {:?}", e);
                Error::<T>::BiddingRejectedBetterBidFound
            },
        )?;

        Ok(sfx_bid)
    }
}
