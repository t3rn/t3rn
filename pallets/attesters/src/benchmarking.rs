#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    submit_attestation {
        let s in 0 .. 4096;
        let signature_len = 65;

        let caller: T::AccountId = whitelisted_caller();
        let ecdsa_key = ecdsa::Public::default();
        let ed25519_key = ed25519::Public::default();
        let sr25519_key = sr25519::Public::default();

        // Register the caller as an attester
        Attesters::<T>::insert(&caller, AttesterInfo {
            ecdsa_key,
            ed25519_key,
            sr25519_key,
        });

        let message = vec![1; s as usize];
        let signature = vec![0; signature_len as usize];

    }: _(RawOrigin::Signed(caller.clone()), message, signature)
    verify {
        // Ensure the event was emitted
        assert_last_event::<T>(Event::AttestationSubmitted(caller).into());
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
