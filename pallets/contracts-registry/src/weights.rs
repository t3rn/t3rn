//! Autogenerated weights for pallet_contracts_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-08, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/circuit
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_contracts_registry
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --raw
// --template=../benchmarking/frame-weight-template.hbs
// --output
// .

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_contracts_registry.
pub trait WeightInfo {
    fn add_new_contract() -> Weight;
    fn purge() -> Weight;
    fn fetch_contracts() -> Weight;
}

/// Weights for pallet_contracts_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_new_contract() -> Weight {
        Weight::from_ref_time(52_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn purge() -> Weight {
        Weight::from_ref_time(37_000_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn fetch_contracts() -> Weight {
        Weight::from_ref_time(53_000_000_u64).saturating_add(T::DbWeight::get().reads(4_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn add_new_contract() -> Weight {
        Weight::from_ref_time(52_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn purge() -> Weight {
        Weight::from_ref_time(37_000_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn fetch_contracts() -> Weight {
        Weight::from_ref_time(53_000_000_u64).saturating_add(RocksDbWeight::get().reads(4_u64))
    }
}
