// Dont import crate imports here
use crate::{
    assert_last_event, assert_last_n_events,
    mock::{
        fast_forward_to, frame_system::ensure_signed, new_test_ext, Balance, Balances, Clock,
        Event as MockEvent, Executors, Origin, Runtime, System,
    },
};

use circuit_mock_runtime::test_utils::{
    generate_xtx_id, produce_and_validate_side_effect, ArgVariant,
};
// Import them here >.>
use circuit_runtime_pallets::pallet_executors::{
    stakes::Stakes,
    subject_metadata::{CandidateMetadata, StakerMetadata},
    BottomStakes, CandidateInfo, CandidatePool, Error, Event, ExecutorConfig, Pallet,
    ScheduledConfigurationRequests, SfxWithMetadataNewtype, StakerInfo, TopStakes, Total,
};
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use hex_literal::hex;
use pallet_circuit::SideEffect;
use sp_runtime::{traits::BlakeTwo256 as Hashing, AccountId32, Percent};
use substrate_abi::{SubstrateAbiConverter as Sabi, TryConvert, ValueMorphism};
use t3rn_primitives::{
    common::{OrderedSet, Range, DEFAULT_ROUND_TERM},
    executors::{
        Bond, ExecutorInfo, ExecutorStatus, Fixtures as StakingFixtures,
        ScheduledConfigurationRequest, ScheduledStakingRequest, StakerAdded, StakerStatus,
        StakingAction,
    },
    monetary::DECIMALS,
};
use xp_channel::{XbiFormat, XbiMetadata};
use xp_format::{Fees, XbiInstruction};

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const FIRST_REQUESTER_NONCE: u32 = 0;

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
            Executors::set_fixtures(Origin::signed(AccountId32::from([41u8; 32])), fixtures),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn fixtures_cannot_be_zero() {
    new_test_ext().execute_with(|| {
        let fixtures: StakingFixtures<Balance> = Default::default();

        assert_noop!(
            Executors::set_fixtures(Origin::root(), fixtures),
            Error::<Runtime>::FixturesCannotBeZero,
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
            min_executor_bond: 1000,
            min_candidate_bond: 1000,
            min_atomic_stake: 500,
            min_total_stake: 500,
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

        assert_ok!(Executors::set_fixtures(Origin::root(), fixtures));

        assert_last_event!(MockEvent::Executors(Event::FixturesConfigured {
            active_set_size: Range {
                min: 1,
                ideal: 3,
                max: 128,
            },
            max_commission: Percent::from_percent(50),
            max_risk: Percent::from_percent(50),
            min_executor_bond: 1000,
            min_candidate_bond: 1000,
            min_atomic_stake: 500,
            min_total_stake: 500,
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
            Executors::schedule_configure_executor(
                Origin::signed(AccountId32::from([3u8; 32])),
                commission,
                risk
            ),
            Error::<Runtime>::TooMuchRisk,
        );
    });
}

#[test]
fn schedule_configure_executor_fails_if_commission_gt_max() {
    new_test_ext().execute_with(|| {
        let commission = Percent::from_percent(99);
        let risk = Percent::from_percent(1);

        assert_noop!(
            Executors::schedule_configure_executor(
                Origin::signed(AccountId32::from([3u8; 32])),
                commission,
                risk
            ),
            Error::<Runtime>::TooMuchCommission,
        );
    });
}

#[test]
fn initial_executor_configuration_is_effective_immediately() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([13u8; 32]);
        let commission = Percent::from_percent(10);
        let risk = Percent::from_percent(42);

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            commission,
            risk
        ));

        assert_eq!(
            <ExecutorConfig<Runtime>>::get(executor.clone()).unwrap(),
            ExecutorInfo { commission, risk }
        );

        assert_last_event!(MockEvent::Executors(Event::ExecutorConfigured {
            executor,
            commission,
            risk,
        }));
    });
}

