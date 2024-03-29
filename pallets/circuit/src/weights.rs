//! Autogenerated weights for pallet_circuit_circuit_portal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-24, STEPS: `[50, ]`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/circuit
// benchmark
// --chain=dev
// --steps=50
// --repeat=100
// --pallet=pallet_circuit
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./src/circuit/src/weights.rs
// --template=../benchmarking/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_circuit_circuit_portal.
pub trait WeightInfo {
    fn on_local_trigger() -> Weight;
    fn on_extrinsic_trigger() -> Weight;
    fn bid_sfx() -> Weight;
    fn cancel_xtx() -> Weight;
    fn confirm_side_effect() -> Weight;
    fn execute_side_effects_with_xbi() -> Weight;
}

/// Storage: `XDNS::Gateways` (r:2 w:0)
/// Proof: `XDNS::Gateways` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::EpochHistory` (r:1 w:0)
/// Proof: `XDNS::EpochHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::VerifierOverviewStoreHistory` (r:1 w:0)
/// Proof: `XDNS::VerifierOverviewStoreHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::XExecSignals` (r:1 w:1)
/// Proof: `Circuit::XExecSignals` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::GatewaysOverviewStore` (r:1 w:1)
/// Proof: `XDNS::GatewaysOverviewStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::Tokens` (r:1 w:0)
/// Proof: `XDNS::Tokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Attesters::Batches` (r:1 w:0)
/// Proof: `Attesters::Batches` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::GatewaysOverviewStoreHistory` (r:1 w:1)
/// Proof: `XDNS::GatewaysOverviewStoreHistory` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `XDNS::SFXABIRegistry` (r:1 w:0)
/// Proof: `XDNS::SFXABIRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `AccountManager::PendingCharges` (r:1 w:1)
/// Proof: `AccountManager::PendingCharges` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::PendingXtxTimeoutsMap` (r:0 w:1)
/// Proof: `Circuit::PendingXtxTimeoutsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::LocalXtxStates` (r:0 w:1)
/// Proof: `Circuit::LocalXtxStates` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::PendingXtxBidsTimeoutsMap` (r:0 w:1)
/// Proof: `Circuit::PendingXtxBidsTimeoutsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::SFX2XTXLinksMap` (r:0 w:1)
/// Proof: `Circuit::SFX2XTXLinksMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
/// Storage: `Circuit::FullSideEffects` (r:0 w:1)
/// Proof: `Circuit::FullSideEffects` (`max_values`: None, `max_size`: None, mode: `Measured`)
fn single_order_weight<T: frame_system::Config>() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `1237`
    //  Estimated: `7177`
    // Minimum execution time: 1_019_000_000 picoseconds.
    Weight::from_parts(1_443_000_000, 0)
        .saturating_add(Weight::from_parts(0, 7177))
        .saturating_add(T::DbWeight::get().reads(11))
        .saturating_add(T::DbWeight::get().writes(9))
}

/// Weights for pallet_circuit_circuit_portal using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn on_local_trigger() -> Weight {
        single_order_weight::<T>()
    }

    fn on_extrinsic_trigger() -> Weight {
        single_order_weight::<T>()
    }

    fn confirm_side_effect() -> Weight {
        single_order_weight::<T>()
    }

    fn cancel_xtx() -> Weight {
        single_order_weight::<T>()
    }

    fn bid_sfx() -> Weight {
        single_order_weight::<T>()
    }

    fn execute_side_effects_with_xbi() -> Weight {
        single_order_weight::<T>()
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn on_local_trigger() -> Weight {
        Weight::from_parts(6_984_000_u64, 0u64)
    }

    fn on_extrinsic_trigger() -> Weight {
        Weight::from_parts(60_000_000_u64, 0u64)
    }

    fn confirm_side_effect() -> Weight {
        Weight::from_parts(60_000_000_u64, 0u64)
    }

    fn cancel_xtx() -> Weight {
        Weight::from_parts(60_000_000_u64, 0u64)
    }

    fn bid_sfx() -> Weight {
        Weight::from_parts(60_000_000_u64, 0u64)
    }

    fn execute_side_effects_with_xbi() -> Weight {
        Weight::from_parts(60_000_000_u64, 0u64)
    }
}
