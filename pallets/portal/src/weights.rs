//! Autogenerated weights for pallet_xdns
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_xdns
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

/// Weight functions needed for pallet_xdns.
pub trait WeightInfo {
    fn register_gateway() -> Weight;
    fn set_owner() -> Weight;
    fn set_operational() -> Weight;
    fn submit_headers() -> Weight;
}

/// Weights for pallet_xdns using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn register_gateway() -> Weight {
        Weight::from_ref_time(72_795_000_u64)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn set_owner() -> Weight {
        Weight::from_ref_time(73_255_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn set_operational() -> Weight {
        Weight::from_ref_time(58_912_000_u64)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn submit_headers() -> Weight {
        Weight::from_ref_time(25_265_000_u64).saturating_add(T::DbWeight::get().reads(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn register_gateway() -> Weight {
        Weight::from_ref_time(72_795_000_u64)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn set_owner() -> Weight {
        Weight::from_ref_time(73_255_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn set_operational() -> Weight {
        Weight::from_ref_time(58_912_000_u64)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn submit_headers() -> Weight {
        Weight::from_ref_time(25_265_000_u64).saturating_add(RocksDbWeight::get().reads(1_u64))
    }
}
