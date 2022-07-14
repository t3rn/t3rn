use crate::{
    pallet::{
        BalanceOf, BottomStakes, CandidateInfo, Config, Error, Event, Fixtures, Pallet, StakerInfo,
        TopStakes, Total,
    },
    stakes::Stakes,
};
use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::*,
    traits::{tokens::WithdrawReasons, LockableCurrency, ReservableCurrency},
};
use sp_runtime::{
    traits::{Saturating, Zero},
    RuntimeDebug,
};
use sp_std::prelude::*;
use t3rn_primitives::{
    common::{OrderedSet, RoundIndex},
    staking::{
        Bond, CandidateBondLessRequest, CapacityStatus, ExecutorStatus, StakeAdjust, StakerAdded,
        StakerStatus, EXECUTOR_LOCK_ID, STAKER_LOCK_ID,
    },
    treasury::Treasury,
};

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// Staker state
pub struct StakerMetadata<AccountId, Balance> {
    /// Staker account
    pub id: AccountId,
    /// All current stakes
    pub stakes: OrderedSet<Bond<AccountId, Balance>>,
    /// Total balance locked for this staker
    pub total: Balance,
    /// Sum of pending revocation amounts + bond less amounts
    pub less_total: Balance,
    /// Status for this staker
    pub status: StakerStatus,
}

