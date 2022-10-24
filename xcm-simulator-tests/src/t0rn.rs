use crate::{
    _Messenger, _hrmp_channel_parachain_inherent_data, _process_messages, ALICE, INITIAL_BALANCE,
};
use frame_support::traits::GenesisBuild;
use t0rn::Origin;
use xcm_emulator::decl_test_parachain;

const T0RN_PARA_ID: u32 = 1;
const T1RN_PARA_ID: u32 = 2;

decl_test_parachain! {
    pub struct T0rn {
        Runtime = t0rn::Runtime,
        Origin = t0rn::Origin,
        XcmpMessageHandler = t0rn::XcmpQueue,
        DmpMessageHandler = t0rn::DmpQueue,
        new_ext = t0rn_ext(T0RN_PARA_ID),
    }
}

decl_test_parachain! {
    pub struct T1rn {
        Runtime = t0rn::Runtime,
        Origin = t0rn::Origin,
        XcmpMessageHandler = t0rn::XcmpQueue,
        DmpMessageHandler = t0rn::DmpQueue,
        new_ext = t0rn_ext(T1RN_PARA_ID),
    }
}

pub fn t0rn_ext(para_id: u32) -> sp_io::TestExternalities {
    use t0rn::{Runtime, System};

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    let parachain_info_config = parachain_info::GenesisConfig {
        parachain_id: para_id.into(),
    };

    <parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
        &parachain_info_config,
        &mut t,
    )
    .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, INITIAL_BALANCE)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Encode;

    use crate::{Network, RococoNet};
    use frame_support::{assert_ok, traits::Currency};
    use polkadot_primitives::v2::Id as ParaId;
    use sp_runtime::{traits::AccountIdConversion, MultiAddress, MultiAddress::Id};
    use t0rn::AccountId;
    use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation, VersionedXcm};
    use xcm_emulator::TestExt;

    fn log_all_events(chain: &str) {
        t0rn::System::events()
            .iter()
            .for_each(|r| println!(">>> [{}] {:?}", chain, r.event));
    }

    #[test]
    fn dmp() {
        Network::reset();

        let remark = t0rn::Call::System(frame_system::Call::<t0rn::Runtime>::remark_with_event {
            remark: "Hello from Rococo!".as_bytes().to_vec(),
        });
        RococoNet::execute_with(|| {
            assert_ok!(rococo_runtime::XcmPallet::force_default_xcm_version(
                rococo_runtime::Origin::root(),
                Some(0)
            ));
            assert_ok!(rococo_runtime::XcmPallet::send_xcm(
                Here,
                Parachain(1),
                Xcm(vec![Transact {
                    origin_type: OriginKind::SovereignAccount,
                    require_weight_at_most: INITIAL_BALANCE as u64,
                    call: remark.encode().into(),
                }]),
            ));
        });

        T0rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T0rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
            )));
        });
    }

    #[test]
    fn ump() {
        Network::reset();

        RococoNet::execute_with(|| {
            let _ = rococo_runtime::Balances::deposit_creating(
                &ParaId::from(1).into_account_truncating(),
                1_000_000_000_000,
            );
        });

        let remark = rococo_runtime::Call::System(
            frame_system::Call::<rococo_runtime::Runtime>::remark_with_event {
                remark: "Hello from Pumpkin!".as_bytes().to_vec(),
            },
        );
        T0rn::execute_with(|| {
            assert_ok!(t0rn::PolkadotXcm::send_xcm(
                Here,
                Parent,
                Xcm(vec![Transact {
                    origin_type: OriginKind::SovereignAccount,
                    require_weight_at_most: INITIAL_BALANCE as u64,
                    call: remark.encode().into(),
                }]),
            ));
        });

        RococoNet::execute_with(|| {
            use rococo_runtime::{Event, System};

            System::events()
                .iter()
                .for_each(|r| println!(">>> [RelayChain] {:?}", r.event));

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::System(frame_system::Event::NewAccount { account: _ })
            )));
        });
    }

    #[test]
    fn xcmp() {
        Network::reset();

        let remark = t0rn::Call::System(frame_system::Call::<t0rn::Runtime>::remark_with_event {
            remark: "Hello from Pumpkin!".as_bytes().to_vec(),
        });

        T0rn::execute_with(|| {
            assert_ok!(t0rn::PolkadotXcm::send_xcm(
                Here,
                MultiLocation::new(1, X1(Parachain(2))),
                Xcm(vec![
                    WithdrawAsset(MultiAssets::from(vec![MultiAsset {
                        id: AssetId::Concrete(MultiLocation::here()),
                        fun: Fungibility::Fungible(10000000)
                    }])),
                    BuyExecution {
                        fees: MultiAsset {
                            id: AssetId::Concrete(MultiLocation::here()),
                            fun: Fungibility::Fungible(10000000)
                        },
                        weight_limit: WeightLimit::Unlimited
                    },
                    Transact {
                        origin_type: OriginKind::SovereignAccount,
                        require_weight_at_most: 10_000_000,
                        call: remark.encode().into(),
                    }
                ]),
            ));

            log_all_events("T0rn");
        });

        T1rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T1rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
            )));
        });
    }

    #[test]
    fn xcmp_through_a_parachain() {
        use t0rn::{Call, PolkadotXcm, Runtime};

        Network::reset();

        // The message goes through: Pumpkin --> Mushroom --> Octopus
        let remark = Call::System(frame_system::Call::<Runtime>::remark_with_event {
            remark: "Hello from Pumpkin!".as_bytes().to_vec(),
        });

        let send_xcm_to_t1rn = Call::PolkadotXcm(pallet_xcm::Call::<Runtime>::send {
            dest: Box::new(VersionedMultiLocation::V1(MultiLocation::new(
                1,
                X1(Parachain(T1RN_PARA_ID)),
            ))),
            message: Box::new(VersionedXcm::V2(Xcm(vec![Transact {
                origin_type: OriginKind::SovereignAccount,
                require_weight_at_most: 10_000_000,
                call: remark.encode().into(),
            }]))),
        });
        T0rn::execute_with(|| {
            assert_ok!(PolkadotXcm::send_xcm(
                Here,
                MultiLocation::new(1, X1(Parachain(T0RN_PARA_ID))),
                Xcm(vec![Transact {
                    origin_type: OriginKind::SovereignAccount,
                    require_weight_at_most: 100_000_000,
                    call: send_xcm_to_t1rn.encode().into(),
                }]),
            ));
        });

        T0rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T0rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. })
            )));
            // assert!(System::events().iter().any(|r| matches!(
            //     r.event,
            //     Event::PolkadotXcm(pallet_xcm::Event::Sent(_, _, _))
            // )));
        });

        T1rn::execute_with(|| {
            use t0rn::{Event, System};
            // execution would fail, but good enough to check if the message is received
            log_all_events("T1rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Fail { .. })
            )));
        });
    }

    #[test]
    fn deduplicate_dmp() {
        Network::reset();
        RococoNet::execute_with(|| {
            assert_ok!(rococo_runtime::XcmPallet::force_default_xcm_version(
                rococo_runtime::Origin::root(),
                Some(0)
            ));
        });

        rococo_send_rmrk("Rococo", 2);
        parachain_receive_and_reset_events(true);

        // a different dmp message in same relay-parent-block allow execution.
        rococo_send_rmrk("Polkadot", 1);
        parachain_receive_and_reset_events(true);

        // same dmp message with same relay-parent-block wouldn't execution
        rococo_send_rmrk("Rococo", 1);
        parachain_receive_and_reset_events(false);

        // different relay-parent-block allow dmp message execution
        RococoNet::execute_with(|| rococo_runtime::System::set_block_number(2));

        rococo_send_rmrk("Rococo", 1);
        parachain_receive_and_reset_events(true);

        // reset can send same dmp message again
        Network::reset();
        RococoNet::execute_with(|| {
            assert_ok!(rococo_runtime::XcmPallet::force_default_xcm_version(
                rococo_runtime::Origin::root(),
                Some(0)
            ));
        });

        rococo_send_rmrk("Rococo", 1);
        parachain_receive_and_reset_events(true);
    }

    fn rococo_send_rmrk(msg: &str, count: u32) {
        let remark = t0rn::Call::System(frame_system::Call::<t0rn::Runtime>::remark_with_event {
            remark: msg.as_bytes().to_vec(),
        });
        RococoNet::execute_with(|| {
            for _ in 0..count {
                assert_ok!(rococo_runtime::XcmPallet::send_xcm(
                    Here,
                    Parachain(1),
                    Xcm(vec![Transact {
                        origin_type: OriginKind::SovereignAccount,
                        require_weight_at_most: INITIAL_BALANCE as u64,
                        call: remark.encode().into(),
                    }]),
                ));
            }
        });
    }

    fn parachain_receive_and_reset_events(received: bool) {
        T0rn::execute_with(|| {
            use t0rn::{Event, System};
            System::events()
                .iter()
                .for_each(|r| println!(">>> {:?}", r.event));

            if received {
                assert!(System::events().iter().any(|r| matches!(
                    r.event,
                    Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
                )));

                System::reset_events();
            } else {
                assert!(System::events().iter().all(|r| !matches!(
                    r.event,
                    Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
                )));
            }
        });
    }

    #[test]
    fn transfer_native_to_sovereign() {}

    #[test]
    fn transfer_asset_to_user() {
        Network::reset();

        let a_id = 1;
        let asset_amt = 100_u128.pow(12);

        T1rn::execute_with(|| {
            use t0rn::{Event, System};

            create_asset(a_id);

            log_all_events("T1rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::Assets(pallet_assets::Event::ForceCreated { .. })
            )));
        });

        T0rn::execute_with(|| {
            use t0rn::{Event, System};

            create_asset(a_id);
            mint_asset(a_id, ALICE, asset_amt);
            log_all_events("T0rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::Assets(pallet_assets::Event::Issued { asset_id, .. }) if asset_id == a_id
            )));

            let multi_asset = concrete_asset_pallet_assets(a_id as u128, asset_amt / 10);

            let dest_para = box MultiLocation::new(1, X1(Parachain(T1RN_PARA_ID))).versioned();
            let alice_on_dest = box MultiLocation::new(
                0,
                X1(AccountId32 {
                    network: Any,
                    id: <[u8; 32]>::from(ALICE),
                }),
            )
            .versioned();

            assert_ok!(t0rn::PolkadotXcm::limited_reserve_transfer_assets(
                Origin::signed(ALICE),
                dest_para,
                alice_on_dest,
                box VersionedMultiAssets::from(MultiAssets::from(vec![multi_asset])),
                0_u32,
                WeightLimit::Unlimited
            ));
            log_all_events("T0rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(_)))
            )));
        });

        T1rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T1rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::Assets(pallet_assets::Event::Issued {
                    asset_id,
                    owner: ALICE,
                    ..
                }) if asset_id == a_id
            )));
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
            )));
        });
    }

    fn concrete_asset_pallet_assets(index: u128, amt: u128) -> MultiAsset {
        MultiAsset {
            id: Concrete(MultiLocation::new(
                0,
                X2(PalletInstance(12), GeneralIndex(index)),
            )),
            fun: Fungible(amt),
        }
    }

    fn create_asset(id: u32) {
        let existential_balance = 1_u128;
        let asset_decimals = 12_u8;

        assert_ok!(t0rn::Assets::force_create(
            Origin::root(),
            id,
            MultiAddress::Id(ALICE),
            true,
            existential_balance,
        ));
        assert_ok!(t0rn::Assets::set_metadata(
            Origin::signed(ALICE),
            id,
            "ASSET".encode(),
            "ASST".encode(),
            asset_decimals,
        ));
    }

    fn mint_asset(id: u32, beneficiary: AccountId, amount: u128) {
        assert_ok!(t0rn::Assets::mint(
            Origin::signed(ALICE),
            id,
            MultiAddress::Id(beneficiary),
            amount.into()
        ));
    }

    #[test]
    fn transfer_asset_to_sovereign_then_back() {
        Network::reset();

        let a_id = 1;
        let asset_amt = 100_u128.pow(12);

        let t1rn_sovereign_addr = AccountId::from(hex_literal::hex!(
            "7369626c01000000000000000000000000000000000000000000000000000000"
        ));

        transfer_to_t1rn(a_id, asset_amt, t1rn_sovereign_addr);

        T0rn::execute_with(|| {
            use t0rn::{Event, System};

            let multi_asset = concrete_asset_pallet_assets(a_id as u128, asset_amt / 10);
            let dest_para = box MultiLocation::new(1, X1(Parachain(T1RN_PARA_ID))).versioned();

            // FIXME: this fails because https://substrate.stackexchange.com/questions/5017/allowtoplevelpaidexecutionfrom-not-supporting-descendorigin
            assert_ok!(t0rn::PolkadotXcm::send(
                Origin::signed(ALICE),
                dest_para,
                box VersionedXcm::V2(Xcm(vec![
                    WithdrawAsset(MultiAssets::from(vec![multi_asset.clone()])),
                    BuyExecution {
                        fees: multi_asset.clone(),
                        weight_limit: WeightLimit::Unlimited
                    },
                    InitiateTeleport {
                        assets: MultiAssetFilter::Wild(WildMultiAsset::All),
                        dest: MultiLocation::new(1, X1(Parachain(T0RN_PARA_ID))),
                        xcm: Xcm(vec![
                            BuyExecution {
                                fees: multi_asset,
                                weight_limit: WeightLimit::Unlimited
                            },
                            DepositAsset {
                                assets: MultiAssetFilter::Wild(WildMultiAsset::All),
                                max_assets: 0,
                                beneficiary: MultiLocation::new(
                                    0,
                                    X1(AccountId32 {
                                        network: Any,
                                        id: <[u8; 32]>::from(ALICE),
                                    })
                                )
                            }
                        ])
                    }
                ]))
            ));
            log_all_events("T0rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. })
            )));
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::PolkadotXcm(pallet_xcm::Event::Sent(ref _origin, ref _dest, ref _msg))
            )));
        });

        T1rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T1rn");

            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
            )));
        });
    }

    fn transfer_to_t1rn(a_id: u32, asset_amt: u128, t1rn_sovereign_addr: sp_runtime::AccountId32) {
        T1rn::execute_with(|| {
            use t0rn::{Event, System};

            create_asset(a_id);

            log_all_events("T1rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::Assets(pallet_assets::Event::ForceCreated { .. })
            )));
        });

        T0rn::execute_with(|| {
            use t0rn::{Event, System};

            create_asset(a_id);
            mint_asset(a_id, ALICE, asset_amt);
            log_all_events("T0rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::Assets(pallet_assets::Event::Issued { asset_id, .. }) if asset_id == a_id
            )));

            let multi_asset = concrete_asset_pallet_assets(a_id as u128, asset_amt / 10);

            let dest_para = box MultiLocation::new(1, X1(Parachain(T1RN_PARA_ID))).versioned();
            let t0rn_on_dest = box MultiLocation::new(1, X1(Parachain(T0RN_PARA_ID))).versioned();

            assert_ok!(t0rn::PolkadotXcm::limited_reserve_transfer_assets(
                Origin::signed(ALICE),
                dest_para,
                t0rn_on_dest,
                box VersionedMultiAssets::from(MultiAssets::from(vec![multi_asset])),
                0_u32,
                WeightLimit::Unlimited
            ));
            log_all_events("T0rn");
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(_)))
            )));
            println!("[T0rn] cleared events..");
            System::reset_events();
        });

        T1rn::execute_with(|| {
            use t0rn::{Event, System};
            log_all_events("T1rn");

            assert!(System::events().iter().any(|r| matches!(
                &r.event,
                Event::Assets(pallet_assets::Event::Issued {
                    asset_id,
                    owner,
                    ..
                }) if asset_id == &a_id && owner == &t1rn_sovereign_addr
            )));
            assert!(System::events().iter().any(|r| matches!(
                r.event,
                Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
            )));
            println!("[T1rn] cleared events..");
            System::reset_events();
        });
    }

    #[test]
    fn transfer_asset() {}

    #[test]
    fn transfer_reserve_then_instruct_to_send_back() {
        // Network::reset();
        //
        // T0rn::execute_with(|| {
        //     use t0rn::System;
        //
        //     assert_ok!(t0rn::PolkadotXcm::send_xcm(
        //         Here,
        //         MultiLocation::new(1, X1(Parachain(T1RN_PARA_ID))),
        //         Xcm(vec![
        //             WithdrawAsset(MultiAssets::from(vec![MultiAsset {
        //                 id: AssetId::Concrete(MultiLocation::here()),
        //                 fun: Fungibility::Fungible(10000000)
        //             }])),
        //             BuyExecution {
        //                 fees: MultiAsset {
        //                     id: AssetId::Concrete(MultiLocation::here()),
        //                     fun: Fungibility::Fungible(10000000)
        //                 },
        //                 weight_limit: WeightLimit::Unlimited
        //             },
        //             Transact {
        //                 origin_type: OriginKind::SovereignAccount,
        //                 require_weight_at_most: 10_000_000,
        //                 call: remark.encode().into(),
        //             }
        //         ]),
        //     ));
        //
        //     System::events()
        //         .iter()
        //         .for_each(|r| println!(">>> [T0rn] {:?}", r.event));
        // });
        //
        // T1rn::execute_with(|| {
        //     use t0rn::{Event, System};
        //     System::events()
        //         .iter()
        //         .for_each(|r| println!(">>> [T1rn] {:?}", r.event));
        //
        //     assert!(System::events().iter().any(|r| matches!(
        //         r.event,
        //         Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
        //     )));
        // });
    }
}
