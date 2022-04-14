use crate::{
    inflation::{InflationInfo, Range},
    mock::*,
    CandidatesForRewards, Error, InflationConfig, RewardsPerCandidatePerRound,
};
use frame_support::{assert_err, assert_ok};
use sp_runtime::Perbill;

// #[test]
// fn it_works_for_default_value() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(Inflation::mint_for_round(Origin::root(), 42));
//     });
// }

#[test]
fn it_claims_zero_rewards_successfully() {
    new_test_ext().execute_with(|| {
        <CandidatesForRewards<Test>>::insert(1, 0);
        assert_ok!(Inflation::claim_rewards(Origin::signed(1)));
        assert_eq!(Balances::free_balance(&1), 0);
    })
}

#[test]
fn it_claims_rewards_successfully() {
    new_test_ext().execute_with(|| {
        let candidate = Origin::signed(1);

        // initialize account with some balance
        Balances::set_balance(Origin::root(), 1, 100, 0).expect("Account should be created fine");
        <CandidatesForRewards<Test>>::insert(1, 0);
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

#[test]
fn it_errors_with_not_candidate_if_a_non_candidate_tries_to_claim_rewards() {
    new_test_ext().execute_with(|| {
        assert_err!(
            Inflation::claim_rewards(Origin::signed(1)),
            Error::<Test>::NotCandidate
        );
    })
}

#[test]
fn it_should_set_inflation_successfully() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(1),
            max: Perbill::from_percent(2),
        };
        let new_round_inflation = Range {
            min: Perbill::from_float(0.0),
            ideal: Perbill::from_float(0.000000076),
            max: Perbill::from_float(0.000000151),
        };
        assert_ok!(Inflation::set_inflation(Origin::root(), new_inflation));
        assert_eq!(
            <InflationConfig<Test>>::get(),
            InflationInfo {
                annual: new_inflation,
                per_round: new_round_inflation,
            }
        );
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