#[test]
fn executor_reconfiguration_gets_scheduled() {
    new_test_ext().execute_with(|| {
        let fixtures = Executors::fixtures();
        let executor = AccountId32::from([14u8; 32]);

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ExecutorConfig<Runtime>>::get(executor.clone()).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(10),
                risk: Percent::from_percent(42)
            }
        );

        assert_last_event!(MockEvent::Executors(Event::ExecutorConfigured {
            executor: executor.clone(),
            commission: Percent::from_percent(10),
            risk: Percent::from_percent(42),
        }));

        assert_eq!(
            <ScheduledConfigurationRequests<Runtime>>::get(executor.clone()).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Clock::current_round()
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
        let fixtures = Executors::fixtures();
        let executor = AccountId32::from([14u8; 32]);
        let other_user = AccountId32::from([15u8; 32]);

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ExecutorConfig<Runtime>>::get(executor.clone()).unwrap(),
            ExecutorInfo {
                commission: Percent::from_percent(10),
                risk: Percent::from_percent(42)
            }
        );

        assert_last_event!(MockEvent::Executors(Event::ExecutorConfigured {
            executor: executor.clone(),
            commission: Percent::from_percent(10),
            risk: Percent::from_percent(42),
        }));

        assert_eq!(
            <ScheduledConfigurationRequests<Runtime>>::get(executor.clone()).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Clock::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay),
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );

        fast_forward_to(
            System::block_number() + (DEFAULT_ROUND_TERM * fixtures.configure_executor_delay),
        );

        assert_ok!(Executors::execute_configure_executor(
            Origin::signed(other_user),
            executor.clone(),
        ));

        assert_last_event!(MockEvent::Executors(Event::ExecutorConfigured {
            executor: executor.clone(),
            commission: Percent::from_percent(20),
            risk: Percent::from_percent(32),
        }));

        assert_eq!(
            <ExecutorConfig<Runtime>>::get(executor.clone()).unwrap(),
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
        let fixtures = Executors::fixtures();
        let executor = AccountId32::from([14u8; 32]);
        let other_user = AccountId32::from([15u8; 32]);

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(10),
            Percent::from_percent(42),
        ));

        assert_ok!(Executors::schedule_configure_executor(
            Origin::signed(executor.clone()),
            Percent::from_percent(20),
            Percent::from_percent(32),
        ));

        assert_eq!(
            <ScheduledConfigurationRequests<Runtime>>::get(executor.clone()).unwrap(),
            ScheduledConfigurationRequest {
                when_executable: Clock::current_round()
                    .index
                    .saturating_add(fixtures.configure_executor_delay),
                commission: Percent::from_percent(20),
                risk: Percent::from_percent(32),
            }
        );

        assert_noop!(
            Executors::cancel_configure_executor(Origin::signed(other_user)),
            Error::<Runtime>::NoSuchConfigurationRequest
        );

        assert_ok!(Executors::cancel_configure_executor(Origin::signed(
            executor.clone()
        )));

        assert_eq!(
            <ExecutorConfig<Runtime>>::get(executor.clone()).unwrap(),
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
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::join_candidates(Origin::signed(executor.clone()), min_candidate_bond, 1),
            Error::<Runtime>::CandidateExists
        );
    });
}

#[test]
fn cannot_join_candidates_as_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_candidate_bond,
        ));

        <StakerInfo<Runtime>>::insert(
            staker.clone(),
            StakerMetadata {
                id: staker.clone(),
                stakes: OrderedSet(vec![]),
                total: 0,
                less_total: 0,
                status: StakerStatus::Active,
            },
        );

        assert_noop!(
            Executors::join_candidates(Origin::signed(staker.clone()), min_candidate_bond, 0),
            Error::<Runtime>::StakerExists
        );
    });
}

#[test]
fn join_candidates_enforces_a_min_bond() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = 10 ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_noop!(
            Executors::join_candidates(Origin::signed(executor.clone()), min_candidate_bond, 0),
            Error::<Runtime>::CandidateBondBelowMin
        );
    });
}

#[test]
fn join_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        <CandidatePool<Runtime>>::set(OrderedSet(vec![Bond {
            owner: executor.clone(),
            amount: 0,
        }]));

        assert_noop!(
            Executors::join_candidates(Origin::signed(executor.clone()), min_candidate_bond, 0),
            Error::<Runtime>::TooLowCandidateCountWeightHintJoinCandidates
        );
    });
}

#[test]
fn join_candidates_fails_on_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        assert_noop!(
            Executors::join_candidates(Origin::signed(executor.clone()), min_candidate_bond, 0),
            Error::<Runtime>::InsufficientBalance
        );
    });
}

#[test]
fn join_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let empty_stakes: Stakes<<Runtime as frame_system::Config>::AccountId, Balance> =
            Default::default();

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            1
        ));

        assert_eq!(
            <CandidateInfo<Runtime>>::get(executor.clone()).unwrap(),
            CandidateMetadata::new(min_candidate_bond),
        );

        assert_eq!(
            <TopStakes<Runtime>>::get(executor.clone()).unwrap(),
            empty_stakes
        );

        assert_eq!(
            <BottomStakes<Runtime>>::get(executor.clone()).unwrap(),
            empty_stakes
        );

        assert_eq!(
            <CandidatePool<Runtime>>::get(),
            OrderedSet(vec![Bond {
                owner: executor.clone(),
                amount: min_candidate_bond,
            }])
        );

        assert_eq!(<Total<Runtime>>::get(), min_candidate_bond);

        assert_last_event!(MockEvent::Executors(Event::CandidateJoined {
            account: executor,
            amount_locked: min_candidate_bond,
            total_locked: min_candidate_bond,
        }));
    });
}

