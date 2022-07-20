use sp_finality_grandpa::{AuthorityId, SetId};
use sp_std::{vec::Vec};
use codec::{Encode, Decode};
use crate::TypeInfo;

pub type ChainId = [u8; 4];

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Parachain {
    // gateway_id of relaychain
    pub relay_chain_id: ChainId,
    // parachain_id
    pub id: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandpaRegistrationData<T> {
    pub first_header: Vec<u8>,
    pub authorities: Option<Vec<AuthorityId>>,
    pub authority_set_id: Option<SetId>,
    pub owner: T,
    pub parachain: Option<Parachain>
}