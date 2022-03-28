use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_system::pallet_prelude::OriginFor;
use frame_system::Config;
use sp_std::vec::Vec;

pub struct LocalTrigger<T: Config> {
    /// Id of the contract which generated the side effects
    contract: T::AccountId,
    /// Side effects generated from the contract call
    side_effects: Vec<Vec<u8>>,
    /// Breakpoints by outbound message index
    round_breakpoints: Vec<u32>
}

pub trait OnLocalTrigger<T: Config> {
    fn on_local_trigger(
        origin: &OriginFor<T>,
        trigger: LocalTrigger<T>,
    ) -> DispatchResultWithPostInfo;
}