#[test]
fn schedule_leave_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::schedule_leave_candidates(Origin::signed(executor.clone()), 0),
            Error::<Runtime>::TooLowCandidateCountToLeaveCandidates
        );
    });
}

#[test]
fn schedule_leave_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let now = Clock::current_round().index;
        let leave_candidates_delay = Executors::fixtures().leave_candidates_delay;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ),);

        assert_eq!(<CandidatePool<Runtime>>::get().0.len(), 0);

        assert_eq!(
            <CandidateInfo<Runtime>>::get(executor.clone())
                .unwrap()
                .status,
            ExecutorStatus::Leaving(now + leave_candidates_delay)
        );

        assert_last_event!(MockEvent::Executors(Event::CandidateExitScheduled {
            exit_allowed_round: now,
            candidate: executor,
            scheduled_exit: now + leave_candidates_delay,
        }));
    });
}

#[test]
fn execute_leave_candidates_fails_on_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let staker = AccountId32::from([15u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ));

        fast_forward_to(
            ((Clock::current_round().index + Executors::fixtures().leave_candidates_delay)
                * DEFAULT_ROUND_TERM)
                .into(),
        );

        assert_noop!(
            Executors::execute_leave_candidates(Origin::signed(executor.clone()), executor, 0),
            Error::<Runtime>::TooLowCandidateStakeCountToLeaveCandidates
        );
    });
}

//
#[test]
fn execute_leave_candidates_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let staker = AccountId32::from([15u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::execute_leave_candidates(Origin::signed(executor.clone()), executor, 0),
            Error::<Runtime>::CandidateNotLeaving
        );
    });
}

#[test]
fn execute_leave_candidates_fails_if_too_early() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let staker = AccountId32::from([15u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ));

        assert_noop!(
            Executors::execute_leave_candidates(Origin::signed(executor.clone()), executor, 1),
            Error::<Runtime>::CandidateCannotLeaveYet
        );
    });
}

#[test]
fn anyone_can_execute_leave_candidates() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let staker = AccountId32::from([15u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ));

        fast_forward_to(
            ((Clock::current_round().index + Executors::fixtures().leave_candidates_delay)
                * DEFAULT_ROUND_TERM)
                .into(),
        );

        assert_ok!(Executors::execute_leave_candidates(
            Origin::signed(staker.clone()),
            executor.clone(),
            1
        ));

        assert_eq!(Executors::staker_info(staker.clone()), None);

        assert_eq!(Executors::top_stakes(executor.clone()), None);

        assert_eq!(Executors::bottom_stakes(executor.clone()), None);

        assert_eq!(Executors::candidate_info(executor.clone()), None);

        assert_eq!(
            Executors::scheduled_staking_requests(staker.clone()),
            vec![]
        );

        assert_eq!(Executors::total_value_locked(), 0);

        assert_last_event!(MockEvent::Executors(Event::CandidateLeft {
            candidate: executor,
            amount_unlocked: min_candidate_bond + min_atomic_stake,
            total_locked: 0,
        }));
    });
}

#[test]
fn cancel_leave_candidates_fails_if_not_executor() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let staker = AccountId32::from([15u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ),);

        assert_noop!(
            Executors::cancel_leave_candidates(Origin::signed(staker.clone()), 1),
            Error::<Runtime>::NoSuchCandidate,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::cancel_leave_candidates(Origin::signed(executor.clone()), 1),
            Error::<Runtime>::CandidateNotLeaving,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_too_low_weight_hint() {
    new_test_ext().execute_with(|| {
        let executor1 = AccountId32::from([14u8; 32]);
        let executor2 = AccountId32::from([13u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor1, min_candidate_bond));

        drop(Balances::deposit_creating(&executor2, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor1.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor2),
            min_candidate_bond,
            1
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor1.clone()),
            2
        ),);

        assert_noop!(
            Executors::cancel_leave_candidates(Origin::signed(executor1), 0),
            Error::<Runtime>::TooLowCandidateCountWeightHintCancelLeaveCandidates,
        );
    });
}

#[test]
fn cancel_leave_candidates_fails_if_already_active() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            2
        ),);

        <CandidatePool<Runtime>>::put(OrderedSet(vec![Bond {
            owner: executor.clone(),
            amount: min_candidate_bond + min_atomic_stake,
        }]));

        assert_noop!(
            Executors::cancel_leave_candidates(Origin::signed(executor.clone()), 1),
            Error::<Runtime>::AlreadyActive,
        );
    });
}

