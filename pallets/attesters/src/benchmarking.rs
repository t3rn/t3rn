// #![cfg(feature = "runtime-benchmarks")]
//
// use super::*;
// use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
// use frame_system::RawOrigin;
// // Import zero sp
//
// benchmarks! {
//     submit_attestation {
//         let s in 0 .. 4096;
//         let signature_len = 65;
//
//         let caller: T::AccountId = whitelisted_caller();
//         let ecdsa_key = ecdsa::Public::default();
//         let ed25519_key = ed25519::Public::default();
//         let sr25519_key = sr25519::Public::default();
//         let target_as_self: [u8; 4] = [3u8; 4];
//
//         // Register the caller as an attester
//         Attesters::<T>::insert(&caller, AttesterInfo {
//             key_ed: ed25519_key,
//             key_sr: sr25519_key,
//             key_ec: ecdsa_key,
//             commission: sp_runtime::traits::Zero::zero(),
//             index: 0,
//         });
//
//         let message = vec![1; s as usize];
//         let signature = vec![0; signature_len as usize];
//
//     }: _(RawOrigin::Signed(caller.clone()), message, signature.into(), target_as_self)
//     verify {
//         // Ensure the event was emitted
//         // System::assert_last_event::<T>(Event::AttestationSubmitted(caller).into());
//
//         Ok(())
//     }
// }
//
// impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
