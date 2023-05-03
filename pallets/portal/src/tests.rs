#[cfg(test)]
mod tests {
    use circuit_mock_runtime::{ExtBuilder, Portal, *};
    use codec::Encode;
    use frame_support::assert_ok;
    use pallet_grandpa_finality_verifier::{
        bridges::test_utils::{authorities, test_header_with_correct_parent},
        types::RelaychainRegistrationData,
    };

    use sp_core::H256;
    use t3rn_primitives::{
        portal::{HeaderResult, HeightResult, Portal as PortalT},
        GatewayVendor, SpeedMode,
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

    fn test_initialize_grandpa_light_client(
        vendor: GatewayVendor,
        registration_data: RelaychainRegistrationData<AccountId>,
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
                let result =
                    Portal::initialize(Origin::root(), gateway_id, registration_data.encode());

                assert_ok!(result);
            });
    }

    #[test]
    fn test_initialize_rococo() {
        test_initialize_grandpa_light_client(
            GatewayVendor::Rococo,
            get_test_initialize_genesis_data(),
        );
    }

    #[test]
    fn test_initialize_kusama() {
        test_initialize_grandpa_light_client(
            GatewayVendor::Kusama,
            get_test_initialize_genesis_data(),
        );
    }

    #[test]
    fn test_initialize_polkadot() {
        test_initialize_grandpa_light_client(
            GatewayVendor::Polkadot,
            get_test_initialize_genesis_data(),
        );
    }

    fn test_get_latest_finalized_header(
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
                if let Some(registration_data) = maybe_registration_data.clone() {
                    let result =
                        Portal::initialize(Origin::root(), gateway_id, registration_data.encode());
                    assert_ok!(result);
                }
                let result = Portal::get_latest_finalized_header(gateway_id);

                assert_ok!(result.clone());

                if let Some(_registration_data) = maybe_registration_data {
                    assert_eq!(
                        result,
                        Ok(HeaderResult::Header(vec![
                            220, 221, 137, 146, 125, 138, 52, 142, 0, 37, 126, 30, 204, 134, 23,
                            244, 94, 219, 81, 24, 239, 255, 62, 162, 249, 150, 27, 42, 217, 183,
                            105, 10
                        ]))
                    );
                }
            });
    }

    #[test]
    fn test_get_latest_finalized_header_rococo() {
        test_get_latest_finalized_header(GatewayVendor::Rococo, None);
    }

    #[test]
    fn test_get_latest_finalized_header_kusama() {
        test_get_latest_finalized_header(GatewayVendor::Kusama, None);
    }

    #[test]
    fn test_get_latest_finalized_header_polkadot() {
        test_get_latest_finalized_header(GatewayVendor::Polkadot, None);
    }

    #[test]
    fn test_get_latest_finalized_header_rococo_with_initialize() {
        test_get_latest_finalized_header(
            GatewayVendor::Rococo,
            Some(get_test_initialize_genesis_data()),
        );
    }

    #[test]
    fn test_get_latest_finalized_header_kusama_with_initialize() {
        test_get_latest_finalized_header(
            GatewayVendor::Kusama,
            Some(get_test_initialize_genesis_data()),
        );
    }

    #[test]
    fn test_get_latest_finalized_header_polkadot_with_initialize() {
        test_get_latest_finalized_header(
            GatewayVendor::Polkadot,
            Some(get_test_initialize_genesis_data()),
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
                let result = Portal::get_latest_finalized_height(gateway_id);
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

    fn test_get_current_epoch(vendor: GatewayVendor) {
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
                let result = Portal::get_current_epoch(gateway_id);
                assert_ok!(result);
            });
    }

    #[test]
    fn test_get_current_epoch_rococo() {
        test_get_current_epoch(GatewayVendor::Rococo);
    }

    #[test]
    fn test_get_current_epoch_kusama() {
        test_get_current_epoch(GatewayVendor::Kusama);
    }

    #[test]
    fn test_get_current_epoch_polkadot() {
        test_get_current_epoch(GatewayVendor::Polkadot);
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
    fn test_read_fast_confirmation_offset_rococo() {
        test_read_fast_confirmation_offset(GatewayVendor::Rococo);
    }

    #[test]
    fn test_read_fast_confirmation_offset_kusama() {
        test_read_fast_confirmation_offset(GatewayVendor::Kusama);
    }

    #[test]
    fn test_read_fast_confirmation_offset_polkadot() {
        test_read_fast_confirmation_offset(GatewayVendor::Polkadot);
    }

    #[test]
    fn test_read_rational_confirmation_offset_rococo() {
        test_read_rational_confirmation_offset(GatewayVendor::Rococo);
    }

    #[test]
    fn test_read_rational_confirmation_offset_kusama() {
        test_read_rational_confirmation_offset(GatewayVendor::Kusama);
    }

    #[test]
    fn test_read_rational_confirmation_offset_polkadot() {
        test_read_rational_confirmation_offset(GatewayVendor::Polkadot);
    }

    fn test_read_rational_confirmation_offset(vendor: GatewayVendor) {
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
                let result = Portal::read_rational_confirmation_offset(gateway_id);
                assert_ok!(result);
                assert_eq!(result.unwrap(), 0);
            });
    }

    #[test]
    fn test_read_finalized_confirmation_offset_rococo() {
        test_read_finalized_confirmation_offset(GatewayVendor::Rococo);
    }

    #[test]
    fn test_read_finalized_confirmation_offset_kusama() {
        test_read_finalized_confirmation_offset(GatewayVendor::Kusama);
    }

    #[test]
    fn test_read_finalized_confirmation_offset_polkadot() {
        test_read_finalized_confirmation_offset(GatewayVendor::Polkadot);
    }

    fn test_read_finalized_confirmation_offset(vendor: GatewayVendor) {
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
                let result = Portal::read_finalized_confirmation_offset(gateway_id);
                assert_ok!(result);
                assert_eq!(result.unwrap(), 0);
            });
    }

    #[test]
    fn test_read_epoch_offset_rococo() {
        test_read_epoch_offset(GatewayVendor::Rococo);
    }

    #[test]
    fn test_read_epoch_offset_kusama() {
        test_read_epoch_offset(GatewayVendor::Kusama);
    }

    #[test]
    fn test_read_epoch_offset_polkadot() {
        test_read_epoch_offset(GatewayVendor::Polkadot);
    }

    fn test_read_epoch_offset(vendor: GatewayVendor) {
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
                let result = Portal::read_epoch_offset(gateway_id);
                assert_ok!(result);
                assert_eq!(result.unwrap(), 2400);
            });
    }

    fn test_read_fast_confirmation_offset(vendor: GatewayVendor) {
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
                let result = Portal::read_fast_confirmation_offset(gateway_id);
                assert_ok!(result);
                assert_eq!(result.unwrap(), 0);
            });
    }

    #[test]
    fn test_header_speed_mode_satisfied_rococo() {
        test_header_speed_mode_satisfied(GatewayVendor::Rococo, None);
    }

    #[test]
    fn test_header_speed_mode_satisfied_kusama() {
        test_header_speed_mode_satisfied(GatewayVendor::Kusama, None);
    }

    #[test]
    fn test_header_speed_mode_satisfied_polkadot() {
        test_header_speed_mode_satisfied(GatewayVendor::Polkadot, None);
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