#[test]
fn cancel_leave_candidates_successfully() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::schedule_leave_candidates(
            Origin::signed(executor.clone()),
            2
        ),);

        assert_ok!(Executors::cancel_leave_candidates(
            Origin::signed(executor.clone()),
            1
        ));

        assert_eq!(
            Executors::candidate_pool().contains(&Bond {
                owner: executor.clone(),
                amount: 0, // ignored by PartialEq
            }),
            true
        );

        assert_eq!(Executors::candidate_info(executor.clone()).is_some(), true);

        assert_last_event!(MockEvent::Executors(Event::CandidateExitCancelled {
            candidate: executor,
        }));
    });
}

#[test]
fn go_offline_fails_if_not_candidate() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);

        assert_noop!(
            Executors::go_offline(Origin::signed(staker.clone())),
            Error::<Runtime>::NoSuchCandidate,
        );
    });
}

#[test]
fn go_offline_successfully() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::go_offline(Origin::signed(executor.clone())));

        assert_eq!(
            Executors::candidate_pool().contains(&Bond {
                owner: executor.clone(),
                amount: 0, // ignored by PartialEq
            }),
            false
        );

        assert_eq!(
            Executors::candidate_info(executor.clone()).unwrap().status,
            ExecutorStatus::Idle
        );

        assert_last_event!(MockEvent::Executors(Event::CandidateWentOffline {
            candidate: executor,
        }));
    });
}

#[test]
fn go_online_fails_if_not_candidate() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);

        assert_noop!(
            Executors::go_online(Origin::signed(staker.clone())),
            Error::<Runtime>::NoSuchCandidate,
        );
    });
}

#[test]
fn go_online_successfully() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::go_offline(Origin::signed(executor.clone())));

        assert_ok!(Executors::go_online(Origin::signed(executor.clone())));

        assert_eq!(
            Executors::candidate_pool().contains(&Bond {
                owner: executor.clone(),
                amount: 0, // ignored by PartialEq
            }),
            true
        );

        assert_eq!(
            Executors::candidate_info(executor.clone()).unwrap().status,
            ExecutorStatus::Active
        );

        assert_last_event!(MockEvent::Executors(Event::CandidateBackOnline {
            candidate: executor,
        }));
    });
}

#[test]
fn stake_fails_on_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        assert_noop!(
            Executors::stake(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake,
                0,
                0
            ),
            Error::<Runtime>::InsufficientBalance
        );
    });
}

#[test]
fn first_staking_requires_min_bond() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::stake(Origin::signed(staker.clone()), executor, 419, 0, 0),
            Error::<Runtime>::StakerBondBelowMin
        );
    });
}

#[test]
fn non_first_stakes_enforce_a_minimum() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::stake(Origin::signed(staker.clone()), executor, 419, 1, 1),
            Error::<Runtime>::StakeBelowMin
        );
    });
}

#[test]
fn candidates_cannot_stake() {
    new_test_ext().execute_with(|| {
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::stake(
                Origin::signed(executor.clone()),
                executor,
                min_atomic_stake,
                0,
                0
            ),
            Error::<Runtime>::CandidateExists
        );
    });
}

#[test]
fn non_first_stakes_enforce_a_stake_count_weight_hint() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::stake(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake,
                1,
                0
            ),
            Error::<Runtime>::TooLowStakeCountToStake
        );
    });
}

#[test]
fn staker_stakings_are_capped() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(
            &staker.clone(),
            101 * min_atomic_stake,
        ));

        let mut stake_count = 0;
        let mut candidate_stake_count = 0;
        let mut exec = 16;

        for i in 1..=100 {
            drop(Balances::deposit_creating(
                &AccountId32::from([exec as u8; 32]),
                min_candidate_bond,
            ));

            assert_ok!(Executors::join_candidates(
                Origin::signed(AccountId32::from([exec as u8; 32])),
                min_candidate_bond,
                i - 1
            ));

            assert_ok!(Executors::stake(
                Origin::signed(staker.clone()),
                AccountId32::from([exec as u8; 32]),
                min_atomic_stake,
                stake_count,
                candidate_stake_count
            ));
            stake_count += 1;
            candidate_stake_count += 1;
            exec += 1;
        }

        assert_noop!(
            Executors::stake(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake,
                100,
                100
            ),
            Error::<Runtime>::MaxStakesExceeded
        );
    });
}

#[test]
fn cannot_stake_twice_on_the_same_candidate() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::stake(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake,
                1,
                1
            ),
            Error::<Runtime>::AlreadyStakedCandidate
        );
    });
}

#[test]
fn cannot_stake_on_non_candidate() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_noop!(
            Executors::stake(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake,
                0,
                0
            ),
            Error::<Runtime>::NoSuchCandidate
        );
    });
}

