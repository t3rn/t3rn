use crate::{
    assert_last_event, assert_last_n_events,
    mock::{Event as MockEvent, *},
    Error, Event,
};
use frame_support::{assert_err, assert_noop, assert_ok};

// #[test]
// fn mint_for_round_requires_root() {
//     new_test_ext().execute_with(|| {
//         assert_noop!(
//             Treasury::mint_for_round(Origin::signed(419), 1, 1_000_000_000),
//             sp_runtime::DispatchError::BadOrigin
//         );
//     })
// }