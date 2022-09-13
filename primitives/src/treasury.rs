use crate::common::RoundInfo;

pub trait Treasury<T: frame_system::Config> {
    fn current_round() -> RoundInfo<T::BlockNumber>;
}
