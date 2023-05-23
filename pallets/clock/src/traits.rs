use sp_std::marker::PhantomData;

use frame_support::pallet_prelude::Weight;
pub trait OnHookQueues<T: frame_system::Config> {
    fn process(n: T::BlockNumber, hook_weight_limit: Weight) -> Weight {
        Default::default()
    }
}

pub struct EmptyOnHookQueues<T> {
    _phantom: PhantomData<T>,
}

impl<T: frame_system::Config> OnHookQueues<T> for EmptyOnHookQueues<T> {}
