use scale_info::prelude::string::String;
use codec::{Decode, Encode};
// use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use crate::{
    xdns::AllowedSideEffect,
    abi::GatewayABIConfig,
    ChainId, GatewaySysProps, GatewayType, GatewayVendor, GatewayGenesisConfig,
};

// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Clone, Eq, PartialEq, Debug, TypeInfo)]
pub struct ErrorMsg {
    pub extrinsic: String,
    pub msg: String,
    pub gateway_id: ChainId
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct RegistrationData {
    pub url: Vec<u8>,
    pub gateway_id: ChainId,
    pub gateway_abi: GatewayABIConfig,
    pub gateway_vendor: GatewayVendor, // Maps to FV
    pub gateway_type: GatewayType,
    pub gateway_genesis: GatewayGenesisConfig,
    pub gateway_sys_props: GatewaySysProps,
    pub allowed_side_effects: Vec<AllowedSideEffect>,
    pub encoded_registration_data: Vec<u8>
}

pub type RococoBridge = ();

pub trait Portal<T: frame_system::Config> {
    fn get_latest_finalized_header(chain_id: ChainId) -> Option<Vec<u8>>;
    fn get_latest_finalized_height(chain_id: ChainId) -> Result<Vec<u8>, DispatchError>;

    fn confirm_and_decode_payload_params(
        gateway_id: [u8; 4],
        encoded_inclusion_data: Vec<u8>,
        value_abi_unsigned_type: Option<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, DispatchError>;
}