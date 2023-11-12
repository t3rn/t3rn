// use super::*;
// #[allow(unused)]
// use crate::Pallet as AccountManager;
// use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
// use frame_system::RawOrigin;
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
//     deposit {
//         let payee: T::AccountId = account("PAYEE", 1_u32, USER_SEED);
//         let escrow: T::AccountId = account("PAYEE", 2_u32, USER_SEED);
//         let recipient: T::AccountId = account("RECIPIENT", 3_u32, USER_SEED);
//         let _ = T::Currency::make_free_balance_be(&payee, BalanceOf::<T>::from(100_u32));
//         let _ = T::Currency::make_free_balance_be(&escrow, BalanceOf::<T>::from(10_u32));
//         let transfer_amt: BalanceOf<T> = BalanceOf::<T>::from(10_u32);
//     }: _(RawOrigin::Root, payee.clone(), recipient.clone(), transfer_amt)
//     verify {
//         assert!(pallet::ExecutionRegistry::<T>::contains_key(&0));
//     }
//
//
//     impl_benchmark_test_suite!(
//         AccountManager,
//         crate::benchmarking::tests::new_test_ext(),
//         crate::mock::Test
//     );
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
//     fn benchmark_deposit() {
//         new_test_ext().execute_with(|| {
//             bench_deposit();
//         })
//     }
// }
