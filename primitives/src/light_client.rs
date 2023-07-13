use crate::{ExecutionSource, SpeedMode};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use t3rn_abi::types::Bytes;

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub enum HeightResult<BlockNumber> {
    Height(BlockNumber),
    NotActive,
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub enum HeaderResult {
    Header(Bytes),
    NotActive,
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct InclusionReceipt<BlockNumber> {
    pub height: BlockNumber,
    pub including_header: Bytes,
    pub message: Bytes,
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct LightClientHeartbeat<T: frame_system::Config> {
    pub last_heartbeat: T::BlockNumber,
    pub last_finalized_height: T::BlockNumber,
    pub last_rational_height: T::BlockNumber,
    pub last_fast_height: T::BlockNumber,
    pub is_halted: bool,
    pub ever_initialized: bool,
}

pub trait LightClient<T: frame_system::Config> {
    fn get_latest_finalized_header(&self) -> HeaderResult;

    fn get_fast_height(&self) -> HeightResult<T::BlockNumber>;

    fn get_rational_height(&self) -> HeightResult<T::BlockNumber>;

    fn get_finalized_height(&self) -> HeightResult<T::BlockNumber>;

    fn get_latest_finalized_header_precompile(&self) -> Bytes;

    fn get_fast_height_precompile(&self) -> T::BlockNumber;

    fn get_rational_height_precompile(&self) -> T::BlockNumber;

    fn get_finalized_height_precompile(&self) -> T::BlockNumber;

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError>;

    fn initialize(
        &self,
        origin: T::Origin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn turn_on(&self, origin: T::Origin) -> Result<bool, DispatchError>;

    fn turn_off(&self, origin: T::Origin) -> Result<bool, DispatchError>;

    fn submit_encoded_headers(&self, encoded_headers_data: Bytes) -> Result<bool, DispatchError>;

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: Option<ExecutionSource>,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion_precompile(
        &self,
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;
}
