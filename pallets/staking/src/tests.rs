use crate::{
    assert_last_event, assert_last_n_events,
    mock::{
        fast_forward_to, new_test_ext, Balance, Event as MockEvent, Origin, Staking, System, Test,
        Treasury,
    },
    pallet::{
        BottomStakes, CandidateInfo, CandidatePool, Config, Error, Event, ExecutorConfig,
        ScheduledConfigurationRequests, StakerInfo, TopStakes, Total,
    },
    stakes::Stakes,
    subject_metadata::{CandidateMetadata, StakerMetadata},
};

use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::Percent;
use t3rn_primitives::{
    common::{OrderedSet, Range, DEFAULT_ROUND_TERM},
    monetary::DECIMALS,
    staking::{
        Bond, ExecutorInfo, ExecutorStatus, Fixtures as StakingFixtures,
        ScheduledConfigurationRequest, ScheduledStakingRequest, StakerAdded, StakerStatus,
        StakingAction,
    },
};

/*
        pub fn set_fixtures(
            origin: OriginFor<T>,
            fixtures: StakingFixtures<BalanceOf<T>>,

        pub fn schedule_configure_executor(
            origin: OriginFor<T>,
            commission: Percent,
            risk: Percent,

        pub fn cancel_configure_executor(origin: OriginFor<T>)

        pub fn candidate_bond_more(origin: OriginFor<T>, amount: BalanceOf<T>)

        pub fn schedule_candidate_bond_less(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,

        pub fn schedule_candidate_bond_less(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {

        pub fn execute_candidate_bond_less(
            origin: OriginFor<T>,
            candidate: T::AccountId,

            pub fn cancel_candidate_bond_less(origin: OriginFor<T>)

        pub fn join_candidates(
            origin: OriginFor<T>,
            bond: BalanceOf<T>,
            candidate_count: u32,

        pub fn schedule_leave_candidates(
            origin: OriginFor<T>,
            candidate_count: u32,

        pub fn execute_leave_candidates(
            origin: OriginFor<T>,
            candidate: T::AccountId,
            candidate_stake_count: u32,

        pub fn cancel_leave_candidates(
            origin: OriginFor<T>,
            candidate_count: u32,

        go_offline
        go_online
*/

#[test]
fn fixtures_can_only_be_set_by_sudo() {
    new_test_ext().execute_with(|| {
        let fixtures: StakingFixtures<Balance> = Default::default();

        assert_noop!(
            Staking::set_fixtures(Origin::signed(419), fixtures),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn fixtures_cannot_be_zero() {
    new_test_ext().execute_with(|| {
        let fixtures: StakingFixtures<Balance> = Default::default();

        assert_noop!(
            Staking::set_fixtures(Origin::root(), fixtures),
            <Error<Test>>::FixturesCannotBeZero,
        );
    });
}

#[test]
fn fixtures_set_and_emitted() {
    new_test_ext().execute_with(|| {
        let fixtures = StakingFixtures {
            active_set_size: Range {
                min: 1,
                ideal: 3,
                max: 128,
            },
            max_commission: Percent::from_percent(50),
            max_risk: Percent::from_percent(50),
            min_executor_bond: 1000_u64,
            min_candidate_bond: 1000_u64,
            min_atomic_stake: 500_u64,
            min_total_stake: 500_u64,
            max_top_stakes_per_candidate: 300,
            max_bottom_stakes_per_candidate: 50,
            max_stakes_per_staker: 100,
            // delays target a 14d term assuming a 6h round term
            configure_executor_delay: 56,
            leave_candidates_delay: 56,
            leave_stakers_delay: 56,
            candidate_bond_less_delay: 56,
            revoke_stake_delay: 56,
        };

        assert_ok!(Staking::set_fixtures(Origin::root(), fixtures));

        assert_last_event!(MockEvent::Staking(Event::FixturesConfigured {
            active_set_size: Range {
                min: 1,
                ideal: 3,
                max: 128,
            },
            max_commission: Percent::from_percent(50),
            max_risk: Percent::from_percent(50),
            min_executor_bond: 1000_u64,
            min_candidate_bond: 1000_u64,
            min_atomic_stake: 500_u64,
            min_total_stake: 500_u64,
            max_top_stakes_per_candidate: 300,
            max_bottom_stakes_per_candidate: 50,
            max_stakes_per_staker: 100,
            // delays target a 14d term assuming a 6h round term
            configure_executor_delay: 56,
            leave_candidates_delay: 56,
            leave_stakers_delay: 56,
            candidate_bond_less_delay: 56,
            revoke_stake_delay: 56,
        }));
    });
}

#[test]
fn schedule_configure_executor_fails_if_risk_gt_max() {
    new_test_ext().execute_with(|| {
        let commission = Percent::from_percent(10);
        let risk = Percent::from_percent(51);

        assert_noop!(
            Staking::schedule_configure_executor(Origin::signed(3), commission, risk),
            <Error<Test>>::TooMuchRisk,
        );
    });
}

#[test]
fn schedule_configure_executor_fails_if_commission_gt_max() {
    new_test_ext().execute_with(|| {
        let commission = Percent::from_percent(99);
        let risk = Percent::from_percent(1);

        assert_noop!(
            Staking::schedule_configure_executor(Origin::signed(3), commission, risk),
            <Error<Test>>::TooMuchCommission,
        );
    });
}

#[test]
fn initial_executor_configuration_is_effective_immediately() {
    new_test_ext().execute_with(|| {
        let executor = 13;
        let commission = Percent::from_percent(10);
        let risk = Percent::from_percent(42);

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            commission,
            risk
        ));

        assert_eq!(
            <ExecutorConfig<Test>>::get(executor).unwrap(),
            ExecutorInfo { commission, risk }
        );

        assert_last_event!(MockEvent::Staking(Event::ExecutorConfigured {
            executor,
            commission,
            risk,
        }));
    });
}

#[test]
fn executor_reconfiguration_gets_scheduled() {
    new_test_ext().execute_with(|| {
        let fixtures = Staking::fixtures();
        let executor = 14;

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ExecutorConfig<Test>>::get(executor).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(10),
                risk: Percent::from_percent(42)
            }
        );

        assert_last_event!(MockEvent::Staking(Event::ExecutorConfigured {
            executor,
            commission: Percent::from_percent(10),
            risk: Percent::from_percent(42),
        }));

        assert_eq!(
            <ScheduledConfigurationRequests<Test>>::get(executor).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Treasury::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay),
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );
    });
}

