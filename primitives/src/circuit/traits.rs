use crate::{
    circuit::CircuitStatus, xtx::LocalState, ExecutionSource, GatewayVendor, SpeedMode, TargetId,
};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchError, DispatchResult, DispatchResultWithPostInfo},
    weights::Weight,
};
use frame_system::{
    pallet_prelude::{BlockNumberFor, OriginFor},
    Config as ConfigSystem,
};
use sp_core::H256;
use sp_std::{fmt::Debug, vec::Vec};

use crate::{circuit::AdaptiveTimeout, light_client::InclusionReceipt};
use t3rn_sdk_primitives::signal::ExecutionSignal;
use t3rn_types::{
    fsx::FullSideEffect,
    sfx::{ConfirmedSideEffect, HardenedSideEffect, SecurityLvl, SideEffect, SideEffectId},
};

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
    pub hardened_side_effects: Vec<
        Vec<
            HardenedSideEffect<
                T::AccountId,
                frame_system::pallet_prelude::BlockNumberFor<T>,
                BalanceOf,
            >,
        >,
    >,
    pub steps_cnt: (u32, u32),
    pub xtx_id: <T as ConfigSystem>::Hash,
}

impl<T: ConfigSystem, Balance> LocalStateExecutionView<T, Balance> {
    pub fn new(
        xtx_id: <T as ConfigSystem>::Hash,
        local_state: LocalState,
        hardened_side_effects: Vec<
            Vec<
                HardenedSideEffect<
                    T::AccountId,
                    frame_system::pallet_prelude::BlockNumberFor<T>,
                    Balance,
                >,
            >,
        >,
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

pub trait CircuitSubmitAPI<T: ConfigSystem, Balance> {
    fn on_extrinsic_trigger(
        origin: OriginFor<T>,
        side_effects: Vec<SideEffect<T::AccountId, Balance>>,
        speed_mode: SpeedMode,
        preferred_security_level: SecurityLvl,
    ) -> DispatchResultWithPostInfo;

    fn on_remote_origin_trigger(
        origin: OriginFor<T>,
        order_origin: T::AccountId,
        side_effects: Vec<SideEffect<T::AccountId, Balance>>,
        speed_mode: SpeedMode,
    ) -> DispatchResultWithPostInfo;

    fn store_gmp_payload(id: H256, payload: Vec<u8>) -> bool;

    fn get_gmp_payload(id: H256) -> Option<Vec<u8>>;

    fn verify_sfx_proof(
        target: TargetId,
        speed_mode: SpeedMode,
        source: Option<ExecutionSource>,
        encoded_proof: Vec<u8>,
    ) -> Result<InclusionReceipt<BlockNumberFor<T>>, DispatchError>;
}

pub trait CircuitDLQ<T: ConfigSystem> {
    fn process_dlq(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight;
    fn process_adaptive_xtx_timeout_queue(
        n: frame_system::pallet_prelude::BlockNumberFor<T>,
        verifier: &GatewayVendor,
    ) -> Weight;
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

pub trait ReadSFX<Hash, Account, Balance, BlockNumber> {
    fn get_fsx_of_xtx(xtx_id: Hash) -> Result<Vec<Hash>, DispatchError>;

    fn get_fsx_status(fsx_id: Hash) -> Result<CircuitStatus, DispatchError>;

    fn get_fsx(
        fsx_id: Hash,
    ) -> Result<FullSideEffect<Account, BlockNumber, Balance>, DispatchError>;

    fn get_xtx_status(
        xtx_id: Hash,
    ) -> Result<(CircuitStatus, AdaptiveTimeout<BlockNumber, TargetId>), DispatchError>;

    fn get_fsx_requester(fsx_id: Hash) -> Result<Account, DispatchError>;
}
