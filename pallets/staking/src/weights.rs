use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn on_initialize() -> Weight;
}

pub struct TreasuryWeight<T>(PhantomData<T>);

// TODO
impl<T: frame_system::Config> WeightInfo for TreasuryWeight<T> {
    fn on_initialize() -> Weight {
        0_u64
    }
}

// TODO
impl WeightInfo for () {
    fn on_initialize() -> Weight {
        0_u64
    }
}
