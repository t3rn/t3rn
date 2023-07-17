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

use t3rn_primitives::circuit::{CircuitStatus, ReadSFX, SideEffect};

t3rn_primitives::reexport_currency_types!();

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct OrderStatusRead<Hash> {
    pub xtx_id: Hash,
    pub status: CircuitStatus,
    pub all_included_sfx: Vec<(Hash, CircuitStatus)>,
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
        OrderStatusRead(OrderStatusRead<T::Hash>),
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
            let status = T::ReadSFX::get_xtx_status(xtx_id)?;
            let sfx_of_xtx = T::ReadSFX::get_fsx_of_xtx(xtx_id)?;
            let mut all_included_sfx = sfx_of_xtx
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
            }));

            Ok(().into())
        }
    }
}

#[cfg(test)]
mod tests {
    use codec::Encode;
    use frame_support::{assert_ok, traits::Hooks};
    use hex_literal::hex;
    use sp_runtime::AccountId32;
    use t3rn_primitives::{clock::OnHookQueues, light_client::LightClientAsyncAPI};

    use t3rn_mini_mock_runtime::{
        prepare_ext_builder_playground, AccountId, Assets, Balance, Balances, BlockNumber, Circuit,
        CircuitError, CircuitEvent, Clock, Event, GlobalOnInitQueues, Hash, MiniRuntime,
        MockedAssetEvent, OrderStatusRead, Origin, Portal, Rewards, System, Vacuum, VacuumEvent,
        ASSET_DOT, POLKADOT_TARGET, XDNS,
    };

    use t3rn_primitives::{
        circuit::types::{OrderSFX, SFXAction},
        claimable::CircuitRole,
        monetary::TRN,
        SpeedMode, TreasuryAccount, TreasuryAccountProvider,
    };
    use t3rn_types::sfx::ConfirmedSideEffect;

    use frame_support::traits::Currency;
    use t3rn_abi::Abi::H256;
    use t3rn_primitives::{circuit::CircuitStatus, monetary::EXISTENTIAL_DEPOSIT};

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

        match expect_xtx_received.clone() {
            Some(event) => {
                let xtx_id = match event.event {
                    Event::Circuit(CircuitEvent::XTransactionReceivedForExec(xtx_id)) => xtx_id,
                    _ => panic!("expect_last_event_to_emit_xtx_id: unexpected event type"),
                };
                xtx_id
            },
            None => panic!("expect_last_event_to_emit_xtx_id: no last event emitted"),
        }
    }

    fn expect_last_event_to_read_order_status() -> OrderStatusRead<Hash> {
        // Recover system event
        let events = System::events();
        let expect_order_status_read = events.last();
        assert!(expect_order_status_read.clone().is_some());

        match expect_order_status_read.clone() {
            Some(event) => {
                let status = match &event.event {
                    Event::Vacuum(VacuumEvent::OrderStatusRead(status)) => status.clone(),
                    _ => panic!("expect_last_event_to_read_order_status: unexpected event type"),
                };
                status
            },
            None => panic!("expect_last_event_to_read_order_status: no last event emitted"),
        }
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
            };

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
            };

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
                to: requester_on_dest.clone(),
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

            assert_ok!(Vacuum::read_order_status(
                Origin::signed(requester.clone()),
                xtx_id
            ));

            let order_status = expect_last_event_to_read_order_status();

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::FinishedAllSteps,
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::FinishedAllSteps),],
                }
            );

            // Check executor's balance before claim - insurance amount should be returned
            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance
            );

            GlobalOnInitQueues::process_hourly(300, u64::MAX);

            // Claim via Rewards
            let claim_res = Rewards::claim(
                Origin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance + 200 as Balance
            );
        });
    }
}
