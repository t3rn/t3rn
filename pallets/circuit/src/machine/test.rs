#[cfg(test)]
pub mod test {
    use circuit_mock_runtime::{
        AccountId, Attesters, Balance, Balances, BlockNumber, Circuit, ExtBuilder, Hash, Runtime,
        System,
    };
    use circuit_runtime_pallets::pallet_circuit::machine::{Machine, PrecompileResult};
    use frame_support::{assert_err, assert_ok};
    use sp_runtime::{DispatchError, ModuleError};
    use t3rn_primitives::circuit::traits::ReadSFX;

    use crate::{
        machine::test_extra::*,
        test_extra_stress::{
            EXECUTOR_1, EXECUTOR_10, EXECUTOR_2, EXECUTOR_3, EXECUTOR_4, EXECUTOR_5, EXECUTOR_6,
            EXECUTOR_7, EXECUTOR_8, EXECUTOR_9, INITIAL_BALANCE, REQUESTER_1,
        },
        tests::ESCROW_ACCOUNT,
    };
    use hex_literal::hex;
    use t3rn_primitives::circuit::{Cause, CircuitStatus};
    use t3rn_types::fsx::SecurityLvl;

    #[test]
    fn attesters_api_receives_sfx_after_finalized_all_steps_for_escrow_security() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .with_default_attestation_targets()
            .build()
            .execute_with(|| {
                stage_single();

                let target_0 = [0u8; 4];

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                let mut local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();

                local_ctx.xtx.status = CircuitStatus::FinishedAllSteps;
                local_ctx.full_side_effects[0][0].security_lvl = SecurityLvl::Escrow;

                Circuit::request_sfx_attestation(&local_ctx);

                let next_batch = Attesters::next_batches(target_0);

                let fsx_ids =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap();
                assert_eq!(fsx_ids.len(), 1);

                assert_eq!(next_batch.unwrap().committed_sfx, Some(fsx_ids));
            });
    }

    #[test]
    fn attesters_api_receives_sfx_after_revert_for_escrow_security() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .with_default_attestation_targets()
            .build()
            .execute_with(|| {
                stage_single();

                let target_0 = [0u8; 4];

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                let mut local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();

                local_ctx.xtx.status = CircuitStatus::Reverted(Cause::Timeout);
                local_ctx.full_side_effects[0][0].security_lvl = SecurityLvl::Escrow;

                Circuit::request_sfx_attestation(&local_ctx);

                let next_batch = Attesters::next_batches(target_0);

                let fsx_ids =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap();
                assert_eq!(fsx_ids.len(), 1);

                assert_eq!(next_batch.unwrap().reverted_sfx, Some(fsx_ids));
            });
    }

    #[test]
    fn attesters_api_does_not_receive_sfx_optimistic_security() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .with_default_attestation_targets()
            .build()
            .execute_with(|| {
                stage_single();

                let target_0 = [0u8; 4];

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                let mut local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();

                local_ctx.xtx.status = CircuitStatus::Committed;
                local_ctx.full_side_effects[0][0].security_lvl = SecurityLvl::Optimistic;

                Circuit::request_sfx_attestation(&local_ctx);

                let next_batch = Attesters::next_batches(target_0);
                assert_eq!(next_batch.clone().unwrap().reverted_sfx, None);
                assert_eq!(next_batch.unwrap().committed_sfx, None);
            });
    }

    #[test]
    fn attesters_api_receives_sfx_after_commit_for_escrow_security() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .with_default_attestation_targets()
            .build()
            .execute_with(|| {
                stage_single();

                let target_0 = [0u8; 4];

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                let mut local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();

                local_ctx.xtx.status = CircuitStatus::Committed;
                local_ctx.full_side_effects[0][0].security_lvl = SecurityLvl::Escrow;

                Circuit::request_sfx_attestation(&local_ctx);

                let next_batch = Attesters::next_batches(target_0);

                let fsx_ids =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap();
                assert_eq!(fsx_ids.len(), 1);

                assert_eq!(next_batch.unwrap().committed_sfx, Some(fsx_ids));
            });
    }

    #[test]
    fn read_sfx_api_get_fsx_if_xtx_exists() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                let fsx_ids =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap();
                assert_eq!(fsx_ids.len(), 1);
                assert_ok!(<Circuit as ReadSFX<
                    Hash,
                    AccountId,
                    Balance,
                    BlockNumber,
                >>::get_fsx(fsx_ids[0]),);
            });
    }

    #[test]
    fn read_sfx_api_errors_get_fsx_if_xtx_does_not_exist() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                assert_err!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx(
                        hex!("810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4")
                            .into(),
                    ),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [59, 0, 0, 0],
                        message: Some("XtxNotFound")
                    })
                );
            });
    }

    #[test]
    fn read_sfx_api_fsx_ids_for_xtx_which_exists() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                assert_eq!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id
                    ),
                    Ok(vec![hex!(
                        "810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4"
                    )
                    .into()])
                );
            });
    }

    #[test]
    fn read_sfx_api_returns_fsx_status_if_exists() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);
                let fsx_ids =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap();
                assert_eq!(fsx_ids.len(), 1);
                assert_eq!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_status(
                        fsx_ids[0]
                    ),
                    Ok(CircuitStatus::Ready)
                );
            });
    }

    #[test]
    fn read_sfx_api_errors_fsx_status_if_does_not_exist() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                assert_err!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_status(
                        hex!("810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4")
                            .into()
                    ),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [59, 0, 0, 0],
                        message: Some("XtxNotFound")
                    })
                );
            });
    }

    #[test]
    fn read_sfx_api_errors_if_fsx_does_not_exist() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                assert_err!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        hex!("810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4")
                            .into()
                    ),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [59, 0, 0, 0],
                        message: Some("XtxNotFound")
                    })
                );
            });
    }

    #[test]
    fn read_sfx_api_returns_xtx_status_if_exists() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);

                assert_eq!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_xtx_status(
                        xtx_id
                    ),
                    Ok((CircuitStatus::Ready, System::block_number() + 400))
                );
            });
    }

    #[test]
    fn read_sfx_api_returns_fsx_requester_if_xtx_exists() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);
                let fsx_id =
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_of_xtx(
                        xtx_id,
                    )
                    .unwrap()[0];

                assert_eq!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_requester(
                        fsx_id
                    ),
                    Ok(circuit_mock_runtime::ALICE)
                );
            });
    }

    #[test]
    fn read_sfx_api_fails_to_return_requester_if_xtx_does_not_exist() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                assert_err!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_fsx_requester(
                        hex!("810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4")
                            .into()
                    ),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [59, 0, 0, 0],
                        message: Some("XtxNotFound")
                    })
                );
            });
    }

    #[test]
    fn read_sfx_api_errors_if_xtx_does_not_exist() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                assert_err!(
                    <Circuit as ReadSFX<Hash, AccountId, Balance, BlockNumber>>::get_xtx_status(
                        hex!("810424cc4a8caa69bd0f1d9ee594f46bc45545a50b4cf8f7e78c41f0804d27a4")
                            .into()
                    ),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [59, 0, 0, 0],
                        message: Some("XtxNotFound")
                    })
                );
            });
    }

    #[test]
    fn machine_kills_from_allowed_states() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let mut xtx_id = setup_empty_xtx_and_force_set_status(None);

                assert!(Machine::<Runtime>::kill(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ),);

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::InBidding));

                assert!(Machine::<Runtime>::kill(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::PendingBidding));

                assert!(Machine::<Runtime>::kill(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(xtx_id);
            });
    }

    #[test]
    fn machine_reverts_kill_for_empty_sfx_from_all_allowed_states() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let mut xtx_id = setup_empty_xtx_and_force_set_status(None);

                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::InBidding));

                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(xtx_id);

                xtx_id = setup_empty_xtx_and_force_set_status(Some(CircuitStatus::PendingBidding));

                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(xtx_id);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_ready() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(None);
                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ),);
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()], 0);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_pending_execution() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(Some(
                    CircuitStatus::PendingExecution,
                ));
                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()], 0);
            });
    }

    #[test]
    fn machine_reverts_single_step_xtx_from_finished() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                let xtx_id = setup_single_sfx_xtx_and_post_bid_and_set_to_ready(Some(
                    CircuitStatus::PendingExecution,
                ));
                assert!(Machine::<Runtime>::revert(
                    xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));
                check_all_state_revert(xtx_id, vec![get_mocked_transfer_sfx()], 0);
            });
    }

    #[test]
    fn machine_does_not_revert_single_step_xtx_when_finished_all_steps() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                let xtx_id = setup_single_sfx_xtx_and_confirm();
                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    false
                );
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::FinishedAllSteps,
                    vec![get_mocked_transfer_sfx()],
                    0,
                );
            });
    }

    #[test]
    fn machine_does_not_revert_single_step_xtx_when_committed() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();
                let xtx_id = setup_single_sfx_xtx_and_confirm();

                assert_ok!(Machine::<Runtime>::compile(
                    &mut Machine::<Runtime>::load_xtx(xtx_id).unwrap(),
                    |_, _, _, _, _| {
                        Ok(PrecompileResult::ForceUpdateStatus(
                            CircuitStatus::Committed,
                        ))
                    },
                    no_post_updates,
                ));

                assert_eq!(
                    Machine::<Runtime>::revert(xtx_id, Cause::Timeout, infallible_no_post_updates,),
                    false
                );
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::Committed,
                    vec![get_mocked_transfer_sfx()],
                    0,
                );
            });
    }

    #[test]
    fn machine_traverses_single_step_xtx_from_requested_till_committed() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                stage_single();

                let xtx_id = setup_single_sfx_xtx_and_confirm();

                assert_ok!(Machine::<Runtime>::compile(
                    &mut Machine::<Runtime>::load_xtx(xtx_id).unwrap(),
                    |_, _, _, _, _| {
                        Ok(PrecompileResult::ForceUpdateStatus(
                            CircuitStatus::Committed,
                        ))
                    },
                    no_post_updates,
                ));
                check_all_single_xtx_state_correct(
                    xtx_id,
                    CircuitStatus::Committed,
                    vec![get_mocked_transfer_sfx_with_executor_enforced()],
                    0,
                );
            });
    }

    #[test]
    fn machine_confirms_xtx_with_10_sfx() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                crate::test_extra_stress::stage();

                const TEN: u32 = 10;
                let (mut local_ctx, sfx_arr_of_10, _sfx_id_arr_of_10) =
                    crate::test_extra_stress::setup_xtx_with_10_sfx([0u8; 4], &REQUESTER_1);

                crate::test_extra_stress::bid_for_n_out_of_10_sfx_in_xtx(
                    TEN,
                    &mut local_ctx,
                    REQUESTER_1,
                );
                assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);
                assert_ok!(Machine::<Runtime>::compile(
                    &mut local_ctx,
                    |_, _, _, _, _| Ok(PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready)),
                    no_post_updates,
                ));
                assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);

                crate::test_extra_stress::confirm_n_out_of_10_sfx_in_xtx_after_bidding(
                    TEN,
                    &mut local_ctx,
                );
                assert_eq!(local_ctx.xtx.status, CircuitStatus::FinishedAllSteps);

                check_all_single_xtx_state_correct(
                    local_ctx.xtx_id,
                    CircuitStatus::FinishedAllSteps,
                    sfx_arr_of_10,
                    0,
                );

                // check requester has its balance subtracted by total amount of max_rewards for all 10 x SFX
                assert_eq!(Balances::free_balance(&REQUESTER_1), 99945);
                // check honest executors' insurance deposits are returned
                assert_eq!(Balances::free_balance(&EXECUTOR_1), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_2), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_3), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_4), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_5), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_6), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_7), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_8), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_9), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_10), INITIAL_BALANCE);
                // check escrow account hasn't collected any extra funds from slashing
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), 0);
            });
    }

    #[test]
    fn machine_kills_and_cleans_xtx_if_only_5_out_of_10_sfx_bid() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                crate::test_extra_stress::stage();
                const FIVE: u32 = 5;

                let (mut local_ctx, _sfx_arr_of_10, _sfx_id_arr_of_10) =
                    crate::test_extra_stress::setup_xtx_with_10_sfx([0u8; 4], &REQUESTER_1);

                crate::test_extra_stress::bid_for_n_out_of_10_sfx_in_xtx(
                    FIVE,
                    &mut local_ctx,
                    REQUESTER_1,
                );

                assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);

                assert!(Machine::<Runtime>::kill(
                    local_ctx.xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_clean(local_ctx.xtx_id);

                // check requester has its balance returned in full
                assert_eq!(Balances::free_balance(&REQUESTER_1), INITIAL_BALANCE);
                // check executors have their balance returned in full
                assert_eq!(Balances::free_balance(&EXECUTOR_1), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_2), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_3), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_4), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_5), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_6), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_7), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_8), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_9), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_10), INITIAL_BALANCE);
                // check escrow account hasn't collected any extra funds from slashing
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), 0);
            });
    }

    #[test]
    fn machine_reverts_xtx_if_only_5_out_of_10_sfx_confirmed() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                crate::test_extra_stress::stage();
                const TEN: u32 = 10;
                const FIVE: u32 = 5;

                let (mut local_ctx, sfx_arr_of_10, _sfx_id_arr_of_10) =
                    crate::test_extra_stress::setup_xtx_with_10_sfx([0u8; 4], &REQUESTER_1);

                crate::test_extra_stress::bid_for_n_out_of_10_sfx_in_xtx(
                    TEN,
                    &mut local_ctx,
                    REQUESTER_1,
                );

                assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);
                assert_ok!(Machine::<Runtime>::compile(
                    &mut local_ctx,
                    |_, _, _, _, _| Ok(PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready)),
                    no_post_updates,
                ));
                assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);

                crate::test_extra_stress::confirm_n_out_of_10_sfx_in_xtx_after_bidding(
                    FIVE,
                    &mut local_ctx,
                );

                assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingExecution);

                assert!(Machine::<Runtime>::revert(
                    local_ctx.xtx_id,
                    Cause::Timeout,
                    infallible_no_post_updates,
                ));

                check_all_state_revert(local_ctx.xtx_id, sfx_arr_of_10, 0);

                // check requester has its balance returned in full
                assert_eq!(Balances::free_balance(&REQUESTER_1), INITIAL_BALANCE);
                // check honest executors have their balance returned in full
                assert_eq!(Balances::free_balance(&EXECUTOR_1), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_2), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_3), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_4), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_5), INITIAL_BALANCE);
                // check dishonest executors have their balance slashed
                assert_eq!(Balances::free_balance(&EXECUTOR_6), 99945);
                assert_eq!(Balances::free_balance(&EXECUTOR_7), 99945);
                assert_eq!(Balances::free_balance(&EXECUTOR_8), 99945);
                assert_eq!(Balances::free_balance(&EXECUTOR_9), 99945);
                assert_eq!(Balances::free_balance(&EXECUTOR_10), 99945);
                // check escrow account collected slashed funds from dishonest executors
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), 275);
            });
    }

    #[test]
    fn machine_confirms_10_xtx_with_10_sfx_each() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                crate::test_extra_stress::stage();
                const TEN: u32 = 10;

                let mut local_context_array_of_10_xtx =
                    crate::test_extra_stress::setup_n_xtx_with_10_sfx_each(
                        TEN,
                        [0u8; 4],
                        &REQUESTER_1,
                    );

                for i in 0..TEN as usize {
                    crate::test_extra_stress::bid_for_n_out_of_10_sfx_in_xtx(
                        TEN,
                        &mut local_context_array_of_10_xtx[i].0,
                        REQUESTER_1,
                    );
                }

                for i in 0..TEN as usize {
                    let local_ctx = &mut local_context_array_of_10_xtx[i].0;
                    assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);
                    assert_ok!(Machine::<Runtime>::compile(
                        local_ctx,
                        |_, _, _, _, _| Ok(PrecompileResult::ForceUpdateStatus(
                            CircuitStatus::Ready
                        )),
                        no_post_updates,
                    ));
                    assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);
                }

                for i in 0..TEN as usize {
                    crate::test_extra_stress::confirm_n_out_of_10_sfx_in_xtx_after_bidding(
                        TEN,
                        &mut local_context_array_of_10_xtx[i].0,
                    );

                    assert_eq!(
                        local_context_array_of_10_xtx[i].0.xtx.status,
                        CircuitStatus::FinishedAllSteps
                    );

                    check_all_single_xtx_state_correct(
                        local_context_array_of_10_xtx[i].0.xtx_id,
                        CircuitStatus::FinishedAllSteps,
                        local_context_array_of_10_xtx[i].1.clone(),
                        i as u32,
                    );
                }
                // check requester has its balance subtracted by total amount of max_rewards for all 10 x 10 x SFX
                assert_eq!(Balances::free_balance(&REQUESTER_1), 99450);
                // check honest executors' insurance deposits are returned
                assert_eq!(Balances::free_balance(&EXECUTOR_1), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_2), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_3), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_4), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_5), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_6), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_7), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_8), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_9), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR_10), INITIAL_BALANCE);
                // check escrow account hasn't collected any extra funds from slashing
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), 0);
            });
    }
}