#[test]
fn cannot_stake_with_insufficient_candidate_stake_count() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let other_staker = AccountId32::from([44u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        drop(Balances::deposit_creating(
            &other_staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::stake(
                Origin::signed(other_staker),
                executor,
                min_atomic_stake,
                0,
                1
            ),
            Error::<Runtime>::TooLowCandidateStakeCountToStake
        );
    });
}

#[test]
fn stake_sets_storage_and_emits_events() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_eq!(
            Executors::total_value_locked(),
            min_atomic_stake + min_candidate_bond
        );

        assert_eq!(
            Executors::candidate_info(executor.clone())
                .unwrap()
                .stake_count,
            1
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Executors(Event::StakeAdded {
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
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::schedule_revoke_stake(Origin::signed(executor.clone()), executor),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn cannot_schedule_revoke_stake_twice() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ),);

        assert_noop!(
            Executors::schedule_revoke_stake(Origin::signed(staker.clone()), executor),
            Error::<Runtime>::PendingStakeRequestAlreadyExists
        );
    });
}

#[test]
fn schedule_revoke_stake_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            2 * min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ),);

        assert_eq!(
            Executors::scheduled_staking_requests(executor.clone()),
            vec![ScheduledStakingRequest {
                staker: staker.clone(),
                action: StakingAction::Revoke(min_atomic_stake),
                when_executable: Clock::current_round().index
                    + Executors::fixtures().revoke_stake_delay,
            }]
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().less_total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Executors(Event::StakeRevocationScheduled {
            round: Clock::current_round().index,
            staker,
            candidate: executor,
            scheduled_exit: Clock::current_round().index + Executors::fixtures().revoke_stake_delay,
        }));
    });
}

#[test]
fn cancel_stake_request_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);

        assert_noop!(
            Executors::cancel_stake_request(Origin::signed(staker.clone()), executor),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_stake_request_fails_if_not_requested() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::cancel_stake_request(Origin::signed(staker.clone()), executor),
            Error::<Runtime>::NoSuchPendingStakeRequest
        );
    });
}

#[test]
fn cancel_stake_request_fails_if_origin_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let other_staker = AccountId32::from([16u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, min_atomic_stake + 1));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1,
        ));

        assert_noop!(
            Executors::cancel_stake_request(Origin::signed(other_staker), executor,),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_stake_request_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ));

        let req = Executors::scheduled_staking_requests(executor.clone())[0].clone();

        assert_ok!(Executors::cancel_stake_request(
            Origin::signed(staker.clone()),
            executor.clone()
        ));

        assert_eq!(
            Executors::scheduled_staking_requests(executor.clone()),
            vec![]
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().less_total,
            0
        );

        assert_last_event!(MockEvent::Executors(Event::StakeRequestCancelled {
            staker,
            executor,
            cancelled_request: req.into(),
        }));
    });
}

#[test]
fn staker_bond_more_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);

        assert_noop!(
            Executors::staker_bond_more(Origin::signed(staker.clone()), executor, 1),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn staker_bond_more_fails_if_pending_stake_revoke() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ));

        assert_noop!(
            Executors::staker_bond_more(Origin::signed(staker.clone()), executor, 1),
            Error::<Runtime>::PendingStakeRevoke
        );
    });
}

#[test]
fn staker_bond_more_fails_if_no_such_stake() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let other_exec = AccountId32::from([16u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::staker_bond_more(Origin::signed(staker.clone()), other_exec, 1),
            Error::<Runtime>::NoSuchStake
        );
    });
}

#[test]
fn staker_bond_more_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::staker_bond_more(
            Origin::signed(staker.clone()),
            executor.clone(),
            1
        ));

        assert_eq!(
            Executors::candidate_info(executor.clone())
                .unwrap()
                .total_counted,
            min_candidate_bond + min_atomic_stake + 1
        );

        assert_eq!(
            Executors::total_value_locked(),
            min_candidate_bond + min_atomic_stake + 1
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().total,
            min_atomic_stake + 1
        );

        assert_last_event!(MockEvent::Executors(Event::StakeIncreased {
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
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);

        assert_noop!(
            Executors::schedule_staker_bond_less(Origin::signed(staker.clone()), executor, 1),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn schedule_staker_bond_less_fails_if_pending_stake_request_exists() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ));

        assert_noop!(
            Executors::schedule_staker_bond_less(Origin::signed(staker.clone()), executor, 1),
            Error::<Runtime>::PendingStakeRequestAlreadyExists
        );
    });
}

#[test]
fn schedule_staker_bond_less_fails_if_no_such_stake() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let other_exec = AccountId32::from([16u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::schedule_staker_bond_less(Origin::signed(staker.clone()), other_exec, 1),
            Error::<Runtime>::NoSuchStake
        );
    });
}

