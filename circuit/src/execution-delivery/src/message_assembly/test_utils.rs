use frame_metadata::{
    DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
};
use sp_core::H256;
use sp_version::{ApisVec, RuntimeVersion};

use crate::crypto::Public;

use crate::message_assembly::chain_generic_metadata::*;
use crate::message_assembly::substrate_gateway_protocol::*;

pub fn create_test_metadata(
    modules_with_functions: Vec<(&'static str, Vec<&'static str>)>,
) -> Metadata {
    let mut module_index = 1;
    let mut modules: Vec<ModuleMetadata> = vec![];

    let fn_metadata_generator = |name: &'static str| -> FunctionMetadata {
        FunctionMetadata {
            name: DecodeDifferent::Encode(name),
            arguments: DecodeDifferent::Decoded(vec![]),
            documentation: DecodeDifferent::Decoded(vec![]),
        }
    };

    let module_metadata_generator = |mod_name: &'static str,
                                     mod_index: u8,
                                     functions: Vec<FunctionMetadata>|
     -> ModuleMetadata {
        ModuleMetadata {
            index: mod_index,
            name: DecodeDifferent::Encode(mod_name),
            storage: None,
            calls: Some(DecodeDifferent::Decoded(functions)),
            event: None,
            constants: DecodeDifferent::Decoded(vec![]),
            errors: DecodeDifferent::Decoded(vec![]),
        }
    };

    for module in modules_with_functions {
        let (module_name, fn_names) = module;
        let functions = fn_names.into_iter().map(fn_metadata_generator).collect();
        modules.push(module_metadata_generator(
            module_name,
            module_index,
            functions,
        ));
        module_index = module_index + 1;
    }

    let runtime_metadata = RuntimeMetadataV13 {
        extrinsic: ExtrinsicMetadata {
            version: 1,
            signed_extensions: vec![DecodeDifferent::Decoded(String::from("test"))],
        },
        modules: DecodeDifferent::Decoded(modules),
    };
    Metadata::new(runtime_metadata)
}

pub fn create_test_runtime_version() -> RuntimeVersion {
    RuntimeVersion {
        spec_name: "circuit-runtime".into(),
        impl_name: "circuit-runtime".into(),
        authoring_version: 1,
        impl_version: 1,
        apis: ApisVec::Owned(vec![([0_u8; 8], 0_u32)]),
        transaction_version: 4,
        spec_version: 13,
    }
}

pub fn create_submitter() -> Public {
    Public::default()
}

pub fn create_test_genesis_hash() -> H256 {
    [0_u8; 32].into()
}

pub fn create_default_test_gateway_protocol() -> SubstrateGatewayProtocol<Public, H256> {
    SubstrateGatewayProtocol::new(
        Metadata::default(),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        create_submitter(),
    )
}

pub fn create_test_stuffed_gateway_protocol(
    submitter: Public,
) -> SubstrateGatewayProtocol<Public, H256> {

    let modules_with_functions: Vec<(&'static str, Vec<&'static str>)> = vec![
        ("balances", vec!["transfer"]),
        ("state", vec!["call"]),
        ("state", vec!["getStorage"]),
        ("state", vec!["setStorage"]),
        ("author", vec!["submitExtrinsic"]),
        ("utility", vec!["batchAll"]),
        ("system", vec!["remark"]),
        ("gateway", vec!["call"]),
        ("gateway", vec!["transfer"]),
        ("gateway", vec!["getStorage"]),
        ("gateway", vec!["setStorage"]),
        ("gateway", vec!["emitEvent"]),
        ("gateway", vec!["custom"]),
    ];

    SubstrateGatewayProtocol::new(
        create_test_metadata(modules_with_functions),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        submitter,
    )
}



pub fn create_test_gateway_protocol(
    modules_with_functions: Vec<(&'static str, Vec<&'static str>)>,
    submitter: Public,
) -> SubstrateGatewayProtocol<Public, H256> {
    SubstrateGatewayProtocol::new(
        create_test_metadata(modules_with_functions),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        submitter,
    )
}
