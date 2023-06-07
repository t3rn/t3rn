#[cfg(test)]
mod tests {
    use ::pallet_eth2_finality_verifier::{
        mock::{generate_epoch_update, generate_initialization},
        types::EthereumInitializationData,
    };
    use circuit_mock_runtime::{ExtBuilder, Portal, *};
    use circuit_test_utils::replay::*;
    use codec::Encode;
    use frame_support::assert_ok;
    use pallet_grandpa_finality_verifier::{
        bridges::test_utils::{authorities, test_header_with_correct_parent},
        mock::produce_mock_headers_range,
        types::RelaychainRegistrationData,
    };

    use pallet_eth2_finality_verifier;
    use std::fs;
    use t3rn_primitives::xdns::Xdns;

    use sp_core::H256;

    use t3rn_primitives::{
        portal::{HeaderResult, HeightResult, Portal as PortalT},
        EthereumToken, ExecutionVendor, GatewayVendor, SpeedMode, TokenInfo,
    };

    fn get_test_initialize_genesis_data() -> RelaychainRegistrationData<AccountId> {
        let genesis: Header = test_header_with_correct_parent(0, None);

        RelaychainRegistrationData::<AccountId> {
            authorities: authorities(),
            first_header: genesis.encode(),
            authority_set_id: 1,
            owner: ALICE,
        }
    }