#[test]
fn schedule_staker_bond_less_cannot_decrease_below_min_bond() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::schedule_staker_bond_less(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake
            ),
            Error::<Runtime>::StakerBondBelowMin
        );
    });
}

//
#[test]
fn schedule_staker_bond_less_cannot_decrease_below_min_stake() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, 2 * min_atomic_stake));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::schedule_staker_bond_less(
                Origin::signed(staker.clone()),
                executor,
                min_atomic_stake / 2
            ),
            Error::<Runtime>::StakeBelowMin
        );
    });
}

#[test]
fn schedule_staker_bond_less_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(&staker, min_atomic_stake + 1));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1
        ));

        let when_executable =
            Clock::current_round().index + Executors::fixtures().revoke_stake_delay;

        assert_eq!(
            Executors::scheduled_staking_requests(executor.clone()),
            vec![ScheduledStakingRequest {
                staker: staker.clone(),
                action: StakingAction::Decrease(1),
                when_executable,
            }]
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().less_total,
            1
        );

        assert_last_event!(MockEvent::Executors(Event::StakeDecreaseScheduled {
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
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake + 1,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1
        ));

        assert_noop!(
            Executors::execute_stake_request(
                Origin::signed(staker.clone()),
                AccountId32::from([41u8; 32]),
                executor
            ),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn execute_stake_request_fails_if_no_such_request() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake + 1,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_noop!(
            Executors::execute_stake_request(Origin::signed(staker.clone()), staker, executor),
            Error::<Runtime>::NoSuchPendingStakeRequest
        );
    });
}

#[test]
fn execute_stake_request_fails_if_not_due_yet() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake + 1,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1
        ));

        let when_executable =
            Clock::current_round().index + Executors::fixtures().revoke_stake_delay;

        fast_forward_to(when_executable - 1);

        assert_noop!(
            Executors::execute_stake_request(Origin::signed(staker.clone()), staker, executor),
            Error::<Runtime>::PendingStakeRequestNotDueYet
        );
    });
}

#[test]
fn executing_staking_action_revoke_successfully_and_leaving_stakers() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor.clone()
        ));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Executors::execute_stake_request(
            Origin::signed(staker.clone()),
            staker.clone(),
            executor.clone().clone()
        ));

        assert_eq!(
            Executors::scheduled_staking_requests(staker.clone()),
            vec![]
        );

        assert_eq!(Executors::staker_info(staker.clone()), None);

        assert_last_n_events!(
            2,
            vec![
                Event::StakeRevoked {
                    staker: staker.clone(),
                    candidate: executor,
                    unstaked: min_atomic_stake,
                },
                Event::StakerLeft {
                    staker: staker.clone(),
                    unstaked: min_atomic_stake,
                }
            ]
        );
    });
}

#[test]
fn executing_staking_action_decrease_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake + 1,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone().clone(),
            min_atomic_stake + 1,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1,
        ));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Executors::execute_stake_request(
            Origin::signed(staker.clone()),
            staker.clone(),
            executor.clone()
        ));

        assert_eq!(
            Executors::total_value_locked(),
            min_candidate_bond + min_atomic_stake
        );

        assert_eq!(
            Executors::scheduled_staking_requests(staker.clone()),
            vec![]
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Executors(Event::StakeDecreased {
            staker: staker.clone(),
            candidate: executor,
            amount: 1,
            in_top: true,
        }));
    });
}

#[test]
fn anyone_can_execute_staking_requests() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake + 1,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_staker_bond_less(
            Origin::signed(staker.clone()),
            executor.clone(),
            1,
        ));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Executors::execute_stake_request(
            Origin::signed(AccountId32::from([255u8; 32])),
            staker.clone(),
            executor.clone()
        ));
    });
}

#[test]
fn schedule_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);

        assert_noop!(
            Executors::schedule_leave_stakers(Origin::signed(staker.clone())),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn schedule_leave_stakers_fails_if_already_leaving() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_revoke_stake(
            Origin::signed(staker.clone()),
            executor
        ));

        assert_noop!(
            Executors::schedule_leave_stakers(Origin::signed(staker.clone())),
            Error::<Runtime>::StakerAlreadyLeaving
        );
    });
}

#[test]
fn schedule_leave_stakers_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        assert_eq!(
            Executors::scheduled_staking_requests(staker.clone()),
            vec![]
        );

        assert_eq!(
            Executors::staker_info(staker.clone()).unwrap().less_total,
            min_atomic_stake
        );

        assert_last_event!(MockEvent::Executors(Event::StakerExitScheduled {
            round: Clock::current_round().index,
            staker: staker.clone(),
            scheduled_exit: Clock::current_round().index + Executors::fixtures().revoke_stake_delay
        }));
    });
}

