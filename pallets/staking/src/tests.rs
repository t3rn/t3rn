use crate::{
    assert_last_event, assert_last_n_events,
    mock::{
        fast_forward_to, new_test_ext, Balance, Event as MockEvent, Origin, Staking, System, Test,
        Treasury,
    },
    pallet::{Error, Event, ExecutorConfig, Fixtures, ScheduledConfigurationRequests},
};

use frame_support::{assert_err, assert_noop, assert_ok};
use sp_runtime::Percent;
use t3rn_primitives::{
    common::{Range, DEFAULT_ROUND_TERM},
    staking::{ExecutorInfo, Fixtures as StakingFixtures, ScheduledConfigurationRequest},
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
        let fixtures = <Fixtures<Test>>::get();
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

//
#[test]
fn anyone_can_execute_scheduled_reconfiguration() {
    new_test_ext().execute_with(|| {
        let fixtures = <Fixtures<Test>>::get();
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
        let fixtures = <Fixtures<Test>>::get();
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
