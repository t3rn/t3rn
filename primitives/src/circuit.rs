use crate::xtx::LocalState;
pub use crate::SpeedMode;
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_system::{pallet_prelude::OriginFor, Config};
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, vec::Vec};
use t3rn_sdk_primitives::signal::ExecutionSignal;
use t3rn_types::sfx::HardenedSideEffect;

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

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalStateExecutionView<T: Config, BalanceOf> {
    pub local_state: LocalState,
    pub hardened_side_effects:
        Vec<Vec<HardenedSideEffect<T::AccountId, T::BlockNumber, BalanceOf>>>,
    pub steps_cnt: (u32, u32),
    pub xtx_id: <T as Config>::Hash,
}

impl<T: Config, Balance> LocalStateExecutionView<T, Balance> {
    pub fn new(
        xtx_id: <T as Config>::Hash,
        local_state: LocalState,
        hardened_side_effects: Vec<Vec<HardenedSideEffect<T::AccountId, T::BlockNumber, Balance>>>,
        steps_cnt: (u32, u32),
    ) -> Self {
        LocalStateExecutionView {
            xtx_id,
            local_state,
            hardened_side_effects,
            steps_cnt,
        }
    }
}

pub trait OnLocalTrigger<T: Config, Balance> {
    fn on_local_trigger(origin: &OriginFor<T>, trigger: LocalTrigger<T>) -> DispatchResult;

    fn load_local_state(
        origin: &OriginFor<T>,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Result<LocalStateExecutionView<T, Balance>, sp_runtime::DispatchError>;

    fn on_signal(origin: &OriginFor<T>, signal: ExecutionSignal<T::Hash>) -> DispatchResult;
}

pub type XExecSignalId<T> = <T as Config>::Hash;
pub type XExecStepSideEffectId<T> = <T as Config>::Hash;
