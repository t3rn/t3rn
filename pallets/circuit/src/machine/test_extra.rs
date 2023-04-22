use frame_support::assert_ok;
use sp_core::H256;

use crate::{
    tests::{ALICE, BOB_RELAYER},
    SideEffect,
};
use frame_support::traits::Currency;

use circuit_mock_runtime::{AccountId, Balance, Balances, BlockNumber, Runtime, System};
use circuit_runtime_pallets::pallet_circuit::{
    machine::{extra::validate_fsx_against_xtx, Machine, PrecompileResult},
    pallet::*,
    state::{Cause, CircuitStatus, LocalXtxCtx},
    Config, Error, XExecSignal,
};
use t3rn_primitives::circuit::SpeedMode;
use t3rn_types::sfx::{ConfirmedSideEffect, FullSideEffect};

use t3rn_primitives::xtx::LocalState;

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

pub const INITIAL_BALANCE_10: Balance = 10 as Balance;

pub fn stage_single() {
    System::set_block_number(1);
    let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE_10);
    let _ = Balances::deposit_creating(&BOB_RELAYER, INITIAL_BALANCE_10);
}

pub fn setup_empty_xtx_and_force_set_status(maybe_status: Option<CircuitStatus>) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Reserved);

    if let Some(status) = maybe_status {
        local_ctx.xtx.status = status;
        XExecSignals::<Runtime>::mutate(local_ctx.xtx_id, |x| {
            *x = Some(local_ctx.xtx.clone());
        });
    }
    local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_force_set_status(maybe_status: Option<CircuitStatus>) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    if let Some(status) = maybe_status {
        local_ctx.xtx.status = status;
        XExecSignals::<Runtime>::mutate(local_ctx.xtx_id, |x| {
            *x = Some(local_ctx.xtx.clone());
        });
    }
    local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_post_bid_and_set_to_ready(
    maybe_status: Option<CircuitStatus>,
) -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    let sfx_id = get_mocked_transfer_sfx_id(local_ctx.xtx_id);

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| { Ok(PrecompileResult::TryBid((sfx_id, 2, BOB_RELAYER))) },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            Ok(PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready))
        },
        no_post_updates,
    )
    .unwrap();

    if let Some(status) = maybe_status {
        Machine::<Runtime>::compile(
            &mut local_ctx,
            |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
             _local_state: LocalState,
             _steps_cnt: (u32, u32),
             _status: CircuitStatus,
             _requester: AccountId| { Ok(PrecompileResult::ForceUpdateStatus(status)) },
            no_post_updates,
        )
        .unwrap();
    }

    local_ctx.xtx_id
}

pub fn setup_single_sfx_xtx_and_confirm() -> H256 {
    let mut local_ctx = Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE).unwrap();
    let sfx_id = get_mocked_transfer_sfx_id(local_ctx.xtx_id);

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Requested);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| { Ok(PrecompileResult::TryBid((sfx_id, 2, BOB_RELAYER))) },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::InBidding);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            Ok(PrecompileResult::ForceUpdateStatus(CircuitStatus::Ready))
        },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::Ready);

    Machine::<Runtime>::compile(
        &mut local_ctx,
        |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
         _local_state: LocalState,
         _steps_cnt: (u32, u32),
         _status: CircuitStatus,
         _requester: AccountId| {
            let confirmation = ConfirmedSideEffect::<AccountId, BlockNumber, Balance> {
                err: None,
                output: None,
                inclusion_data: vec![1, 2, 3],
                executioner: BOB_RELAYER,
                received_at: 2,
                cost: None,
            };
            Ok(PrecompileResult::TryConfirm(sfx_id, confirmation))
        },
        no_post_updates,
    )
    .unwrap();

    assert_eq!(local_ctx.xtx.status, CircuitStatus::FinishedAllSteps);

    local_ctx.xtx_id
}

pub fn check_all_state_clean(xtx_id: H256) {
    assert_eq!(XExecSignals::<Runtime>::get(xtx_id), None);
    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert_eq!(LocalXtxStates::<Runtime>::get(xtx_id), None);
    assert_eq!(FullSideEffects::<Runtime>::get(xtx_id), None);
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn check_all_single_xtx_state_correct(
    xtx_id: H256,
    success_state: CircuitStatus,
    _committed_sfx: Vec<SideEffect<AccountId, Balance>>,
    requester_nonce: u32,
) {
    let expected_steps_cnt = if success_state >= CircuitStatus::Finished {
        (1, 1)
    } else {
        (0, 1)
    };

    assert_eq!(
        XExecSignals::<Runtime>::get(xtx_id),
        Some(XExecSignal {
            status: success_state,
            requester: ALICE,
            timeouts_at: 401u32,
            delay_steps_at: None,
            requester_nonce,
            steps_cnt: expected_steps_cnt,
            speed_mode: SpeedMode::Finalized,
        })
    );
    let local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();
    assert_ok!(validate_fsx_against_xtx(&local_ctx));

    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn check_all_state_revert(
    xtx_id: H256,
    _reverted_sfx: Vec<SideEffect<AccountId, Balance>>,
    requester_nonce: u32,
) {
    assert_eq!(
        XExecSignals::<Runtime>::get(xtx_id),
        Some(XExecSignal {
            status: CircuitStatus::Reverted(Cause::Timeout),
            requester: ALICE,
            timeouts_at: 401u32,
            delay_steps_at: None,
            requester_nonce,
            steps_cnt: (0, 1),
            speed_mode: SpeedMode::Finalized,
        })
    );

    let local_ctx = Machine::<Runtime>::load_xtx(xtx_id).unwrap();
    assert_ok!(validate_fsx_against_xtx(&local_ctx));

    assert_eq!(PendingXtxTimeoutsMap::<Runtime>::get(xtx_id), None);
    assert!(LocalXtxStates::<Runtime>::get(xtx_id).is_some());
    assert_eq!(PendingXtxBidsTimeoutsMap::<Runtime>::get(xtx_id), None);
}

pub fn get_mocked_transfer_sfx() -> SideEffect<AccountId, Balance> {
    SideEffect {
        target: [0u8, 0u8, 0u8, 0u8],
        max_reward: 2,
        action: [116, 114, 97, 110],
        encoded_args: vec![
            vec![
                42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247, 0,
                118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
            ],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        signature: vec![],
        enforce_executor: None,
        insurance: 3,
        reward_asset_id: None,
    }
}

pub fn get_mocked_transfer_sfx_with_executor_enforced() -> SideEffect<AccountId, Balance> {
    SideEffect {
        target: [0u8, 0u8, 0u8, 0u8],
        max_reward: 2,
        action: [116, 114, 97, 110],
        encoded_args: vec![
            vec![
                42, 246, 86, 215, 84, 26, 25, 17, 173, 225, 126, 30, 234, 99, 78, 169, 50, 247, 0,
                118, 125, 167, 191, 15, 94, 94, 97, 126, 250, 236, 22, 62,
            ],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        signature: vec![],
        enforce_executor: Some(BOB_RELAYER),
        insurance: 3,
        reward_asset_id: None,
    }
}

pub fn get_mocked_transfer_sfx_id(xtx_id: H256) -> H256 {
    let sfx = get_mocked_transfer_sfx();
    sfx.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(&xtx_id.0, 0)
}
