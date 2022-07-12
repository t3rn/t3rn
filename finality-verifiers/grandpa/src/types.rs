pub use t3rn::{
    xdns::{Parachain, AllowedSideEffect},
    ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};
use sp_finality_grandpa::SetId;
use sp_std::{vec::Vec};

pub type ChainId = [u8; 4];

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Parachain {
    // gateway_id of relaychain
    pub relay_chain_id: ChainId,
    // parachain_id
    pub id: u32,
}

pub struct RegistrationData<T: Config> {
    first_header: Vec<u8>,
    authorities: Vec<T::AccountId>,
    authority_set_id: SetId,
    gateway_id: ChainId,
    parachain: Option<Parachain>
}