impl<
        AccountId: Ord + Clone,
        Balance: Copy
            + sp_std::ops::AddAssign
            + sp_std::ops::Add<Output = Balance>
            + sp_std::ops::SubAssign
            + sp_std::ops::Sub<Output = Balance>
            + Ord
            + Zero
            + Default
            + Saturating,
    > StakerMetadata<AccountId, Balance>
{
    pub fn new(id: AccountId, collator: AccountId, amount: Balance) -> Self {
        StakerMetadata {
            id,
            stakes: OrderedSet::from(vec![Bond {
                owner: collator,
                amount,
            }]),
            total: amount,
            less_total: Balance::zero(),
            status: StakerStatus::Active,
        }
    }

    //
    pub fn default_with_total(id: AccountId, amount: Balance) -> Self {
        StakerMetadata {
            id,
            total: amount,
            stakes: OrderedSet::from(vec![]),
            less_total: Balance::zero(),
            status: StakerStatus::Active,
        }
    }

    pub fn total(&self) -> Balance {
        self.total
    }

    pub fn total_add_if<T, F>(&mut self, amount: Balance, check: F) -> DispatchResult
    where
        T: Config,
        T::AccountId: From<AccountId>,
        BalanceOf<T>: From<Balance>,
        F: Fn(Balance) -> DispatchResult,
    {
        let total = self.total.saturating_add(amount);

        check(total)?;

        self.total = total;

        self.adjust_bond_lock::<T>(StakeAdjust::Increase(amount))
    }

    pub fn total_sub_if<T, F>(&mut self, amount: Balance, check: F) -> DispatchResult
    where
        T: Config,
        T::AccountId: From<AccountId>,
        BalanceOf<T>: From<Balance>,
        F: Fn(Balance) -> DispatchResult,
    {
        let total = self.total.saturating_sub(amount);

        check(total)?;

        self.total = total;

        self.adjust_bond_lock::<T>(StakeAdjust::Decrease)?;

        Ok(())
    }

    pub fn total_add<T, F>(&mut self, amount: Balance) -> DispatchResult
    where
        T: Config,
        T::AccountId: From<AccountId>,
        BalanceOf<T>: From<Balance>,
    {
        self.total = self.total.saturating_add(amount);

        self.adjust_bond_lock::<T>(StakeAdjust::Increase(amount))?;

        Ok(())
    }

    pub fn total_sub<T>(&mut self, amount: Balance) -> DispatchResult
    where
        T: Config,
        T::AccountId: From<AccountId>,
        BalanceOf<T>: From<Balance>,
    {
        self.total = self.total.saturating_sub(amount);

        self.adjust_bond_lock::<T>(StakeAdjust::Decrease)?;

        Ok(())
    }

    //

    pub fn is_active(&self) -> bool {
        matches!(self.status, StakerStatus::Active)
    }

    pub fn add_stake(&mut self, bond: Bond<AccountId, Balance>) -> bool {
        let amt = bond.amount;
        if self.stakes.insert(bond) {
            self.total = self.total.saturating_add(amt);
            true
        } else {
            false
        }
    }

    // Return Some(remaining balance), must be more than MinTotalStake
    // Return None if stake not found
    pub fn rm_stake(&mut self, collator: &AccountId) -> Option<Balance> {
        let mut amt: Option<Balance> = None;
        let stakes = self
            .stakes
            .0
            .iter()
            .filter_map(|x| {
                if &x.owner == collator {
                    amt = Some(x.amount);
                    None
                } else {
                    Some(x.clone())
                }
            })
            .collect();
        if let Some(balance) = amt {
            self.stakes = OrderedSet::from(stakes);
            self.total = self.total.saturating_sub(balance);
            Some(self.total)
        } else {
            None
        }
    }

    pub fn increase_stake<T: Config>(
        &mut self,
        candidate: AccountId,
        amount: Balance,
    ) -> DispatchResult
    where
        BalanceOf<T>: From<Balance>,
        T::AccountId: From<AccountId>,
        StakerMetadata<T::AccountId, BalanceOf<T>>: From<StakerMetadata<AccountId, Balance>>,
    {
        let staker_id: T::AccountId = self.id.clone().into();
        let candidate_id: T::AccountId = candidate.clone().into();
        let balance_amt: BalanceOf<T> = amount.into();
        // increase stake
        for x in &mut self.stakes.0 {
            if x.owner == candidate {
                let before_amount: BalanceOf<T> = x.amount.into();
                x.amount = x.amount.saturating_add(amount);
                self.total = self.total.saturating_add(amount);
                // update collator state stake
                let mut collator_state =
                    <CandidateInfo<T>>::get(&candidate_id).ok_or(Error::<T>::NoSuchCandidate)?;
                T::Currency::reserve(&self.id.clone().into(), balance_amt)?;
                let before = collator_state.total_counted;
                let in_top = collator_state.increase_stake::<T>(
                    &candidate_id,
                    staker_id.clone(),
                    before_amount,
                    balance_amt,
                )?;
                let after = collator_state.total_counted;
                if collator_state.is_active() && (before != after) {
                    Pallet::<T>::update_active(candidate_id.clone(), after);
                }
                <CandidateInfo<T>>::insert(&candidate_id, collator_state);
                let new_total_staked = <Total<T>>::get().saturating_add(balance_amt);
                <Total<T>>::put(new_total_staked);
                let nom_st: StakerMetadata<T::AccountId, BalanceOf<T>> = self.clone().into();
                <StakerInfo<T>>::insert(&staker_id, nom_st);
                Pallet::<T>::deposit_event(Event::StakeIncreased {
                    staker: staker_id,
                    candidate: candidate_id,
                    amount: balance_amt,
                    in_top,
                });
                return Ok(())
            }
        }
        Err(Error::<T>::NoSuchStake.into())
    }

    /// Updates the bond locks for this staker.
    ///
    /// This will take the current self.total and ensure that a lock of the same amount is applied
    /// and when increasing the bond lock will also ensure that the account has enough free balance.
    ///
    /// `additional_required_balance` should reflect the change to the amount that should be locked if
    /// positive, 0 otherwise (e.g. `min(0, change_in_total_bond)`). This is necessary because it is
    /// not possible to query the amount that is locked for a given lock id.
    pub fn adjust_bond_lock<T: Config>(
        &mut self,
        additional_required_balance: StakeAdjust<Balance>,
    ) -> DispatchResult
    where
        BalanceOf<T>: From<Balance>,
        T::AccountId: From<AccountId>,
    {
        match additional_required_balance {
            StakeAdjust::Increase(amount) => {
                ensure!(
                    <Pallet<T>>::get_staker_stakable_free_balance(&self.id.clone().into())
                        >= amount.into(),
                    Error::<T>::InsufficientBalance,
                );

                // additional sanity check: shouldn't ever want to lock more than total
                if amount > self.total {
                    log::warn!("LOGIC ERROR: request to reserve more than bond total");
                    return Err(DispatchError::Other("Invalid additional_required_balance"))
                }
            },
            StakeAdjust::Decrease => (), // do nothing on decrease
        };

        if self.total.is_zero() {
            T::Currency::remove_lock(STAKER_LOCK_ID, &self.id.clone().into());
        } else {
            T::Currency::set_lock(
                STAKER_LOCK_ID,
                &self.id.clone().into(),
                self.total.into(),
                WithdrawReasons::all(),
            );
        }

        Ok(())
    }

    /// Retrieves the bond amount that a staker has provided towards a collator.
    /// Returns `None` if missing.
    pub fn get_bond_amount(&self, collator: &AccountId) -> Option<Balance> {
        self.stakes
            .0
            .iter()
            .find(|b| &b.owner == collator)
            .map(|b| b.amount)
    }
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
/// All candidate info except the top and bottom stakes
pub struct CandidateMetadata<Balance> {
    /// This candidate's self bond amount
    pub bond: Balance,
    /// Total number of stakes to this candidate
    pub stake_count: u32,
    /// Self bond + sum of top stakes
    pub total_counted: Balance,
    /// The smallest top stake amount
    pub lowest_top_stake_amount: Balance,
    /// The highest bottom stake amount
    pub highest_bottom_stake_amount: Balance,
    /// The smallest bottom stake amount
    pub lowest_bottom_stake_amount: Balance,
    /// Capacity status for top stakes
    pub top_capacity: CapacityStatus,
    /// Capacity status for bottom stakes
    pub bottom_capacity: CapacityStatus,
    /// Maximum 1 pending request to decrease candidate self bond at any given time
    pub request: Option<CandidateBondLessRequest<Balance>>,
    /// Current status of the executor
    pub status: ExecutorStatus,
}

impl<
        Balance: Copy
            + Zero
            + PartialOrd
            + sp_std::ops::AddAssign
            + sp_std::ops::SubAssign
            + sp_std::ops::Sub<Output = Balance>
            + sp_std::fmt::Debug
            + Saturating,
    > CandidateMetadata<Balance>
{
    pub fn new(bond: Balance) -> Self {
        CandidateMetadata {
            bond,
            stake_count: 0u32,
            total_counted: bond,
            lowest_top_stake_amount: Zero::zero(),
            highest_bottom_stake_amount: Zero::zero(),
            lowest_bottom_stake_amount: Zero::zero(),
            top_capacity: CapacityStatus::Empty,
            bottom_capacity: CapacityStatus::Empty,
            request: None,
            status: ExecutorStatus::Active,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, ExecutorStatus::Active)
    }

    pub fn is_leaving(&self) -> bool {
        matches!(self.status, ExecutorStatus::Leaving(_))
    }

    pub fn schedule_leave<T: Config>(&mut self) -> Result<(RoundIndex, RoundIndex), DispatchError> {
        ensure!(!self.is_leaving(), Error::<T>::CandidateAlreadyLeaving);
        let now = T::Treasury::current_round().index;
        let when = now + <Fixtures<T>>::get().leave_candidates_delay;
        self.status = ExecutorStatus::Leaving(when);
        Ok((now, when))
    }

    pub fn can_leave<T: Config>(&self) -> DispatchResult {
        if let ExecutorStatus::Leaving(when) = self.status {
            ensure!(
                T::Treasury::current_round().index >= when,
                Error::<T>::CandidateCannotLeaveYet
            );
            Ok(())
        } else {
            Err(Error::<T>::CandidateNotLeaving.into())
        }
    }

    pub fn go_offline(&mut self) {
        self.status = ExecutorStatus::Idle;
    }

    pub fn go_online(&mut self) {
        self.status = ExecutorStatus::Active;
    }

    pub fn bond_more<T: Config>(&mut self, who: T::AccountId, amount: Balance) -> DispatchResult
    where
        BalanceOf<T>: From<Balance>,
    {
        T::Currency::reserve(&who, amount.into())?;
        let new_total = <Total<T>>::get().saturating_add(amount.into());
        <Total<T>>::put(new_total);
        self.bond = self.bond.saturating_add(amount);
        self.total_counted = self.total_counted.saturating_add(amount);
        <Pallet<T>>::deposit_event(Event::CandidateBondedMore {
            candidate: who,
            amount: amount.into(),
            total_bond: self.bond.into(),
        });
        Ok(())
    }

    /// Schedule executable decrease of executor candidate self bond
    /// Returns the round at which the executor can execute the pending request
    pub fn schedule_bond_less<T: Config>(
        &mut self,
        less: Balance,
    ) -> Result<RoundIndex, DispatchError>
    where
        BalanceOf<T>: Into<Balance>,
    {
        // ensure no pending request
        ensure!(
            self.request.is_none(),
            Error::<T>::PendingCandidateRequestAlreadyExists
        );

        // ensure bond above min after decrease
        ensure!(self.bond > less, Error::<T>::CandidateBondBelowMin);

        let fixtures = <Fixtures<T>>::get();

        ensure!(
            self.bond - less >= fixtures.min_candidate_bond.into(),
            Error::<T>::CandidateBondBelowMin
        );

        let when_executable =
            T::Treasury::current_round().index + fixtures.candidate_bond_less_delay;

        self.request = Some(CandidateBondLessRequest {
            amount: less,
            when_executable,
        });

        Ok(when_executable)
    }

    /// Execute pending request to decrease the executor self bond
    /// Returns the event to be emitted
    pub fn execute_bond_less<T: Config>(&mut self, who: T::AccountId) -> DispatchResult
    where
        BalanceOf<T>: From<Balance>,
    {
        let request = self
            .request
            .ok_or(Error::<T>::NoSuchPendingCandidateRequest)?;
        ensure!(
            request.when_executable <= T::Treasury::current_round().index,
            Error::<T>::PendingCandidateRequestNotDueYet
        );
        T::Currency::unreserve(&who, request.amount.into());
        let new_total_staked = <Total<T>>::get().saturating_sub(request.amount.into());
        <Total<T>>::put(new_total_staked);
        // Arithmetic assumptions are self.bond > less && self.bond - less > CollatorMinBond
        // (assumptions enforced by `schedule_bond_less`; if storage corrupts, must re-verify)
        self.bond = self.bond.saturating_sub(request.amount);
        self.total_counted = self.total_counted.saturating_sub(request.amount);
        let event = Event::CandidateBondedLess {
            candidate: who.clone().into(),
            amount: request.amount.into(),
            total_bond: self.bond.into(),
        };
        // reset s.t. no pending request
        self.request = None;
        // update candidate pool value because it must change if self bond changes
        if self.is_active() {
            Pallet::<T>::update_active(who.into(), self.total_counted.into());
        }
        Pallet::<T>::deposit_event(event);
        Ok(())
    }

    /// Cancel candidate bond less request
    pub fn cancel_bond_less<T: Config>(&mut self, who: T::AccountId) -> DispatchResult
    where
        BalanceOf<T>: From<Balance>,
    {
        let request = self
            .request
            .ok_or(Error::<T>::NoSuchPendingCandidateRequest)?;
        let event = Event::CandidateBondLessCancelled {
            candidate: who.clone().into(),
            amount: request.amount.into(),
            execute_round: request.when_executable,
        };
        self.request = None;
        Pallet::<T>::deposit_event(event);
        Ok(())
    }

    /// Reset top stakes metadata
    pub fn reset_top_data<T: Config>(
        &mut self,
        candidate: T::AccountId,
        top_stakes: &Stakes<T::AccountId, BalanceOf<T>>,
    ) where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        self.lowest_top_stake_amount = top_stakes.lowest_stake_amount().into();
        self.top_capacity = top_stakes.top_capacity::<T>();
        let old_total_counted = self.total_counted;
        self.total_counted = self.bond.saturating_add(top_stakes.total.into());
        // CandidatePool value for candidate always changes if top stakes total changes
        // so we moved the update into this function to deduplicate code and patch a bug that
        // forgot to apply the update when increasing top stake
        if old_total_counted != self.total_counted && self.is_active() {
            Pallet::<T>::update_active(candidate, self.total_counted.into());
        }
    }

    /// Reset bottom stakes metadata
    pub fn reset_bottom_data<T: Config>(
        &mut self,
        bottom_stakes: &Stakes<T::AccountId, BalanceOf<T>>,
    ) where
        BalanceOf<T>: Into<Balance>,
    {
        self.lowest_bottom_stake_amount = bottom_stakes.lowest_stake_amount().into();
        self.highest_bottom_stake_amount = bottom_stakes.highest_stake_amount().into();
        self.bottom_capacity = bottom_stakes.bottom_capacity::<T>();
    }

    /// Add stake
    /// Returns whether staker was added and an optional negative total counted remainder
    /// for if a bottom stake was kicked
    /// MUST ensure no stake exists for this candidate in the `StakerInfo` before call
    pub fn add_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        stake: Bond<T::AccountId, BalanceOf<T>>,
    ) -> Result<(StakerAdded<Balance>, Option<Balance>), DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let mut less_total_staked = None;
        let staker_added = match self.top_capacity {
            CapacityStatus::Full => {
                // top is full, insert into top iff the lowest_top < amount
                if self.lowest_top_stake_amount < stake.amount.into() {
                    // bumps lowest top to the bottom inside this function call
                    less_total_staked = self.add_top_stake::<T>(candidate, stake);
                    StakerAdded::ToTop {
                        new_total: self.total_counted,
                    }
                } else {
                    // if bottom is full, only insert if greater than lowest bottom (which will
                    // be bumped out)
                    if matches!(self.bottom_capacity, CapacityStatus::Full) {
                        ensure!(
                            stake.amount.into() > self.lowest_bottom_stake_amount,
                            Error::<T>::CannotDelegateLessThanOrEqualToLowestBottomWhenFull
                        );
                        // need to subtract from total staked
                        less_total_staked = Some(self.lowest_bottom_stake_amount);
                    }
                    // insert into bottom
                    self.add_bottom_stake::<T>(false, candidate, stake);
                    StakerAdded::ToBottom
                }
            },
            // top is either empty or partially full
            _ => {
                self.add_top_stake::<T>(candidate, stake);
                StakerAdded::ToTop {
                    new_total: self.total_counted,
                }
            },
        };
        Ok((staker_added, less_total_staked))
    }

    /// Add stake to top stake
    /// Returns Option<negative_total_staked_remainder>
    /// Only call if lowest top stake is less than stake.amount || !top_full
    pub fn add_top_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        stake: Bond<T::AccountId, BalanceOf<T>>,
    ) -> Option<Balance>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let mut less_total_staked = None;
        let mut top_stakes =
            <TopStakes<T>>::get(candidate).expect("CandidateInfo existence => TopStakes existence");
        if top_stakes.stakes.len() as u32 == <Fixtures<T>>::get().max_top_stakes_per_candidate {
            // pop lowest top stake
            let new_bottom_stake = top_stakes.stakes.pop().expect("");
            top_stakes.total = top_stakes.total.saturating_sub(new_bottom_stake.amount);
            if matches!(self.bottom_capacity, CapacityStatus::Full) {
                less_total_staked = Some(self.lowest_bottom_stake_amount);
            }
            self.add_bottom_stake::<T>(true, candidate, new_bottom_stake);
        }
        // insert into top
        top_stakes.insert_sorted_greatest_to_least(stake);
        // update candidate info
        self.reset_top_data::<T>(candidate.clone(), &top_stakes);
        if less_total_staked.is_none() {
            // only increment stake count if we are not kicking a bottom stake
            self.stake_count = self.stake_count.saturating_add(1u32);
        }
        <TopStakes<T>>::insert(&candidate, top_stakes);
        less_total_staked
    }

    /// Add stake to bottom stakes
    /// Check before call that if capacity is full, inserted stake is higher than lowest
    /// bottom stake (and if so, need to adjust the total storage item)
    /// CALLER MUST ensure(lowest_bottom_to_be_kicked.amount < stake.amount)
    pub fn add_bottom_stake<T: Config>(
        &mut self,
        bumped_from_top: bool,
        candidate: &T::AccountId,
        stake: Bond<T::AccountId, BalanceOf<T>>,
    ) where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let mut bottom_stakes = <BottomStakes<T>>::get(candidate)
            .expect("CandidateInfo existence => BottomStakes existence");
        // if bottom is full, kick the lowest bottom (which is expected to be lower than input
        // as per check)
        let increase_stake_count = if bottom_stakes.stakes.len() as u32
            == <Fixtures<T>>::get().max_bottom_stakes_per_candidate
        {
            let lowest_bottom_to_be_kicked = bottom_stakes
                .stakes
                .pop()
                .expect("if at full capacity (>0), then >0 bottom stakes exist; qed");
            // EXPECT lowest_bottom_to_be_kicked.amount < stake.amount enforced by caller
            // if lowest_bottom_to_be_kicked.amount == stake.amount, we will still kick
            // the lowest bottom to enforce first come first served
            bottom_stakes.total = bottom_stakes
                .total
                .saturating_sub(lowest_bottom_to_be_kicked.amount);
            // update staker state
            // unreserve kicked bottom
            T::Currency::unreserve(
                &lowest_bottom_to_be_kicked.owner,
                lowest_bottom_to_be_kicked.amount,
            );
            // total staked is updated via propagation of lowest bottom stake amount prior
            // to call
            let mut staker_state = <StakerInfo<T>>::get(&lowest_bottom_to_be_kicked.owner)
                .expect("Delegation existence => StakerInfo existence");
            let leaving = staker_state.stakes.0.len() == 1usize;
            staker_state.rm_stake(candidate);
            <Pallet<T>>::stake_remove_request_with_state(
                &candidate,
                &lowest_bottom_to_be_kicked.owner,
                &mut staker_state,
            );

            Pallet::<T>::deposit_event(Event::StakeKicked {
                staker: lowest_bottom_to_be_kicked.owner.clone(),
                candidate: candidate.clone(),
                unstaked: lowest_bottom_to_be_kicked.amount,
            });
            if leaving {
                <StakerInfo<T>>::remove(&lowest_bottom_to_be_kicked.owner);
                Pallet::<T>::deposit_event(Event::StakerLeft {
                    staker: lowest_bottom_to_be_kicked.owner,
                    unstaked: lowest_bottom_to_be_kicked.amount,
                });
            } else {
                <StakerInfo<T>>::insert(&lowest_bottom_to_be_kicked.owner, staker_state);
            }
            false
        } else {
            !bumped_from_top
        };
        // only increase stake count if new bottom stake (1) doesn't come from top &&
        // (2) doesn't pop the lowest stake from the bottom
        if increase_stake_count {
            self.stake_count = self.stake_count.saturating_add(1u32);
        }
        bottom_stakes.insert_sorted_greatest_to_least(stake);
        self.reset_bottom_data::<T>(&bottom_stakes);
        <BottomStakes<T>>::insert(candidate, bottom_stakes);
    }

    /// Remove stake
    /// Removes from top if amount is above lowest top or top is not full
    /// Return Ok(if_total_counted_changed)
    pub fn rm_stake_if_exists<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        amount: Balance,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let amount_geq_lowest_top = amount >= self.lowest_top_stake_amount;
        let top_is_not_full = !matches!(self.top_capacity, CapacityStatus::Full);
        let lowest_top_eq_highest_bottom =
            self.lowest_top_stake_amount == self.highest_bottom_stake_amount;
        let stake_dne_err: DispatchError = Error::<T>::NoSuchStake.into();
        if top_is_not_full || (amount_geq_lowest_top && !lowest_top_eq_highest_bottom) {
            self.rm_top_stake::<T>(candidate, staker)
        } else if amount_geq_lowest_top && lowest_top_eq_highest_bottom {
            let result = self.rm_top_stake::<T>(candidate, staker.clone());
            if result == Err(stake_dne_err) {
                // worst case removal
                self.rm_bottom_stake::<T>(candidate, staker)
            } else {
                result
            }
        } else {
            self.rm_bottom_stake::<T>(candidate, staker)
        }
    }

    /// Remove top stake, bumps top bottom stake if exists
    pub fn rm_top_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let old_total_counted = self.total_counted;
        // remove top stake
        let mut top_stakes =
            <TopStakes<T>>::get(candidate).expect("CandidateInfo exists => TopStakes exists");
        let mut actual_amount_option: Option<BalanceOf<T>> = None;
        top_stakes.stakes = top_stakes
            .stakes
            .clone()
            .into_iter()
            .filter(|d| {
                if d.owner != staker {
                    true
                } else {
                    actual_amount_option = Some(d.amount);
                    false
                }
            })
            .collect();
        let actual_amount = actual_amount_option.ok_or(Error::<T>::NoSuchStake)?;
        top_stakes.total = top_stakes.total.saturating_sub(actual_amount);
        // if bottom nonempty => bump top bottom to top
        if !matches!(self.bottom_capacity, CapacityStatus::Empty) {
            let mut bottom_stakes =
                <BottomStakes<T>>::get(candidate).expect("bottom is nonempty as just checked");
            // expect already stored greatest to least by bond amount
            let highest_bottom_stake = bottom_stakes.stakes.remove(0);
            bottom_stakes.total = bottom_stakes
                .total
                .saturating_sub(highest_bottom_stake.amount);
            self.reset_bottom_data::<T>(&bottom_stakes);
            <BottomStakes<T>>::insert(candidate, bottom_stakes);
            // insert highest bottom into top stakes
            top_stakes.insert_sorted_greatest_to_least(highest_bottom_stake);
        }
        // update candidate info
        self.reset_top_data::<T>(candidate.clone(), &top_stakes);
        self.stake_count = self.stake_count.saturating_sub(1u32);
        <TopStakes<T>>::insert(candidate, top_stakes);
        // return whether total counted changed
        Ok(old_total_counted == self.total_counted)
    }

    /// Remove bottom stake
    /// Returns if_total_counted_changed: bool
    pub fn rm_bottom_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance>,
    {
        // remove bottom stake
        let mut bottom_stakes =
            <BottomStakes<T>>::get(candidate).expect("CandidateInfo exists => BottomStakes exists");
        let mut actual_amount_option: Option<BalanceOf<T>> = None;
        bottom_stakes.stakes = bottom_stakes
            .stakes
            .clone()
            .into_iter()
            .filter(|d| {
                if d.owner != staker {
                    true
                } else {
                    actual_amount_option = Some(d.amount);
                    false
                }
            })
            .collect();
        let actual_amount = actual_amount_option.ok_or(Error::<T>::NoSuchStake)?;
        bottom_stakes.total = bottom_stakes.total.saturating_sub(actual_amount);
        // update candidate info
        self.reset_bottom_data::<T>(&bottom_stakes);
        self.stake_count = self.stake_count.saturating_sub(1u32);
        <BottomStakes<T>>::insert(candidate, bottom_stakes);
        Ok(false)
    }

    /// Increase stake amount
    pub fn increase_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        bond: BalanceOf<T>,
        more: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let lowest_top_eq_highest_bottom =
            self.lowest_top_stake_amount == self.highest_bottom_stake_amount;
        let bond_geq_lowest_top = bond.into() >= self.lowest_top_stake_amount;
        let stake_dne_err: DispatchError = Error::<T>::NoSuchStake.into();
        if bond_geq_lowest_top && !lowest_top_eq_highest_bottom {
            // definitely in top
            self.increase_top_stake::<T>(candidate, staker.clone(), more)
        } else if bond_geq_lowest_top && lowest_top_eq_highest_bottom {
            // update top but if error then update bottom (because could be in bottom because
            // lowest_top_eq_highest_bottom)
            let result = self.increase_top_stake::<T>(candidate, staker.clone(), more);
            if result == Err(stake_dne_err) {
                self.increase_bottom_stake::<T>(candidate, staker, bond, more)
            } else {
                result
            }
        } else {
            self.increase_bottom_stake::<T>(candidate, staker, bond, more)
        }
    }

    /// Increase top stake
    pub fn increase_top_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        more: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let mut top_stakes =
            <TopStakes<T>>::get(candidate).expect("CandidateInfo exists => TopStakes exists");
        let mut in_top = false;
        top_stakes.stakes = top_stakes
            .stakes
            .clone()
            .into_iter()
            .map(|d| {
                if d.owner != staker {
                    d
                } else {
                    in_top = true;
                    let new_amount = d.amount.saturating_add(more);
                    Bond {
                        owner: d.owner,
                        amount: new_amount,
                    }
                }
            })
            .collect();
        ensure!(in_top, Error::<T>::NoSuchStake);
        top_stakes.total = top_stakes.total.saturating_add(more);
        top_stakes.sort_greatest_to_least();
        self.reset_top_data::<T>(candidate.clone(), &top_stakes);
        <TopStakes<T>>::insert(candidate, top_stakes);
        Ok(true)
    }

    /// Increase bottom stake
    pub fn increase_bottom_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        bond: BalanceOf<T>,
        more: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let mut bottom_stakes =
            <BottomStakes<T>>::get(candidate).ok_or(Error::<T>::NoSuchCandidate)?;
        let mut stake_option: Option<Bond<T::AccountId, BalanceOf<T>>> = None;
        let in_top_after = if (bond.saturating_add(more)).into() > self.lowest_top_stake_amount {
            // bump it from bottom
            bottom_stakes.stakes = bottom_stakes
                .stakes
                .clone()
                .into_iter()
                .filter(|d| {
                    if d.owner != staker {
                        true
                    } else {
                        stake_option = Some(Bond {
                            owner: d.owner.clone(),
                            amount: d.amount.saturating_add(more),
                        });
                        false
                    }
                })
                .collect();
            let stake = stake_option.ok_or(Error::<T>::NoSuchStake)?;
            bottom_stakes.total = bottom_stakes.total.saturating_sub(bond);
            // add it to top
            let mut top_stakes = <TopStakes<T>>::get(candidate)
                .expect("CandidateInfo existence => TopStakes existence");
            // if top is full, pop lowest top
            if matches!(top_stakes.top_capacity::<T>(), CapacityStatus::Full) {
                // pop lowest top stake
                let new_bottom_stake = top_stakes
                    .stakes
                    .pop()
                    .expect("Top capacity full => Exists at least 1 top stake");
                top_stakes.total = top_stakes.total.saturating_sub(new_bottom_stake.amount);
                bottom_stakes.insert_sorted_greatest_to_least(new_bottom_stake);
            }
            // insert into top
            top_stakes.insert_sorted_greatest_to_least(stake);
            self.reset_top_data::<T>(candidate.clone(), &top_stakes);
            <TopStakes<T>>::insert(candidate, top_stakes);
            true
        } else {
            let mut in_bottom = false;
            // just increase the stake
            bottom_stakes.stakes = bottom_stakes
                .stakes
                .clone()
                .into_iter()
                .map(|d| {
                    if d.owner != staker {
                        d
                    } else {
                        in_bottom = true;
                        Bond {
                            owner: d.owner,
                            amount: d.amount.saturating_add(more),
                        }
                    }
                })
                .collect();
            ensure!(in_bottom, Error::<T>::NoSuchStake);
            bottom_stakes.total = bottom_stakes.total.saturating_add(more);
            bottom_stakes.sort_greatest_to_least();
            false
        };
        self.reset_bottom_data::<T>(&bottom_stakes);
        <BottomStakes<T>>::insert(candidate, bottom_stakes);
        Ok(in_top_after)
    }

    /// Decrease stake
    pub fn decrease_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        bond: Balance,
        less: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        let lowest_top_eq_highest_bottom =
            self.lowest_top_stake_amount == self.highest_bottom_stake_amount;
        let bond_geq_lowest_top = bond >= self.lowest_top_stake_amount;
        let stake_dne_err: DispatchError = Error::<T>::NoSuchStake.into();
        if bond_geq_lowest_top && !lowest_top_eq_highest_bottom {
            // definitely in top
            self.decrease_top_stake::<T>(candidate, staker.clone(), bond.into(), less)
        } else if bond_geq_lowest_top && lowest_top_eq_highest_bottom {
            // update top but if error then update bottom (because could be in bottom because
            // lowest_top_eq_highest_bottom)
            let result = self.decrease_top_stake::<T>(candidate, staker.clone(), bond.into(), less);
            if result == Err(stake_dne_err) {
                self.decrease_bottom_stake::<T>(candidate, staker, less)
            } else {
                result
            }
        } else {
            self.decrease_bottom_stake::<T>(candidate, staker, less)
        }
    }

    /// Decrease top stake
    pub fn decrease_top_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        bond: BalanceOf<T>,
        less: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance> + From<Balance>,
    {
        // The stake after the `decrease-stake` will be strictly less than the
        // highest bottom stake
        let bond_after_less_than_highest_bottom =
            bond.saturating_sub(less).into() < self.highest_bottom_stake_amount;
        // The top stakes is full and the bottom stakes has at least one stake
        let full_top_and_nonempty_bottom = matches!(self.top_capacity, CapacityStatus::Full)
            && !matches!(self.bottom_capacity, CapacityStatus::Empty);
        let mut top_stakes = <TopStakes<T>>::get(candidate).ok_or(Error::<T>::NoSuchCandidate)?;
        let in_top_after = if bond_after_less_than_highest_bottom && full_top_and_nonempty_bottom {
            let mut stake_option: Option<Bond<T::AccountId, BalanceOf<T>>> = None;
            // take stake from top
            top_stakes.stakes = top_stakes
                .stakes
                .clone()
                .into_iter()
                .filter(|d| {
                    if d.owner != staker {
                        true
                    } else {
                        top_stakes.total = top_stakes.total.saturating_sub(d.amount);
                        stake_option = Some(Bond {
                            owner: d.owner.clone(),
                            amount: d.amount.saturating_sub(less),
                        });
                        false
                    }
                })
                .collect();
            let stake = stake_option.ok_or(Error::<T>::NoSuchStake)?;
            // pop highest bottom by reverse and popping
            let mut bottom_stakes = <BottomStakes<T>>::get(candidate)
                .expect("CandidateInfo existence => BottomStakes existence");
            let highest_bottom_stake = bottom_stakes.stakes.remove(0);
            bottom_stakes.total = bottom_stakes
                .total
                .saturating_sub(highest_bottom_stake.amount);
            // insert highest bottom into top
            top_stakes.insert_sorted_greatest_to_least(highest_bottom_stake);
            // insert previous top into bottom
            bottom_stakes.insert_sorted_greatest_to_least(stake);
            self.reset_bottom_data::<T>(&bottom_stakes);
            <BottomStakes<T>>::insert(candidate, bottom_stakes);
            false
        } else {
            // keep it in the top
            let mut is_in_top = false;
            top_stakes.stakes = top_stakes
                .stakes
                .clone()
                .into_iter()
                .map(|d| {
                    if d.owner != staker {
                        d
                    } else {
                        is_in_top = true;
                        Bond {
                            owner: d.owner,
                            amount: d.amount.saturating_sub(less),
                        }
                    }
                })
                .collect();
            ensure!(is_in_top, Error::<T>::NoSuchStake);
            top_stakes.total = top_stakes.total.saturating_sub(less);
            top_stakes.sort_greatest_to_least();
            true
        };
        self.reset_top_data::<T>(candidate.clone(), &top_stakes);
        <TopStakes<T>>::insert(candidate, top_stakes);
        Ok(in_top_after)
    }

    /// Decrease bottom stake
    pub fn decrease_bottom_stake<T: Config>(
        &mut self,
        candidate: &T::AccountId,
        staker: T::AccountId,
        less: BalanceOf<T>,
    ) -> Result<bool, DispatchError>
    where
        BalanceOf<T>: Into<Balance>,
    {
        let mut bottom_stakes =
            <BottomStakes<T>>::get(candidate).expect("CandidateInfo exists => BottomStakes exists");
        let mut in_bottom = false;
        bottom_stakes.stakes = bottom_stakes
            .stakes
            .clone()
            .into_iter()
            .map(|d| {
                if d.owner != staker {
                    d
                } else {
                    in_bottom = true;
                    Bond {
                        owner: d.owner,
                        amount: d.amount.saturating_sub(less),
                    }
                }
            })
            .collect();
        ensure!(in_bottom, Error::<T>::NoSuchStake);
        bottom_stakes.sort_greatest_to_least();
        self.reset_bottom_data::<T>(&bottom_stakes);
        <BottomStakes<T>>::insert(candidate, bottom_stakes);
        Ok(false)
    }
}