#[test]
fn execute_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_noop!(
            Executors::execute_leave_stakers(
                Origin::signed(AccountId32::from([41u8; 32])),
                staker.clone(),
                1
            ),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn execute_leave_stakers_fails_with_insufficient_weight_hint() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_noop!(
            Executors::execute_leave_stakers(Origin::signed(staker.clone()), staker.clone(), 0),
            Error::<Runtime>::TooLowStakeCountToLeaveStakers
        );
    });
}

#[test]
fn execute_leave_stakers_fails_if_not_due_yet() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        assert_noop!(
            Executors::execute_leave_stakers(Origin::signed(staker.clone()), staker.clone(), 1),
            Error::<Runtime>::StakerCannotLeaveYet
        );
    });
}

#[test]
fn anyone_can_execute_leave_stakers() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Executors::execute_leave_stakers(
            Origin::signed(AccountId32::from([41u8; 32])),
            staker,
            1
        ));
    });
}

#[test]
fn cancel_leave_stakers_fails_if_not_staker() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        assert_noop!(
            Executors::cancel_leave_stakers(Origin::signed(AccountId32::from([41u8; 32]))),
            Error::<Runtime>::NoSuchStaker
        );
    });
}

#[test]
fn cancel_leave_stakers_fails_if_not_leaving() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_noop!(
            Executors::cancel_leave_stakers(Origin::signed(staker.clone())),
            Error::<Runtime>::StakerNotLeaving
        );
    });
}

