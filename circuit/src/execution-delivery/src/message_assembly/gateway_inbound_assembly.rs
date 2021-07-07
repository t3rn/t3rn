#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use crate::message_assembly::signer::app::UncheckedExtrinsicV4;

pub trait GatewayInboundAssembly {
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> Result<UncheckedExtrinsicV4<Vec<u8>>, &'static str>;
    fn assemble_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> Result<Vec<u8>, &'static str>;
    fn assemble_signed_tx_offline(
        &self,
        call_bytes: Vec<u8>,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Vec<u8>>, &'static str>;
}
