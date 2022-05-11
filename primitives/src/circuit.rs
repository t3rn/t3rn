use crate::{side_effect::FullSideEffect, transfers::EscrowedBalanceOf, xtx::LocalState};
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_system::{pallet_prelude::OriginFor, Config};
use sp_std::{fmt::Debug, vec::Vec};

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalTrigger<T: Config> {
    /// Id of the contract which generated the side effects
    pub contract: T::AccountId,
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

// TODO: provide full side effects
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalStateExecutionView<T: Config, E: crate::EscrowTrait<T>> {
    pub xtx_id: <T as Config>::Hash,
    pub local_state: LocalState,
    pub completed_full_side_effects:
        Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, E>>>>,
    pub steps_cnt: (u32, u32),
}

// pub struct HardenedSideEffect {}

impl<T: Config, E: crate::EscrowTrait<T>> LocalStateExecutionView<T, E> {
    pub fn new(
        xtx_id: <T as Config>::Hash,
        local_state: LocalState,
        completed_full_side_effects: Vec<
            Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, E>>>,
        >,
        steps_cnt: (u32, u32),
    ) -> Self {
        LocalStateExecutionView {
            xtx_id,
            local_state,
            completed_full_side_effects,
            steps_cnt,
        }
    }
}

pub trait OnLocalTrigger<T: Config, E: crate::EscrowTrait<T>> {
    fn on_local_trigger(origin: &OriginFor<T>, trigger: LocalTrigger<T>) -> DispatchResult;

    fn load_local_state(
        origin: &OriginFor<T>,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Result<LocalStateExecutionView<T, E>, sp_runtime::DispatchError>;
}
