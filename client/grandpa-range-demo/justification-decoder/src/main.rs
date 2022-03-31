use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_finality_grandpa::{AuthorityId, AuthoritySignature};
use sp_runtime::traits::Header as HeaderT;
use sp_runtime::{generic, traits::BlakeTwo256};
use sp_std::prelude::*;

// default MFV settings
pub type BlockNumber = u32;
pub type Hasher = BlakeTwo256;
pub type Header = generic::Header<BlockNumber, Hasher>;

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub struct GrandpaJustification<Header: HeaderT> {
    pub round: u64,
    pub commit:
        finality_grandpa::Commit<Header::Hash, Header::Number, AuthoritySignature, AuthorityId>,
    pub votes_ancestries: Vec<Header>,
}

fn main() {
    let payload = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1])
        .expect("file not found!");
    let hex = if payload.starts_with("0x") {
        &payload[2..]
    } else {
        &payload
    };
    let notification = hex::decode(hex).expect("Hex decoding error");
    let justification =
        GrandpaJustification::<Header>::decode(&mut &*notification).expect("SCALE decoding error");
    println!("{:?}", justification.commit.target_number);
}
