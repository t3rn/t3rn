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
use sp_core::{crypto::AccountId32, hexdisplay::AsBytesRef, H160, U256};
use t3rn_abi::{
    recode::recode_bytes_with_descriptor, sfx_abi::PerCodecAbiDescriptors, Codec, FilledAbi, SFXAbi,
};
use t3rn_primitives::{
    circuit::{
        AdaptiveTimeout, CircuitStatus, OrderOrigin, ReadSFX, SFXAction, SecurityLvl, SideEffect,
    },
    xdns::Xdns,
};
use t3rn_types::sfx::TargetId;

t3rn_primitives::reexport_currency_types!();

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct OrderStatusRead<Hash, BlockNumber> {
    pub xtx_id: Hash,
    pub status: CircuitStatus,
    pub all_included_sfx: Vec<(Hash, CircuitStatus)>,
    pub timeouts_at: AdaptiveTimeout<BlockNumber, TargetId>,
}

// emit OrderCreated(id, destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward, nonce);
// event RemoteOrderIndexedCreated(bytes32 indexed id, uint32 indexed nonce, address indexed sender, bytes input);
// where input = abi.encode(destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward)
pub fn get_remote_order_abi_descriptor() -> Vec<u8> {
    b"RemoteOrderIndexed:Log(sfxId+:Bytes32,nonce:Value32,sender+:Account20,destination:Bytes4,asset:Bytes4,targetAccount:Account32,amount:Value256,rewardAsset:Account20,insurance:Value256,maxReward:Value256)".to_vec()
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct RemoteEVMOrderLocalized<T: Config> {
    pub from: H160,
    pub destination: TargetId,
    pub asset: u32,
    pub target_account: AccountId32,
    pub reward_asset: H160,
    pub amount: U256,
    pub insurance: U256,
    pub max_reward: U256,
    pub nonce: u32,
    phantom: PhantomData<T>,
}

impl<AccountId, Balance, T: Config> TryInto<SideEffect<AccountId, Balance>>
    for RemoteEVMOrderLocalized<T>
where
    u32: From<Asset>,
    Balance: Encode + Decode,
    AccountId: Encode,
    Input: AsBytesRef,
    Destination: From<[u8; 4]>,
    [u8; 4]: From<Destination>,
{
    type Error = DispatchError;

    fn try_into(self) -> Result<SideEffect<AccountId, Balance>, Self::Error> {
        let mut encoded_args: Vec<Vec<u8>> = vec![];

        encoded_args.push(
            <Asset as Into<u32>>::into(self.asset)
                .to_le_bytes()
                .to_vec(),
        );
        encoded_args.push(self.target_account.encode());
        encoded_args.push(self.amount.as_u128().encode());

        let max_reward = Balance::decode(&mut &self.max_reward.as_u128().encode()[..])
            .map_err(|_| DispatchError::Other("Failed to decode max_reward from remote order"))?;

        let insurance = Balance::decode(&mut &self.insurance.as_u128().encode()[..])
            .map_err(|_| DispatchError::Other("Failed to decode insurance from remote order"))?;

        let side_effect = SideEffect {
            target: self.destination,
            max_reward,
            insurance,
            action: *b"tass",
            encoded_args,
            signature: vec![],
            enforce_executor: None,
            reward_asset_id: Some(
                T::Xdns::get_token_by_eth_address(self.destination, self.reward_asset)?.token_id,
            ),
        };

        Ok(side_effect)
    }
}
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use t3rn_primitives::{
        xdns::{TokenRecord, Xdns},
        TokenInfo,
    };

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        type CircuitSubmitAPI: CircuitSubmitAPI<Self, BalanceOf<Self>>;
        type Xdns: Xdns<Self, BalanceOf<Self>>;
        type ReadSFX: ReadSFX<Self::Hash, Self::AccountId, BalanceOf<Self>, BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        OrderStatusRead(OrderStatusRead<T::Hash, BlockNumberFor<T>>),
    }

    #[pallet::error]
    pub enum Error<T> {
        // Define your errors here
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100_000)]
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

            T::CircuitSubmitAPI::on_extrinsic_trigger(
                origin,
                side_effects,
                speed_mode,
                SecurityLvl::Optimistic,
            )?;

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub fn remote_order(
            origin: OriginFor<T>,
            order_remote_proof: Vec<u8>,
            remote_target_id: TargetId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin.clone())?;
            // Assume finalized speed mode to remote order operations to avoid disputes in events of chain reorgs.
            let speed_mode = SpeedMode::Finalized;

            let verified_event_bytes = T::CircuitSubmitAPI::verify_sfx_proof(
                remote_target_id,
                speed_mode.clone(),
                Some([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 113, 105, 211, 136, 32, 223, 209, 23, 195,
                    250, 31, 34, 166, 151, 219, 165, 141, 144, 186, 6,
                ]), // replace with proper mapper to remoteOrder contract's address
                order_remote_proof,
            )?
            .message;

            let recoded_message = recode_bytes_with_descriptor(
                verified_event_bytes,
                get_remote_order_abi_descriptor(),
                Codec::Rlp,
                Codec::Scale,
            )?;

            let decoded_remote_order: RemoteEVMOrderLocalized<T> =
                RemoteEVMOrderLocalized::decode(&mut recoded_message.as_slice()).map_err(|e| {
                    log::error!(
                        "Vecuum::remote_order -- error decoding remote order: {:?}",
                        e
                    );
                    DispatchError::Other("Vecuum::remote_order -- error decoding remote order")
                })?;

            let mut side_effect: SideEffect<T::AccountId, BalanceOf<T>> =
                decoded_remote_order.clone().try_into()?;

            let remote_origin: OrderOrigin<T::AccountId> =
                OrderOrigin::from_remote_nonce(decoded_remote_order.nonce);

            // Based on target and asset, derive the intent of the order.
            // If the target is a remote chain, then the intent is to transfer funds to the remote chain. We don't need to go into details of assets order transfers or swaps.
            // If target is a local chain (t3rn) && asset used for reward payout equals the asset transferred on remote chain - assume bridge operation of wrapped assets.
            if &decoded_remote_order.destination == &[3, 3, 3, 3]
                && T::Xdns::list_available_mint_assets(remote_target_id)
                    .iter()
                    .any(|asset: &TokenRecord| {
                        &asset.token_id == &decoded_remote_order.asset
                            && match &asset.token_props {
                                TokenInfo::Ethereum(info) =>
                                    info.address == Some(decoded_remote_order.reward_asset.into()),
                                _ => false,
                            }
                    })
            {

                // ToDo: assess whether we need to undergo the whole SFX confirmaiton process or can just confirm the side effect, mint and divide assets between user and executor

                // Mint wrapped assets on local chain.
                let amount = decoded_remote_order.amount.as_u128();
                // In the context of minting assets, max reward is the net reward to executor.
                let max_reward = decoded_remote_order.max_reward.as_u128();
                assert!(amount > max_reward, "Vecuum::remote_order -- amount must be greater than max_reward for minting assets");

                // T::Xdns::mint_asset(decoded_remote_order.asset, amount, Origin::signed(T::TreasurAccounts::get_account(Treasury::Escrow())))?;
                // side_effect.enforce_executor = Some(who.clone());

                // Try bridge wrap assets.
                //  T::CircuitSubmitAPI::confirm()
            }

            // For remote order + remote reward, assume on_remote_origin_trigger
            T::CircuitSubmitAPI::on_remote_origin_trigger(
                origin.clone(),
                remote_origin.to_account_id(),
                vec![side_effect],
                speed_mode,
            )
            //
        }

        #[pallet::weight(100_000)]
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
    use codec::{Decode, Encode};
    use frame_support::{assert_err, assert_ok, traits::Hooks};
    use hex_literal::hex;
    use sp_runtime::AccountId32;
    use t3rn_mini_mock_runtime::{
        hotswap_latest_receipt_header_root, initialize_eth2_with_3rd_epoch,
        prepare_ext_builder_playground, AccountId, Assets, Balance, Balances, BlockNumber, Circuit,
        CircuitError, CircuitEvent, Clock, EthereumEventInclusionProof, GlobalOnInitQueues, Hash,
        MiniRuntime, MockedAssetEvent, OrderStatusRead, Portal, Rewards, RuntimeEvent as Event,
        RuntimeOrigin, System, Vacuum, VacuumEvent, ASSET_DOT, ASSET_USDT, ETHEREUM_TARGET,
        POLKADOT_TARGET, XDNS,
    };
    use t3rn_primitives::{
        circuit::CircuitSubmitAPI, clock::OnHookQueues, light_client::LightClientAsyncAPI,
        portal::Portal as PortalT, ExecutionSource,
    };

    use t3rn_primitives::{
        circuit::types::{OrderSFX, SFXAction},
        claimable::CircuitRole,
        monetary::TRN,
        GatewayVendor, SpeedMode, TreasuryAccount, TreasuryAccountProvider,
    };
    use t3rn_types::sfx::ConfirmedSideEffect;

    use frame_support::{traits::Currency, weights::Weight};

    use t3rn_primitives::{
        circuit::{AdaptiveTimeout, CircuitStatus},
        monetary::EXISTENTIAL_DEPOSIT,
    };
    use t3rn_types::fsx::TargetId;

    fn activate_all_light_clients() {
        for &gateway in XDNS::all_gateway_ids().iter() {
            println!("Activating light client for gateway: {:?}", gateway);
            Portal::turn_on(RuntimeOrigin::root(), gateway).unwrap();
        }
        XDNS::process_all_verifier_overviews(System::block_number());
        XDNS::process_overview(System::block_number());
    }

    fn mint_required_assets_for_optimistic_actors(
        requester: AccountId,
        executor: AccountId,
        max_reward: Balance,
        insurance: Balance,
        asset_id: u32,
    ) {
        assert!(XDNS::all_token_ids().contains(&asset_id));
        // Load requester enough some funds
        let issuer_is_escrow_account = MiniRuntime::get_treasury_account(TreasuryAccount::Escrow);
        Balances::deposit_creating(&requester, (100_000 * TRN) as Balance); // To cover fees
        Balances::deposit_creating(&executor, (100_000 * TRN) as Balance); // To cover fees
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(issuer_is_escrow_account.clone()),
            asset_id,
            requester.clone(),
            max_reward + (EXISTENTIAL_DEPOSIT as Balance),
        ));
        assert_ok!(Assets::mint(
            RuntimeOrigin::signed(issuer_is_escrow_account),
            asset_id,
            executor.clone(),
            insurance + (EXISTENTIAL_DEPOSIT as Balance),
        ));
        assert_eq!(
            Assets::balance(asset_id, &requester),
            max_reward + (EXISTENTIAL_DEPOSIT as Balance)
        );
        assert_eq!(
            Assets::balance(asset_id, &executor),
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
                ASSET_DOT,
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
                RuntimeOrigin::signed(requester.clone()),
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
                ASSET_DOT,
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(executor.clone()),
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(executor.clone()),
                expected_sfx_hash,
                confirmation_transfer_1
            ));

            assert_ok!(Vacuum::read_order_status(
                RuntimeOrigin::signed(requester),
                xtx_id
            ));

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

            GlobalOnInitQueues::process_hourly(300, Weight::MAX);

            // Claim via Rewards
            let _claim_res = Rewards::claim(
                RuntimeOrigin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance + 200 as Balance
            );
        });
    }

    #[test]
    fn optimistic_order_single_sfx_vacuum_delivers_to_circuit_and_confirms_for_eth_targets() {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            activate_all_light_clients();
            initialize_eth2_with_3rd_epoch();

            // Derive all arguments out of below proof sourced with eth2-proof client-side library
            // ⬅️found receipt for tx:  0xf714d9b6c4634af669d7d221d1867f7c5ca2f75ebea1f7c3ace9b4a3cb7fa1be
            // 🔃parsed receipt to hex form
            // ⬅️found block for receipt:  0x1b628d03b6b26de7caa76063fa0828e08af08a9b57eec97910060d2a29f2cd3b 4249964n
            // ⬅️fetched all 153 sibling transaction receipts
            // Computed Root:  2b8a7400f4aab71416fa71c5ba414ac58082ee06efcb24790c1d41f3e2b92da0
            // 🧮proof-calculated receipts root vs block receipts root:  0x2b8a7400f4aab71416fa71c5ba414ac58082ee06efcb24790c1d41f3e2b92da0 0x2b8a7400f4aab71416fa71c5ba414ac58082ee06efcb24790c1d41f3e2b92da0
            // {
            //   proof: [
            //     'f90131a0cd835eed90dc62ce38fc629e51135f41e4d6aa8297eb209e2a0e0671018671fda0a533ebcc27f0346239b90d9625797a4d3027c2dafd1e8e45bc70def13e849d52a0bb6981a076cb6757607b849c003cd09734faa5e0a9d619b22868790b213afc41a0110af21e6f78640e0823ddffa343c8030c872d9ee8883be2344b0c26c155766aa0182ff8dd6cde4c456052afc52216945e410c4ed63e05d08014ff61082d91f137a095885cd002c84f0d2922ab930206956f8519d80a0a04b6fab93ba3b77742aa64a0abdcdaa69bcad2dca0e92bade6cd518bea0ead479d3c4f96206c4903904a1f60a0751dfe96598cd3661e0d427b6ee1dc4f95a788844f513a52425e55317d9b0254a0d896334d2ddb4cc6da360fb18cf372f7e79de3b58478fc87ba2a810918e3e0e08080808080808080',
            //     'f90211a0a8e2160c776bcc991c1d50762cd9b38585617a1f206439d9e5e335c7bdfb084fa0da54f1b4b7ea617a9cd2d0accd4f646ed34aa2183b664fe96619e9af8b3fbf7aa00bda5ec2920c3476af307c1dda1e43c46edab24ecf62f6f850012ac284889f5fa04006fc7e35fd3890759225c74fb71084ca7e08bd1b4fc5f614e739d56b13e204a0b19ef750dfe7f91c7ea477072dc15ab7b4442e526bbff97914f5733a835529f2a0efcefc132e0fedf33b7167c6a554e434b44c35056ba5156251ed4e3622f3fb53a0d4935577ba2db06a0eeadccd29c6c1daf5d666f93a5162538beb9777b3fd13b4a08fcdd95b21e31e82a7309c3a48387c95fd7c58fd2838f2c3e7322f8afdcf9eb7a03596782f64dc0e0e27823d53a5a8972b07a3f0f7d65149b965dea5baf553b6cda04340700abefb489952a8c8881d19fe2b6e6f715c5a695f28cc70c0289e61ebcda075212cbff4f6074b5337721513113dee29ba72bf04f35c3149e8d4470f5bfe3fa0da1f5f9d9b1d11b3ed5c266799c800caa0bdd1eea4b9c17245f6fb88ed4e8c26a0276e843807e30f49535bf5680dcf3cac312fe6782f85961c23be2faece351924a0dc6b80acaed92a6fd49d0804736b74dbe194484783b2eb500b4c1c666ff1fffea07a3ba7d5812866f7c0587273a58555b262ccbf90408dc93098516156d91e1a63a049408d6701a7e298be52cdbeb0ffdf5226a46eb584a1a174410e421cdaa1b54180',
            //     'f901af20b901ab02f901a7018377a37fb9010000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000800100000100000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000800f89df89b947169d38820dfd117c3fa1f22a697dba58d90ba06f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bfa00000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8cea00000000000000000000000000000000000000000000000000000000003473bc0'
            //   ],
            //   root: '2b8a7400f4aab71416fa71c5ba414ac58082ee06efcb24790c1d41f3e2b92da0',
            //   index: Uint8Array(1) [ 54 ],
            //   value: '02f901a7018377a37fb9010000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000800100000100000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000800f89df89b947169d38820dfd117c3fa1f22a697dba58d90ba06f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bfa00000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8cea00000000000000000000000000000000000000000000000000000000003473bc0'
            // }
            let rlp_encoded_usdt_transfer_event = EthereumEventInclusionProof {
                witness: vec![
                    hex!("f90131a0cd835eed90dc62ce38fc629e51135f41e4d6aa8297eb209e2a0e0671018671fda0a533ebcc27f0346239b90d9625797a4d3027c2dafd1e8e45bc70def13e849d52a0bb6981a076cb6757607b849c003cd09734faa5e0a9d619b22868790b213afc41a0110af21e6f78640e0823ddffa343c8030c872d9ee8883be2344b0c26c155766aa0182ff8dd6cde4c456052afc52216945e410c4ed63e05d08014ff61082d91f137a095885cd002c84f0d2922ab930206956f8519d80a0a04b6fab93ba3b77742aa64a0abdcdaa69bcad2dca0e92bade6cd518bea0ead479d3c4f96206c4903904a1f60a0751dfe96598cd3661e0d427b6ee1dc4f95a788844f513a52425e55317d9b0254a0d896334d2ddb4cc6da360fb18cf372f7e79de3b58478fc87ba2a810918e3e0e08080808080808080").into(),
                    hex!("f90211a0a8e2160c776bcc991c1d50762cd9b38585617a1f206439d9e5e335c7bdfb084fa0da54f1b4b7ea617a9cd2d0accd4f646ed34aa2183b664fe96619e9af8b3fbf7aa00bda5ec2920c3476af307c1dda1e43c46edab24ecf62f6f850012ac284889f5fa04006fc7e35fd3890759225c74fb71084ca7e08bd1b4fc5f614e739d56b13e204a0b19ef750dfe7f91c7ea477072dc15ab7b4442e526bbff97914f5733a835529f2a0efcefc132e0fedf33b7167c6a554e434b44c35056ba5156251ed4e3622f3fb53a0d4935577ba2db06a0eeadccd29c6c1daf5d666f93a5162538beb9777b3fd13b4a08fcdd95b21e31e82a7309c3a48387c95fd7c58fd2838f2c3e7322f8afdcf9eb7a03596782f64dc0e0e27823d53a5a8972b07a3f0f7d65149b965dea5baf553b6cda04340700abefb489952a8c8881d19fe2b6e6f715c5a695f28cc70c0289e61ebcda075212cbff4f6074b5337721513113dee29ba72bf04f35c3149e8d4470f5bfe3fa0da1f5f9d9b1d11b3ed5c266799c800caa0bdd1eea4b9c17245f6fb88ed4e8c26a0276e843807e30f49535bf5680dcf3cac312fe6782f85961c23be2faece351924a0dc6b80acaed92a6fd49d0804736b74dbe194484783b2eb500b4c1c666ff1fffea07a3ba7d5812866f7c0587273a58555b262ccbf90408dc93098516156d91e1a63a049408d6701a7e298be52cdbeb0ffdf5226a46eb584a1a174410e421cdaa1b54180").into(),
                    hex!("f901af20b901ab02f901a7018377a37fb9010000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000800100000100000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000800f89df89b947169d38820dfd117c3fa1f22a697dba58d90ba06f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bfa00000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8cea00000000000000000000000000000000000000000000000000000000003473bc0").into(),
                ],
                index: vec![54],
                block_number: 100118,
                event: hex!("f89b947169d38820dfd117c3fa1f22a697dba58d90ba06f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bfa00000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8cea00000000000000000000000000000000000000000000000000000000003473bc0").into()
            };

            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from(hex!("0000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8ce"));

            mint_required_assets_for_optimistic_actors(
                requester.clone(),
                executor.clone(),
                200u128,
                50u128,
                ASSET_USDT,
            );

            let sfx_action = SFXAction::Transfer(
                ETHEREUM_TARGET,
                ASSET_USDT,
                requester_on_dest.clone(),
                55000000u128,
            );

            let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
                sfx_action,
                max_reward: 200u128,
                insurance: 50u128,
                reward_asset: ASSET_USDT,
                remote_origin_nonce: None,
            };

            assert_ok!(Vacuum::order(
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(requester.clone()),
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
                        there: ETHEREUM_TARGET,
                        dlq: None
                    },
                }
            );

            assert_ok!(Circuit::bid_sfx(
                RuntimeOrigin::signed(executor.clone()),
                expected_sfx_hash,
                198 as Balance,
            ));

            // Assert executor has insurance amount locked
            assert_eq!(
                Assets::balance(ASSET_USDT, &executor),
                EXISTENTIAL_DEPOSIT as Balance
            );

            // Complete bidding
            System::set_block_number(System::block_number() + 3);
            Clock::on_initialize(System::block_number());
            assert_ok!(Vacuum::read_order_status(
                RuntimeOrigin::signed(requester.clone()),
                xtx_id
            ));
            assert_eq!(
                expect_last_event_to_read_order_status().status,
                CircuitStatus::Ready
            );

            // Confirm
            let confirmation_transfer_1 = ConfirmedSideEffect::<AccountId32, BlockNumber, Balance> {
                err: None,
                output: None,
                inclusion_data: rlp_encoded_usdt_transfer_event.event.clone(),
                executioner: executor.clone(),
                received_at: System::block_number(),
                cost: None,
            };

            assert!(hotswap_latest_receipt_header_root(hex!("2b8a7400f4aab71416fa71c5ba414ac58082ee06efcb24790c1d41f3e2b92da0").into()));

            assert_ok!(Circuit::verify_sfx_proof(
                ETHEREUM_TARGET,
                SpeedMode::Fast,
                Some(ExecutionSource::from(hex!("0000000000000000000000007169D38820dfd117C3FA1f22a697dBA58d90BA06"))), // USDT address on Sepolia
                rlp_encoded_usdt_transfer_event.encode(),
            ));

            assert_ok!(Circuit::confirm_side_effect(
                RuntimeOrigin::signed(executor.clone()),
                expected_sfx_hash,
                confirmation_transfer_1
            ));

            assert_ok!(Vacuum::read_order_status(
                RuntimeOrigin::signed(requester),
                xtx_id
            ));

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
                        there: ETHEREUM_TARGET,
                        dlq: None
                    },
                }
            );

            // Check executor's balance before claim - insurance amount should be returned
            assert_eq!(
                Assets::balance(ASSET_USDT, &executor),
                EXISTENTIAL_DEPOSIT as Balance + 50 as Balance
            );

            GlobalOnInitQueues::process_hourly(300, Weight::MAX);

            // Claim via Rewards
            let _claim_res = Rewards::claim(
                RuntimeOrigin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_USDT, &executor),
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
                ASSET_DOT,
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(executor.clone()),
                expected_sfx_hash,
                198 as Balance,
            ));

            // Complete bidding
            System::set_block_number(System::block_number() + 3);
            Clock::on_initialize(System::block_number());
            assert_ok!(Vacuum::read_order_status(
                RuntimeOrigin::signed(requester.clone()),
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
                    RuntimeOrigin::signed(executor.clone()),
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
                Weight::MAX,
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
                RuntimeOrigin::signed(requester.clone()),
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
                RuntimeOrigin::signed(executor),
                expected_sfx_hash,
                confirmation_transfer
            ),);

            assert_ok!(Vacuum::read_order_status(
                RuntimeOrigin::signed(requester),
                xtx_id
            ));

            assert_eq!(Circuit::get_dlq(xtx_id), None);
        });
    }
}
