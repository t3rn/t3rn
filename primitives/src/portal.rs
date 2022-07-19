use scale_info::prelude::string::String;
use codec::{Decode, Encode};
use frame_system::pallet_prelude::OriginFor;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use crate::{
    side_effect::interface::SideEffectInterface,
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