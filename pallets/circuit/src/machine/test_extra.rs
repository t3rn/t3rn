use sp_core::H256;

use crate::{
    tests::{ALICE, BOB_RELAYER},
    SideEffect,
};
use circuit_mock_runtime::{AccountId, Balance, BlockNumber, Runtime};
use circuit_runtime_pallets::pallet_circuit::{
    machine::{Machine, PrecompileResult},
    pallet::*,
    state::{Cause, CircuitStatus, LocalXtxCtx},
    Config, Error, XExecSignal,
};
use t3rn_primitives::{
    side_effect::{ConfirmedSideEffect, FullSideEffect, SFXBid},
    xtx::LocalState,
};

pub fn no_post_updates<T: Config>(
    _status_change: (CircuitStatus, CircuitStatus),
    _local_ctx: &LocalXtxCtx<T>,
) -> Result<(), Error<T>> {
    Ok(())
}

pub fn no_mangle<T: Config>(
    _current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
    _local_state: LocalState,
    _steps_cnt: (u32, u32),
    _status: CircuitStatus,
    _requester: AccountId,
) -> Result<PrecompileResult<T>, Error<T>> {
    Ok(PrecompileResult::Continue)
}

pub fn infallible_no_post_updates<T: Config>(
    _status_change: (CircuitStatus, CircuitStatus),
    _local_ctx: &LocalXtxCtx<T>,
) {
}

pub fn setup_empty_xtx_and_force_set_status(maybe_status: Option<CircuitStatus>) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(&mut local_ctx, no_mangle, no_post_updates).unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Reserved);

    if let Some(status) = maybe_status {
        local_ctx.xtx.status = status;
        XExecSignals::<Runtime>::mutate(local_ctx.xtx_id, |x| {
            *x = Some(local_ctx.xtx.clone());
        });
    }
    return local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_force_set_status(maybe_status: Option<CircuitStatus>) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(&mut local_ctx, no_mangle, no_post_updates).unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    if let Some(status) = maybe_status {
        local_ctx.xtx.status = status;
        XExecSignals::<Runtime>::mutate(local_ctx.xtx_id, |x| {
            *x = Some(local_ctx.xtx.clone());
        });
    }
    return local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_post_bid(maybe_status: Option<CircuitStatus>) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(&mut local_ctx, no_mangle, no_post_updates).unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            Ok(PrecompileResult::ForceUpdateStatus(
                CircuitStatus::InBidding,
            ))
        },
        no_post_updates,
    )
    .unwrap();

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            current_fsx[0].best_bid = Some(SFXBid::<AccountId, Balance, u32>::new_none_optimistic(
                2,
                3,
                BOB_RELAYER,
                ALICE,
                None,
            ));

            Ok(PrecompileResult::UpdateFSX(current_fsx.clone()))
        },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);

    if let Some(status) = maybe_status {
        local_ctx.xtx.status = status;
        XExecSignals::<Runtime>::mutate(local_ctx.xtx_id, |x| {
            *x = Some(local_ctx.xtx.clone());
        });
    }

    return local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_confirm() -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(&mut local_ctx, no_mangle, no_post_updates).unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            Ok(PrecompileResult::ForceUpdateStatus(
                CircuitStatus::InBidding,
            ))
        },
        no_post_updates,
    )
    .unwrap();

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            current_fsx[0].best_bid = Some(SFXBid::<AccountId, Balance, u32>::new_none_optimistic(
                2,
                3,
                BOB_RELAYER,
                ALICE,
                None,
            ));
            Ok(PrecompileResult::UpdateFSX(current_fsx.clone()))
        },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            current_fsx[0].confirmed =
                Some(ConfirmedSideEffect::<AccountId, BlockNumber, Balance> {
                    err: None,
                    output: None,
                    inclusion_data: vec![1, 2, 3],
                    executioner: BOB_RELAYER,
                    received_at: 2,
                    cost: None,
                });
            Ok(PrecompileResult::UpdateFSX(current_fsx.clone()))
        },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::FinishedAllSteps);

    return local_ctx.xtx_id
}