#[test]
fn anyone_can_execute_scheduled_reconfiguration() {
    new_test_ext().execute_with(|| {
        let fixtures = Staking::fixtures();
        let executor = 14;
        let other_user = 15;

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ExecutorConfig<Test>>::get(executor).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(10),
                risk: Percent::from_percent(42)
            }
        );

        assert_last_event!(MockEvent::Staking(Event::ExecutorConfigured {
            executor,
            commission: Percent::from_percent(10),
            risk: Percent::from_percent(42),
        }));

        assert_eq!(
            <ScheduledConfigurationRequests<Test>>::get(executor).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Treasury::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay),
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );

        fast_forward_to(
            System::block_number()
                + (DEFAULT_ROUND_TERM * fixtures.configure_executor_delay) as u64,
        );

        assert_ok!(Staking::execute_configure_executor(
            Origin::signed(other_user),
            executor,
        ));

        assert_last_event!(MockEvent::Staking(Event::ExecutorConfigured {
            executor,
            commission: Percent::from_percent(20),
            risk: Percent::from_percent(32),
        }));

        assert_eq!(
            <ExecutorConfig<Test>>::get(executor).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );
    });
}

#[test]
fn only_executor_can_cancel_scheduled_configuration() {
    new_test_ext().execute_with(|| {
        let fixtures = Staking::fixtures();
        let executor = 14;
        let other_user = 15;

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Staking::schedule_configure_executor(
            Origin::signed(executor),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ScheduledConfigurationRequests<Test>>::get(executor).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Treasury::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay),
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );

        assert_noop!(
            Staking::cancel_configure_executor(Origin::signed(other_user)),
            Error::<Test>::NoSuchConfigurationRequest
        );

        assert_ok!(Staking::cancel_configure_executor(Origin::signed(executor)));

        assert_eq!(
            <ExecutorConfig<Test>>::get(executor).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(10),
                risk: Percent::from_percent(42),
            }
        );
    });
}

#[test]
fn cannot_double_join_candidates() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::join_candidates(Origin::signed(executor), min_candidate_bond, 1),
            <Error<Test>>::CandidateExists
        );
    });
}

