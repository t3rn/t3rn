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
use t3rn_abi::recode::Codec;
use t3rn_types::sfx::Sfx4bId;

// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Clone, Eq, PartialEq, Debug, TypeInfo)]
pub struct ErrorMsg {
    pub extrinsic: String,
    pub msg: String,
    pub gateway_id: ChainId,
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct RegistrationData {
    pub url: Vec<u8>,
    pub gateway_id: ChainId,
    pub gateway_abi: GatewayABIConfig,
    pub gateway_vendor: GatewayVendor,
    // Maps to FV
    pub gateway_type: GatewayType,
    pub gateway_genesis: GatewayGenesisConfig,
    pub gateway_sys_props: TokenSysProps,
    pub allowed_side_effects: Vec<Sfx4bId>,
    pub encoded_registration_data: Vec<u8>,
}

pub trait Portal<T: frame_system::Config> {
    fn get_latest_finalized_header(chain_id: ChainId) -> Result<Option<Vec<u8>>, DispatchError>;

    fn get_latest_finalized_height(chain_id: ChainId) -> Result<Option<Vec<u8>>, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
        abi_descriptor: Vec<u8>,
        out_codec: Codec,
    ) -> Result<Vec<u8>, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
        abi_descriptor: Vec<u8>,
        out_codec: Codec,
    ) -> Result<Vec<u8>, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
        abi_descriptor: Vec<u8>,
        out_codec: Codec,
    ) -> Result<Vec<u8>, DispatchError>;

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError>;
}
