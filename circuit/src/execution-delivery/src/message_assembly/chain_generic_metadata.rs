#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::fmt::Debug;

use sp_std::vec;

use frame_metadata::{DecodeDifferent, ExtrinsicMetadata, RuntimeMetadataV13};
use sp_std::default::Default;
use sp_std::prelude::*;

#[derive(Debug)]
pub struct Metadata {
    runtime_metadata: RuntimeMetadataV13,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            runtime_metadata: RuntimeMetadataV13 {
                modules: frame_metadata::DecodeDifferent::Decoded(vec![]),
                extrinsic: ExtrinsicMetadata {
                    version: 4,
                    signed_extensions: vec![],
                },
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum MetadataError {
    CallNotFound(&'static str),
    ModuleNotFound(&'static str),
}

impl Metadata {
    pub fn lookup_module_and_call_indices(
        &self,
        lookup_module_name: &'static str,
        lookup_call_name: &'static str,
    ) -> Result<(u8, u8), &'static str> {
        let _module_index: i32 = -1;
        let _call_index: i32 = -1;

        let module_found = convert(self.runtime_metadata.modules.clone())?
            .into_iter()
            .find(|module| module.name.clone() == DecodeDifferent::Encode(lookup_module_name))
            .ok_or(MetadataError::ModuleNotFound(
                "Module with a given name doesn't exist as per the current metadata",
            ))
            .unwrap();

        let module_index: u8 = module_found.index;

        let calls = match module_found.calls {
            Some(module_calls) => convert(module_calls)?.into_iter(),
            None => vec![].into_iter(),
        };

        let mut call_counter = 0;
        let _call_found = calls
            .clone()
            .find(|call| {
                call_counter += 1;
                call.name.clone() == DecodeDifferent::Encode(lookup_call_name)
            })
            .ok_or(MetadataError::CallNotFound(
                "Call with a given name doesn't exist on that module as per the current metadata",
            ))
            .unwrap();

        let call_index: u8 = call_counter - 1;

        Ok((module_index, call_index))
    }
}

fn convert<B: 'static, O: 'static>(dd: DecodeDifferent<B, O>) -> Result<O, &'static str> {
    match dd {
        DecodeDifferent::Decoded(value) => Ok(value),
        _ => Err("ConversionError::ExpectedDecoded"),
    }
}

// pub struct GatewayProtocolConfig {
//     pub block_number_type_size: u16,
//     pub hash_size: u16,
//     pub hasher: Type::String,
//     // pub header: Type,
// }
//
// // let h = BlakeTwo256::hash_of(&(BlakeTwo256::hash(b"awesome.dot"), 0u128));
// // and now hardcoded:
// // version: sp_version::RuntimeVersion, -> Version_type
// // era: sp_runtime::generic::Era,  ->
// // genesis_hash: Hash,
// // nonce: Nonce,
// // tip: Balance,
//
// /// When resolving a Solidity file, this holds all the resolved items
// #[derive(Serialize, Deserialize, PartialEq, Clone, Encode, Decode, Eq, Debug)]
// pub struct GatewayGenesis {
//     /// address length in bytes
//     pub address_length: u32,
//     /// value length in bytes
//     pub value_type_size: u32,
//     /// value length in bytes
//     pub decimals: u32,
//     /// value length in bytes
//     pub structs: Vec<StructDecl>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    struct TestGatewayProtocol {}

    impl SubstrateOutboundProtocol for TestGatewayProtocol {}

    impl SubstrateOutboundProtocol for TestGatewayProtocol {}

    #[test]
    fn successfully_creates_example_eth_like_chain_description() {
        let test_gateway_protocol = GatewayProtocol::new(TestChainConfig);

        test_gateway_protocol.get_genesis_config_abi();

        /// impl get_protocol_info -> GenesisInfo { types info + structs info }
        //     pub gateway_inbound_protocol: Box<dyn GatewayInboundProtocol>,
        // let outbound_message = self.gateway_inbound_protocol.transfer_escrow(
        //     <T as CircuitTrait>::AccountId32Converter::convert(self.escrow_account.clone()),
        //     <T as CircuitTrait>::AccountId32Converter::convert(self.requester.clone()),
        //     <T as CircuitTrait>::AccountId32Converter::convert(to.clone()),
        //     <T as CircuitTrait>::ToStandardizedGatewayBalance::convert(value).into(),
        //     self.inner_exec_transfers,
        //     self.gateway_pointer.gateway_type.clone(),
        // );

        // -> code | instantiate_gateway (pointer) ->
        //          { metadata, runtime_version, genesis_hash, submitter_pair } // messaging_params
        //          Opt GenesisConfig {  } // messaging_params
        //          let my_specific_gateway = GatewayImpl::new()
        //          let sgp = SubstrateGatewayProtocol::new(my_specific_gateway)
        //

        // impl trait dry_run
        gateway_protocol.dry_message_dispatch();

        gateway_protocol.dry_verify_message_sender();

        //impt trait outbound_messages

        // gateway_protocol.confirm_inclusion(
        //     block_hash,
        //     value,
        //     proof,
        //     trie_pointer
        // ) {
        //
        //     // access pallet-multi-verifier::get_block_by_hash()
        //
        //
        // }
    }
}
