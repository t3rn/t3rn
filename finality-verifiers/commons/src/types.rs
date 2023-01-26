#![cfg_attr(not(feature = "std"), no_std)]
use scale_info::prelude::vec::Vec;

pub type LightClientId = [u8; 4];
pub type ChainId = [u8; 4];
pub type ShardId = ChainId;
pub type Bytes = Vec<u8>;

pub enum Codec {
    SCALE,
    RLP,
    XBI,
}
