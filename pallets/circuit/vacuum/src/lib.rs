#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
};

use frame_system::pallet_prelude::*;
pub use pallet::*;

use sp_std::{convert::TryInto, vec::Vec};
use t3rn_primitives::{
    circuit::{traits::CircuitSubmitAPI, types::OrderSFX},
    SpeedMode,
};
pub type Asset = u32;
pub type Destination = [u8; 4];
pub type Input = Vec<u8>;
use scale_info::TypeInfo;

use t3rn_primitives::circuit::{AdaptiveTimeout, CircuitStatus, ReadSFX, SideEffect};
use t3rn_types::sfx::TargetId;

t3rn_primitives::reexport_currency_types!();

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct OrderStatusRead<Hash, BlockNumber> {
    pub xtx_id: Hash,
    pub status: CircuitStatus,
    pub all_included_sfx: Vec<(Hash, CircuitStatus)>,
    pub timeouts_at: AdaptiveTimeout<BlockNumber, TargetId>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type CircuitSubmitAPI: CircuitSubmitAPI<Self, BalanceOf<Self>>;
        type ReadSFX: ReadSFX<Self::Hash, Self::AccountId, BalanceOf<Self>, Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        OrderStatusRead(OrderStatusRead<T::Hash, T::BlockNumber>),
    }

    #[pallet::error]
    pub enum Error<T> {
        // Define your errors here
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn order(
            origin: OriginFor<T>,
            sfx_actions: Vec<
                OrderSFX<T::AccountId, Asset, BalanceOf<T>, Destination, Input, BalanceOf<T>>,
            >,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            let side_effects: Vec<SideEffect<T::AccountId, BalanceOf<T>>> = sfx_actions
                .into_iter()
                .map(|sfx_action| sfx_action.try_into())
                .collect::<Result<Vec<SideEffect<T::AccountId, BalanceOf<T>>>, DispatchError>>()?;

            T::CircuitSubmitAPI::on_extrinsic_trigger(origin, side_effects, speed_mode)?;

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn read_order_status(
            _origin: OriginFor<T>,
            xtx_id: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let (status, timeouts_at) = T::ReadSFX::get_xtx_status(xtx_id)?;
            let sfx_of_xtx = T::ReadSFX::get_fsx_of_xtx(xtx_id)?;
            let all_included_sfx = sfx_of_xtx
                .into_iter()
                .map(|sfx| {
                    let fsx_status = T::ReadSFX::get_fsx_status(sfx)?;
                    Ok((sfx, fsx_status))
                })
                .collect::<Result<Vec<(T::Hash, CircuitStatus)>, DispatchError>>()?;

            Self::deposit_event(Event::OrderStatusRead(OrderStatusRead {
                xtx_id,
                status,
                all_included_sfx,
                timeouts_at,
            }));

            Ok(().into())
        }
    }
}

#[cfg(test)]
mod tests {
    use codec::Encode;
    use frame_support::{assert_err, assert_ok, traits::Hooks};
    use hex_literal::hex;
    use sp_runtime::AccountId32;
    use t3rn_primitives::{clock::OnHookQueues, light_client::LightClientAsyncAPI};

    use t3rn_mini_mock_runtime::{
        prepare_ext_builder_playground, AccountId, Assets, Balance, Balances, BlockNumber, Circuit,
        CircuitError, CircuitEvent, Clock, Event, GlobalOnInitQueues, Hash, MiniRuntime,
        MockedAssetEvent, OrderStatusRead, Origin, Portal, Rewards, System, Vacuum, VacuumEvent,
        ASSET_DOT, POLKADOT_TARGET, XDNS,
    };
    use t3rn_primitives::portal::Portal as PortalT;

    use t3rn_primitives::{
        circuit::types::{OrderSFX, SFXAction},
        claimable::CircuitRole,
        monetary::TRN,
        GatewayVendor, SpeedMode, TreasuryAccount, TreasuryAccountProvider,
    };
    use t3rn_types::sfx::ConfirmedSideEffect;

    use frame_support::traits::Currency;

    use t3rn_primitives::{
        circuit::{AdaptiveTimeout, CircuitStatus},
        monetary::EXISTENTIAL_DEPOSIT,
    };
    use t3rn_types::fsx::TargetId;

    fn activate_all_light_clients() {
        for &gateway in XDNS::all_gateway_ids().iter() {
            Portal::turn_on(Origin::root(), gateway).unwrap();
        }
        XDNS::process_all_verifier_overviews(System::block_number());
        XDNS::process_overview(System::block_number());
    }

