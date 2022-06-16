use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn on_initialize() -> Weight;
    fn mint_for_round() -> Weight;
    fn claim_rewards() -> Weight;
    fn set_inflation() -> Weight;
    fn set_rewards_alloc() -> Weight;
    fn set_round_term() -> Weight;
    fn add_beneficiary() -> Weight;
    fn remove_beneficiary() -> Weight;
}

pub struct TreasuryWeight<T>(PhantomData<T>);

// TODO
impl<T: frame_system::Config> WeightInfo for TreasuryWeight<T> {
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

    fn set_round_term() -> Weight {
        0_u64
    }

    fn add_beneficiary() -> Weight {
        0_u64
    }

    fn remove_beneficiary() -> Weight {
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

    fn set_round_term() -> Weight {
        0_u64
    }

    fn add_beneficiary() -> Weight {
        0_u64
    }

    fn remove_beneficiary() -> Weight {
        0_u64
    }
}
