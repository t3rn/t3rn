use crate::{
    pallet::{
        BalanceOf, CandidateInfo, Config, Error, Event, Fixtures, Pallet, ScheduledStakingRequests,
        StakerInfo, Total,
    },
    subject_metadata::StakerMetadata,
};
use frame_support::{dispatch::DispatchResultWithPostInfo, ensure, traits::ReservableCurrency};
use sp_runtime::traits::Saturating;
use sp_std::{vec, vec::Vec};
use t3rn_primitives::{
    executors::{ScheduledStakingRequest, StakingAction},
    treasury::Treasury,
};

impl<T: Config> Pallet<T> {
    /// Schedules a [StakingAction::Revoke] for the staker, towards a given executor.
    pub(crate) fn stake_schedule_revoke(
        executor: T::AccountId,
        staker: T::AccountId,
    ) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);

        ensure!(
            !scheduled_requests.iter().any(|req| req.staker == staker),
            <Error<T>>::PendingStakeRequestAlreadyExists,
        );

        let bonded_amount = state
            .get_bond_amount(&executor)
            .ok_or(<Error<T>>::NoSuchStake)?;

        let now = T::Treasury::current_round().index;
        let when = now.saturating_add(<Fixtures<T>>::get().revoke_stake_delay);

        scheduled_requests.push(ScheduledStakingRequest {
            staker: staker.clone(),
            action: StakingAction::Revoke(bonded_amount),
            when_executable: when,
        });

        state.less_total = state.less_total.saturating_add(bonded_amount);

        <ScheduledStakingRequests<T>>::insert(executor.clone(), scheduled_requests);
        <StakerInfo<T>>::insert(staker.clone(), state);

        Self::deposit_event(Event::StakeRevocationScheduled {
            round: now,
            staker,
            candidate: executor,
            scheduled_exit: when,
        });

        Ok(().into())
    }

    /// Schedules a [StakingAction::Decrease] for the staker, towards a given executor.
    pub(crate) fn stake_schedule_bond_decrease(
        executor: T::AccountId,
        staker: T::AccountId,
        decrease_amount: BalanceOf<T>,
    ) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);

        ensure!(
            !scheduled_requests.iter().any(|req| req.staker == staker),
            <Error<T>>::PendingStakeRequestAlreadyExists,
        );

        let bonded_amount = state
            .get_bond_amount(&executor)
            .ok_or(<Error<T>>::NoSuchStake)?;

        ensure!(
            bonded_amount > decrease_amount,
            <Error<T>>::StakerBondBelowMin
        );

        let new_amount: BalanceOf<T> = bonded_amount - decrease_amount;
        let fixtures = <Fixtures<T>>::get();

        ensure!(
            new_amount >= fixtures.min_atomic_stake,
            <Error<T>>::StakeBelowMin
        );

        // Net Total is total after pending orders are executed
        let net_total = state.total.saturating_sub(state.less_total);

        // Net Total is always >= MinTotalStake
        let max_subtracted_amount = net_total.saturating_sub(<Fixtures<T>>::get().min_total_stake);

        ensure!(
            decrease_amount <= max_subtracted_amount,
            <Error<T>>::StakerBondBelowMin
        );

        let now = T::Treasury::current_round().index;
        let when = now.saturating_add(fixtures.revoke_stake_delay);

        scheduled_requests.push(ScheduledStakingRequest {
            staker: staker.clone(),
            action: StakingAction::Decrease(decrease_amount),
            when_executable: when,
        });

        state.less_total = state.less_total.saturating_add(decrease_amount);

        <ScheduledStakingRequests<T>>::insert(executor.clone(), scheduled_requests);
        <StakerInfo<T>>::insert(staker.clone(), state);

        Self::deposit_event(Event::StakeDecreaseScheduled {
            staker,
            candidate: executor,
            amount: decrease_amount,
            execute_round: when,
        });

        Ok(().into())
    }

    /// Cancels the staker's existing [ScheduledStakingRequest] towards a given executor.
    pub(crate) fn stake_cancel_request(
        executor: T::AccountId,
        staker: T::AccountId,
    ) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);

        let request = Self::cancel_request_with_state(&staker, &mut state, &mut scheduled_requests)
            .ok_or(<Error<T>>::NoSuchPendingStakeRequest)?;

        <ScheduledStakingRequests<T>>::insert(executor.clone(), scheduled_requests);
        <StakerInfo<T>>::insert(staker.clone(), state);

        Self::deposit_event(Event::StakeRequestCancelled {
            staker,
            executor,
            cancelled_request: request.into(),
        });

        Ok(().into())
    }

    fn cancel_request_with_state(
        staker: &T::AccountId,
        state: &mut StakerMetadata<T::AccountId, BalanceOf<T>>,
        scheduled_requests: &mut Vec<ScheduledStakingRequest<T::AccountId, BalanceOf<T>>>,
    ) -> Option<ScheduledStakingRequest<T::AccountId, BalanceOf<T>>> {
        let request_idx = scheduled_requests
            .iter()
            .position(|req| &req.staker == staker)?;

        let request = scheduled_requests.remove(request_idx);
        let amount = request.action.amount();

        state.less_total = state.less_total.saturating_sub(amount);

        Some(request)
    }

    /// Executes the staker's existing [ScheduledStakingRequest] towards a given executor.
    pub(crate) fn stake_execute_scheduled_request(
        executor: T::AccountId,
        staker: T::AccountId,
    ) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);

        let request_idx = scheduled_requests
            .iter()
            .position(|req| req.staker == staker)
            .ok_or(<Error<T>>::NoSuchPendingStakeRequest)?;

        let request = &scheduled_requests[request_idx];
        let fixtures = <Fixtures<T>>::get();
        let now = T::Treasury::current_round().index;

        ensure!(
            request.when_executable <= now,
            <Error<T>>::PendingStakeRequestNotDueYet
        );

        match request.action {
            StakingAction::Revoke(amount) => {
                // revoking last stake => leaving set of stakers
                let leaving = if state.stakes.0.len() == 1usize {
                    true
                } else {
                    ensure!(
                        state.total.saturating_sub(fixtures.min_total_stake) >= amount,
                        <Error<T>>::StakerBondBelowMin
                    );
                    false
                };

                // remove from pending requests
                let amount = scheduled_requests.remove(request_idx).action.amount();
                state.less_total = state.less_total.saturating_sub(amount);

                // remove stake from staker state
                state.rm_stake(&executor);

                // remove stake from executor state stakes
                Self::staker_leaves_candidate(executor.clone(), staker.clone(), amount)?;

                Self::deposit_event(Event::StakeRevoked {
                    staker: staker.clone(),
                    candidate: executor.clone(),
                    unstaked: amount,
                });

                <ScheduledStakingRequests<T>>::insert(executor, scheduled_requests);

                if leaving {
                    <StakerInfo<T>>::remove(&staker);
                    Self::deposit_event(Event::StakerLeft {
                        staker,
                        unstaked: amount,
                    });
                } else {
                    <StakerInfo<T>>::insert(&staker, state);
                }

                Ok(().into())
            },
            StakingAction::Decrease(_) => {
                // remove from pending requests
                let amount = scheduled_requests.remove(request_idx).action.amount();
                state.less_total = state.less_total.saturating_sub(amount);
                sp_std::if_std! { println!("StakingAction::Decrease"); }
                // decrease stake
                for bond in &mut state.stakes.0 {
                    if bond.owner == executor {
                        return if bond.amount > amount {
                            let amount_before: BalanceOf<T> = bond.amount;
                            bond.amount = bond.amount.saturating_sub(amount);

                            state.total = state.total.saturating_sub(amount);
                            let new_total: BalanceOf<T> = state.total;

                            ensure!(
                                new_total >= fixtures.min_atomic_stake,
                                <Error<T>>::StakeBelowMin
                            );
                            ensure!(
                                new_total >= fixtures.min_total_stake,
                                <Error<T>>::StakerBondBelowMin
                            );

                            let mut executor_info = <CandidateInfo<T>>::get(&executor)
                                .ok_or(<Error<T>>::NoSuchCandidate)?;

                            T::Currency::unreserve(&staker, amount);

                            // need to go into decrease_stake
                            let in_top = executor_info.decrease_stake::<T>(
                                &executor,
                                staker.clone(),
                                amount_before,
                                amount,
                            )?;

                            <CandidateInfo<T>>::insert(&executor, executor_info);

                            let new_total_staked = <Total<T>>::get().saturating_sub(amount);

                            <Total<T>>::put(new_total_staked);

                            <ScheduledStakingRequests<T>>::insert(
                                executor.clone(),
                                scheduled_requests,
                            );

                            <StakerInfo<T>>::insert(staker.clone(), state);

                            Self::deposit_event(Event::StakeDecreased {
                                staker,
                                candidate: executor.clone(),
                                amount,
                                in_top,
                            });

                            Ok(().into())
                        } else {
                            // must rm entire stake if bond.amount <= less or cancel request
                            Err(<Error<T>>::StakeBelowMin.into())
                        }
                    }
                }
                Err(<Error<T>>::NoSuchStake.into())
            },
        }
    }

    /// Schedules [StakingAction::Revoke] for the staker, towards all delegated executor.
    /// The last fulfilled request causes the staker to leave the set of stakers.
    pub(crate) fn staker_schedule_revoke_all(staker: T::AccountId) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut updated_scheduled_requests = vec![];
        let now = T::Treasury::current_round().index;
        let when = now.saturating_add(<Fixtures<T>>::get().leave_stakers_delay);

        // it is assumed that a multiple stakes to the same executor does not exist, else this
        // will cause a bug - the last duplicate stake update will be the only one applied.
        let mut existing_revoke_count = 0;
        for bond in state.stakes.0.clone() {
            let executor = bond.owner;
            let bonded_amount = bond.amount;
            let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);

            // cancel any existing requests
            let request =
                Self::cancel_request_with_state(&staker, &mut state, &mut scheduled_requests);
            let request = match request {
                Some(revoke_req) if matches!(revoke_req.action, StakingAction::Revoke(_)) => {
                    existing_revoke_count += 1;
                    revoke_req // re-insert the same Revoke request
                },
                _ => ScheduledStakingRequest {
                    staker: staker.clone(),
                    action: StakingAction::Revoke(bonded_amount),
                    when_executable: when,
                },
            };

            scheduled_requests.push(request);
            state.less_total = state.less_total.saturating_add(bonded_amount);
            updated_scheduled_requests.push((executor, scheduled_requests));
        }

        if existing_revoke_count == state.stakes.0.len() {
            return Err(<Error<T>>::StakerAlreadyLeaving.into())
        }

        updated_scheduled_requests
            .into_iter()
            .for_each(|(executor, scheduled_requests)| {
                <ScheduledStakingRequests<T>>::insert(executor, scheduled_requests);
            });

        <StakerInfo<T>>::insert(staker.clone(), state);
        Self::deposit_event(Event::StakerExitScheduled {
            round: now,
            staker,
            scheduled_exit: when,
        });

        Ok(().into())
    }

    /// Cancels every [StakingAction::Revoke] request for a staker towards a executor.
    /// Each stake must have a [StakingAction::Revoke] scheduled that must be allowed to be
    /// executed in the current round, for this function to succeed.
    pub(crate) fn staker_cancel_scheduled_revoke_all(
        staker: T::AccountId,
    ) -> DispatchResultWithPostInfo {
        let mut state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;
        let mut updated_scheduled_requests = vec![];

        // pre-validate that all stakes have a Revoke request.
        for bond in &state.stakes.0 {
            let executor = bond.owner.clone();
            let scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);
            scheduled_requests
                .iter()
                .find(|req| req.staker == staker && matches!(req.action, StakingAction::Revoke(_)))
                .ok_or(<Error<T>>::StakerNotLeaving)?;
        }

        // cancel all requests
        for bond in state.stakes.0.clone() {
            let executor = bond.owner.clone();
            let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(&executor);
            Self::cancel_request_with_state(&staker, &mut state, &mut scheduled_requests);
            updated_scheduled_requests.push((executor, scheduled_requests));
        }

        updated_scheduled_requests
            .into_iter()
            .for_each(|(executor, scheduled_requests)| {
                <ScheduledStakingRequests<T>>::insert(executor, scheduled_requests);
            });

        <StakerInfo<T>>::insert(staker.clone(), state);
        Self::deposit_event(Event::StakerExitCancelled { staker });

        Ok(().into())
    }

    /// Executes every [StakingAction::Revoke] request for a staker towards a executor.
    /// Each stake must have a [StakingAction::Revoke] scheduled that must be allowed to be
    /// executed in the current round, for this function to succeed.
    pub(crate) fn staker_execute_scheduled_revoke_all(
        staker: T::AccountId,
        stake_count: u32,
    ) -> DispatchResultWithPostInfo {
        let state = <StakerInfo<T>>::get(&staker).ok_or(<Error<T>>::NoSuchStaker)?;

        ensure!(
            stake_count >= (state.stakes.0.len() as u32),
            Error::<T>::TooLowStakeCountToLeaveStakers
        );

        let now = T::Treasury::current_round().index;

        let mut validated_scheduled_requests = vec![];

        // pre-validate that all stakes have a Revoke request that can be executed now.
        for bond in &state.stakes.0 {
            let scheduled_requests = <ScheduledStakingRequests<T>>::get(&bond.owner);

            let request_idx = scheduled_requests
                .iter()
                .position(|req| {
                    req.staker == staker && matches!(req.action, StakingAction::Revoke(_))
                })
                .ok_or(<Error<T>>::StakerNotLeaving)?;

            let request = &scheduled_requests[request_idx];

            ensure!(
                request.when_executable <= now,
                <Error<T>>::StakerCannotLeaveYet
            );

            validated_scheduled_requests.push((bond.clone(), scheduled_requests, request_idx))
        }

        let mut updated_scheduled_requests = vec![];

        // we do not update the staker state, since the it will be completely removed
        for (bond, mut scheduled_requests, request_idx) in validated_scheduled_requests {
            let executor = bond.owner;

            if let Err(error) =
                Self::staker_leaves_candidate(executor.clone(), staker.clone(), bond.amount)
            {
                log::warn!(
                    "STORAGE CORRUPTED \nDelegator {:?} leaving executor failed with error: {:?}",
                    staker,
                    error
                );
            }

            // remove the scheduled request, since it is fulfilled
            scheduled_requests.remove(request_idx).action.amount();
            updated_scheduled_requests.push((executor, scheduled_requests));
        }

        updated_scheduled_requests
            .into_iter()
            .for_each(|(executor, scheduled_requests)| {
                <ScheduledStakingRequests<T>>::insert(executor, scheduled_requests);
            });

        Self::deposit_event(Event::StakerLeft {
            staker: staker.clone(),
            unstaked: state.total,
        });

        <StakerInfo<T>>::remove(&staker);

        Ok(().into())
    }

    /// Removes the staker's existing [ScheduledStakingRequest] towards a given executor, if exists.
    /// The state needs to be persisted by the caller of this function.
    pub(crate) fn stake_remove_request_with_state(
        executor: &T::AccountId,
        staker: &T::AccountId,
        state: &mut StakerMetadata<T::AccountId, BalanceOf<T>>,
    ) {
        let mut scheduled_requests = <ScheduledStakingRequests<T>>::get(executor);

        let maybe_request_idx = scheduled_requests
            .iter()
            .position(|req| &req.staker == staker);

        if let Some(request_idx) = maybe_request_idx {
            let request = scheduled_requests.remove(request_idx);
            let amount = request.action.amount();
            state.less_total = state.less_total.saturating_sub(amount);
            <ScheduledStakingRequests<T>>::insert(executor, scheduled_requests);
        }
    }

    /// Returns true if a [ScheduledStakingRequest] exists for a given stake
    pub fn stake_request_exists(executor: &T::AccountId, staker: &T::AccountId) -> bool {
        <ScheduledStakingRequests<T>>::get(executor)
            .iter()
            .any(|req| &req.staker == staker)
    }

    /// Returns true if a [StakingAction::Revoke] [ScheduledStakingRequest] exists for a given stake
    pub fn stake_request_revoke_exists(executor: &T::AccountId, staker: &T::AccountId) -> bool {
        <ScheduledStakingRequests<T>>::get(executor)
            .iter()
            .any(|req| &req.staker == staker && matches!(req.action, StakingAction::Revoke(_)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::Test;
    use t3rn_primitives::{
        common::OrderedSet,
        executors::{Bond, StakerStatus},
    };

    #[test]
    fn test_cancel_request_with_state_removes_request_for_correct_staker_and_updates_state() {
        let mut state = StakerMetadata {
            id: 1,
            stakes: OrderedSet::from(vec![Bond {
                amount: 100,
                owner: 2,
            }]),
            total: 100,
            less_total: 100,
            status: StakerStatus::Active,
        };
        let mut scheduled_requests = vec![
            ScheduledStakingRequest {
                staker: 1,
                when_executable: 1,
                action: StakingAction::Revoke(100),
            },
            ScheduledStakingRequest {
                staker: 2,
                when_executable: 1,
                action: StakingAction::Decrease(50),
            },
        ];
        let removed_request =
            <Pallet<Test>>::cancel_request_with_state(&1, &mut state, &mut scheduled_requests);

        assert_eq!(
            removed_request,
            Some(ScheduledStakingRequest {
                staker: 1,
                when_executable: 1,
                action: StakingAction::Revoke(100),
            })
        );
        assert_eq!(
            scheduled_requests,
            vec![ScheduledStakingRequest {
                staker: 2,
                when_executable: 1,
                action: StakingAction::Decrease(50),
            },]
        );
        assert_eq!(
            state,
            StakerMetadata {
                id: 1,
                stakes: OrderedSet::from(vec![Bond {
                    amount: 100,
                    owner: 2,
                }]),
                total: 100,
                less_total: 0,
                status: StakerStatus::Active,
            }
        );
    }

    #[test]
    fn test_cancel_request_with_state_does_nothing_when_request_does_not_exist() {
        let mut state = StakerMetadata {
            id: 1,
            stakes: OrderedSet::from(vec![Bond {
                amount: 100,
                owner: 2,
            }]),
            total: 100,
            less_total: 100,
            status: StakerStatus::Active,
        };
        let mut scheduled_requests = vec![ScheduledStakingRequest {
            staker: 2,
            when_executable: 1,
            action: StakingAction::Decrease(50),
        }];
        let removed_request =
            <Pallet<Test>>::cancel_request_with_state(&1, &mut state, &mut scheduled_requests);

        assert_eq!(removed_request, None,);
        assert_eq!(
            scheduled_requests,
            vec![ScheduledStakingRequest {
                staker: 2,
                when_executable: 1,
                action: StakingAction::Decrease(50),
            },]
        );
        assert_eq!(
            state,
            StakerMetadata {
                id: 1,
                stakes: OrderedSet::from(vec![Bond {
                    amount: 100,
                    owner: 2,
                }]),
                total: 100,
                less_total: 100,
                status: StakerStatus::Active,
            }
        );
    }
}
