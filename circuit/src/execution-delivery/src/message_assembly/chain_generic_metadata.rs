#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::fmt::Debug;

use sp_std::vec;


use frame_metadata::{
    DecodeDifferent, ExtrinsicMetadata, RuntimeMetadataV13
};
use sp_std::prelude::*;
use sp_std::default::Default;


#[derive(Debug)]
pub struct Metadata {
    runtime_metadata: RuntimeMetadataV13,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            runtime_metadata: RuntimeMetadataV13 {
                modules: frame_metadata::DecodeDifferent::Decoded(vec![]),
                extrinsic:
                    ExtrinsicMetadata {
                        version: 4,
                        signed_extensions: vec![],
                    }
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

    pub fn lookup_module_and_call_indices(&self, lookup_module_name: &'static str, _lookup_call_name: &'static str) -> Result<(u8, u8), &'static str>
    {
        let _module_index: i32 = -1;
        let _call_index: i32 = -1;


        let module_found = convert(self.runtime_metadata.modules.clone())?.into_iter()
            .find(|module| {
                module.name.clone() == DecodeDifferent::Encode(lookup_module_name)
            })
            .ok_or(MetadataError::ModuleNotFound("Module with a given name doesn't exist as per the current metadata")).unwrap();

        let module_index: u8 = module_found.index;

        let calls = match module_found.calls {
            Some(module_calls) => {
                convert(module_calls)?.into_iter()
            },
            None => vec![].into_iter()
        };

        let mut call_counter = 0;
        let _call_found = calls.clone().find(|call| {
            call_counter += 1;
            call.name.clone() == DecodeDifferent::Encode(lookup_module_name)
        })
        .ok_or(MetadataError::CallNotFound("Call with a given name doesn't exist on that module as per the current metadata")).unwrap();

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
