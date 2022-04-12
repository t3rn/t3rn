//! Benchmarking setup for pallet-xdns

use super::*;
use crate::Pallet as AccountManager;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

const USER_SEED: u32 = 999666;
pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
    state_version: 0,
};

benchmarks! {
    // TODO: implement me
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::Test;
    use frame_support::assert_ok;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        TestExternalities::new(t)
    }
}

impl_benchmark_test_suite!(
    AccountManager,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::Test
);
