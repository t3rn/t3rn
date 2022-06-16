use super::*;

#[allow(unused)]
use crate::Pallet as Treasury;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

// TODO
benchmarks! {
    mint {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), s)
    verify {
        assert_eq!(() Some(s));
    }

    impl_benchmark_test_suite!(Treasury, crate::mock::new_test_ext(), crate::mock::Test);
}