    fn mint_required_assets_for_optimistic_actors(
        requester: AccountId,
        executor: AccountId,
        max_reward: Balance,
        insurance: Balance,
    ) {
        assert!(XDNS::all_token_ids().contains(&ASSET_DOT));
        // Load requester enough some funds
        let issuer_is_escrow_account = MiniRuntime::get_treasury_account(TreasuryAccount::Escrow);
        Balances::deposit_creating(&requester, (100_000 * TRN) as Balance); // To cover fees
        Balances::deposit_creating(&executor, (100_000 * TRN) as Balance); // To cover fees
        assert_ok!(Assets::mint(
            Origin::signed(issuer_is_escrow_account.clone()),
            ASSET_DOT,
            requester.clone(),
            max_reward + (EXISTENTIAL_DEPOSIT as Balance),
        ));
        assert_ok!(Assets::mint(
            Origin::signed(issuer_is_escrow_account),
            ASSET_DOT,
            executor.clone(),
            insurance + (EXISTENTIAL_DEPOSIT as Balance),
        ));
        assert_eq!(
            Assets::balance(ASSET_DOT, &requester),
            max_reward + (EXISTENTIAL_DEPOSIT as Balance)
        );
        assert_eq!(
            Assets::balance(ASSET_DOT, &executor),
            insurance + (EXISTENTIAL_DEPOSIT as Balance)
        );
    }

    fn expect_last_event_to_emit_xtx_id() -> Hash {
        // Recover system event
        let events = System::events();
        let expect_xtx_received = events.last();
        assert!(expect_xtx_received.clone().is_some());

        match expect_xtx_received {
            Some(event) => match event.event {
                Event::Circuit(CircuitEvent::XTransactionReceivedForExec(xtx_id)) => xtx_id,
                _ => panic!("expect_last_event_to_emit_xtx_id: unexpected event type"),
            },
            None => panic!("expect_last_event_to_emit_xtx_id: no last event emitted"),
        }
    }

    fn expect_last_event_to_read_order_status() -> OrderStatusRead<Hash, BlockNumber> {
        // Recover system event
        let events = System::events();
        let expect_order_status_read = events.last();
        assert!(expect_order_status_read.clone().is_some());

        match expect_order_status_read {
            Some(event) => match &event.event {
                Event::Vacuum(VacuumEvent::OrderStatusRead(status)) => status.clone(),
                _ => panic!("expect_last_event_to_read_order_status: unexpected event type"),
            },
            None => panic!("expect_last_event_to_read_order_status: no last event emitted"),
        }
    }

