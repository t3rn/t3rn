use crate::pallet::{Config, Fixtures};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_runtime::{
    traits::{Saturating, Zero},
    RuntimeDebug,
};
use sp_std::prelude::*;
use t3rn_primitives::executors::{Bond, CapacityStatus};

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// Type for top and bottom stake storage item
pub struct Stakes<AccountId, Balance> {
    pub stakes: Vec<Bond<AccountId, Balance>>,
    pub total: Balance,
}

impl<A, B: Default> Default for Stakes<A, B> {
    fn default() -> Stakes<A, B> {
        Stakes {
            stakes: Vec::new(),
            total: B::default(),
        }
    }
}

impl<AccountId, Balance: Copy + Ord + sp_std::ops::AddAssign + Zero + Saturating>
    Stakes<AccountId, Balance>
{
    pub fn sort_greatest_to_least(&mut self) {
        self.stakes.sort_by(|a, b| b.amount.cmp(&a.amount));
    }

    /// Insert sorted greatest to least and increase .total accordingly
    /// Insertion respects first come first serve so new stakes are pushed after existing
    /// stakes if the amount is the same
    pub fn insert_sorted_greatest_to_least(&mut self, stake: Bond<AccountId, Balance>) {
        self.total = self.total.saturating_add(stake.amount);
        // if stakes nonempty && last_element == stake.amount => push input and return
        if !self.stakes.is_empty() {
            // if last_element == stake.amount => push the stake and return early
            if self.stakes[self.stakes.len() - 1].amount == stake.amount {
                self.stakes.push(stake);
                // early return
                return
            }
        }
        // else binary search insertion
        match self
            .stakes
            .binary_search_by(|x| stake.amount.cmp(&x.amount))
        {
            // sorted insertion on sorted vec
            // enforces first come first serve for equal bond amounts
            Ok(i) => {
                let mut new_index = i + 1;
                while new_index <= (self.stakes.len() - 1) {
                    if self.stakes[new_index].amount == stake.amount {
                        new_index = new_index.saturating_add(1);
                    } else {
                        self.stakes.insert(new_index, stake);
                        return
                    }
                }
                self.stakes.push(stake)
            },
            Err(i) => self.stakes.insert(i, stake),
        }
    }

    /// Return the capacity status for top stakes
    pub fn top_capacity<T: Config>(&self) -> CapacityStatus {
        match &self.stakes {
            x if x.len() >= <Fixtures<T>>::get().max_top_stakes_per_candidate as usize =>
                CapacityStatus::Full,
            x if x.is_empty() => CapacityStatus::Empty,
            _ => CapacityStatus::Partial,
        }
    }

    /// Return the capacity status for bottom stakes
    pub fn bottom_capacity<T: Config>(&self) -> CapacityStatus {
        match &self.stakes {
            x if x.len() >= <Fixtures<T>>::get().max_bottom_stakes_per_candidate as usize =>
                CapacityStatus::Full,
            x if x.is_empty() => CapacityStatus::Empty,
            _ => CapacityStatus::Partial,
        }
    }

    /// Return last stake amount without popping the stake
    pub fn lowest_stake_amount(&self) -> Balance {
        self.stakes
            .last()
            .map(|x| x.amount)
            .unwrap_or(Balance::zero())
    }

    /// Return highest stake amount
    pub fn highest_stake_amount(&self) -> Balance {
        self.stakes
            .first()
            .map(|x| x.amount)
            .unwrap_or(Balance::zero())
    }
}
