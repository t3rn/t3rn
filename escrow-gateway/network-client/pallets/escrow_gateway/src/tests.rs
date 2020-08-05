// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use escrow_gateway_primitives::{Phase};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		assert_ok!(EscrowGateway::do_something(Origin::signed(1), 42));
		// asserting that the stored value is equal to what we stored
		assert_eq!(EscrowGateway::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the correct error is thrown on None value
		assert_noop!(
			EscrowGateway::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn it_matches_execute_phase_correctly() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		assert_ok!(EscrowGateway::call(Origin::signed(1), Phase::Execute));
		// assert dummy value equals the correct path
		assert_eq!(EscrowGateway::something(), Some(0));
	});
}

#[test]
fn it_matches_commit_phase_correctly() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		assert_ok!(EscrowGateway::call(Origin::signed(1), Phase::Commit));
		// assert dummy value equals the correct path
		assert_eq!(EscrowGateway::something(), Some(1));
	});
}

#[test]
fn it_matches_revert_phase_correctly() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		assert_ok!(EscrowGateway::call(Origin::signed(1), Phase::Revert));
		// assert dummy value equals the correct path
		assert_eq!(EscrowGateway::something(), Some(2));
	});
}