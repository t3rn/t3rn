#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Compact, Encode};
use sp_runtime::RuntimeAppPublic;
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use crate::message_assembly::chain_generic_metadata::Metadata;
use crate::message_assembly::signer::app::{
    GenericAddress, GenericExtra, MultiSig, Public, Signature, SignedPayload, UncheckedExtrinsicV4,
};
use crate::{compose_call, AuthorityId};

use super::gateway_inbound_assembly::GatewayInboundAssembly;

pub struct SubstrateGatewayAssembly<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone,
{
    pub metadata: Metadata,
    pub runtime_version: RuntimeVersion,
    pub genesis_hash: Hash,
    pub submitter: Authority,
}

impl<Authority, Hash> SubstrateGatewayAssembly<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone,
{
    pub fn new(
        metadata: Metadata,
        runtime_version: RuntimeVersion,
        genesis_hash: Hash,
        submitter_pair: Authority,
    ) -> Self {
        SubstrateGatewayAssembly {
            metadata,
            runtime_version,
            genesis_hash,
            submitter: submitter_pair,
        }
    }
}

impl<Authority, Hash> GatewayInboundAssembly for SubstrateGatewayAssembly<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone,
    Public: From<Authority>,
{
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> UncheckedExtrinsicV4<Vec<u8>> {
        let call = self.assemble_call(module_name, fn_name, data, to, value, gas);
        self.assemble_signed_tx_offline(call, 0)
    }

    /// Who will like to assemble the call?
    /// dev writes composable smart contract ->
    /// -> @ink / solidity / wasm code could now be either
    ///     - a) pre-executed on versatile VM and decomposed into assemble_call from VM (calls + transfers)
    ///     - b) constructed directly as a code analysis
    ///     - c) code without pre-execution would be submitted to gateways using only the call to contracts (escrow exec)
    ///     for a) b) and c) i can expect in this point to have arguments already split
    fn assemble_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> Vec<u8> {
        let call = compose_call!(
            self.metadata,
            module_name,
            fn_name,
            data,
            to,
            Compact(value),
            Compact(gas)
        );
        // e.g. call = ([1, 2], MultiAddress::Id(0101010101010101010101010101010101010101010101010101010101010101 (5C62Ck4U...)), 3, 2)
        call.encode()
    }

    fn assemble_signed_tx_offline(
        &self,
        call_bytes: Vec<u8>,
        nonce: u32,
    ) -> UncheckedExtrinsicV4<Vec<u8>> {
        let extra = GenericExtra::new(sp_runtime::generic::Era::Immortal, nonce);

        let raw_payload = SignedPayload::from_raw(
            call_bytes.clone(),
            extra.clone(),
            (
                nonce,
                self.runtime_version.transaction_version,
                &self.genesis_hash,
                &self.genesis_hash,
                (),
                (),
                (),
            ),
        );

        let signed = GenericAddress::from(AuthorityId::from(self.submitter.clone().into()));

        let signature: Signature = raw_payload
            .using_encoded(|payload| AuthorityId::from(self.submitter.clone()).sign(&payload))
            .expect("Signature should be valid");

        UncheckedExtrinsicV4::new_signed(call_bytes, signed, MultiSig::from(signature), extra)
    }
}

#[cfg(test)]
pub mod tests {
    use codec::Compact;
    use frame_metadata::{
        DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
    };
    use sp_core::H256;
    use sp_io::TestExternalities;
    use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
    use sp_version::{ApisVec, RuntimeVersion};

    use crate::message_assembly::signer::app::Public;
    use crate::KEY_TYPE;

    use super::*;
    use frame_support::assert_err;
    use sp_runtime::generic::Era;

    fn create_submitter() -> Public {
        AuthorityId::default()
    }

    pub fn create_test_genesis_hash() -> H256 {
        [9_u8; 32].into()
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

    pub fn create_test_metadata_struct() -> Metadata {
        let module_name = "ModuleName";
        let fn_name = "FnName";
        let module_index = 1;

        let fn_metadata_generator = |index: usize| -> FunctionMetadata {
            let mut name: String = fn_name.to_string();
            name.push_str(index.to_string().as_str());
            FunctionMetadata {
                name: DecodeDifferent::Encode(Box::leak(name.into_boxed_str())),
                arguments: DecodeDifferent::Decoded(vec![]),
                documentation: DecodeDifferent::Decoded(vec![]),
            }
        };

        let functions = vec![1, 2, 3]
            .into_iter()
            .map(fn_metadata_generator)
            .collect();

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
    #[should_panic(expected = "Module with a given name doesn't exist as per the current metadata")]
    fn sga_assemble_call_panics_when_module_is_missing_from_metadata() {
        let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
            create_test_metadata_struct(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        );

        sga.assemble_call(
            "MissingModuleName",
            "FnName",
            vec![0, 1, 2],
            [1_u8; 32],
            3,
            2,
        );
    }

    #[test]
    #[should_panic(
        expected = "Call with a given name doesn't exist on that module as per the current metadata"
    )]
    fn sga_assemble_call_panics_when_function_is_missing_from_metadata() {
        let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
            create_test_metadata_struct(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        );

        sga.assemble_call(
            "ModuleName",
            "MissingFnName",
            vec![0, 1, 2],
            [1_u8; 32],
            3,
            2,
        );
    }

    #[test]
    fn sga_creates_encoded_call_correctly() {
        let keystore = KeyStore::new();

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));

        ext.execute_with(|| {
            let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
                create_test_metadata_struct(),
                create_test_runtime_version(),
                create_test_genesis_hash(),
                create_submitter(),
            );

            let actual_call_bytes =
                sga.assemble_call("ModuleName", "FnName2", vec![3, 3, 3], [1_u8; 32], 3, 2);

            let expected_call_bytes = (
                (1_u8, 1_u8),
                vec![3_u8, 3_u8, 3_u8],
                [1_u8; 32],
                Compact(3_u16),
                Compact(2_u16),
            )
                .encode();

            assert_eq!(actual_call_bytes, expected_call_bytes)
        });
    }

    #[test]
    fn sga_signs_encoded_call_correctly() {
        let keystore = KeyStore::new();

        let submitter_pub_key = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Generates key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));

        ext.execute_with(|| {
            let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
                create_test_metadata_struct(),
                create_test_runtime_version(),
                create_test_genesis_hash(),
                AuthorityId::from(submitter_pub_key),
            );

            let test_call_bytes =
                sga.assemble_call("ModuleName", "FnName3", vec![0, 1, 2], [1_u8; 32], 3, 2);

            let actual_call_bytes = test_call_bytes.clone();

            let actual_tx_signed = sga.assemble_signed_tx_offline(test_call_bytes, 0);

            let signature = actual_tx_signed.signature.unwrap();

            assert_eq!(actual_tx_signed.function, actual_call_bytes);
            assert_eq!(
                signature.clone().0,
                GenericAddress::from(AuthorityId::from(submitter_pub_key))
            );
            //TODO: add verification for signature
            assert_eq!(signature.clone().2, GenericExtra::new(Era::Immortal, 0));
        });
    }
}
