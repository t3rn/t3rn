use crate::{mock::*, CandidatesForRewards, Error};
use frame_support::{assert_err, assert_ok, StorageValue};

// #[test]
// fn it_works_for_default_value() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(Inflation::mint_for_round(Origin::root(), 42));
//     });
// }

#[test]
fn it_claims_rewards_successfully() {
    new_test_ext().execute_with(|| {
        <CandidatesForRewards<Test>>::insert(1, 0);
        assert_ok!(Inflation::claim_rewards(Origin::signed(1)));
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
