use scale_info::prelude::string::String;
use crate::ChainId;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Clone, Eq, PartialEq, Debug, TypeInfo)]
pub struct ErrorMsg {
    pub extrinsic: String,
    pub msg: String,
    pub gateway_id: ChainId
}

pub type RococoBridge = ();