#[test]
fn cannot_join_candidates_as_staker() {
    new_test_ext().execute_with(|| {
        let staker = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_candidate_bond,
        ));

        <StakerInfo<Test>>::insert(
            &staker,
            StakerMetadata {
                id: staker,
                stakes: OrderedSet(vec![]),
                total: 0,
                less_total: 0,
                status: StakerStatus::Active,
            },
        );

        assert_noop!(
            Staking::join_candidates(Origin::signed(staker), min_candidate_bond, 0),
            <Error<Test>>::StakerExists
        );
    });
}

#[test]
fn join_candidates_enforces_a_min_bond() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_noop!(
            Staking::join_candidates(Origin::signed(executor), min_candidate_bond, 0),
            <Error<Test>>::CandidateBondBelowMin
        );
    });
}

#[test]
fn join_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        <CandidatePool<Test>>::set(OrderedSet(vec![Bond {
            owner: executor,
            amount: 0,
        }]));

        assert_noop!(
            Staking::join_candidates(Origin::signed(executor), min_candidate_bond, 0),
            <Error<Test>>::TooLowCandidateCountWeightHintJoinCandidates
        );
    });
}

#[test]
fn join_candidates_fails_on_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        assert_noop!(
            Staking::join_candidates(Origin::signed(executor), min_candidate_bond, 0),
            <Error<Test>>::InsufficientBalance
        );
    });
}

#[test]
fn join_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let empty_stakes: Stakes<<Test as frame_system::Config>::AccountId, Balance> =
            Default::default();

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            1
        ));

        assert_eq!(
            <CandidateInfo<Test>>::get(&executor).unwrap(),
            CandidateMetadata::new(min_candidate_bond),
        );

        assert_eq!(<TopStakes<Test>>::get(&executor).unwrap(), empty_stakes);

        assert_eq!(<BottomStakes<Test>>::get(&executor).unwrap(), empty_stakes);

        assert_eq!(
            <CandidatePool<Test>>::get(),
            OrderedSet(vec![Bond {
                owner: executor,
                amount: min_candidate_bond,
            }])
        );

        assert_eq!(<Total<Test>>::get(), min_candidate_bond);

        assert_last_event!(MockEvent::Staking(Event::CandidateJoined {
            account: executor,
            amount_locked: min_candidate_bond,
            total_locked: min_candidate_bond,
        }));
    });
}

#[test]
fn schedule_leave_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::schedule_leave_candidates(Origin::signed(executor), 0),
            <Error<Test>>::TooLowCandidateCountToLeaveCandidates
        );
    });
}

#[test]
fn schedule_leave_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let now = Treasury::current_round().index;
        let leave_candidates_delay = Staking::fixtures().leave_candidates_delay;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            1
        ),);

        assert_eq!(<CandidatePool<Test>>::get().0.len(), 0);

        assert_eq!(
            <CandidateInfo<Test>>::get(executor).unwrap().status,
            ExecutorStatus::Leaving(now + leave_candidates_delay)
        );

        assert_last_event!(MockEvent::Staking(Event::CandidateExitScheduled {
            exit_allowed_round: now,
            candidate: executor,
            scheduled_exit: now + leave_candidates_delay,
        }));
    });
}

#[test]
fn execute_leave_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let staker = 15;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            1
        ));

        fast_forward_to(
            ((Treasury::current_round().index + Staking::fixtures().leave_candidates_delay)
                * DEFAULT_ROUND_TERM)
                .into(),
        );

        assert_noop!(
            Staking::execute_leave_candidates(Origin::signed(executor), executor, 0),
            <Error<Test>>::TooLowCandidateStakeCountToLeaveCandidates
        );
    });
}

//
#[test]
fn execute_leave_candidates_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let staker = 15;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::execute_leave_candidates(Origin::signed(executor), executor, 0),
            <Error<Test>>::CandidateNotLeaving
        );
    });
}

#[test]
fn execute_leave_candidates_fails_if_too_early() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let staker = 15;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            1
        ));

        assert_noop!(
            Staking::execute_leave_candidates(Origin::signed(executor), executor, 1),
            <Error<Test>>::CandidateCannotLeaveYet
        );
    });
}

