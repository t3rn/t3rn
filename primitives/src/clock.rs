use crate::common::RoundInfo;
use frame_support::{pallet_prelude::Weight, sp_runtime::traits::Zero};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::marker::PhantomData;

pub trait Clock<T: frame_system::Config> {
    fn current_round() -> RoundInfo<BlockNumberFor<T>>;
    fn round_duration() -> BlockNumberFor<T>;
}

pub struct ClockMock<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> Clock<T> for ClockMock<T> {
    fn current_round() -> RoundInfo<BlockNumberFor<T>> {
        Default::default()
    }

    fn round_duration() -> BlockNumberFor<T> {
        Zero::zero()
    }
}

pub trait OnHookQueues<T: frame_system::Config> {
    // Process the queues for the given block number, handle the intervals internally.
    fn process(n: BlockNumberFor<T>, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per week.
    fn process_weekly(n: BlockNumberFor<T>, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per 2 weeks.
    fn process_bi_weekly(n: BlockNumberFor<T>, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per day.
    fn process_daily(n: BlockNumberFor<T>, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per hour.
    fn process_hourly(n: BlockNumberFor<T>, hook_weight_limit: Weight) -> Weight;
}

pub struct EmptyOnHookQueues<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> OnHookQueues<T> for EmptyOnHookQueues<T> {
    fn process(_n: BlockNumberFor<T>, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_weekly(_n: BlockNumberFor<T>, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_bi_weekly(_n: BlockNumberFor<T>, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_daily(_n: BlockNumberFor<T>, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }

    fn process_hourly(_n: BlockNumberFor<T>, _hook_weight_limit: Weight) -> Weight {
        Zero::zero()
    }
}
