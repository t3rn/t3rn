#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
};

use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_std::{convert::TryInto, prelude::*, vec::Vec};
use t3rn_primitives::{
    circuit::{traits::CircuitSubmitAPI, types::OrderSFX},
    SpeedMode,
};
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
pub type Asset = u32;
pub type Destination = [u8; 4];
pub type Input = Vec<u8>;
use frame_support::sp_runtime::Saturating;
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, hexdisplay::AsBytesRef, H160, H256, U256};
use t3rn_abi::{
    evm_ingress_logs::{get_remote_order_abi_descriptor, RemoteEVMOrderLog},
    recode::recode_bytes_with_descriptor,
    Codec,
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
pub struct OrderStatusRead<Hash, BlockNumber, Account> {
    pub xtx_id: Hash,
    pub status: CircuitStatus,
    pub all_included_sfx: Vec<(Hash, CircuitStatus, Option<Account>)>,
    pub timeouts_at: AdaptiveTimeout<BlockNumber, TargetId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, TypeInfo)]
pub struct RemoteEVMOrderLocalized {
    pub from: H160,
    pub destination: TargetId,
    pub asset: u32,
    pub target_account: AccountId32,
    pub reward_asset: H160,
    pub amount: U256,
    pub insurance: U256,
    pub max_reward: U256,
    pub nonce: u32,
}

