use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_system::pallet_prelude::OriginFor;
use frame_system::Config;
use sp_std::vec::Vec;

pub trait OnLocalTrigger<T: Config> {
    fn on_local_trigger(
        origin: &OriginFor<T>,
        side_effects: Vec<Vec<u8>>,
    ) -> DispatchResultWithPostInfo;
}
