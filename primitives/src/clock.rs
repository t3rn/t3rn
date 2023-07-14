use crate::common::RoundInfo;
use frame_support::pallet_prelude::Weight;
use sp_runtime::traits::Zero;
use sp_std::marker::PhantomData;

pub trait Clock<T: frame_system::Config> {
    fn current_round() -> RoundInfo<T::BlockNumber>;
    fn round_duration() -> T::BlockNumber;
}

pub struct ClockMock<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> Clock<T> for ClockMock<T> {
    fn current_round() -> RoundInfo<T::BlockNumber> {
        Default::default()
    }

    fn round_duration() -> T::BlockNumber {
        Zero::zero()
    }
}

pub trait OnHookQueues<T: frame_system::Config> {
    // Process the queues for the given block number, handle the intervals internally.
    fn process(n: T::BlockNumber, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per week.
    fn process_weekly(n: T::BlockNumber, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per day.
    fn process_daily(n: T::BlockNumber, hook_weight_limit: Weight) -> Weight;
    // Process the queues once per hour.
    fn process_hourly(n: T::BlockNumber, hook_weight_limit: Weight) -> Weight;
}

pub struct EmptyOnHookQueues<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> OnHookQueues<T> for EmptyOnHookQueues<T> {
    fn process(_n: T::BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }

    fn process_weekly(_n: T::BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }

    fn process_daily(_n: T::BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }

    fn process_hourly(_n: T::BlockNumber, _hook_weight_limit: Weight) -> Weight {
        0
    }
}
