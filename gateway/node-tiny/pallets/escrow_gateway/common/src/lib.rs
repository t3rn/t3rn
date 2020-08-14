use codec::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
#[codec(compact)]
#[repr(u8)]
pub enum Phase {
    Execute = 0,
    Commit = 1,
    Revert= 2,
}