#[test]
fn cancel_leave_stakers_successfully() {
    new_test_ext().execute_with(|| {
        let staker = AccountId32::from([15u8; 32]);
        let executor = AccountId32::from([14u8; 32]);
        let min_candidate_bond = (1000 * 10) ^ DECIMALS as u128;
        let min_atomic_stake = (500 * 10) ^ DECIMALS as u128;

        drop(Balances::deposit_creating(&executor, min_candidate_bond));

        drop(Balances::deposit_creating(
            &staker.clone(),
            min_atomic_stake,
        ));

        assert_ok!(Executors::join_candidates(
            Origin::signed(executor.clone()),
            min_candidate_bond,
            0
        ));

        assert_ok!(Executors::stake(
            Origin::signed(staker.clone()),
            executor.clone(),
            min_atomic_stake,
            0,
            0
        ));

        assert_ok!(Executors::schedule_leave_stakers(Origin::signed(
            staker.clone()
        )));

        fast_forward_to((Executors::fixtures().revoke_stake_delay * DEFAULT_ROUND_TERM).into());

        assert_ok!(Executors::cancel_leave_stakers(Origin::signed(
            staker.clone()
        )));

        assert_eq!(
            Executors::scheduled_staking_requests(staker.clone()),
            vec![]
        );

        assert_eq!(Executors::staker_info(staker.clone()).is_some(), true);

        assert_last_event!(MockEvent::Executors(Event::StakerExitCancelled { staker }));
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_tran() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        let se = SideEffect {
            target: [0u8, 0u8, 0u8, 0u8],
            max_reward: 2,
            action: [116, 114, 97, 110],
            encoded_args: vec![
                vec![
                    42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247,
                    0, 118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            signature: vec![],
            enforce_executor: Some(
                [
                    53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85,
                    53, 110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
                ]
                .into(),
            ),
            insurance: 3,
            reward_asset_id: None,
        };

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::Transfer {
                    dest: AccountId32::new([
                        42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50,
                        247, 0, 118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62
                    ]),
                    value: 1,
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_aliq() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        let se =
            produce_and_validate_side_effect(*b"aliq", 0, 1, t3rn_abi::Codec::Scale, ArgVariant::A);

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::AddLiquidity {
                    asset_a: 1,
                    asset_b: 151587081,
                    amount_a: 12009965891327239886942633203474172169,
                    amount_b_max_limit: 1,
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_swap() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        let se =
            produce_and_validate_side_effect(*b"swap", 1, 1, t3rn_abi::Codec::Scale, ArgVariant::A);

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::Swap {
                    asset_out: 151587081,
                    asset_in: 151587081,
                    amount: 1,
                    max_limit: 1,
                    discount: Default::default()
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_cevm() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        // This doesn't work for `cevm`:  produce_and_validate_side_effect(*b"cevm", 1, 1, t3rn_abi::Codec::Rlp, ArgVariant::B);
        // Take the values produced by the function above and pad with a 1 in front of the Option args
        let se = SideEffect {
            target: [0u8, 0u8, 0u8, 0u8],
            max_reward: 2,
            action: [99, 101, 118, 109],
            encoded_args: vec![
                vec![
                    0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                vec![6, 6, 6, 6, 6, 6, 6, 6],
                vec![
                    0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                vec![
                    1, 0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    1, 0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 90, 98, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
            ],
            signature: vec![],
            enforce_executor: Some(
                [
                    53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85,
                    53, 110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
                ]
                .into(),
            ),
            insurance: 3,
            reward_asset_id: None,
        };

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::CallEvm {
                    source: hex!("0000000000000000000000000000000000000000").into(),
                    target: hex!("005a620200000000000000000000000000000000").into(),
                    value: 40000000.into(),
                    input: vec![6, 6, 6, 6, 6, 6, 6, 6],
                    gas_limit: 40000000,
                    max_fee_per_gas: 40000000.into(),
                    max_priority_fee_per_gas: Some(10240000001i64.into()),
                    nonce: Some(10240000001i64.into()),
                    access_list: vec![]
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_wasm() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        // This doesn't work for `wasm`:  produce_and_validate_side_effect(*b"wasm", 1, 1, t3rn_abi::Codec::Scale, ArgVariant::B);
        let se = SideEffect {
            target: [0u8, 0u8, 0u8, 0u8],
            max_reward: 2,
            action: [119, 97, 115, 109],
            encoded_args: vec![
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            ],
            signature: vec![],
            enforce_executor: Some(
                [
                    53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85,
                    53, 110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
                ]
                .into(),
            ),
            insurance: 3,
            reward_asset_id: None,
        };

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::CallWasm {
                    dest: AccountId32::new([
                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                        9, 9, 9, 9, 9, 9, 9
                    ]),
                    value: 1,
                    gas_limit: 1,
                    storage_deposit_limit: Some(257),
                    data: vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_proper_conversion_from_sfx_to_xbi_for_call() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);

        // This doesn't work for `cgen`:  produce_and_validate_side_effect(*b"cgen", 1, 1, t3rn_abi::Codec::Scale, ArgVariant::B);
        let se = SideEffect {
            target: [0u8, 0u8, 0u8, 0u8],
            max_reward: 2,
            action: [99, 97, 108, 108],
            encoded_args: vec![
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            ],
            signature: vec![],
            enforce_executor: Some(
                [
                    53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85,
                    53, 110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
                ]
                .into(),
            ),
            insurance: 3,
            reward_asset_id: None,
        };

        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();

        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();

        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);

        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi: XbiFormat = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone())
            .try_into()
            .unwrap();

        assert_eq!(
            xbi,
            XbiFormat {
                instr: XbiInstruction::CallCustom {
                    caller: AccountId32::new([
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]),
                    dest: AccountId32::new([
                        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                        9, 9, 9, 9, 9, 9, 9
                    ]),
                    value: 1,
                    input: vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                    limit: 1,
                    additional_params: vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
                },
                metadata,
            }
        );
    });
}

#[test]
fn check_creation_sfx_with_metadata_new_type() {
    new_test_ext().execute_with(|| {
        let origin = Origin::signed(ALICE);
        let se: SideEffect<AccountId32, u128> = SideEffect {
            target: [0u8, 0u8, 0u8, 0u8],
            max_reward: 2,
            action: [116, 114, 97, 110],
            encoded_args: vec![
                vec![
                    42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247,
                    0, 118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            signature: vec![],
            enforce_executor: Some(
                [
                    53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85,
                    53, 110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
                ]
                .into(),
            ),
            insurance: 3,
            reward_asset_id: None,
        };
        let xtx_id: sp_core::H256 = generate_xtx_id::<Hashing>(ALICE, FIRST_REQUESTER_NONCE);
        let executor = ensure_signed(origin.clone()).unwrap();
        let account_to_32: AccountId32 = Decode::decode(&mut &executor.encode()[..]).unwrap();
        let nonce_always_0_because_we_use_seed = 0;
        let bypass_most_metadata_checks_default_para_id = Default::default();
        let sfx_id = se.generate_id::<Hashing>(xtx_id.as_ref(), 0u32);
        let max_exec_cost = 10;
        let max_notifications_cost = 20;
        let metadata = XbiMetadata::new(
            bypass_most_metadata_checks_default_para_id,
            bypass_most_metadata_checks_default_para_id,
            Default::default(),
            Fees::new(None, Some(max_exec_cost), Some(max_notifications_cost)),
            Some(account_to_32),
            nonce_always_0_because_we_use_seed,
            Some(&sfx_id.encode()),
        );

        let xbi = SfxWithMetadataNewtype::<Runtime>::new(se.clone(), metadata.clone());

        assert!(
            xbi == SfxWithMetadataNewtype {
                side_effect: se,
                metadata,
            }
        );
    });
}
