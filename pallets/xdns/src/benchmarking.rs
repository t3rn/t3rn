// //! Benchmarking setup for pallet-xdns
//
// use super::*;
// use crate::Pallet as XDNS;
// use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
// use frame_system::RawOrigin;
// use sp_std::prelude::*;
// use t3rn_primitives::{xdns::Xdns, TokenSysProps};
//
// use sp_runtime::create_runtime_str;
// use sp_version::RuntimeVersion;
//
// const USER_SEED: u32 = 999666;
// pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
//     spec_name: create_runtime_str!("test-runtime"),
//     impl_name: create_runtime_str!("test-runtime"),
//     authoring_version: 1,
//     spec_version: 1,
//     impl_version: 1,
//     apis: sp_version::create_apis_vec!([]),
//     transaction_version: 1,
//     state_version: 0,
// };
//
// benchmarks! {
//     add_new_xdns_record {
//         let url = b"some_url".to_vec();
//         let gateway_id = b"test";
//         let gateway_abi: GatewayABIConfig = Default::default();
//
//         let gateway_vendor = GatewayVendor::Rococo;
//         let gateway_type = GatewayType::TxOnly(0);
//
//         let gateway_genesis = GatewayGenesisConfig {
//             modules_encoded: None,
//             extrinsics_version: 0u8,
//             genesis_hash: Default::default(),
//         };
//
//         let gateway_sys_props = TokenSysProps {
//             ss58_format: 0,
//             token_symbol: Encode::encode(""),
//             token_decimals: 0,
//         };
//     }: _(RawOrigin::Root, url, *gateway_id, gateway_abi, gateway_vendor, gateway_type, gateway_genesis, gateway_sys_props, vec![])
//     verify {
//         assert!(
//             XDNSRegistry::<T>::get(T::Hashing::hash(b"test"))
//                 .is_some()
//         );
//     }
//
//     update_ttl {
//         let url = b"some_url".to_vec();
//
//         let gateway_id = b"gate";
//         let gateway_abi: GatewayABIConfig = Default::default();
//         let gateway_vendor = GatewayVendor::Rococo;
//         let gateway_type = GatewayType::TxOnly(0);
//
//         let gateway_genesis = GatewayGenesisConfig {
//             modules_encoded: None,
//             extrinsics_version: 0u8,
//             genesis_hash: Default::default(),
//         };
//
//         let gateway_sys_props = TokenSysProps {
//             ss58_format: 0,
//             token_symbol: Encode::encode(""),
//             token_decimals: 0,
//         };
//
//         XDNS::<T>::add_new_xdns_record(
//             RawOrigin::Root.into(),
//             url,
//             *gateway_id,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             vec![],
//         )
//         .unwrap();
//
//         let gateway_hash = T::Hashing::hash(b"gate");
//
//     }: _(RawOrigin::Root, *b"gate", 2)
//     verify {
//         assert_eq!(
//             XDNSRegistry::<T>::get(gateway_hash)
//                 .unwrap()
//                 .last_finalized,
//             Some(2)
//         );
//     }
//
//     purge_xdns_record {
//         let requester: T::AccountId = account("TEST", 1u32, USER_SEED);
//         let url = b"some_url".to_vec();
//
//         let gateway_id = b"gate";
//         let gateway_abi: GatewayABIConfig = Default::default();
//         let gateway_vendor = GatewayVendor::Rococo;
//         let gateway_type = GatewayType::TxOnly(0);
//
//         let gateway_genesis = GatewayGenesisConfig {
//             modules_encoded: None,
//             extrinsics_version: 0u8,
//             genesis_hash: Default::default(),
//         };
//
//         let gateway_sys_props = TokenSysProps {
//             ss58_format: 0,
//             token_symbol: Encode::encode(""),
//             token_decimals: 0,
//         };
//
//         XDNS::<T>::add_new_xdns_record(
//             RawOrigin::Root.into(),
//             url,
//             *gateway_id,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             vec![],
//         )
//         .unwrap();
//
//         let gateway_hash = T::Hashing::hash(b"gate");
//     }: _(RawOrigin::Root, requester, gateway_hash.clone())
//     verify{
//         assert!(
//             XDNSRegistry::<T>::get(gateway_hash)
//                 .is_none()
//         );
//     }
//
//     best_available {
//         let url = b"some_url".to_vec();
//
//         let gateway_id = b"gate";
//         let gateway_abi: GatewayABIConfig = Default::default();
//         let gateway_vendor = GatewayVendor::Rococo;
//         let gateway_type = GatewayType::TxOnly(0);
//
//         let gateway_genesis = GatewayGenesisConfig {
//             modules_encoded: None,
//             extrinsics_version: 0u8,
//             genesis_hash: Default::default(),
//         };
//
//         let gateway_sys_props = TokenSysProps {
//             ss58_format: 0,
//             token_symbol: Encode::encode(""),
//             token_decimals: 0,
//         };
//
//         XDNS::<T>::add_new_xdns_record(
//             RawOrigin::Root.into(),
//             url,
//             *gateway_id,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             vec![],
//         )
//         .unwrap();
//     }: {XDNS::<T>::best_available(*b"gate")}
//     verify{}
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mock::Test;
//     use frame_support::assert_ok;
//     use sp_io::TestExternalities;
//
//     pub fn new_test_ext() -> TestExternalities {
//         let t = frame_system::GenesisConfig::default()
//             .build_storage::<Test>()
//             .unwrap();
//         TestExternalities::new(t)
//     }
//
//     #[test]
//     fn benchmark_add_new_xdns_record() {
//         new_test_ext().execute_with(|| {
//             assert_ok!(test_benchmark_add_new_xdns_record::<Test>());
//         });
//     }
//
//     #[test]
//     fn benchmark_update_ttl() {
//         new_test_ext().execute_with(|| {
//             assert_ok!(test_benchmark_update_ttl::<Test>());
//         });
//     }
//
//     #[test]
//     fn benchmark_purge_xdns_record() {
//         new_test_ext().execute_with(|| {
//             assert_ok!(test_benchmark_purge_xdns_record::<Test>());
//         });
//     }
//
//     #[test]
//     fn benchmark_best_available() {
//         new_test_ext().execute_with(|| {
//             assert_ok!(test_benchmark_best_available::<Test>());
//         });
//     }
// }
//
// impl_benchmark_test_suite!(
//     XDNS,
//     crate::benchmarking::tests::new_test_ext(),
//     crate::mock::Test
// );
