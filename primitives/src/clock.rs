use crate::common::RoundInfo;
use frame_support::pallet_prelude::*;

pub trait Clock<T: frame_system::Config> {
    fn current_round() -> RoundInfo<T::BlockNumber>;
}

pub struct ClockMock<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> Clock<T> for ClockMock<T> {
    fn current_round() -> RoundInfo<T::BlockNumber> {
        Default::default()
    }
}
