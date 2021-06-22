#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use codec::{Decode, Encode};
use t3rn_primitives::UncheckedExtrinsicV4;

#[derive(Clone, Encode, Decode, PartialEq, Eq)]
pub struct SingedBytes {
    pub signature: Vec<u8>,
    pub extra: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}

pub trait GatewayInboundAssembly {
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> UncheckedExtrinsicV4<Vec<u8>>;
    fn assemble_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> Vec<u8>;
    fn assemble_signed_tx_offline(
        &self,
        call_bytes: Vec<u8>,
        nonce: u32,
    ) -> UncheckedExtrinsicV4<Vec<u8>>;
}
