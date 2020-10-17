use codec::{Decode, Encode};
use sp_std::vec::Vec;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct EscrowExecuteResult {
    result: Vec<u8>,
}
