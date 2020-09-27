use codec::{Decode, Encode};
use frame_support::sp_runtime::traits::Saturating;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, ExistenceRequirement, Time},
};
use sp_std::{convert::TryInto, prelude::*, vec::Vec};
use system;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct EscrowExecuteResult {
    result: Vec<u8>,
}
