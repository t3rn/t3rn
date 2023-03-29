use codec::{Decode, Encode};
use scale_info::prelude::string::String;
// use frame_support::dispatch::DispatchResult;
use crate::{
    gateway::GatewayABIConfig, ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor,
    TokenSysProps,
};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use t3rn_abi::{recode::Codec, types::Bytes};
use t3rn_light_client_commons::traits::{BlockHeightResult, HeaderResult};
use t3rn_types::sfx::Sfx4bId;

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct RegistrationData {
    pub url: Bytes,
    pub gateway_id: ChainId,
    pub gateway_abi: GatewayABIConfig,
    pub gateway_vendor: GatewayVendor,
    // Maps to FV
    pub gateway_type: GatewayType,
    pub gateway_genesis: GatewayGenesisConfig,
    pub gateway_sys_props: TokenSysProps,
    pub allowed_side_effects: Vec<Sfx4bId>,
    pub encoded_registration_data: Bytes,
}

pub trait Portal<T: frame_system::Config> {
    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<HeaderResult, DispatchError>;

    fn get_latest_finalized_height(
        gateway_id: ChainId,
    ) -> Result<BlockHeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_updated_height(
        gateway_id: ChainId,
    ) -> Result<BlockHeightResult<T::BlockNumber>, DispatchError>;

    fn get_current_epoch(
        gateway_id: ChainId,
    ) -> Result<BlockHeightResult<T::BlockNumber>, DispatchError>;

    fn read_fast_confirmation_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn read_rational_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError>;

    fn read_epoch_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError>;

    fn initialize(
        origin: T::Origin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn turn_on(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;

    fn turn_off(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;
}