    fn test_initialize_and_submit_grandpa(
        vendor: GatewayVendor,
        registration_data: RelaychainRegistrationData<AccountId>,
        submission_data: Vec<u8>,
    ) {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let gateway_id = match vendor {
                    GatewayVendor::Rococo => [0, 0, 0, 0],
                    GatewayVendor::Kusama => *b"ksma",
                    GatewayVendor::Polkadot => *b"pdot",
                    _ => unreachable!(),
                };
                assert_ok!(Portal::initialize(
                    Origin::root(),
                    gateway_id,
                    registration_data.encode()
                ));

                let result = Portal::get_latest_finalized_header(gateway_id);

                assert_ok!(result);

                match Portal::get_latest_finalized_header(gateway_id) {
                    Ok(HeaderResult::Header(header)) => {
                        assert_eq!(
                            header,
                            [
                                220, 221, 137, 146, 125, 138, 52, 142, 0, 37, 126, 30, 204, 134,
                                23, 244, 94, 219, 81, 24, 239, 255, 62, 162, 249, 150, 27, 42, 217,
                                183, 105, 10
                            ]
                        );
                    },
                    _ => panic!("Header not found"),
                }

                assert_eq!(
                    Portal::get_finalized_height(gateway_id),
                    Ok(HeightResult::Height(0))
                );

                assert_eq!(
                    Portal::get_rational_height(gateway_id),
                    Ok(HeightResult::Height(0))
                );

                assert_eq!(
                    Portal::get_fast_height(gateway_id),
                    Ok(HeightResult::Height(0))
                );

                assert_ok!(Portal::submit_encoded_headers(gateway_id, submission_data));
                match Portal::get_latest_finalized_header(gateway_id) {
                    Ok(HeaderResult::Header(header)) => {
                        assert_eq!(
                            header,
                            [
                                172, 9, 75, 83, 28, 226, 187, 127, 149, 100, 145, 226, 203, 67, 35,
                                94, 211, 209, 132, 186, 118, 10, 175, 12, 86, 9, 184, 148, 239,
                                120, 180, 177
                            ]
                        );
                    },
                    _ => panic!("Header not found"),
                }

                assert_eq!(
                    Portal::get_finalized_height(gateway_id),
                    Ok(HeightResult::Height(5))
                );

                assert_eq!(
                    Portal::get_rational_height(gateway_id),
                    Ok(HeightResult::Height(5))
                );

                assert_eq!(
                    Portal::get_fast_height(gateway_id),
                    Ok(HeightResult::Height(5))
                );
            });
    }

    #[test]
    fn test_initialize_and_submit_ethereum() {
        let init = generate_initialization(None, None);

        let submission_data = generate_epoch_update(
            0,
            3,
            Some(
                init.checkpoint
                    .justified_beacon
                    .hash_tree_root::<Runtime>()
                    .unwrap(),
            ),
            Some(
                init.checkpoint
                    .finalized_beacon
                    .hash_tree_root::<Runtime>()
                    .unwrap(),
            ),
            None,
            None,
        );

        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                assert_ok!(Portal::initialize(Origin::root(), *b"eth2", init.encode()));

                assert_eq!(
                    Portal::get_latest_finalized_header(*b"eth2"),
                    Ok(HeaderResult::NotActive)
                ); // need to submit first epoch

                assert_eq!(
                    Portal::get_finalized_height(*b"eth2"),
                    Ok(HeightResult::Height(31))
                );

                assert_eq!(
                    Portal::get_rational_height(*b"eth2"),
                    Ok(HeightResult::Height(31))
                );

                assert_eq!(
                    Portal::get_fast_height(*b"eth2"),
                    Ok(HeightResult::Height(63))
                );

                assert_ok!(Portal::submit_encoded_headers(
                    *b"eth2",
                    submission_data.encode()
                ));

                assert_eq!(
                    Portal::get_finalized_height(*b"eth2"),
                    Ok(HeightResult::Height(63))
                );

                assert_eq!(
                    Portal::get_rational_height(*b"eth2"),
                    Ok(HeightResult::Height(63))
                );

                assert_eq!(
                    Portal::get_fast_height(*b"eth2"),
                    Ok(HeightResult::Height(95))
                );
            });
    }

    fn test_register_ethereum_light_client() {
        let init = generate_initialization(None, None);

        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let result = Portal::register_gateway(
                    Origin::root(),
                    [44u8; 4],
                    [0u8; 4],
                    GatewayVendor::Ethereum,
                    ExecutionVendor::EVM,
                    t3rn_abi::Codec::Rlp,
                    None,
                    None,
                    vec![(*b"tran", None)],
                    TokenInfo::Ethereum(EthereumToken {
                        address: Some([0u8; 20]),
                        decimals: 0,
                        symbol: vec![0u8; 1],
                    }),
                    init.encode(),
                );

                assert_ok!(result);
            });
    }

    #[test]
    fn test_register_ethereum() {
        test_register_ethereum_light_client();
    }

    #[test]
    fn test_initialize_and_submit_rococo() {
        let data = produce_mock_headers_range(1, 5);
        test_initialize_and_submit_grandpa(
            GatewayVendor::Rococo,
            get_test_initialize_genesis_data(),
            data.encode(),
        );
    }

    #[test]
    fn test_initialize_and_submit_kusama() {
        let data = produce_mock_headers_range(1, 5);
        test_initialize_and_submit_grandpa(
            GatewayVendor::Kusama,
            get_test_initialize_genesis_data(),
            data.encode(),
        );
    }

    #[test]
    fn test_initialize_and_submit_polkadot() {
        let data = produce_mock_headers_range(1, 5);
        test_initialize_and_submit_grandpa(
            GatewayVendor::Polkadot,
            get_test_initialize_genesis_data(),
            data.encode(),
        );
    }

    fn test_get_latest_finalized_height(vendor: GatewayVendor) {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let gateway_id = match vendor {
                    GatewayVendor::Rococo => [0, 0, 0, 0],
                    GatewayVendor::Kusama => *b"ksma",
                    GatewayVendor::Polkadot => *b"pdot",
                    _ => unreachable!(),
                };
                let result = Portal::get_finalized_height(gateway_id);
                assert_ok!(result.clone());
                assert_eq!(result.unwrap(), HeightResult::Height(0));
            });
    }

    #[test]
    fn test_get_latest_finalized_height_rococo() {
        test_get_latest_finalized_height(GatewayVendor::Rococo);
    }

    #[test]
    fn test_get_latest_finalized_height_kusama() {
        test_get_latest_finalized_height(GatewayVendor::Kusama);
    }

    #[test]
    fn test_get_latest_finalized_height_polkadot() {
        test_get_latest_finalized_height(GatewayVendor::Polkadot);
    }

    #[test]
    fn test_turn_on_rococo() {
        test_turn_on(GatewayVendor::Rococo);
    }

    #[test]
    fn test_turn_on_kusama() {
        test_turn_on(GatewayVendor::Kusama);
    }

    #[test]
    fn test_turn_on_polkadot() {
        test_turn_on(GatewayVendor::Polkadot);
    }

    fn test_turn_on(vendor: GatewayVendor) {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let gateway_id = match vendor {
                    GatewayVendor::Rococo => [0, 0, 0, 0],
                    GatewayVendor::Kusama => *b"ksma",
                    GatewayVendor::Polkadot => *b"pdot",
                    _ => unreachable!(),
                };
                let origin = Origin::root();
                let result = Portal::turn_on(origin, gateway_id);
                assert_ok!(result);
            });
    }

    #[test]
    fn test_header_speed_mode_satisfied_rococo_with_initialize() {
        test_header_speed_mode_satisfied(
            GatewayVendor::Rococo,
            Some(get_test_initialize_genesis_data()),
        );
    }

    #[test]
    fn test_header_speed_mode_satisfied_kusama_with_initialize() {
        test_header_speed_mode_satisfied(
            GatewayVendor::Kusama,
            Some(get_test_initialize_genesis_data()),
        );
    }

    #[test]
    #[ignore]
    fn run_e2e_tests() {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let mut paths: Vec<_> = fs::read_dir("fixtures/")
                    .unwrap()
                    .map(|r| r.unwrap())
                    .collect();
                paths.sort_by_key(|dir| dir.path());

                for entry in paths {
                    let path = entry.path();
                    let file = fs::read_to_string(&path).unwrap();
                    let data: ExtrinsicParam = serde_json::from_str(&file).unwrap();
                    assert_ok!(replay_and_evaluate_extrinsic::<Runtime>(&data));
                }
            })
    }

    #[test]
    fn test_header_speed_mode_satisfied_polkadot_with_initialize() {
        test_header_speed_mode_satisfied(
            GatewayVendor::Polkadot,
            Some(get_test_initialize_genesis_data()),
        );
    }
    fn test_header_speed_mode_satisfied(
        vendor: GatewayVendor,
        maybe_registration_data: Option<RelaychainRegistrationData<AccountId>>,
    ) {
        ExtBuilder::default()
            .with_standard_sfx_abi()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let gateway_id = match vendor {
                    GatewayVendor::Rococo => [0, 0, 0, 0],
                    GatewayVendor::Kusama => *b"ksma",
                    GatewayVendor::Polkadot => *b"pdot",
                    _ => unreachable!(),
                };
                let header: Vec<u8> = match maybe_registration_data.clone() {
                    Some(registration_data) => {
                        let result = Portal::initialize(
                            Origin::root(),
                            gateway_id,
                            registration_data.encode(),
                        );
                        assert_ok!(result);

                        let result = Portal::get_latest_finalized_header(gateway_id);

                        assert_ok!(result.clone());

                        assert_eq!(
                            result,
                            Ok(HeaderResult::Header(vec![
                                220, 221, 137, 146, 125, 138, 52, 142, 0, 37, 126, 30, 204, 134,
                                23, 244, 94, 219, 81, 24, 239, 255, 62, 162, 249, 150, 27, 42, 217,
                                183, 105, 10
                            ]))
                        );
                        vec![
                            220, 221, 137, 146, 125, 138, 52, 142, 0, 37, 126, 30, 204, 134, 23,
                            244, 94, 219, 81, 24, 239, 255, 62, 162, 249, 150, 27, 42, 217, 183,
                            105, 10,
                        ]
                    },
                    None => H256::zero().encode(),
                };
                let speed_mode = SpeedMode::Fast;
                let result = Portal::header_speed_mode_satisfied(gateway_id, header, speed_mode);
                assert_ok!(result);

                let is_satisfied_res = result.unwrap();

                match maybe_registration_data {
                    Some(_) => assert!(is_satisfied_res),
                    None => assert!(!is_satisfied_res),
                }
            });
    }
}
