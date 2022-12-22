#[cfg(test)]
pub mod test {

    use circuit_mock_runtime::{ExtBuilder, Runtime, System};
    use circuit_runtime_pallets::pallet_circuit::{
        machine::{Machine, PrecompileResult},
        state::{Cause, CircuitStatus},
    };

    use crate::machine::test_extra::*;

    #[test]
    fn machine_kills_from_allowed_states() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let mut xtx_id = setup_empty_xtx_and_force_set_status(None);

                assert_eq!(
                    Machine::<Runtime>::kill(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::InBidding));

                assert_eq!(
                    Machine::<Runtime>::kill(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::PendingBidding));

                assert_eq!(
                    Machine::<Runtime>::kill(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);
            });
    }

    #[test]
    fn machine_reverts_kill_for_empty_sfx_from_all_allowed_states() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let mut xtx_id = setup_empty_xtx_and_force_set_status(None);

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::InBidding));

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::PendingBidding));

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                let mut xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::Ready));

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);

                xtx_id =
                    setup_empty_xtx_and_force_set_status(Some(CircuitStatus::PendingExecution));

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );

                check_all_state_clean(xtx_id);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_ready() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id = setup_single_sfx_xtx_and_post_bid(None);
                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()]);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_pending_execution() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id =
                    setup_single_sfx_xtx_and_post_bid(Some(CircuitStatus::PendingExecution));
                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()]);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_finished() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id = setup_single_sfx_xtx_and_post_bid(Some(CircuitStatus::Finished));
                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    true
                );
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()]);
            });
    }

    #[test]
    fn machine_does_not_revert_single_step_xtx_when_finished_all_steps() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id = setup_single_sfx_xtx_and_confirm();
                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    false
                );
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::FinishedAllSteps,
                    vec![get_mocked_transfer_sfx()],
                );
            });
    }

    #[test]
    fn machine_does_not_revert_single_step_xtx_when_committed() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id = setup_single_sfx_xtx_and_confirm();
                Machine::<Runtime>::compile(
                    &mut Machine::<Runtime>::load_xtx(xtx_id).unwrap(),
                    |_, _, _, _, _| {
                        Ok(PrecompileResult::ForceUpdateStatus(
                            CircuitStatus::Committed,
                        ))
                    },
                    no_post_updates,
                )
                .unwrap();

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    false
                );
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::Committed,
                    vec![get_mocked_transfer_sfx()],
                );
            });
    }

    #[test]
    fn machine_traverses_single_step_xtx_from_requested_till_committed() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);
                let xtx_id = setup_single_sfx_xtx_and_confirm();
                Machine::<Runtime>::compile(
                    &mut Machine::<Runtime>::load_xtx(xtx_id).unwrap(),
                    |_, _, _, _, _| {
                        Ok(PrecompileResult::ForceUpdateStatus(
                            CircuitStatus::Committed,
                        ))
                    },
                    no_post_updates,
                )
                .unwrap();
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::Committed,
                    vec![get_mocked_transfer_sfx()],
                );
            });
    }
}