#[test]
fn anyone_can_execute_leave_candidates() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let staker = 15;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            1
        ));

        fast_forward_to(
            ((Treasury::current_round().index + Staking::fixtures().leave_candidates_delay)
                * DEFAULT_ROUND_TERM)
                .into(),
        );

        assert_ok!(Staking::execute_leave_candidates(
            Origin::signed(staker),
            executor,
            1
        ));

        assert_eq!(Staking::staker_info(staker), None);

        assert_eq!(Staking::top_stakes(executor), None);

        assert_eq!(Staking::bottom_stakes(executor), None);

        assert_eq!(Staking::candidate_info(executor), None);

        assert_eq!(Staking::scheduled_staking_requests(staker), vec![]);

        assert_eq!(Staking::total_value_locked(), 0);

        assert_last_event!(MockEvent::Staking(Event::CandidateLeft {
            candidate: executor,
            amount_unlocked: min_candidate_bond + min_atomic_stake,
            total_locked: 0,
        }));
    });
}

#[test]
fn cancel_leave_candidates_fails_if_not_executor() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let staker = 15;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            1
        ),);

        assert_noop!(
            Staking::cancel_leave_candidates(Origin::signed(staker), 1),
            <Error<Test>>::NoSuchCandidate,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::cancel_leave_candidates(Origin::signed(executor), 1),
            <Error<Test>>::CandidateNotLeaving,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_too_low_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor1 = 14;
        let executor2 = 13;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor1,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &executor2,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor1),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor2),
            min_candidate_bond,
            1
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor1),
            2
        ),);

        assert_noop!(
            Staking::cancel_leave_candidates(Origin::signed(executor1), 0),
            <Error<Test>>::TooLowCandidateCountWeightHintCancelLeaveCandidates,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_already_active() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            2
        ),);

        <CandidatePool<Test>>::put(OrderedSet(vec![Bond {
            owner: executor,
            amount: min_candidate_bond + min_atomic_stake,
        }]));

        assert_noop!(
            Staking::cancel_leave_candidates(Origin::signed(executor), 1),
            <Error<Test>>::AlreadyActive,
        );
    });
}

#[test]
fn cancel_leave_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::schedule_leave_candidates(
            Origin::signed(executor),
            2
        ),);

        assert_ok!(Staking::cancel_leave_candidates(
            Origin::signed(executor),
            1
        ));

        assert_eq!(
            Staking::candidate_pool().contains(&Bond {
                owner: executor,
                amount: 0, // ignored by PartialEq
            }),
            true
        );

        assert_eq!(Staking::candidate_info(executor).is_some(), true);

        assert_last_event!(MockEvent::Staking(Event::CandidateExitCancelled {
            candidate: executor,
        }));
    });
}

#[test]
fn go_offline_fails_if_not_candidate() {
    new_test_ext().execute_with(|| {
        let staker = 15;

        assert_noop!(
            Staking::go_offline(Origin::signed(staker)),
            <Error<Test>>::NoSuchCandidate,
        );
    });
}

#[test]
fn go_offline_successfully() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::go_offline(Origin::signed(executor)));

        assert_eq!(
            Staking::candidate_pool().contains(&Bond {
                owner: executor,
                amount: 0, // ignored by PartialEq
            }),
            false
        );

        assert_eq!(
            Staking::candidate_info(executor).unwrap().status,
            ExecutorStatus::Idle
        );

        assert_last_event!(MockEvent::Staking(Event::CandidateWentOffline {
            candidate: executor,
        }));
    });
}

#[test]
fn go_online_fails_if_not_candidate() {
    new_test_ext().execute_with(|| {
        let staker = 15;

        assert_noop!(
            Staking::go_online(Origin::signed(staker)),
            <Error<Test>>::NoSuchCandidate,
        );
    });
}

#[test]
fn go_online_successfully() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::go_offline(Origin::signed(executor)));

        assert_ok!(Staking::go_online(Origin::signed(executor)));

        assert_eq!(
            Staking::candidate_pool().contains(&Bond {
                owner: executor,
                amount: 0, // ignored by PartialEq
            }),
            true
        );

        assert_eq!(
            Staking::candidate_info(executor).unwrap().status,
            ExecutorStatus::Active
        );

        assert_last_event!(MockEvent::Staking(Event::CandidateBackOnline {
            candidate: executor,
        }));
    });
}

#[test]
fn stake_fails_on_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, min_atomic_stake, 0, 0),
            Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn first_staking_requires_min_bond() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, 419, 0, 0),
            Error::<Test>::StakerBondBelowMin
        );
    });
}

#[test]
fn non_first_stakes_enforce_a_minimum() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, 419, 1, 1),
            <Error<Test>>::StakeBelowMin
        );
    });
}

