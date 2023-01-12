#[cfg(test)]
pub mod test {
    use crate::{
        square_up::test_extra::*,
        tests::{ALICE, BOB},
        SFXBid,
    };
    use sp_runtime::{DispatchError, ModuleError};
    use t3rn_primitives::{
        account_manager::{
            AccountManager as AccountManagerInterface, Outcome, RequestCharge, Settlement,
        },
        claimable::{BenefitSource, CircuitRole},
    };

    use circuit_mock_runtime::{
        AccountId, AccountManager, AssetId, Balance, Balances, BlockNumber, ExtBuilder, Hash,
        Runtime, System,
    };
    use circuit_runtime_pallets::pallet_circuit::{
        machine::Machine, square_up::SquareUp, state::LocalXtxCtx,
    };
    use frame_support::{assert_err, assert_ok, traits::Currency};
    use sp_core::H256;

    use sp_runtime::DispatchResult;
    use t3rn_primitives::side_effect::ConfirmedSideEffect;

    fn request_and_bid_single_sfx_xtx(
        local_ctx: &mut LocalXtxCtx<Runtime>,
        bid: &SFXBid<AccountId, Balance, AssetId>,
    ) -> DispatchResult {
        assert_ok!(SquareUp::<Runtime>::try_request(&local_ctx));
        let requester = local_ctx.xtx.requester.clone();
        let fsx = local_ctx.full_side_effects[0][0].clone();
        let sfx_id = fsx
            .input
            .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                &local_ctx.xtx_id.0,
                0,
            );

        let bid_id = bid.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>, Runtime>(sfx_id);

        assert_ok!(SquareUp::<Runtime>::try_bid(
            sfx_id,
            &bid.requester.clone(),
            &bid.executor.clone(),
            &bid,
            None
        ));

        local_ctx.full_side_effects[0][0].best_bid = Some(bid.clone());

        assert_eq!(
            <AccountManager as AccountManagerInterface<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::get_charge_or_fail(sfx_id),
            Ok(RequestCharge {
                payee: requester,
                offered_reward: fsx.input.max_reward,
                maybe_asset_id: fsx.input.reward_asset_id,
                charge_fee: 0,
                recipient: None,
                source: BenefitSource::TrafficRewards,
                role: CircuitRole::Executor
            })
        );
        assert_eq!(
            <AccountManager as AccountManagerInterface<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
                AssetId,
            >>::get_charge_or_fail(bid_id),
            Ok(RequestCharge {
                payee: bid.executor.clone(),
                offered_reward: bid.amount + bid.insurance + bid.reserved_bond.unwrap_or(0),
                maybe_asset_id: bid.reward_asset_id.clone(),
                charge_fee: 0,
                recipient: Some(bid.requester.clone()),
                source: BenefitSource::TrafficRewards,
                role: CircuitRole::Executor
            })
        );