pub fn check_all_state_clean(xtx_id: H256) {
    assert_eq!(XExecSignals::<Runtime>::get(xtx_id), None);
    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert_eq!(LocalXtxStates::<Runtime>::get(xtx_id), None);
    assert_eq!(FullSideEffects::<Runtime>::get(xtx_id), None);
    // assert_eq!(PendingSFXBids::<Runtime>::get(xtx_id), None);
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn check_all_single_xtx_state_correct(
    xtx_id: H256,
    success_state: CircuitStatus,
    committed_sfx: Vec<SideEffect<AccountId, Balance>>,
) {
    let expected_steps_cnt = if success_state >= CircuitStatus::Finished {
        (1, 1)
    } else {
        (0, 1)
    };

    assert_eq!(
        XExecSignals::<Runtime>::get(xtx_id),
        Some(XExecSignal {
            status: success_state.clone(),
            requester: ALICE,
            timeouts_at: 401u32,
            delay_steps_at: None,
            requester_nonce: 0u32,
            steps_cnt: expected_steps_cnt,
        })
    );
    if committed_sfx.is_empty() {
        assert_eq!(FullSideEffects::<Runtime>::get(xtx_id), None);
    } else {
        match FullSideEffects::<Runtime>::get(xtx_id) {
            Some(fsx_vector) => {
                assert_eq!(
                    fsx_vector
                        .clone()
                        .into_iter()
                        .flatten()
                        .map(|fsx| fsx.input)
                        .collect::<Vec<SideEffect<AccountId, Balance>>>(),
                    committed_sfx
                );
                let maybe_sfx_confirmation = if success_state.clone() >= CircuitStatus::Finished {
                    Some(ConfirmedSideEffect::<AccountId, BlockNumber, Balance> {
                        err: None,
                        output: None,
                        inclusion_data: vec![1, 2, 3],
                        executioner: BOB_RELAYER,
                        received_at: 2,
                        cost: None,
                    })
                } else {
                    None
                };
                for fsx in fsx_vector.into_iter().flatten() {
                    assert_eq!(fsx.confirmed, maybe_sfx_confirmation);
                }
            },
            None => assert!(false),
        }
    }

    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert!(LocalXtxStates::<Runtime>::get(xtx_id).is_some());
    committed_sfx
        .iter()
        .enumerate()
        .for_each(|(sfx_index, sfx)| {
            assert_eq!(
                PendingSFXBids::<Runtime>::get(
                    xtx_id,
                    sfx.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                        &xtx_id[..], sfx_index as u32,
                    )
                ),
                None
            );
        });
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn check_all_state_revert(xtx_id: H256, reverted_sfx: Vec<SideEffect<AccountId, Balance>>) {
    assert_eq!(
        XExecSignals::<Runtime>::get(xtx_id),
        Some(XExecSignal {
            status: CircuitStatus::Reverted(Cause::Timeout),
            requester: ALICE,
            timeouts_at: 401u32,
            delay_steps_at: None,
            requester_nonce: 0u32,
            steps_cnt: (0, 1),
        })
    );
    if reverted_sfx.is_empty() {
        assert_eq!(FullSideEffects::<Runtime>::get(xtx_id), None);
    } else {
        match FullSideEffects::<Runtime>::get(xtx_id) {
            Some(fsx_vector) => {
                assert_eq!(
                    fsx_vector
                        .clone()
                        .into_iter()
                        .flatten()
                        .map(|fsx| fsx.input)
                        .collect::<Vec<SideEffect<AccountId, Balance>>>(),
                    reverted_sfx
                );

                for fsx in fsx_vector.into_iter().flatten() {
                    assert_eq!(fsx.confirmed, None);
                }
            },
            None => assert!(false),
        }
    }

    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert!(LocalXtxStates::<Runtime>::get(xtx_id).is_some());
    reverted_sfx
        .iter()
        .enumerate()
        .for_each(|(sfx_index, sfx)| {
            assert_eq!(
            PendingSFXBids::<Runtime>::get(
                xtx_id,
                sfx.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                    &xtx_id[..], sfx_index as u32,
                )
            ),
            None
        );
        });
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn get_mocked_transfer_sfx() -> SideEffect<AccountId, Balance> {
    SideEffect {
        target: [0u8, 0u8, 0u8, 0u8],
        max_reward: 2,
        encoded_action: vec![116, 114, 97, 110],
        encoded_args: vec![
            vec![
                212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44,
                133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
            ],
            vec![
                42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247, 0,
                118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
            ],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        ],
        signature: vec![],
        enforce_executor: Some(
            [
                53, 68, 51, 51, 51, 101, 66, 98, 53, 86, 117, 103, 72, 105, 111, 70, 111, 85, 53,
                110, 71, 77, 98, 85, 97, 82, 50, 117, 89, 99, 111, 121,
            ]
            .into(),
        ),
        insurance: 3,
        reward_asset_id: None,
    }
}
