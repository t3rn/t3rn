#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use crate::message_assembly::signer::app::{Call, UncheckedExtrinsicV4};
use codec::Encode;

pub trait GatewayInboundAssembly {
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        args: Vec<Vec<u8>>,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str>;
    fn assemble_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        args: Vec<Vec<u8>>,
    ) -> Result<Call, &'static str>;
    fn assemble_signed_tx_offline(
        &self,
        call: Call,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str>;
}
