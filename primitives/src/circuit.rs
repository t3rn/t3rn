use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_system::{pallet_prelude::OriginFor, Config};
use sp_std::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalTrigger<T: Config> {
    /// Id of the contract which generated the side effects
    contract: T::AccountId,
    /// Side effects generated from the contract call
    side_effects: Vec<Vec<u8>>,
    /// Breakpoints by outbound message index
    round_breakpoints: Vec<u32>,
}

impl<T: Config> LocalTrigger<T> {
    pub fn new(
        contract: T::AccountId,
        side_effects: Vec<Vec<u8>>,
        round_breakpoints: Vec<u32>,
    ) -> Self {
        LocalTrigger {
            contract,
            side_effects,
            round_breakpoints,
        }
    }
}

pub trait OnLocalTrigger<T: Config> {
    fn on_local_trigger(origin: &OriginFor<T>, trigger: LocalTrigger<T>) -> DispatchResult;
}
