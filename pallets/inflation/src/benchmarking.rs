//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Inflation;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    mint {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), s)
    verify {
        assert_eq!(() Some(s));
    }

    impl_benchmark_test_suite!(Inflation, crate::mock::new_test_ext(), crate::mock::Test);
}