#[test]
fn candidates_cannot_stake() {
    new_test_ext().execute_with(|| {
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::stake(Origin::signed(executor), executor, min_atomic_stake, 0, 0),
            Error::<Test>::CandidateExists
        );
    });
}

#[test]
fn non_first_stakes_enforce_a_stake_count_weight_hint() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, min_atomic_stake, 1, 0),
            <Error<Test>>::TooLowStakeCountToStake
        );
    });
}

#[test]
fn staker_stakings_are_capped() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            101 * min_atomic_stake,
        ));

        let mut stake_count = 0;
        let mut candidate_stake_count = 0;
        let mut exec = 16;

        for i in 1..=100 {
            drop(<Test as Config>::Currency::deposit_creating(
                &exec,
                min_candidate_bond,
            ));

            assert_ok!(Staking::join_candidates(
                Origin::signed(exec),
                min_candidate_bond,
                i - 1
            ));

            assert_ok!(Staking::stake(
                Origin::signed(staker),
                exec,
                min_atomic_stake,
                stake_count,
                candidate_stake_count
            ));
            stake_count = stake_count + 1;
            candidate_stake_count = candidate_stake_count + 1;
            exec = exec + 1;
        }

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, min_atomic_stake, 100, 100),
            <Error<Test>>::MaxStakesExceeded
        );
    });
}

#[test]
fn cannot_stake_twice_on_the_same_candidate() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, min_atomic_stake, 1, 1),
            <Error<Test>>::AlreadyStakedCandidate
        );
    });
}

#[test]
fn cannot_stake_on_non_candidate() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_noop!(
            Staking::stake(Origin::signed(staker), executor, min_atomic_stake, 0, 0),
            <Error<Test>>::NoSuchCandidate
        );
    });
}

#[test]
fn cannot_stake_with_insufficient_candidate_stake_count() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let other_staker = 44;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &other_staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::stake(
                Origin::signed(other_staker),
                executor,
                min_atomic_stake,
                0,
                1
            ),
            <Error<Test>>::TooLowCandidateStakeCountToStake
        );
    });
}

#[test]
fn stake_sets_storage_and_emits_events() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_eq!(
            Staking::total_value_locked(),
            min_atomic_stake + min_candidate_bond
        );

        assert_eq!(Staking::candidate_info(executor).unwrap().stake_count, 1);

        assert_eq!(
            Staking::staker_info(staker).unwrap().total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Staking(Event::StakeAdded {
            staker,
            amount_locked: min_atomic_stake,
            candidate: executor,
            staker_position: StakerAdded::ToTop {
                new_total: min_atomic_stake + min_candidate_bond
            },
        }));
    });
}

#[test]
fn schedule_revoke_stake_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::schedule_revoke_stake(Origin::signed(executor), executor),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn cannot_schedule_revoke_stake_twice() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ),);

        assert_noop!(
            Staking::schedule_revoke_stake(Origin::signed(staker), executor),
            <Error<Test>>::PendingStakeRequestAlreadyExists
        );
    });
}

#[test]
fn schedule_revoke_stake_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ),);

        assert_eq!(
            Staking::scheduled_staking_requests(executor),
            vec![ScheduledStakingRequest {
                staker,
                action: StakingAction::Revoke(min_atomic_stake),
                when_executable: Treasury::current_round().index
                    + Staking::fixtures().revoke_stake_delay,
            }]
        );

        assert_eq!(
            Staking::staker_info(staker).unwrap().less_total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Staking(Event::StakeRevocationScheduled {
            round: Treasury::current_round().index,
            staker,
            candidate: executor,
            scheduled_exit: Treasury::current_round().index
                + Staking::fixtures().revoke_stake_delay,
        }));
    });
}

#[test]
fn cancel_stake_request_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;

        assert_noop!(
            Staking::cancel_stake_request(Origin::signed(staker), executor),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_stake_request_fails_if_not_requested() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::cancel_stake_request(Origin::signed(staker), executor),
            <Error<Test>>::NoSuchPendingStakeRequest
        );
    });
}

#[test]
fn cancel_stake_request_fails_if_origin_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let other_staker = 16;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1,
        ));

        assert_noop!(
            Staking::cancel_stake_request(Origin::signed(other_staker), executor,),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_stake_request_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ));

        let req = Staking::scheduled_staking_requests(executor)[0].clone();

        assert_ok!(Staking::cancel_stake_request(
            Origin::signed(staker),
            executor
        ));

        assert_eq!(Staking::scheduled_staking_requests(executor), vec![]);

        assert_eq!(Staking::staker_info(staker).unwrap().less_total, 0);

        assert_last_event!(MockEvent::Staking(Event::StakeRequestCancelled {
            staker,
            executor,
            cancelled_request: req.into(),
        }));
    });
}