        Ok(())
    }

    pub fn assert_pending_charges_no_longer_exist(charged_ids: Vec<H256>) {
        for id in charged_ids {
            assert_err!(
                <AccountManager as AccountManagerInterface<
                    AccountId,
                    Balance,
                    Hash,
                    BlockNumber,
                    AssetId,
                >>::get_charge_or_fail(id),
                DispatchError::Module(ModuleError {
                    index: 125,
                    error: [5, 0, 0, 0],
                    message: Some("NoChargeOfGivenIdRegistered")
                })
            );
        }
    }

    #[test]
    fn square_up_locks_up_requester_with_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 3);
                let local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                assert_ok!(SquareUp::<Runtime>::try_request(&local_ctx));

                assert_eq!(Balances::free_balance(&ALICE), 1);
            });
    }

    #[test]
    fn square_up_fails_lock_up_requester_without_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 1);
                let local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                assert_err!(
                    SquareUp::<Runtime>::try_request(&local_ctx),
                    DispatchError::Module(ModuleError {
                        index: 108,
                        error: [8, 0, 0, 0],
                        message: Some("RequesterNotEnoughBalance")
                    })
                );

                // Balance stays unchanged
                assert_eq!(Balances::free_balance(&ALICE), 1);
            });
    }

    #[test]
    fn square_up_locks_up_first_bidder_with_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 4);
                let _local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                assert_ok!(SquareUp::<Runtime>::try_bid(
                    H256::repeat_byte(1),
                    &BOB,
                    &ALICE,
                    &SFXBid {
                        amount: 2,
                        insurance: 1,
                        reserved_bond: None,
                        reward_asset_id: None,
                        executor: ALICE,
                        requester: BOB,
                    },
                    None
                ));

                assert_eq!(Balances::free_balance(&ALICE), 1);
            });
    }

    #[test]
    fn square_up_kills_xtx_with_its_all_bids() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 10);
                let mut local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                let bid = SFXBid {
                    amount: 2,
                    insurance: 1,
                    reserved_bond: None,
                    reward_asset_id: None,
                    executor: ALICE,
                    requester: BOB,
                };

                let sfx_id = local_ctx.full_side_effects[0][0]
                    .input
                    .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                    &local_ctx.xtx_id.0,
                    0,
                );

                let bid_id = bid.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>, Runtime>(sfx_id);

                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                assert_eq!(SquareUp::<Runtime>::kill(&local_ctx), true);

                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);
            });
    }

    #[test]
    fn square_up_finalizes_and_successfully_commits_finished_fsx() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 10);
                let _ = Balances::deposit_creating(&BOB, 10);

                const REQUESTER: AccountId = ALICE;
                const EXECUTOR: AccountId = BOB;

                let mut local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                let bid = SFXBid {
                    amount: 2,
                    insurance: 1,
                    reserved_bond: None,
                    reward_asset_id: None,
                    executor: EXECUTOR,
                    requester: REQUESTER,
                };
                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));
                let fsx = local_ctx.full_side_effects[0][0].clone();
                let sfx_id = fsx
                    .input
                    .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                        &local_ctx.xtx_id.0,
                        0,
                    );

                let bid_id = bid.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>, Runtime>(sfx_id);

                local_ctx.full_side_effects[0][0].confirmed = Some(ConfirmedSideEffect {
                    err: None,
                    output: None,
                    inclusion_data: vec![0, 1, 2, 3],
                    executioner: BOB,
                    received_at: 1 as BlockNumber,
                    cost: None,
                });
                assert_eq!(SquareUp::<Runtime>::bind_bidders(&local_ctx), true);
                assert_eq!(SquareUp::<Runtime>::finalize(&local_ctx), true);

                assert_eq!(
                    <AccountManager as AccountManagerInterface<
                        AccountId,
                        Balance,
                        Hash,
                        BlockNumber,
                        AssetId,
                    >>::get_settlement(sfx_id),
                    Some(Settlement {
                        requester: ALICE,
                        recipient: BOB,
                        settlement_amount: 2,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor
                    })
                );
                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);
            });
    }

    const INITIAL_BALANCE: Balance = 10;

    #[test]
    fn square_up_finalize_reverts_xtx_if_some_fsx_are_unconfirmed() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, INITIAL_BALANCE);
                let _ = Balances::deposit_creating(&BOB, INITIAL_BALANCE);

                const REQUESTER: AccountId = ALICE;
                const EXECUTOR: AccountId = BOB;

                let mut local_ctx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                let bid = SFXBid {
                    amount: 2,
                    insurance: 1,
                    reserved_bond: None,
                    reward_asset_id: None,
                    executor: EXECUTOR,
                    requester: REQUESTER,
                };

                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                let fsx = local_ctx.full_side_effects[0][0].clone();
                let sfx_id = fsx
                    .input
                    .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
                        &local_ctx.xtx_id.0,
                        0,
                    );

                let bid_id = bid.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>, Runtime>(sfx_id);

                // assert_eq!(SquareUp::<Runtime>::bind_bidders(&local_ctx), true);
                assert_eq!(SquareUp::<Runtime>::finalize(&local_ctx), true);

                assert_eq!(
                    <AccountManager as AccountManagerInterface<
                        AccountId,
                        Balance,
                        Hash,
                        BlockNumber,
                        AssetId,
                    >>::get_settlement(sfx_id),
                    None,
                );
                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);

                assert_eq!(Balances::free_balance(&REQUESTER), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR), INITIAL_BALANCE - bid.insurance - bid.amount);
            });
    }
}
