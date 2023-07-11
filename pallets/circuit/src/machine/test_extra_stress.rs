use sp_core::H256;

use crate::{test_extra::*, SideEffect};
use circuit_mock_runtime::{AccountId, Balance, Balances, BlockNumber, Runtime, System};
use circuit_runtime_pallets::pallet_circuit::{
    machine::{Machine, PrecompileResult},
    state::{CircuitStatus, LocalXtxCtx},
};
use codec::Encode;
use frame_support::{assert_ok, traits::Currency};
use sp_runtime::AccountId32;
use t3rn_primitives::xtx::LocalState;

use t3rn_types::{fsx::FullSideEffect, sfx::ConfirmedSideEffect};

pub const REQUESTER_1: AccountId = AccountId32::new([1u8; 32]);
pub const REQUESTER_2: AccountId = AccountId32::new([2u8; 32]);
pub const FROM_ACCOUNT: AccountId = AccountId32::new([111u8; 32]);
pub const TO_ACCOUNT: AccountId = AccountId32::new([222u8; 32]);

pub const EXECUTOR_1: AccountId = AccountId32::new([10u8; 32]);
pub const EXECUTOR_2: AccountId = AccountId32::new([11u8; 32]);
pub const EXECUTOR_3: AccountId = AccountId32::new([12u8; 32]);
pub const EXECUTOR_4: AccountId = AccountId32::new([13u8; 32]);
pub const EXECUTOR_5: AccountId = AccountId32::new([14u8; 32]);
pub const EXECUTOR_6: AccountId = AccountId32::new([15u8; 32]);
pub const EXECUTOR_7: AccountId = AccountId32::new([16u8; 32]);
pub const EXECUTOR_8: AccountId = AccountId32::new([17u8; 32]);
pub const EXECUTOR_9: AccountId = AccountId32::new([18u8; 32]);
pub const EXECUTOR_10: AccountId = AccountId32::new([19u8; 32]);

pub const INITIAL_BALANCE: Balance = 100_000;

pub fn stage() {
    System::set_block_number(1);

    let _ = Balances::deposit_creating(&REQUESTER_1, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&REQUESTER_2, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_1, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_2, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_3, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_4, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_5, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_6, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_7, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_8, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_9, INITIAL_BALANCE);
    let _ = Balances::deposit_creating(&EXECUTOR_10, INITIAL_BALANCE);
}

pub fn stage_transfer_sfx(
    target: [u8; 4],
    max_reward: Balance,
    insurance: Balance,
) -> SideEffect<AccountId, Balance> {
    let mut insurance_and_reward = vec![];
    insurance_and_reward.extend_from_slice(&max_reward.encode());
    insurance_and_reward.append(&mut insurance.encode());
    SideEffect {
        target,
        max_reward,
        // Encoded Transfer SFX
        action: [116, 114, 97, 110],
        encoded_args: vec![TO_ACCOUNT.encode(), max_reward.encode()],
        signature: vec![],
        enforce_executor: None,
        insurance,
        reward_asset_id: None,
    }
}

pub fn setup_n_xtx_with_10_sfx_each(
    n: u32,
    target: [u8; 4],
    requester: &AccountId,
) -> Vec<(
    LocalXtxCtx<Runtime, Balance>,
    Vec<SideEffect<AccountId, Balance>>,
    Vec<H256>,
)> {
    let mut n_xtx_array = vec![];
    for _i in 0..n {
        n_xtx_array.push(setup_xtx_with_10_sfx(target, requester));
        frame_system::Pallet::<Runtime>::inc_account_nonce(requester);
    }
    n_xtx_array
}

