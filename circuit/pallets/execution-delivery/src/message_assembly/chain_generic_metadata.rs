#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::fmt::Debug;

use sp_std::vec;

use frame_metadata::decode_different::DecodeDifferent;
use frame_metadata::v13::{ExtrinsicMetadata, RuntimeMetadataV13};
use frame_metadata::RuntimeMetadataLastVersion;
use frame_support::ensure;
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
                modules: DecodeDifferent::Decoded(vec![]),
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
    pub fn new(runtime_metadata: RuntimeMetadataV13) -> Self {
        Self { runtime_metadata }
    }

    /// Returns a tuple containing the module index and call index
    pub fn lookup_module_and_call_indices(
        &self,
        lookup_module_name: &'static str,
        lookup_call_name: &'static str,
    ) -> Result<(u8, u8), &'static str> {
        let _module_index: i32 = -1;
        let _call_index: i32 = -1;

        let module_found = convert(self.runtime_metadata.modules.clone())?
            .into_iter()
            .find(|module| module.name.clone() == DecodeDifferent::Encode(lookup_module_name));

        ensure!(
            module_found.is_some(),
            "Module with a given name doesn't exist as per the current metadata"
        );

        let module = module_found.unwrap();

        let module_index: u8 = module.index;
        let calls = match module.calls {
            Some(module_calls) => convert(module_calls)?.into_iter(),
            None => vec![].into_iter(),
        };

        let mut call_counter = 0;
        let call_found = calls.clone().find(|call| {
            call_counter += 1;
            call.name.clone() == DecodeDifferent::Encode(lookup_call_name)
        });

        ensure!(
            call_found.is_some(),
            "Call with a given name doesn't exist on that module as per the current metadata",
        );

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

#[cfg(test)]
mod tests {
    use super::Metadata;
    use frame_metadata::decode_different::DecodeDifferent;
    use frame_metadata::v13::{
        ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
    };
    use frame_support::assert_err;

    pub fn create_test_metadata_struct() -> Metadata {
        let module_name = "ModuleName";
        let fn_name = "FnName";
        let module_index = 1;
        let functions = vec![FunctionMetadata {
            name: DecodeDifferent::Encode(fn_name),
            arguments: DecodeDifferent::Decoded(vec![]),
            documentation: DecodeDifferent::Decoded(vec![]),
        }];

        let module_metadata = ModuleMetadata {
            index: module_index,
            name: DecodeDifferent::Encode(module_name),
            storage: None,
            calls: Some(DecodeDifferent::Decoded(functions)),
            event: None,
            constants: DecodeDifferent::Decoded(vec![]),
            errors: DecodeDifferent::Decoded(vec![]),
        };

        let runtime_metadata = RuntimeMetadataV13 {
            extrinsic: ExtrinsicMetadata {
                version: 1,
                signed_extensions: vec![DecodeDifferent::Decoded(String::from("test"))],
            },
            modules: DecodeDifferent::Decoded(vec![module_metadata]),
        };
        Metadata::new(runtime_metadata)
    }

    #[test]
    fn metadata_lookup_should_return_error_when_module_is_missing() {
        let metadata = create_test_metadata_struct();

        assert_err!(
            metadata.lookup_module_and_call_indices("missing", "FnName"),
            "Module with a given name doesn't exist as per the current metadata"
        );
    }

    #[test]
    fn metadata_lookup_should_return_error_when_function_is_missing() {
        let metadata = create_test_metadata_struct();

        assert_err!(
            metadata.lookup_module_and_call_indices("ModuleName", "missing"),
            "Call with a given name doesn't exist on that module as per the current metadata"
        );
    }
}
