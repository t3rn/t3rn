use crate::common::RoundInfo;
use frame_support::pallet_prelude::*;
use sp_runtime::traits::Zero;

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
