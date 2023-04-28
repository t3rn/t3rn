use codec::{Decode, Encode};
use frame_system::pallet_prelude::OriginFor;
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
pub struct LightClientHeartbeat<T: frame_system::Config> {
    pub last_heartbeat: T::BlockNumber,
    pub last_finalized_height: T::BlockNumber,
    pub last_updated_height: T::BlockNumber,
    pub is_halted: bool,
    pub ever_initialized: bool,
}

pub trait LightClient<T: frame_system::Config> {
    fn get_latest_finalized_header(&self) -> Result<HeaderResult, DispatchError>;

    fn get_latest_finalized_height(&self) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_updated_height(&self) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError>;

    fn read_fast_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError>;

    fn read_rational_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError>;

    fn read_finalized_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError>;

    fn get_current_epoch(&self) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn read_epoch_offset(&self) -> Result<T::BlockNumber, DispatchError>;

    fn initialize(
        &self,
        origin: T::RuntimeOrigin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn turn_on(&self, origin: T::RuntimeOrigin) -> Result<bool, DispatchError>;

    fn turn_off(&self, origin: T::RuntimeOrigin) -> Result<bool, DispatchError>;

    fn submit_encoded_headers(&self, encoded_headers_data: Bytes) -> Result<bool, DispatchError>;

    fn submit_finality_header(
        &self,
        origin: OriginFor<T>,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError>;

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;
}
