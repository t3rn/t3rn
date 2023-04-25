use crate::{circuit::CircuitStatus, xtx::LocalState, SpeedMode};
use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_system::{pallet_prelude::OriginFor, Config as ConfigSystem};
use sp_std::{fmt::Debug, vec::Vec};

use t3rn_sdk_primitives::signal::ExecutionSignal;
use t3rn_types::sfx::HardenedSideEffect;
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalTrigger<T: ConfigSystem> {
    /// Id of the contract which generated the side effects
    pub contract: T::AccountId,
    /// Side effects generated from the contract call
    pub submitted_side_effects: Vec<Vec<u8>>,
    pub speed_mode: SpeedMode,
    pub maybe_xtx_id: Option<T::Hash>,
}

impl<T: ConfigSystem> LocalTrigger<T> {
    pub fn new(
        contract: T::AccountId,
        submitted_side_effects: Vec<Vec<u8>>,
        speed_mode: SpeedMode,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Self {
        LocalTrigger {
            contract,
            submitted_side_effects,
            speed_mode,
            maybe_xtx_id,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct LocalStateExecutionView<T: ConfigSystem, BalanceOf> {
    pub local_state: LocalState,
    pub hardened_side_effects:
        Vec<Vec<HardenedSideEffect<T::AccountId, T::BlockNumber, BalanceOf>>>,
    pub steps_cnt: (u32, u32),
    pub xtx_id: <T as ConfigSystem>::Hash,
}

impl<T: ConfigSystem, Balance> LocalStateExecutionView<T, Balance> {
    pub fn new(
        xtx_id: <T as ConfigSystem>::Hash,
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

pub trait OnLocalTrigger<T: ConfigSystem, Balance> {
    fn on_local_trigger(
        origin: &OriginFor<T>,
        trigger: LocalTrigger<T>,
    ) -> Result<LocalStateExecutionView<T, Balance>, sp_runtime::DispatchError>;

    fn load_local_state(
        origin: &OriginFor<T>,
        maybe_xtx_id: Option<T::Hash>,
    ) -> Result<LocalStateExecutionView<T, Balance>, sp_runtime::DispatchError>;

    fn on_signal(origin: &OriginFor<T>, signal: ExecutionSignal<T::Hash>) -> DispatchResult;
}

pub type XExecSignalId<T> = <T as ConfigSystem>::Hash;
pub type XExecStepSideEffectId<T> = <T as ConfigSystem>::Hash;

pub trait ReadSFX<Hash> {
    fn get_fsx_status(fsx_id: Hash) -> Result<CircuitStatus, DispatchError>;

    fn get_xtx_status(xtx_id: Hash) -> Result<CircuitStatus, DispatchError>;
}
