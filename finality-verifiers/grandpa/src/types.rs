use crate::{bridges::header_chain::justification::GrandpaJustification, TypeInfo};
use codec::{Decode, Encode};
use sp_consensus_grandpa::{AuthorityId, SetId};
use sp_std::vec::Vec;
use sp_trie::StorageProof;

pub type ChainId = [u8; 4];

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct RelaychainRegistrationData<T> {
    pub first_header: Vec<u8>,
    pub authorities: Vec<AuthorityId>,
    pub authority_set_id: SetId,
    pub owner: T,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct ParachainRegistrationData {
    // gateway_id of relaychain
    pub relay_gateway_id: ChainId,
    // parachain_id
    pub id: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct GrandpaHeaderData<Header: sp_runtime::traits::Header> {
    pub signed_header: Header,
    pub range: Vec<Header>,
    pub justification: GrandpaJustification<Header>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct GrandpaHeadersQuickSync<Header: sp_runtime::traits::Header> {
    pub signed_header: Header,
    pub latest_range_of_101: Vec<Header>,
    pub justification: GrandpaJustification<Header>,
}
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct RelaychainInclusionProof<Header: sp_runtime::traits::Header> {
    /// this is the item we're proving to be included in a specfic block (e.g. event, storage entry, etc)
    pub encoded_payload: Vec<u8>,
    pub payload_proof: StorageProof,
    pub block_hash: Header::Hash,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct ParachainInclusionProof<Header: sp_runtime::traits::Header> {
    /// this is the item we're proving to be included in a specfic block (e.g. event, storage entry, etc)
    pub encoded_payload: Vec<u8>,
    pub header_proof: StorageProof,
    pub payload_proof: StorageProof,
    pub relay_block_hash: Header::Hash,
}
