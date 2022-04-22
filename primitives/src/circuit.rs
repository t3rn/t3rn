use crate::{side_effect::FullSideEffect, xtx::LocalState};
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_system::{pallet_prelude::OriginFor, Config};
use sp_std::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalTrigger<T: Config> {
    /// Id of the contract which generated the side effects
    contract: T::AccountId,
    /// Side effects generated from the contract call
    pub submitted_side_effects: Vec<Vec<u8>>,
    pub maybe_xtx_id: Option<T::Hash>,
}

impl<T: Config> LocalTrigger<T> {
    pub fn new(
        contract: T::AccountId,
        submitted_side_effects: Vec<Vec<u8>>,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Self {
        LocalTrigger {
            contract,
            submitted_side_effects,
            maybe_xtx_id,
        }
    }
}

// TODO: remove u128 when we remove escrowtrait
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalStateExecutionView<T: Config> {
    pub execution_id: T::Hash,
    pub local_state: LocalState,
    pub full_side_effects: Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, u128>>>,
    pub steps_cnt: (u32, u32),
}

impl<T: Config> LocalStateExecutionView<T> {
    pub fn new(
        execution_id: T::Hash,
        local_state: LocalState,
        full_side_effects: Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, u128>>>,
        steps_cnt: (u32, u32),
    ) -> Self {
        LocalStateExecutionView {
            execution_id,
            local_state,
            full_side_effects,
            steps_cnt,
        }
    }
}

pub trait OnLocalTrigger<T: Config> {
    fn on_local_trigger(origin: &OriginFor<T>, trigger: LocalTrigger<T>) -> DispatchResult;

    fn load_local_state(
        origin: &OriginFor<T>,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Result<LocalStateExecutionView<T>, sp_runtime::DispatchError>;
}
