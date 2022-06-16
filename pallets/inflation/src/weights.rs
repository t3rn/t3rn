use frame_support::weights::Weight;
use std::marker::PhantomData;

pub trait WeightInfo {
    fn on_initialize() -> Weight;
    fn mint_for_round() -> Weight;
    fn claim_rewards() -> Weight;
    fn set_inflation() -> Weight;
    fn set_rewards_alloc() -> Weight;
    fn set_blocks_per_round() -> Weight;
}

pub struct InflationWeight<T>(PhantomData<T>);

// TODO
impl<T: frame_system::Config> WeightInfo for InflationWeight<T> {
    fn on_initialize() -> Weight {
        0_u64
    }

    fn mint_for_round() -> Weight {
        0_u64
    }

    fn claim_rewards() -> Weight {
        0_u64
    }

    fn set_inflation() -> Weight {
        0_u64
    }

    fn set_rewards_alloc() -> Weight {
        0_u64
    }

    fn set_blocks_per_round() -> Weight {
        0_u64
    }
}

// TODO
impl WeightInfo for () {
    fn on_initialize() -> Weight {
        0_u64
    }

    fn mint_for_round() -> Weight {
        0_u64
    }

    fn claim_rewards() -> Weight {
        0_u64
    }

    fn set_inflation() -> Weight {
        0_u64
    }

    fn set_rewards_alloc() -> Weight {
        0_u64
    }

    fn set_blocks_per_round() -> Weight {
        0_u64
    }
}