#[test]
fn staker_bond_more_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;

        assert_noop!(
            Staking::staker_bond_more(Origin::signed(staker), executor, 1),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn staker_bond_more_fails_if_pending_stake_revoke() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ));

        assert_noop!(
            Staking::staker_bond_more(Origin::signed(staker), executor, 1),
            <Error<Test>>::PendingStakeRevoke
        );
    });
}

#[test]
fn staker_bond_more_fails_if_no_such_stake() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let other_exec = 16;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::staker_bond_more(Origin::signed(staker), other_exec, 1),
            <Error<Test>>::NoSuchStake
        );
    });
}

#[test]
fn staker_bond_more_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::staker_bond_more(
            Origin::signed(staker),
            executor,
            1
        ));

        assert_eq!(
            Staking::candidate_info(executor).unwrap().total_counted,
            min_candidate_bond + min_atomic_stake + 1
        );

        assert_eq!(
            Staking::total_value_locked(),
            min_candidate_bond + min_atomic_stake + 1
        );

        assert_eq!(
            Staking::staker_info(staker).unwrap().total,
            min_atomic_stake + 1
        );

        assert_last_event!(MockEvent::Staking(Event::StakeIncreased {
            staker,
            candidate: executor,
            amount: 1,
            in_top: true
        }));
    });
}

#[test]
fn schedule_staker_bond_less_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;

        assert_noop!(
            Staking::schedule_staker_bond_less(Origin::signed(staker), executor, 1),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn schedule_staker_bond_less_fails_if_pending_stake_request_exists() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ));

        assert_noop!(
            Staking::schedule_staker_bond_less(Origin::signed(staker), executor, 1),
            <Error<Test>>::PendingStakeRequestAlreadyExists
        );
    });
}

#[test]
fn schedule_staker_bond_less_fails_if_no_such_stake() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let other_exec = 16;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::schedule_staker_bond_less(Origin::signed(staker), other_exec, 1),
            <Error<Test>>::NoSuchStake
        );
    });
}

#[test]
fn schedule_staker_bond_less_cannot_decrease_below_min_bond() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::schedule_staker_bond_less(Origin::signed(staker), executor, min_atomic_stake),
            <Error<Test>>::StakerBondBelowMin
        );
    });
}

//
#[test]
fn schedule_staker_bond_less_cannot_decrease_below_min_stake() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            2 * min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::schedule_staker_bond_less(
                Origin::signed(staker),
                executor,
                min_atomic_stake / 2
            ),
            <Error<Test>>::StakeBelowMin
        );
    });
}

#[test]
fn schedule_staker_bond_less_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1
        ));

        let when_executable =
            Treasury::current_round().index + Staking::fixtures().revoke_stake_delay;

        assert_eq!(
            Staking::scheduled_staking_requests(executor),
            vec![ScheduledStakingRequest {
                staker,
                action: StakingAction::Decrease(1),
                when_executable,
            }]
        );

        assert_eq!(Staking::staker_info(staker).unwrap().less_total, 1);

        assert_last_event!(MockEvent::Staking(Event::StakeDecreaseScheduled {
            staker,
            candidate: executor,
            amount: 1,
            execute_round: when_executable
        }));
    });
}

#[test]
fn execute_stake_request_fails_if_no_such_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1
        ));

        assert_noop!(
            Staking::execute_stake_request(Origin::signed(staker), 419, executor),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn execute_stake_request_fails_if_no_such_request() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_noop!(
            Staking::execute_stake_request(Origin::signed(staker), staker, executor),
            <Error<Test>>::NoSuchPendingStakeRequest
        );
    });
}

#[test]
fn execute_stake_request_fails_if_not_due_yet() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1
        ));

        fast_forward_to(419);

        assert_noop!(
            Staking::execute_stake_request(Origin::signed(staker), staker, executor),
            <Error<Test>>::PendingStakeRequestNotDueYet
        );
    });
}

