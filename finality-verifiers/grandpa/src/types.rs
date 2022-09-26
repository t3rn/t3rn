use crate::{bridges::header_chain::justification::GrandpaJustification, TypeInfo};
use codec::{Decode, Encode};
use sp_finality_grandpa::{AuthorityId, SetId};
use sp_std::vec::Vec;
use sp_trie::StorageProof;

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
    pub parachain: Option<Parachain>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct RelaychainHeaderData<Header: sp_runtime::traits::Header> {
    pub signed_header: Header,
    pub range: Vec<Header>,
    pub justification: GrandpaJustification<Header>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct ParachainHeaderData<Header: sp_runtime::traits::Header> {
    pub relay_block_hash: Header::Hash, // relaychain header hash that contains the parachains header
    pub range: Vec<Header>,
    pub proof: StorageProof,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct InclusionData<Header: sp_runtime::traits::Header> {
    /// this is the item we're proving to be included in a specfic block (e.g. event, storage entry, etc)
    pub encoded_payload: Vec<u8>,
    pub proof: StorageProof,
    pub block_hash: Header::Hash,
}
