//! Benchmarking setup for pallet-circuit-execution-delivery

use super::*;
use bp_test_utils::test_header;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};

use frame_system::{Origin, RawOrigin};

use t3rn_primitives::abi::GatewayABIConfig;

use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

use crate::{
    CurrentHeader, DefaultPolkadotLikeGateway, EthLikeKeccak256ValU32Gateway,
    EthLikeKeccak256ValU64Gateway, PolkadotLikeValU64Gateway,
};

pub use crate::Pallet as ExecDelivery;

pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
};

fn insert_default_xdns_record<T: pallet_xdns::Config>() {
    let url = b"some_url".to_vec();

    let gateway_abi: GatewayABIConfig = Default::default();
    let gateway_vendor = GatewayVendor::Substrate;
    let gateway_type = GatewayType::TxOnly(0);

    let gateway_genesis = GatewayGenesisConfig {
        modules_encoded: None,
        // signed_extensions: None,
        runtime_version: TEST_RUNTIME_VERSION,
        extrinsics_version: 0u8,
        genesis_hash: Default::default(),
    };
    pallet_xdns::Pallet::<T>::add_new_xdns_record(
        Origin::<T>::Root.into(),
        url,
        [0, 0, 0, 0],
        gateway_abi,
        gateway_vendor,
        gateway_type,
        gateway_genesis,
        vec![],
    )
    .unwrap();
}

benchmarks! {
    register_gateway_default_polka {
        let url = b"ws://localhost:9944".to_vec();
        let gateway_id = [0; 4];
        let gateway_abi: GatewayABIConfig = Default::default();

        let gateway_vendor = GatewayVendor::Substrate;
        let gateway_type = GatewayType::ProgrammableInternal(0);

        let _gateway_pointer = GatewayPointer {
            id: [0; 4],
            vendor: GatewayVendor::Substrate,
            gateway_type: GatewayType::ProgrammableInternal(0),
        };

        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded: None,
            // signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, DefaultPolkadotLikeGateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
    verify{}

    register_gateway_polka_u64 {
        let url = b"ws://localhost:9944".to_vec();
        let gateway_id = [0; 4];
        let gateway_abi: GatewayABIConfig = Default::default();

        let gateway_vendor = GatewayVendor::Substrate;
        let gateway_type = GatewayType::ProgrammableInternal(0);

        let _gateway_pointer = GatewayPointer {
            id: [0; 4],
            vendor: GatewayVendor::Substrate,
            gateway_type: GatewayType::ProgrammableInternal(0),
        };

        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded: None,
            // signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, PolkadotLikeValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
    verify{}

    register_gateway_default_eth {
        let url = b"ws://localhost:9944".to_vec();
        let gateway_id = [0; 4];
        let gateway_abi: GatewayABIConfig = Default::default();

        let gateway_vendor = GatewayVendor::Substrate;
        let gateway_type = GatewayType::ProgrammableInternal(0);

        let _gateway_pointer = GatewayPointer {
            id: [0; 4],
            vendor: GatewayVendor::Substrate,
            gateway_type: GatewayType::ProgrammableInternal(0),
        };

        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded: None,
            // signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU32Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
    verify{}

    register_gateway_eth_u64 {
        let url = b"ws://localhost:9944".to_vec();
        let gateway_id = [0; 4];
        let gateway_abi: GatewayABIConfig = Default::default();

        let gateway_vendor = GatewayVendor::Substrate;
        let gateway_type = GatewayType::ProgrammableInternal(0);

        let _gateway_pointer = GatewayPointer {
            id: [0; 4],
            vendor: GatewayVendor::Substrate,
            gateway_type: GatewayType::ProgrammableInternal(0),
        };

        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded: None,
            // signed_extensions: None,
            runtime_version: TEST_RUNTIME_VERSION,
            genesis_hash: Default::default(),
            extrinsics_version: 0u8,
        };

        let first_header: CurrentHeader<T, EthLikeKeccak256ValU64Gateway> = test_header(0u32.into());

        let authorities = Some(vec![]);
    }: { Pallet::<T>::register_gateway(RawOrigin::Root.into(), url, gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, first_header.encode(), authorities, vec![]).unwrap()}
    verify{}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::Test;
    use crate::tests::new_test_ext;
    use frame_support::assert_ok;

    #[test]
    fn benchmark_register_gateway_default_polka() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_default_polka::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_polka_u64() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_polka_u64::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_default_eth() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_default_eth::<Test>());
        })
    }

    #[test]
    fn benchmark_register_gateway_eth_u64() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_gateway_eth_u64::<Test>());
        })
    }
}

impl_benchmark_test_suite!(
    ExecDelivery,
    crate::tests::new_test_ext(),
    crate::mock::Test
);
