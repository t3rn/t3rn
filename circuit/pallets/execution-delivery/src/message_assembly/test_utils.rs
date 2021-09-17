use frame_metadata::{
    DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
};
use sp_core::H256;
use sp_version::{ApisVec, RuntimeVersion};

use sp_std::vec;
use sp_std::vec::Vec;

use crate::AuthorityId;

use crate::message_assembly::chain_generic_metadata::*;
use crate::message_assembly::substrate_gateway_protocol::*;

#[cfg(any(feature = "std", test))]
use relay_substrate_client::{ChainBase, Client};

pub fn create_test_metadata(
    modules_with_functions: Vec<(&'static str, Vec<&'static str>)>,
) -> Metadata {
    let mut module_index = 0;
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
            signed_extensions: vec![DecodeDifferent::Encode("test")],
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
        transaction_version: 1,
        spec_version: 1,
    }
}

pub fn create_submitter() -> AuthorityId {
    AuthorityId::default()
}

pub fn create_test_genesis_hash() -> H256 {
    [
        228, 91, 54, 23, 242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49,
        11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240,
    ]
    .into()
}

pub fn get_dummy_modules_with_functions() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("state", vec!["call"]),
        ("state", vec!["getStorage"]),
        ("state", vec!["setStorage"]),
        ("ModuleName", vec!["FnName"]),
        ("ModuleName", vec!["FnName1"]),
        ("ModuleName", vec!["FnName2"]),
        ("ModuleName", vec!["FnName3"]),
        ("author", vec!["submitExtrinsic"]),
        ("Balances", vec!["transfer"]),
        ("Utility", vec!["batchAll"]),
        ("system", vec!["remark"]),
        ("gateway", vec!["call"]),
        ("gateway", vec!["getStorage"]),
        ("gateway", vec!["transfer"]),
        ("gateway", vec!["emitEvent"]),
        ("gateway", vec!["custom"]),
        ("gatewayEscrowed", vec!["callStatic"]),
        ("gatewayEscrowed", vec!["callEscrowed"]),
    ]
}

pub fn create_default_test_gateway_protocol() -> SubstrateGatewayProtocol<AuthorityId, H256> {
    SubstrateGatewayProtocol::new(
        create_test_metadata(get_dummy_modules_with_functions()),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        create_submitter(),
    )
}

pub fn create_test_stuffed_gateway_protocol(
    submitter: AuthorityId,
) -> SubstrateGatewayProtocol<AuthorityId, H256> {
    SubstrateGatewayProtocol::new(
        create_test_metadata(get_dummy_modules_with_functions()),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        submitter,
    )
}

#[cfg(any(feature = "std", test))]
pub async fn create_gateway_protocol_from_client<Chain: relay_substrate_client::Chain>(
    client: &Client<Chain>,
    submitter: AuthorityId,
) -> SubstrateGatewayProtocol<AuthorityId, <Chain as ChainBase>::Hash> {
    // TODO: we need to fetch the metadata from client. but the runtime rpc is not initiated
    // for rpc in relay_substrate_client. once that is in, we can replace that with this
    SubstrateGatewayProtocol::new(
        create_test_metadata(get_dummy_modules_with_functions()),
        client
            .runtime_version()
            .await
            .expect("must return runtime version"),
        client.genesis_hash,
        submitter,
    )
}

pub fn create_test_gateway_protocol(
    modules_with_functions: Vec<(&'static str, Vec<&'static str>)>,
    submitter: AuthorityId,
) -> SubstrateGatewayProtocol<AuthorityId, H256> {
    SubstrateGatewayProtocol::new(
        create_test_metadata(modules_with_functions),
        create_test_runtime_version(),
        create_test_genesis_hash(),
        submitter,
    )
}