#[test]
fn executing_staking_action_revoke_successfully_and_leaving_stakers() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Staking::execute_stake_request(
            Origin::signed(staker),
            staker,
            executor
        ));

        assert_eq!(Staking::scheduled_staking_requests(staker), vec![]);

        assert_eq!(Staking::staker_info(staker), None);

        assert_last_n_events!(
            2,
            vec![
                Event::StakeRevoked {
                    staker,
                    candidate: executor,
                    unstaked: min_atomic_stake,
                },
                Event::StakerLeft {
                    staker,
                    unstaked: min_atomic_stake,
                }
            ]
        );
    });
}

#[test]
fn executing_staking_action_decrease_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1,
        ));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Staking::execute_stake_request(
            Origin::signed(staker),
            staker,
            executor
        ));

        assert_eq!(
            Staking::total_value_locked(),
            min_candidate_bond + min_atomic_stake
        );

        assert_eq!(Staking::scheduled_staking_requests(staker), vec![]);

        assert_eq!(
            Staking::staker_info(staker).unwrap().total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Staking(Event::StakeDecreased {
            staker,
            candidate: executor,
            amount: 1,
            in_top: true,
        }));
    });
}

#[test]
fn anyone_can_execute_staking_requests() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake + 1,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_staker_bond_less(
            Origin::signed(staker),
            executor,
            1,
        ));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Staking::execute_stake_request(
            Origin::signed(419),
            staker,
            executor
        ));
    });
}

#[test]
fn schedule_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;

        assert_noop!(
            Staking::schedule_leave_stakers(Origin::signed(staker)),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn schedule_leave_stakers_fails_if_already_leaving() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_revoke_stake(
            Origin::signed(staker),
            executor
        ));

        assert_noop!(
            Staking::schedule_leave_stakers(Origin::signed(staker)),
            <Error<Test>>::StakerAlreadyLeaving
        );
    });
}

#[test]
fn schedule_leave_stakers_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        assert_eq!(Staking::scheduled_staking_requests(staker), vec![]);

        assert_eq!(
            Staking::staker_info(staker).unwrap().less_total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Staking(Event::StakerExitScheduled {
            round: Treasury::current_round().index,
            staker,
            scheduled_exit: Treasury::current_round().index
                + Staking::fixtures().revoke_stake_delay
        }));
    });
}

#[test]
fn execute_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Staking::execute_leave_stakers(Origin::signed(419), staker, 1),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn execute_leave_stakers_fails_with_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_noop!(
            Staking::execute_leave_stakers(Origin::signed(staker), staker, 0),
            <Error<Test>>::TooLowStakeCountToLeaveStakers
        );
    });
}

#[test]
fn execute_leave_stakers_fails_if_not_due_yet() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        assert_noop!(
            Staking::execute_leave_stakers(Origin::signed(staker), staker, 1),
            <Error<Test>>::StakerCannotLeaveYet
        );
    });
}

#[test]
fn anyone_can_execute_leave_stakers() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Staking::execute_leave_stakers(
            Origin::signed(419),
            staker,
            1
        ));
    });
}

#[test]
fn cancel_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        assert_noop!(
            Staking::cancel_leave_stakers(Origin::signed(419)),
            <Error<Test>>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_leave_stakers_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Staking::cancel_leave_stakers(Origin::signed(staker)),
            <Error<Test>>::StakerNotLeaving
        );
    });
}

#[test]
fn cancel_leave_stakers_successfully() {
    new_test_ext().execute_with(|| {
        let staker = 15;
        let executor = 14;
        let min_candidate_bond = 1000 * 10 ^ DECIMALS as u64;
        let min_atomic_stake = 500 * 10 ^ DECIMALS as u64;

        drop(<Test as Config>::Currency::deposit_creating(
            &executor,
            min_candidate_bond,
        ));

        drop(<Test as Config>::Currency::deposit_creating(
            &staker,
            min_atomic_stake,
        ));

        assert_ok!(Staking::join_candidates(
            Origin::signed(executor),
            min_candidate_bond,
            0
        ));

        assert_ok!(Staking::stake(
            Origin::signed(staker),
            executor,
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Staking::schedule_leave_stakers(Origin::signed(staker)));

        fast_forward_to((Staking::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Staking::cancel_leave_stakers(Origin::signed(staker)));

        assert_eq!(Staking::scheduled_staking_requests(staker), vec![]);

        assert_eq!(Staking::staker_info(staker).is_some(), true);

        assert_last_event!(MockEvent::Staking(Event::StakerExitCancelled { staker }));
    });
}
