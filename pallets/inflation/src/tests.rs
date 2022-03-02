use crate::{
    assert_last_event,
    inflation::{CandidateRole, InflationInfo, Range, RewardsAllocationConfig, RoundInfo},
    mock::{Event as MockEvent, *},
    CandidatesForRewards, CurrentRound, Error, Event, InflationConfig, RewardsPerCandidatePerRound,
};
use frame_support::{assert_err, assert_ok};
use sp_runtime::Perbill;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(Inflation::mint_for_round(Origin::root(), 42));
    });
}

#[test]
fn it_claims_zero_rewards_successfully() {
    new_test_ext().execute_with(|| {
        <CandidatesForRewards<Test>>::insert(1, CandidateRole::Developer, 0);
        assert_ok!(Inflation::claim_rewards(Origin::signed(1)));
        assert_eq!(Balances::free_balance(1), 0);
    })
}

#[test]
fn it_claims_rewards_successfully() {
    new_test_ext().execute_with(|| {
        // claimer
        let candidate = Origin::signed(1);

        // initialize account with some balance
        Balances::set_balance(Origin::root(), 1, 100, 0).expect("Account should be created fine");
        <CandidatesForRewards<Test>>::insert(1, CandidateRole::Executor, 0);
        <RewardsPerCandidatePerRound<Test>>::insert(1, 1, 1);
        <RewardsPerCandidatePerRound<Test>>::insert(1, 2, 1);

        assert_ok!(Inflation::claim_rewards(candidate));

        // assert balance allocated
        assert_eq!(Balances::free_balance(&1), 102);

        // assert storage is empty for candidate
        let remaining_storage = <RewardsPerCandidatePerRound<Test>>::iter_key_prefix(&1).count();
        assert_eq!(remaining_storage, 0);
    })
}

#[test] // FIXME
fn it_should_set_inflation_successfully() {
    new_test_ext().execute_with(|| {
        // FIXME: mock pallet genesis should set round term to 5
        // force round length to be 5
        <CurrentRound<Test>>::put(RoundInfo::new(1_u32, 0_u32.into(), 5_u32));

        // input annual inflation config
        let actual_annual_inflation_config = Range {
            min: Perbill::from_percent(3),
            ideal: Perbill::from_percent(4),
            max: Perbill::from_percent(5),
        };

        // what we expect to get auto derived
        let expected_round_inflation_config = Range {
            min: Perbill::from_parts(57),
            ideal: Perbill::from_parts(75),
            max: Perbill::from_parts(93),
        };

        assert_ok!(Inflation::set_inflation(
            Origin::root(),
            actual_annual_inflation_config
        ));

        // assert new inflation config got stored
        assert_eq!(
            <InflationConfig<Test>>::get(),
            InflationInfo {
                annual: actual_annual_inflation_config,
                round: expected_round_inflation_config,
                rewards_alloc: RewardsAllocationConfig {
                    developer: Perbill::zero(),
                    executor: Perbill::zero(),
                }
            }
        );

        // FIXME: no evmitted events found
        // // assert new inflation config was emitted
        // assert_last_event!(MockEvent::Inflation(Event::InflationConfigChanged {
        // 	annual_min: actual_annual_inflation_config.min,
        // 	annual_ideal: actual_annual_inflation_config.ideal,
        // 	annual_max: actual_annual_inflation_config.max,
        // 	round_min: expected_round_inflation_config.min,
        // 	round_ideal: expected_round_inflation_config.ideal,
        // 	round_max: expected_round_inflation_config.max,
        // }));
    })
}

#[test]
fn it_should_fail_with_invalid_inflation_rate() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(2),
            max: Perbill::from_percent(1),
        };
        assert_err!(
            Inflation::set_inflation(Origin::root(), new_inflation),
            Error::<Test>::InvalidInflationSchedule
        );
    })
}