    fn prepare_transfer_asset_confirmation(
        asset_id: u32,
        executor: AccountId,
        destination: AccountId,
        amount: Balance,
    ) -> ConfirmedSideEffect<AccountId32, BlockNumber, Balance> {
        let mut scale_encoded_transfer_event = MockedAssetEvent::<MiniRuntime>::Transferred {
            asset_id,
            from: executor.clone(),
            to: destination,
            amount,
        }
        .encode();
        // append an extra pallet event index byte as the second byte
        scale_encoded_transfer_event.insert(0, 4u8);

        ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
            err: None,
            output: None,
            inclusion_data: scale_encoded_transfer_event,
            executioner: executor,
            received_at: System::block_number(),
            cost: None,
        }
    }

    fn mock_signal_halt(_target: TargetId, verifier: GatewayVendor) {
        let mut current_heartbeat = Portal::get_latest_heartbeat(&POLKADOT_TARGET).unwrap();
        current_heartbeat.is_halted = true;
        let current_epoch_does_not_move = current_heartbeat.last_finalized_height;
        // advance 1 epoch
        System::set_block_number(System::block_number() + 32);
        XDNS::on_new_epoch(verifier, current_epoch_does_not_move, current_heartbeat);
    }

    fn mock_signal_unhalt(_target: TargetId, verifier: GatewayVendor) {
        let mut current_heartbeat = Portal::get_latest_heartbeat(&POLKADOT_TARGET).unwrap();
        current_heartbeat.is_halted = false;
        current_heartbeat.last_finalized_height += 1;
        let current_epoch_moves = current_heartbeat.last_finalized_height + 1;
        // advance 1 epoch
        System::set_block_number(System::block_number() + 32);
        XDNS::on_new_epoch(verifier, current_epoch_moves, current_heartbeat);
    }

    #[test]
    fn optimistic_order_single_sfx_vacuum_delivers_to_circuit() {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from([3u8; 32]);

            mint_required_assets_for_optimistic_actors(
                requester.clone(),
                executor,
                200u128,
                50u128,
            );

            let sfx_action = SFXAction::Transfer(POLKADOT_TARGET, 1u32, requester_on_dest, 100u128);
            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: ASSET_DOT,
                remote_origin_nonce: None,
            };

            activate_all_light_clients();

            assert_ok!(Vacuum::order(
                Origin::signed(requester.clone()),
                vec![sfx_order],
                SpeedMode::Fast,
            ));

            let xtx_id = expect_last_event_to_emit_xtx_id();

            assert_eq!(
                xtx_id,
                Hash::from(hex!(
                    "0162cabd6f37c15015e94be4174f7ad95fa0d6f094da6aea5525ce11731308a1"
                ))
            );

            // Expect balance of requester to be reduced by max_reward
            assert_eq!(
                Assets::balance(ASSET_DOT, &requester),
                EXISTENTIAL_DEPOSIT as Balance
            );
        });
    }

    #[test]
    fn optimistic_order_single_sfx_vacuum_delivers_to_circuit_and_rewards_executor_at_successful_confirm(
    ) {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from([3u8; 32]);

            mint_required_assets_for_optimistic_actors(
                requester.clone(),
                executor.clone(),
                200u128,
                50u128,
            );

            let sfx_action = SFXAction::Transfer(
                POLKADOT_TARGET,
                ASSET_DOT,
                requester_on_dest.clone(),
                100u128,
            );
            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: ASSET_DOT,
                remote_origin_nonce: None,
            };

            activate_all_light_clients();

            assert_ok!(Vacuum::order(
                Origin::signed(requester.clone()),
                vec![sfx_order],
                SpeedMode::Fast,
            ));

            let xtx_id = expect_last_event_to_emit_xtx_id();

            assert_eq!(
                xtx_id,
                Hash::from(hex!(
                    "0162cabd6f37c15015e94be4174f7ad95fa0d6f094da6aea5525ce11731308a1"
                ))
            );

            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));

            let order_status = expect_last_event_to_read_order_status();

            let expected_sfx_hash = Hash::from(hex!(
                "6fd0ce38a35bcc001dc78cbe7b258dd71cca7dff301891e13b73598572908744"
            ));

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::PendingBidding,
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::PendingBidding)],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 817,
                        estimated_height_there: 824,
                        submit_by_height_here: 417,
                        submit_by_height_there: 424,
                        emergency_timeout_here: 417,
                        there: [1, 1, 1, 1],
                        dlq: None
                    },
                }
            );

            assert_ok!(Circuit::bid_sfx(
                Origin::signed(executor.clone()),
                expected_sfx_hash,
                198 as Balance,
            ));

            // Assert executor has insurance amount locked
            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance
            );

            // Complete bidding
            System::set_block_number(System::block_number() + 3);
            Clock::on_initialize(System::block_number());
            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));
            assert_eq!(
                expect_last_event_to_read_order_status().status,
                CircuitStatus::Ready
            );

            let mut scale_encoded_transfer_event = MockedAssetEvent::<MiniRuntime>::Transferred {
                asset_id: ASSET_DOT,
                from: executor.clone(),
                to: requester_on_dest,
                amount: 100 as Balance,
            }
            .encode();
            // append an extra pallet event index byte as the second byte
            scale_encoded_transfer_event.insert(0, 4u8);

            // Confirm
            let confirmation_transfer_1 = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                inclusion_data: scale_encoded_transfer_event,
                executioner: executor.clone(),
                received_at: System::block_number(),
                cost: None,
            };

            assert_ok!(Circuit::confirm_side_effect(
                Origin::signed(executor.clone()),
                expected_sfx_hash,
                confirmation_transfer_1
            ));

            assert_ok!(Vacuum::read_order_status(Origin::signed(requester), xtx_id));

            let order_status = expect_last_event_to_read_order_status();

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::FinishedAllSteps,
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::FinishedAllSteps),],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 817,
                        estimated_height_there: 824,
                        submit_by_height_here: 417,
                        submit_by_height_there: 424,
                        emergency_timeout_here: 417,
                        there: [1, 1, 1, 1],
                        dlq: None
                    },
                }
            );

            // Check executor's balance before claim - insurance amount should be returned
            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance
            );

            GlobalOnInitQueues::process_hourly(300, u64::MAX);

            // Claim via Rewards
            let _claim_res = Rewards::claim(
                Origin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance + 200 as Balance
            );
        });
    }

    #[test]
    fn optimistic_order_single_sfx_vacuum_delivers_to_circuit_and_handles_potential_delays_via_dlq_eventually(
    ) {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from([3u8; 32]);

            mint_required_assets_for_optimistic_actors(
                requester.clone(),
                executor.clone(),
                200u128,
                50u128,
            );

            let sfx_action = SFXAction::Transfer(
                POLKADOT_TARGET,
                ASSET_DOT,
                requester_on_dest.clone(),
                100u128,
            );
            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: ASSET_DOT,
                remote_origin_nonce: None,
            };

            activate_all_light_clients();

            assert_ok!(Vacuum::order(
                Origin::signed(requester.clone()),
                vec![sfx_order],
                SpeedMode::Fast,
            ));

            let xtx_id = expect_last_event_to_emit_xtx_id();

            assert_eq!(
                xtx_id,
                Hash::from(hex!(
                    "a602173a905f72f4f93410c69db65f52480f67b7e947309b254fe718f611a0a7"
                ))
            );

            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));

            let order_status = expect_last_event_to_read_order_status();

            let expected_sfx_hash = Hash::from(hex!(
                "484c277dfcfb25b51c8e12fc2e7eb286bb9315775db60635872d51a40d5bb253"
            ));

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::PendingBidding,
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::PendingBidding)],
                    timeouts_at: AdaptiveTimeout {
                        estimated_height_here: 817,
                        estimated_height_there: 824,
                        submit_by_height_here: 417,
                        submit_by_height_there: 424,
                        emergency_timeout_here: 417,
                        there: [1, 1, 1, 1],
                        dlq: None
                    },
                }
            );

            assert_ok!(Circuit::bid_sfx(
                Origin::signed(executor.clone()),
                expected_sfx_hash,
                198 as Balance,
            ));

            // Complete bidding
            System::set_block_number(System::block_number() + 3);
            Clock::on_initialize(System::block_number());
            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));
            assert_eq!(
                expect_last_event_to_read_order_status().status,
                CircuitStatus::Ready
            );

            // Here interrupt the LightClient availability and move Xtx to DLQ
            mock_signal_halt(POLKADOT_TARGET, GatewayVendor::Polkadot);

            let confirmation_transfer = prepare_transfer_asset_confirmation(
                ASSET_DOT,
                executor.clone(),
                requester_on_dest.clone(),
                100u128,
            );

            assert_err!(
                Circuit::confirm_side_effect(
                    Origin::signed(executor.clone()),
                    expected_sfx_hash,
                    confirmation_transfer
                ),
                CircuitError::<MiniRuntime>::ConfirmationFailed
            );

            // Wait for after XTX timeout
            System::set_block_number(System::block_number() + 401);
            // Trigger XTX revert queue and expect move to DLQ
            Circuit::process_emergency_revert_xtx_queue(
                System::block_number(),
                System::block_number(),
                u64::MAX,
            );
            // Verify that XTX is in DLQ
            assert_eq!(
                Circuit::get_dlq(xtx_id),
                Some((
                    System::block_number(),
                    vec![POLKADOT_TARGET],
                    SpeedMode::Finalized
                ))
            );

            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));

            let order_status = expect_last_event_to_read_order_status();

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::Ready,
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::Ready),],
                    timeouts_at: AdaptiveTimeout {
                        estimated_height_here: 817,
                        estimated_height_there: 824,
                        submit_by_height_here: 417,
                        submit_by_height_there: 424,
                        emergency_timeout_here: 417,
                        there: [1, 1, 1, 1],
                        dlq: Some(453)
                    },
                }
            );

            // Now activate the LightClient again and expect the DLQ to be processed
            mock_signal_unhalt(POLKADOT_TARGET, GatewayVendor::Polkadot);

            // Advance 1 block
            System::set_block_number(System::block_number() + 1);
            // Try to confirm again
            let confirmation_transfer = prepare_transfer_asset_confirmation(
                ASSET_DOT,
                executor.clone(),
                requester_on_dest,
                100u128,
            );

            assert_ok!(Circuit::confirm_side_effect(
                Origin::signed(executor),
                expected_sfx_hash,
                confirmation_transfer
            ),);

            assert_ok!(Vacuum::read_order_status(Origin::signed(requester), xtx_id));

            assert_eq!(Circuit::get_dlq(xtx_id), None);
        });
    }
}