impl<AccountId, Balance> TryInto<SideEffect<AccountId, Balance>> for RemoteEVMOrderLocalized
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
            // target: [3u8; 4],
            max_reward,
            insurance,
            action: *b"tass",
            encoded_args,
            signature: vec![],
            enforce_executor: None,
            reward_asset_id: None,
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
    pub trait Config: frame_system::Config + pallet_xdns::Config {
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
        OrderStatusRead(OrderStatusRead<T::Hash, BlockNumberFor<T>, T::AccountId>),
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
        pub fn single_order(
            origin: OriginFor<T>,
            destination: TargetId,
            asset: Asset,
            amount: BalanceOf<T>,
            reward_asset: Asset,
            max_reward: BalanceOf<T>,
            insurance: BalanceOf<T>,
            target_account: T::AccountId,
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            let sfx_order =
                OrderSFX::<T::AccountId, Asset, BalanceOf<T>, TargetId, Vec<u8>, BalanceOf<T>> {
                    sfx_action: SFXAction::Transfer(destination, asset, target_account, amount),
                    max_reward,
                    insurance,
                    reward_asset,
                    remote_origin_nonce: None,
                };

            let side_effect: SideEffect<T::AccountId, BalanceOf<T>> = sfx_order.try_into()?;

            T::CircuitSubmitAPI::on_extrinsic_trigger(
                origin,
                sp_std::vec![side_effect],
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
            speed_mode: SpeedMode,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin.clone())?;

            let verified_event_bytes = T::CircuitSubmitAPI::verify_sfx_proof(
                remote_target_id,
                speed_mode.clone(),
                Some(T::Xdns::get_remote_order_contract_address(remote_target_id)?.0),
                order_remote_proof, // 0,150,80,46,240,61,236,212,170,189,54,140,196,111,92,44,34,166,69,148,140
            )?
            .message;

            let recoded_message = recode_bytes_with_descriptor(
                verified_event_bytes.clone(),
                get_remote_order_abi_descriptor(),
                Codec::Rlp,
                Codec::Scale,
            )?;

            let decoded_remote_order_log = RemoteEVMOrderLog::decode(&mut &recoded_message[..])
                .map_err(|e| {
                    log::error!(
                        "Vecuum::remote_order -- error decoding RemoteEVMOrderLog: {:?}",
                        e
                    );
                    DispatchError::Other("Vecuum::remote_order -- error decoding RemoteEVMOrderLog")
                })?;

            let decoded_remote_order: RemoteEVMOrderLocalized = RemoteEVMOrderLocalized {
                from: decoded_remote_order_log.sender,
                destination: decoded_remote_order_log.destination,
                asset: T::Xdns::get_token_by_eth_address(
                    remote_target_id,
                    decoded_remote_order_log.reward_asset,
                )?
                .token_id,
                target_account: decoded_remote_order_log.target_account,
                reward_asset: decoded_remote_order_log.reward_asset,
                amount: decoded_remote_order_log.amount,
                insurance: decoded_remote_order_log.insurance,
                max_reward: decoded_remote_order_log.max_reward,
                nonce: decoded_remote_order_log.nonce,
            };

            assert!(T::Xdns::is_target_active(
                remote_target_id.clone(),
                &SecurityLvl::Optimistic
            ));

            let mut side_effect: SideEffect<T::AccountId, BalanceOf<T>> =
                decoded_remote_order.clone().try_into()?;

            side_effect.reward_asset_id = Some(decoded_remote_order.asset);

            let remote_origin: OrderOrigin<T::AccountId> =
                OrderOrigin::from_remote_nonce(decoded_remote_order.nonce);

            // Based on target and asset, derive the intent of the order.
            // If the target is a remote chain, then the intent is to transfer funds to the remote chain. We don't need to go into details of assets order transfers or swaps.
            // If target is a local chain (t3rn) && asset used for reward payout equals the asset transferred on remote chain - assume bridge operation of wrapped assets.
            if &decoded_remote_order.destination == &[3, 3, 3, 3]
                && T::Xdns::list_available_mint_assets([3, 3, 3, 3])
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
                // Mint wrapped assets on local chain.
                let amount = BalanceOf::<T>::decode(
                    &mut &decoded_remote_order.amount.as_u128().encode()[..],
                )
                .map_err(|e| {
                    DispatchError::Other("Vacuum::remote_order -- error decoding amount")
                })?;
                // In the context of minting assets, max reward is the net reward to executor.
                let max_reward = BalanceOf::<T>::decode(
                    &mut &decoded_remote_order.max_reward.as_u128().encode()[..],
                )
                .map_err(|e| {
                    DispatchError::Other("Vacuum::remote_order -- error decoding amount")
                })?;

                assert!(amount >= max_reward, "Vacuum::remote_order -- amount must be greater (rarely equal) than max_reward for minting assets");
                let asset = decoded_remote_order.asset;
                assert!(
                    T::Xdns::check_asset_is_mintable([3, 3, 3, 3], asset),
                    "Vacuum::remote_order -- asset is not mintable"
                );

                let target_account =
                    T::AccountId::decode(&mut &decoded_remote_order.target_account.encode()[..])
                        .map_err(|e| {
                            DispatchError::Other(
                                "Vacuum::remote_order -- error decoding target_account",
                            )
                        })?;

                // Mint wrapped assets on local chain.
                T::Xdns::mint(asset, who, max_reward)?;
                T::Xdns::mint(asset, target_account, max_reward.saturating_sub(amount))?;
                Ok(().into())
            } else {
                // For remote order + remote reward, assume on_remote_origin_trigger
                T::CircuitSubmitAPI::on_remote_origin_trigger(
                    origin.clone(),
                    remote_origin.to_account_id(),
                    vec![side_effect],
                    speed_mode,
                )
            }
        }

        #[pallet::weight(100_000)]
        pub fn read_order_status(
            _origin: OriginFor<T>,
            xtx_id: T::Hash,
        ) -> DispatchResultWithPostInfo {
            Self::emit_order_status(xtx_id)
        }

        #[pallet::weight(100_000)]
        pub fn read_all_pending_orders_status(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            for xtx_id in T::ReadSFX::get_pending_xtx_ids() {
                Self::emit_order_status(xtx_id)?;
            }
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn emit_order_status(xtx_id: T::Hash) -> DispatchResultWithPostInfo {
            let (status, timeouts_at) = T::ReadSFX::get_xtx_status(xtx_id)?;
            let sfx_of_xtx = T::ReadSFX::get_fsx_of_xtx(xtx_id)?;
            let all_included_sfx = sfx_of_xtx
                .into_iter()
                .map(|sfx| {
                    let fsx_status = T::ReadSFX::get_fsx_status(sfx)?;
                    let fsx_executor = T::ReadSFX::get_fsx_executor(sfx)?;
                    Ok((sfx, fsx_status, fsx_executor))
                })
                .collect::<Result<Vec<(T::Hash, CircuitStatus, Option<T::AccountId>)>, DispatchError>>()?;

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
    use sp_core::H256;
    use sp_runtime::{traits::Keccak256, AccountId32};
    use sp_std::convert::TryInto;
    pub use t3rn_mini_mock_runtime::{
        activate_all_light_clients, hotswap_latest_receipt_header_root,
        initialize_eth2_with_3rd_epoch, prepare_ext_builder_playground, AccountId, AssetId, Assets,
        Balance, Balances, BlockNumber, Circuit, CircuitError, CircuitEvent, Clock, ConfigVacuum,
        EthereumEventInclusionProof, GlobalOnInitQueues, Hash, MiniRuntime, MockedAssetEvent,
        OrderStatusRead, Portal, Rewards, RuntimeEvent as Event, RuntimeOrigin, System, Vacuum,
        VacuumEvent, ASSET_DOT, ASSET_ETH, ASSET_USDT, ETHEREUM_TARGET, POLKADOT_TARGET, XDNS,
    };

    use t3rn_primitives::{
        circuit::{
            types::{OrderSFX, SFXAction},
            CircuitSubmitAPI,
        },
        claimable::CircuitRole,
        clock::OnHookQueues,
        light_client::LightClientAsyncAPI,
        monetary::TRN,
        portal::Portal as PortalT,
        EthereumToken, ExecutionSource, GatewayVendor, SpeedMode, TokenInfo, TreasuryAccount,
        TreasuryAccountProvider,
    };
    use t3rn_types::sfx::{ConfirmedSideEffect, SideEffect};

    use frame_support::{traits::Currency, weights::Weight};

    use t3rn_primitives::{
        circuit::{AdaptiveTimeout, CircuitStatus},
        monetary::EXISTENTIAL_DEPOSIT,
    };
    use t3rn_types::fsx::TargetId;

    pub fn mint_required_assets_for_optimistic_actors(
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
        let requester_starting_balance = Assets::balance(ASSET_DOT, &requester);
        let executor_starting_balance = Assets::balance(ASSET_DOT, &executor);
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
            max_reward + (EXISTENTIAL_DEPOSIT as Balance) + requester_starting_balance
        );
        assert_eq!(
            Assets::balance(asset_id, &executor),
            insurance + (EXISTENTIAL_DEPOSIT as Balance) + executor_starting_balance
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

    fn expect_last_event_to_read_order_status() -> OrderStatusRead<Hash, BlockNumber, AccountId32> {
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
    fn optimistic_order_single_decoded_sfx_vacuum_delivers_to_circuit() {
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

            activate_all_light_clients();

            assert_ok!(Vacuum::single_order(
                RuntimeOrigin::signed(requester.clone()),
                POLKADOT_TARGET,
                ASSET_DOT,
                100u128,
                ASSET_DOT,
                200u128,
                50u128,
                requester_on_dest.clone(),
                SpeedMode::Fast
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
    fn optimistic_order_single_rlp_encoded_sfx_vacuum_delivers_to_circuit() {
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

            activate_all_light_clients();

            assert_ok!(Vacuum::single_order(
                RuntimeOrigin::signed(requester.clone()),
                ETHEREUM_TARGET,
                ASSET_DOT,
                100u128,
                ASSET_DOT,
                200u128,
                50u128,
                requester_on_dest.clone(),
                SpeedMode::Fast
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

    fn make_whole_vacuum_trip_including_minting_and_confirmation(
        reward_and_requested_asset: AssetId,
        executor: AccountId32,
        requester: AccountId32,
        requester_on_dest: AccountId32,
    ) {
        mint_required_assets_for_optimistic_actors(
            requester.clone(),
            executor.clone(),
            200u128,
            50u128,
            ASSET_DOT,
        );

        let sfx_action = SFXAction::Transfer(
            POLKADOT_TARGET,
            reward_and_requested_asset,
            requester_on_dest.clone(),
            100u128,
        );
        let sfx_order = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action,
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: reward_and_requested_asset,
            remote_origin_nonce: None,
        };

        assert_ok!(Vacuum::order(
            RuntimeOrigin::signed(requester.clone()),
            vec![sfx_order.clone()],
            SpeedMode::Fast,
        ));

        let xtx_id = expect_last_event_to_emit_xtx_id();

        let sfx: SideEffect<AccountId32, Balance> = sfx_order.try_into().unwrap();
        let expected_sfx_hash = sfx.generate_id::<Keccak256>(xtx_id.0.as_slice(), 0);

        assert_ok!(Circuit::bid_sfx(
            RuntimeOrigin::signed(executor.clone()),
            expected_sfx_hash,
            198 as Balance,
        ));

        let mut scale_encoded_transfer_event = MockedAssetEvent::<MiniRuntime>::Transferred {
            asset_id: reward_and_requested_asset,
            from: executor.clone(),
            to: requester_on_dest.clone(),
            amount: 100 as Balance,
        }
        .encode();
        // Complete bidding
        System::set_block_number(System::block_number() + 3);
        Clock::on_initialize(System::block_number());

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
    }

    #[test]
    fn optimistic_order_four_times_in_dispersed_intervals_sfx_correctly_rewards_executor_at_successful_confirm(
    ) {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from([3u8; 32]);

            activate_all_light_clients();

            for loop_index in 0..3 {
                make_whole_vacuum_trip_including_minting_and_confirmation(
                    ASSET_DOT,
                    executor.clone(),
                    requester.clone(),
                    requester_on_dest.clone(),
                );
            }

            // Check executor's balance before claim - insurance amount should be returned
            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                3 * (EXISTENTIAL_DEPOSIT as Balance + 50 as Balance) // 3 x insurance returns
            );

            System::set_block_number(300);
            GlobalOnInitQueues::process_hourly(300, Weight::MAX);

            // Claim via Rewards
            let _claim_res = Rewards::claim(
                RuntimeOrigin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                3 * (EXISTENTIAL_DEPOSIT as Balance + 50 as Balance + 200 as Balance)
            );

            // Make one more request in the next round
            System::set_block_number(301);
            activate_all_light_clients();

            make_whole_vacuum_trip_including_minting_and_confirmation(
                ASSET_DOT,
                executor.clone(),
                requester.clone(),
                requester_on_dest.clone(),
            );

            GlobalOnInitQueues::process_hourly(600, Weight::MAX);

            // Claim via Rewards
            let _claim_res = Rewards::claim(
                RuntimeOrigin::signed(executor.clone()),
                Some(CircuitRole::Executor),
            );

            assert_eq!(
                Assets::balance(ASSET_DOT, &executor),
                4 * (EXISTENTIAL_DEPOSIT as Balance + 50 as Balance + 200 as Balance)
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
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::PendingBidding,
                        None
                    )],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
                        there: [1, 1, 1, 1],
                        dlq: None
                    },
                }
            );

            System::reset_events();

            assert_ok!(Vacuum::read_all_pending_orders_status(
                RuntimeOrigin::signed(requester.clone()),
            ));

            let order_status = expect_last_event_to_read_order_status();

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::PendingBidding,
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::PendingBidding,
                        None
                    )],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
                        there: [1, 1, 1, 1],
                        dlq: None
                    },
                }
            );

            System::reset_events();

            assert_ok!(Vacuum::read_all_pending_orders_status(
                RuntimeOrigin::signed(requester.clone()),
            ));

            let order_status = expect_last_event_to_read_order_status();

            assert_eq!(
                order_status,
                OrderStatusRead {
                    xtx_id,
                    status: CircuitStatus::PendingBidding,
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::PendingBidding,
                        None
                    )],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
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
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::FinishedAllSteps,
                        Some(AccountId32::new([1u8; 32]))
                    ),],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
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
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::PendingBidding, None)],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 49,
                        estimated_height_there: 56,
                        submit_by_height_here: 41,
                        submit_by_height_there: 40,
                        emergency_timeout_here: 433,
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
                    all_included_sfx: vec![(expected_sfx_hash, CircuitStatus::FinishedAllSteps, Some(executor.clone())),],
                    timeouts_at: AdaptiveTimeout::<BlockNumber, TargetId> {
                        estimated_height_here: 49,
                        estimated_height_there: 56,
                        submit_by_height_here: 41,
                        submit_by_height_there: 40,
                        emergency_timeout_here: 433,
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
    fn vacuum_delivers_remote_order_with_local_reward_delivers_to_circuit_and_confirms_for_eth_targets(
    ) {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            activate_all_light_clients();
            initialize_eth2_with_3rd_epoch();

            // Derive all arguments out of below proof sourced with eth2-proof client-side library
            // ⬅️found receipt for tx:  0x189bb96988ab037da92d57ae514f414a26a15326f8a7681947db35dbbcfd47e6
            // 🔃parsed receipt to hex form
            // ⬅️found block for receipt:  0x189bb96988ab037da92d57ae514f414a26a15326f8a7681947db35dbbcfd47e6 4249964n
            // ⬅️fetched all 153 sibling transaction receipts
            // Computed Root:  0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966
            // 🧮proof-calculated receipts root vs block receipts root:  0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966 0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966
            // {
            //   proof: [
            //     'f8b1a0d573cf15a54d7ad0d0498169820a6dafd9633adec0f688d9d859d1741cdac5b6a03c3d817222f0515e057962c0db4bfd20674e227a3b3c43811b2f8f1357e127cea03fe55d8df8a0c96030c87b2f2caf2bc6f102b5546e097fda3f6eb8c89370ea6ca00e3ccf072c8ad2c828f4d859e0776b24cf345009c5cf3107413186a0617e5f7980808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080',
            //     'f90211a086ff47818c4ed0f1d97d62c83605e662754d5c21d94e807c2142f7294eefda64a034960e2a1e3ec16b96816e81f842a51291989f5606c41aadba21b8069adaad64a057f37f57fecb0d336797db86604ea1d73a7933d5e6043c371729ca90989d5f8ca034754510b4bf1c613de0202a877a92be3e81ae1aacef713ac63d7fcdbeb84e05a099ec62a5e2b7bd634351e10bbdad33afbb3d9b52cb9b0a449fd38cec498624aba0f6b2fa50ec73e180a7c8b61cceacc8bfdf9214ef3df95abf4fa7b2e0ee38bb1aa0bc1b4735f691d698650aa757c1fac347e47b272dca9780293a091755f1eb49c4a0bb81253270782d099bd0cb4b5ab1fe9faad27c1657d6e55c7bce1129b0426011a03a2e7a799dc4f627368880fa7bfe6116ad913eaa04c1b200ecc5f4980ced9558a0649a03c1e451f6033a6b359f819c019ca9eb5fdb140bc5f01d9f369e2f1d69e8a04172cab94d1db98e819c0f5513d4944806a110862d9d8b67981cec1f5e1b3358a02e73b02f64e412bb54b38dc94bd232934438951d765abfa71df971606f9b6f74a0089c2c3e3b0c99a5f22d0c8c2e29c5e89835aa3d0848722eb1a52ef1cd0bae56a05b5a11c3bd83a7a0c764b11ccdf98de8ae91012637ab318f5ba980d303d199dba05fe9d9c59da9e5e95049f9baaa514da634b87daf0307d8de047399395b3ec606a002373b6677d1b3d1ebe4501df696607732e075a452e05f9b6aa8323fe7ee05d880',
            //     'f902d420b902d002f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064'
            //   ],
            //   root: '76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966',
            //   index: Uint8Array(1) [ 40 ],
            // 
            //   value: '02f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064',
            //   event: 'f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064'
            let rlp_encoded_remote_order_local_reward_event = EthereumEventInclusionProof {
                witness: vec![
                    hex!("f8b1a0d573cf15a54d7ad0d0498169820a6dafd9633adec0f688d9d859d1741cdac5b6a03c3d817222f0515e057962c0db4bfd20674e227a3b3c43811b2f8f1357e127cea03fe55d8df8a0c96030c87b2f2caf2bc6f102b5546e097fda3f6eb8c89370ea6ca00e3ccf072c8ad2c828f4d859e0776b24cf345009c5cf3107413186a0617e5f7980808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080").into(),
                    hex!("f90211a086ff47818c4ed0f1d97d62c83605e662754d5c21d94e807c2142f7294eefda64a034960e2a1e3ec16b96816e81f842a51291989f5606c41aadba21b8069adaad64a057f37f57fecb0d336797db86604ea1d73a7933d5e6043c371729ca90989d5f8ca034754510b4bf1c613de0202a877a92be3e81ae1aacef713ac63d7fcdbeb84e05a099ec62a5e2b7bd634351e10bbdad33afbb3d9b52cb9b0a449fd38cec498624aba0f6b2fa50ec73e180a7c8b61cceacc8bfdf9214ef3df95abf4fa7b2e0ee38bb1aa0bc1b4735f691d698650aa757c1fac347e47b272dca9780293a091755f1eb49c4a0bb81253270782d099bd0cb4b5ab1fe9faad27c1657d6e55c7bce1129b0426011a03a2e7a799dc4f627368880fa7bfe6116ad913eaa04c1b200ecc5f4980ced9558a0649a03c1e451f6033a6b359f819c019ca9eb5fdb140bc5f01d9f369e2f1d69e8a04172cab94d1db98e819c0f5513d4944806a110862d9d8b67981cec1f5e1b3358a02e73b02f64e412bb54b38dc94bd232934438951d765abfa71df971606f9b6f74a0089c2c3e3b0c99a5f22d0c8c2e29c5e89835aa3d0848722eb1a52ef1cd0bae56a05b5a11c3bd83a7a0c764b11ccdf98de8ae91012637ab318f5ba980d303d199dba05fe9d9c59da9e5e95049f9baaa514da634b87daf0307d8de047399395b3ec606a002373b6677d1b3d1ebe4501df696607732e075a452e05f9b6aa8323fe7ee05d880").into(),
                    hex!("f902d420b902d002f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064").into(),
                ],
                index: vec![40],
                block_number: 100118,
                event: hex!("f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064").into()
            };

            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([2u8; 32]);
            let requester_on_dest = AccountId32::from(hex!("000000000000000000000000F85A57d965aEcD289c625Cae6161d0Ab5141bC66")); // 0xF85A57d965aEcD289c625Cae6161d0Ab5141bC66

            mint_required_assets_for_optimistic_actors(
                requester.clone(),
                executor.clone(),
                200u128,
                50u128,
                ASSET_ETH,
            );

            // Add remote order address to XDNS
            assert_ok!(
                XDNS::add_remote_order_address(
                    RuntimeOrigin::root(),
                    ETHEREUM_TARGET,
                    H256::from([0,0,0,0,0,0,0,0,0,0,0,0,150,80,46,240,61,236,212,170,189,54,140,196,111,92,44,34,166,69,148,140]),
                )
            );

            assert!(hotswap_latest_receipt_header_root(hex!("76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966").into()));

            activate_all_light_clients();

            assert_ok!(Vacuum::remote_order(
                RuntimeOrigin::signed(executor.clone()),
                rlp_encoded_remote_order_local_reward_event.encode(),
                ETHEREUM_TARGET,
                SpeedMode::Fast,
            ));

            let xtx_id = expect_last_event_to_emit_xtx_id();

            assert_eq!(
                xtx_id,
                Hash::from(hex!(
                    "ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
                ))
            );
        });
    }

    #[test]
    fn vacuum_delivers_remote_bridging_order_and_mints_ordered_assets() {
        let mut ext = prepare_ext_builder_playground();
        ext.execute_with(|| {
            activate_all_light_clients();
            initialize_eth2_with_3rd_epoch();
            // Derive all arguments out of below proof sourced with eth2-proof client-side library
            // ⬅️found receipt for tx:  0x189bb96988ab037da92d57ae514f414a26a15326f8a7681947db35dbbcfd47e6
            // 🔃parsed receipt to hex form
            // ⬅️found block for receipt:  0x189bb96988ab037da92d57ae514f414a26a15326f8a7681947db35dbbcfd47e6 4249964n
            // ⬅️fetched all 153 sibling transaction receipts
            // Computed Root:  0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966
            // 🧮proof-calculated receipts root vs block receipts root:  0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966 0x76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966
            // {
            //   proof: [
            //     'f8b1a0d573cf15a54d7ad0d0498169820a6dafd9633adec0f688d9d859d1741cdac5b6a03c3d817222f0515e057962c0db4bfd20674e227a3b3c43811b2f8f1357e127cea03fe55d8df8a0c96030c87b2f2caf2bc6f102b5546e097fda3f6eb8c89370ea6ca00e3ccf072c8ad2c828f4d859e0776b24cf345009c5cf3107413186a0617e5f7980808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080',
            //     'f90211a086ff47818c4ed0f1d97d62c83605e662754d5c21d94e807c2142f7294eefda64a034960e2a1e3ec16b96816e81f842a51291989f5606c41aadba21b8069adaad64a057f37f57fecb0d336797db86604ea1d73a7933d5e6043c371729ca90989d5f8ca034754510b4bf1c613de0202a877a92be3e81ae1aacef713ac63d7fcdbeb84e05a099ec62a5e2b7bd634351e10bbdad33afbb3d9b52cb9b0a449fd38cec498624aba0f6b2fa50ec73e180a7c8b61cceacc8bfdf9214ef3df95abf4fa7b2e0ee38bb1aa0bc1b4735f691d698650aa757c1fac347e47b272dca9780293a091755f1eb49c4a0bb81253270782d099bd0cb4b5ab1fe9faad27c1657d6e55c7bce1129b0426011a03a2e7a799dc4f627368880fa7bfe6116ad913eaa04c1b200ecc5f4980ced9558a0649a03c1e451f6033a6b359f819c019ca9eb5fdb140bc5f01d9f369e2f1d69e8a04172cab94d1db98e819c0f5513d4944806a110862d9d8b67981cec1f5e1b3358a02e73b02f64e412bb54b38dc94bd232934438951d765abfa71df971606f9b6f74a0089c2c3e3b0c99a5f22d0c8c2e29c5e89835aa3d0848722eb1a52ef1cd0bae56a05b5a11c3bd83a7a0c764b11ccdf98de8ae91012637ab318f5ba980d303d199dba05fe9d9c59da9e5e95049f9baaa514da634b87daf0307d8de047399395b3ec606a002373b6677d1b3d1ebe4501df696607732e075a452e05f9b6aa8323fe7ee05d880',
            //     'f902d420b902d002f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064'
            //   ],
            //   root: '76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966',
            //   index: Uint8Array(1) [ 40 ],
            // 
            //   value: '02f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064',
            //   event: 'f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064'

            assert_ok!(
                XDNS::add_remote_order_address(
                    RuntimeOrigin::root(),
                    ETHEREUM_TARGET,
                    H256::from([0,0,0,0,0,0,0,0,0,0,0,0,150,80,46,240,61,236,212,170,189,54,140,196,111,92,44,34,166,69,148,140]),
                )
            );

            // Enroll asset as mintable
            assert_ok!(XDNS::enroll_bridge_asset(
                RuntimeOrigin::root(),
                1u32,
                [3, 3, 3, 3],
                TokenInfo::Ethereum(EthereumToken {
                    decimals: 18,
                    symbol: b"sepl".to_vec(),
                    address: Some([0; 20])
                })
            ));

            let rlp_encoded_remote_order_local_reward_event = EthereumEventInclusionProof {
                witness: vec![
                    hex!("f8b1a0d573cf15a54d7ad0d0498169820a6dafd9633adec0f688d9d859d1741cdac5b6a03c3d817222f0515e057962c0db4bfd20674e227a3b3c43811b2f8f1357e127cea03fe55d8df8a0c96030c87b2f2caf2bc6f102b5546e097fda3f6eb8c89370ea6ca00e3ccf072c8ad2c828f4d859e0776b24cf345009c5cf3107413186a0617e5f7980808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080").into(),
                    hex!("f90211a086ff47818c4ed0f1d97d62c83605e662754d5c21d94e807c2142f7294eefda64a034960e2a1e3ec16b96816e81f842a51291989f5606c41aadba21b8069adaad64a057f37f57fecb0d336797db86604ea1d73a7933d5e6043c371729ca90989d5f8ca034754510b4bf1c613de0202a877a92be3e81ae1aacef713ac63d7fcdbeb84e05a099ec62a5e2b7bd634351e10bbdad33afbb3d9b52cb9b0a449fd38cec498624aba0f6b2fa50ec73e180a7c8b61cceacc8bfdf9214ef3df95abf4fa7b2e0ee38bb1aa0bc1b4735f691d698650aa757c1fac347e47b272dca9780293a091755f1eb49c4a0bb81253270782d099bd0cb4b5ab1fe9faad27c1657d6e55c7bce1129b0426011a03a2e7a799dc4f627368880fa7bfe6116ad913eaa04c1b200ecc5f4980ced9558a0649a03c1e451f6033a6b359f819c019ca9eb5fdb140bc5f01d9f369e2f1d69e8a04172cab94d1db98e819c0f5513d4944806a110862d9d8b67981cec1f5e1b3358a02e73b02f64e412bb54b38dc94bd232934438951d765abfa71df971606f9b6f74a0089c2c3e3b0c99a5f22d0c8c2e29c5e89835aa3d0848722eb1a52ef1cd0bae56a05b5a11c3bd83a7a0c764b11ccdf98de8ae91012637ab318f5ba980d303d199dba05fe9d9c59da9e5e95049f9baaa514da634b87daf0307d8de047399395b3ec606a002373b6677d1b3d1ebe4501df696607732e075a452e05f9b6aa8323fe7ee05d880").into(),
                    hex!("f902d420b902d002f902cc018354abeab9010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000004000000000000000000000001000000000020000000000000000000800000000000000000000000000000000000000000000000004010000000000000000000000000000000000002000000000000000000000000000000000200000000000000400000000060000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000008f901c1f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064").into(),
                ],
                index: vec![40],
                block_number: 100118,
                event: hex!("f901be9496502ef03decd4aabd368cc46f5c2c22a645948cf884a07f1c6663f3b95396ee5e22d3f5fff2058cf091e620a0b1907eda0138b382c8b6a0e2da230e52caecf528190bb0f28767e3e02a2df185bcd070c3f019537e4d5844a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000f85a57d965aecd289c625cae6161d0ab5141bc66b90120000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e003030303000000000000000000000000000000000000000000000000000000000000e80300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064").into()
            };

            let executor = AccountId32::from([1u8; 32]);
            let requester = AccountId32::from([0u8; 32]);
            let requester_on_dest = AccountId32::from(hex!("000000000000000000000000F85A57d965aEcD289c625Cae6161d0Ab5141bC66")); // 0xF85A57d965aEcD289c625Cae6161d0Ab5141bC66

            assert!(hotswap_latest_receipt_header_root(hex!("76435ece9646ad97cbaf7e8190af314df7f577b31f02f9c8ff12d3ea5a68b966").into()));

            activate_all_light_clients();

            // Check executor + requester have no assets prior to the order
            assert_eq!(Assets::balance(1u32, &requester), 0);
            assert_eq!(Assets::balance(1u32, &executor), 0);

            assert_ok!(Vacuum::remote_order(
                RuntimeOrigin::signed(executor.clone()),
                rlp_encoded_remote_order_local_reward_event.encode(),
                ETHEREUM_TARGET,
                SpeedMode::Fast,
            ));

            // Check that assets were minted according to the bridge order formula:
            // user: amount - max_reward
            // executor: max_reward (net)
            assert_eq!(Assets::balance(1u32, &requester), 0);
            assert_eq!(Assets::balance(1u32, &executor), 100);
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
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::PendingBidding,
                        None
                    )],
                    timeouts_at: AdaptiveTimeout {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
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

            // Wait for after XTX emergency timeout
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
                    all_included_sfx: vec![(
                        expected_sfx_hash,
                        CircuitStatus::Ready,
                        Some(AccountId32::new([1u8; 32]))
                    ),],
                    timeouts_at: AdaptiveTimeout {
                        estimated_height_here: 97,
                        estimated_height_there: 152,
                        submit_by_height_here: 65,
                        submit_by_height_there: 88,
                        emergency_timeout_here: 433,
                        there: [1, 1, 1, 1],
                        dlq: Some(469)
                    },
                }
            );

            // Now activate the LightClient again and expect the DLQ to be processed
            mock_signal_unhalt(POLKADOT_TARGET, GatewayVendor::Polkadot);
            activate_all_light_clients();

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
