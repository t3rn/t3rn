pub use crate::light_client::{HeaderResult, HeightResult, InclusionReceipt};
use crate::{
    gateway::GatewayABIConfig, ChainId, ExecutionVendor, GatewayGenesisConfig, GatewayType,
    GatewayVendor, SpeedMode, TokenInfo,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use t3rn_abi::{recode::Codec, types::Bytes};
use t3rn_types::sfx::Sfx4bId;

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct RegistrationData {
    pub url: Bytes,
    pub gateway_id: ChainId,
    pub gateway_abi: GatewayABIConfig,
    pub gateway_vendor: GatewayVendor,
    execution_vendor: ExecutionVendor,
    pub gateway_type: GatewayType,
    pub gateway_genesis: GatewayGenesisConfig,
    pub gateway_sys_props: TokenInfo,
    pub allowed_side_effects: Vec<Sfx4bId>,
    pub encoded_registration_data: Bytes,
}

pub trait Portal<T: frame_system::Config> {
    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<HeaderResult, DispatchError>;

    fn get_latest_finalized_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_updated_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_current_epoch(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn read_fast_confirmation_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn read_rational_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError>;

    fn read_finalized_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError>;

    fn read_epoch_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn header_speed_mode_satisfied(
        gateway_id: [u8; 4],
        header: Bytes,
        speed_mode: SpeedMode,
    ) -> Result<bool, DispatchError>;

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn initialize(
        origin: T::Origin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn submit_encoded_headers(
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>,
    ) -> Result<(), DispatchError>;

    fn turn_on(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;

    fn turn_off(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;
}
