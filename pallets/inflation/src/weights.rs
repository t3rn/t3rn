use frame_support::traits::Get;
use frame_support::weights::Weight;
use std::marker::PhantomData;

pub trait WeightInfo {
    fn update_round_on_initialize() -> Weight;
}

pub struct InflationWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for InflationWeight<T> {
    fn update_round_on_initialize() -> Weight {
        (0 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn update_round_on_initialize() -> Weight {
        0
    }
}
