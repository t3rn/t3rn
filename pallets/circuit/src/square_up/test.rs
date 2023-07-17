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

    use crate::tests::ESCROW_ACCOUNT;
    use sp_runtime::DispatchResult;
    use t3rn_types::sfx::ConfirmedSideEffect;

    const REQUESTER: AccountId = ALICE;
    const EXECUTOR: AccountId = BOB;
    const INITIAL_BALANCE: Balance = 10;

    fn stage_single_sfx_xtx() -> (
        LocalXtxCtx<Runtime, Balance>,
        Hash,
        SFXBid<AccountId, Balance, AssetId>,
        Hash,
    ) {
        System::set_block_number(1);

        let _ = Balances::deposit_creating(&REQUESTER, INITIAL_BALANCE);
        let _ = Balances::deposit_creating(&EXECUTOR, INITIAL_BALANCE);

        let local_ctx =
            Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &REQUESTER, None).unwrap();

        let bid = SFXBid {
            amount: 2,
            insurance: 1,
            reserved_bond: None,
            reward_asset_id: None,
            executor: EXECUTOR,
            requester: REQUESTER,
        };
        let sfx_id = local_ctx.full_side_effects[0][0]
            .input
            .generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>>(
            &local_ctx.xtx_id.0,
            0,
        );
        let bid_id = bid.generate_id::<circuit_runtime_pallets::pallet_circuit::SystemHashing<Runtime>, Runtime>(sfx_id);
        (local_ctx, sfx_id, bid, bid_id)
    }

    fn request_and_bid_single_sfx_xtx(
        local_ctx: &mut LocalXtxCtx<Runtime, Balance>,
        bid: &SFXBid<AccountId, Balance, AssetId>,
    ) -> DispatchResult {
        assert_ok!(SquareUp::<Runtime>::try_request(local_ctx));
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
            bid,
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
                source: BenefitSource::TrafficFees,
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
                offered_reward: bid.insurance + bid.reserved_bond.unwrap_or(0),
                maybe_asset_id: bid.reward_asset_id,
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
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&REQUESTER, 3);

                let local_ctx =
                    Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE, None).unwrap();

                assert_ok!(SquareUp::<Runtime>::try_request(&local_ctx));
                assert_eq!(Balances::free_balance(&REQUESTER), 1);
            });
    }

    #[test]
    fn square_up_fails_lock_up_requester_without_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&REQUESTER, 1);
                let local_ctx =
                    Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &ALICE, None).unwrap();

                assert_err!(
                    SquareUp::<Runtime>::try_request(&local_ctx),
                    DispatchError::Module(ModuleError {
                        index: 10,
                        error: [2, 0, 0, 0],
                        message: Some("InsufficientBalance")
                    })
                );

                // Balance stays unchanged
                assert_eq!(Balances::free_balance(&REQUESTER), 1);
            });
    }

    #[test]
    fn square_up_locks_up_first_bidder_with_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&REQUESTER, 10);
                let _ = Balances::deposit_creating(&EXECUTOR, 10);
                let local_ctx =
                    Machine::<Runtime>::setup(&[get_mocked_transfer_sfx()], &REQUESTER, None)
                        .unwrap();
                assert_ok!(SquareUp::<Runtime>::try_request(&local_ctx));

                assert_ok!(SquareUp::<Runtime>::try_bid(
                    get_mocked_transfer_sfx_id(local_ctx.xtx_id),
                    &REQUESTER,
                    &EXECUTOR,
                    &SFXBid {
                        amount: get_mocked_transfer_sfx().max_reward,
                        insurance: get_mocked_transfer_sfx().insurance,
                        reserved_bond: None,
                        reward_asset_id: None,
                        executor: EXECUTOR,
                        requester: REQUESTER,
                    },
                    None
                ));

                assert_eq!(
                    Balances::free_balance(&REQUESTER),
                    10 - get_mocked_transfer_sfx().max_reward
                );
                // only reserve the insurance
                assert_eq!(
                    Balances::free_balance(&EXECUTOR),
                    10 - get_mocked_transfer_sfx().insurance
                );
            });
    }

    #[test]
    fn square_up_kills_xtx_with_its_all_bids() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let (mut local_ctx, sfx_id, bid, bid_id) = stage_single_sfx_xtx();

                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                assert!(SquareUp::<Runtime>::kill(&local_ctx));

                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);

                assert_eq!(Balances::free_balance(&REQUESTER), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR), INITIAL_BALANCE);
            });
    }

    #[test]
    fn square_up_kills_xtx_with_its_all_bids_even_when_executor_assigned_to_request_charge() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let (mut local_ctx, sfx_id, bid, bid_id) = stage_single_sfx_xtx();

                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));
                assert!(SquareUp::<Runtime>::bind_bidders(&mut local_ctx));
                assert!(SquareUp::<Runtime>::kill(&local_ctx));

                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);

                assert_eq!(Balances::free_balance(&REQUESTER), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&EXECUTOR), INITIAL_BALANCE);
            });
    }

    #[test]
    fn square_up_finalizes_and_successfully_commits_finished_fsx() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let (mut local_ctx, sfx_id, bid, bid_id) = stage_single_sfx_xtx();

                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                local_ctx.full_side_effects[0][0].confirmed = Some(ConfirmedSideEffect {
                    err: None,
                    output: None,
                    inclusion_data: vec![0, 1, 2, 3],
                    executioner: EXECUTOR,
                    received_at: 1 as BlockNumber,
                    cost: None,
                });

                assert!(SquareUp::<Runtime>::bind_bidders(&mut local_ctx));
                assert!(SquareUp::<Runtime>::finalize(&local_ctx));

                assert_eq!(
                    <AccountManager as AccountManagerInterface<
                        AccountId,
                        Balance,
                        Hash,
                        BlockNumber,
                        AssetId,
                    >>::get_settlement(sfx_id),
                    Some(Settlement {
                        requester: REQUESTER,
                        recipient: EXECUTOR,
                        settlement_amount: bid.amount,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficFees,
                        role: CircuitRole::Executor,
                        maybe_asset_id: None,
                    })
                );
                assert_pending_charges_no_longer_exist(vec![sfx_id, bid_id]);

                assert_eq!(
                    Balances::free_balance(&REQUESTER),
                    INITIAL_BALANCE - bid.amount
                );
                assert_eq!(Balances::free_balance(&EXECUTOR), INITIAL_BALANCE);
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), 0);
            });
    }

    #[test]
    fn square_up_finalize_reverts_xtx_if_some_fsx_are_unconfirmed() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let (mut local_ctx, sfx_id, bid, bid_id) = stage_single_sfx_xtx();
                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                assert!(SquareUp::<Runtime>::bind_bidders(&mut local_ctx));
                assert!(SquareUp::<Runtime>::finalize(&local_ctx));

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

                // Requester has its balance restored
                assert_eq!(Balances::free_balance(&REQUESTER), INITIAL_BALANCE);
                // Executor is slashed
                assert_eq!(
                    Balances::free_balance(&EXECUTOR),
                    INITIAL_BALANCE - bid.insurance
                );
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), bid.insurance);
            });
    }

    #[test]
    fn square_up_finalize_slashes_bidder_even_if_unassigned_to_request_charge() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let (mut local_ctx, sfx_id, bid, bid_id) = stage_single_sfx_xtx();
                assert_ok!(request_and_bid_single_sfx_xtx(&mut local_ctx, &bid));

                assert!(SquareUp::<Runtime>::finalize(&local_ctx));

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

                // Requester has its balance restored
                assert_eq!(Balances::free_balance(&REQUESTER), INITIAL_BALANCE);
                // Executor is slashed
                assert_eq!(
                    Balances::free_balance(&EXECUTOR),
                    INITIAL_BALANCE - bid.insurance
                );
                assert_eq!(Balances::free_balance(&ESCROW_ACCOUNT), bid.insurance);
            });
    }
}
