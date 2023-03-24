#[cfg(test)]
mod tests {
    use circuit_mock_runtime::{ExtBuilder, Portal, *};
    use frame_support::assert_ok;
    use t3rn_primitives::{portal::Portal as PortalT, GatewayVendor};

    fn test_get_latest_finalized_header(vendor: GatewayVendor) {
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
                let result = Portal::get_latest_finalized_header(gateway_id);
                assert_ok!(result);
            });
    }

    #[test]
    fn test_get_latest_finalized_header_rococo() {
        test_get_latest_finalized_header(GatewayVendor::Rococo);
    }

    #[test]
    fn test_get_latest_finalized_header_kusama() {
        test_get_latest_finalized_header(GatewayVendor::Kusama);
    }

    #[test]
    fn test_get_latest_finalized_header_polkadot() {
        test_get_latest_finalized_header(GatewayVendor::Polkadot);
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
                assert_ok!(result);
                assert_eq!(result.unwrap(), Some(0));
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
}