pub fn setup_xtx_with_10_sfx(
    target: [u8; 4],
    requester: &AccountId,
) -> (
    LocalXtxCtx<Runtime, Balance>,
    Vec<SideEffect<AccountId, Balance>>,
    Vec<H256>,
) {
    let mut sfx_arr_of_10 = vec![];
    let mut sfx_id_arr_of_10 = vec![];

    for sfx_index in 0u32..10u32 {
        sfx_arr_of_10.push(stage_transfer_sfx(
            target,
            (sfx_index + 1) as Balance,
            (sfx_index + 1) as Balance,
        ));
    }

    let mut local_ctx = Machine::<Runtime>::setup(&sfx_arr_of_10, requester, None).unwrap();
    assert!(Machine::<Runtime>::compile(
        &mut local_ctx,
        |_, _, _, _, _| Ok(PrecompileResult::TryRequest),
        no_post_updates
    )
    .unwrap());
    // Double check that Xtx is stored under the same ID as the one returned by setup
    assert_eq!(
        local_ctx.xtx_id,
        Machine::<Runtime>::load_xtx(local_ctx.xtx_id)
            .unwrap()
            .xtx_id
    );
    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);

    for sfx_index in 0u32..10u32 {
        sfx_id_arr_of_10.push(
            sfx_arr_of_10[sfx_index as usize]
                .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                    &local_ctx.xtx_id[..],
                    sfx_index,
                ),
        );
    }

    (local_ctx, sfx_arr_of_10, sfx_id_arr_of_10)
}

pub fn bid_for_n_out_of_10_sfx_in_xtx(
    n: u32,
    local_ctx: &mut LocalXtxCtx<Runtime, Balance>,
    _requester: AccountId,
) {
    if n > 10 {
        panic!("Can't bid for more than 10 SFXs");
    }

    assert_eq!(local_ctx.xtx.status, CircuitStatus::PendingBidding);
    assert_eq!(
        local_ctx.xtx_id,
        Machine::<Runtime>::load_xtx(local_ctx.xtx_id)
            .unwrap()
            .xtx_id
    );
    const STEP_INDEX: usize = 0_usize;
    let fsx_step = local_ctx.full_side_effects[STEP_INDEX].clone();

    assert_eq!(fsx_step.len(), 10);

    for sfx_index in 0u32..n {
        let next_bidder = match sfx_index {
            0 => EXECUTOR_1,
            1 => EXECUTOR_2,
            2 => EXECUTOR_3,
            3 => EXECUTOR_4,
            4 => EXECUTOR_5,
            5 => EXECUTOR_6,
            6 => EXECUTOR_7,
            7 => EXECUTOR_8,
            8 => EXECUTOR_9,
            9 => EXECUTOR_10,
            _ => panic!("Invalid sfx index"),
        };

        let bid_amount = (sfx_index + 1) as Balance;

        let sfx_id = fsx_step[sfx_index as usize]
            .input
            .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &local_ctx.xtx_id.0,
            sfx_index,
        );

        assert_ok!(Machine::<Runtime>::compile(
            local_ctx,
            |_current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
             _local_state: LocalState,
             _steps_cnt: (u32, u32),
             _status: CircuitStatus,
             _requester: AccountId| {
                Ok(PrecompileResult::TryBid((sfx_id, bid_amount, next_bidder)))
            },
            no_post_updates,
        ));
    }
}

pub fn confirm_n_out_of_10_sfx_in_xtx_after_bidding(
    n: u32,
    local_ctx: &mut LocalXtxCtx<Runtime, Balance>,
) {
    if n > 10 {
        panic!("Can't bid for more than 10 SFXs");
    }
    for sfx_index in 0u32..n {
        let next_executor = match sfx_index {
            0 => EXECUTOR_1,
            1 => EXECUTOR_2,
            2 => EXECUTOR_3,
            3 => EXECUTOR_4,
            4 => EXECUTOR_5,
            5 => EXECUTOR_6,
            6 => EXECUTOR_7,
            7 => EXECUTOR_8,
            8 => EXECUTOR_9,
            9 => EXECUTOR_10,
            _ => panic!("Invalid sfx index"),
        };

        let sfx_confirmation = ConfirmedSideEffect::<AccountId, BlockNumber, Balance> {
            err: None,
            output: None,
            inclusion_data: vec![1, 2, 3],
            executioner: next_executor,
            received_at: 2,
            cost: None,
        };

        assert_ok!(Machine::<Runtime>::compile(
            local_ctx,
            |current_fsx: &mut Vec<FullSideEffect<AccountId, BlockNumber, Balance>>,
             _local_state: LocalState,
             _steps_cnt: (u32, u32),
             _status: CircuitStatus,
             _requester: AccountId| {
                current_fsx[sfx_index as usize].confirmed = Some(sfx_confirmation);
                Ok(PrecompileResult::TryUpdateFSX(current_fsx.clone()))
            },
            no_post_updates,
        ));
    }
}
