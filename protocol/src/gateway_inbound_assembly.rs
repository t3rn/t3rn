#![cfg_attr(not(feature = "std"), no_std)]

use crate::signer::app::{Call, UncheckedExtrinsicV4};
use sp_std::vec::*;

pub trait GatewayInboundAssembly {
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        args: Vec<u8>,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str>;
    fn assemble_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        args: Vec<u8>,
    ) -> Result<Call, &'static str>;
    fn assemble_signed_tx_offline(
        &self,
        call: Call,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str>;
    fn assemble_signed_batch_call(
        &self,
        calls: Vec<Call>,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str>;
}
