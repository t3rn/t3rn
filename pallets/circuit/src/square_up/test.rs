#[cfg(test)]
pub mod test {
    use crate::{
        square_up::test_extra::*,
        tests::{ALICE, BOB},
        SFXBid,
    };
    use sp_runtime::{DispatchError, ModuleError};

    use circuit_mock_runtime::{Balances, ExtBuilder, Runtime, System};
    use circuit_runtime_pallets::pallet_circuit::{machine::Machine, square_up::SquareUp};
    use frame_support::{assert_err, assert_ok, traits::Currency};
    use sp_core::H256;

    #[test]
    fn square_up_locks_up_requester_with_enough_native_currency() {
        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                System::set_block_number(1);

                let _ = Balances::deposit_creating(&ALICE, 3);
                let local_xtx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                assert_ok!(SquareUp::<Runtime>::try_request(&local_xtx));

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
                let local_xtx =
                    Machine::<Runtime>::load_xtx(setup_single_sfx_xtx_and_force_set_status(None))
                        .unwrap();

                assert_err!(
                    SquareUp::<Runtime>::try_request(&local_xtx),
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
                let _local_xtx =
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
}
