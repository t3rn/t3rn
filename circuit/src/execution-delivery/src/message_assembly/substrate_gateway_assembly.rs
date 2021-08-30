#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode};

use sp_application_crypto::Public;
use sp_runtime::{MultiSignature, RuntimeAppPublic};
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use crate::message_assembly::chain_generic_metadata::Metadata;
use crate::message_assembly::signer::app::{
    Args, Call, GenericAddress, GenericExtra, Signature, SignedPayload, UncheckedExtrinsicV4,
};
use crate::{AuthorityId};

use super::gateway_inbound_assembly::GatewayInboundAssembly;

pub struct SubstrateGatewayAssembly<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + Encode + sp_std::fmt::Debug,
{
    pub metadata: Metadata,
    pub runtime_version: RuntimeVersion,
    pub genesis_hash: Hash,
    pub submitter: Authority,
}

impl<Authority, Hash> SubstrateGatewayAssembly<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + Encode + sp_std::fmt::Debug,
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
    Hash: Clone + Encode + sp_std::fmt::Debug,
{
    fn assemble_signed_call(
        &self,
        module_name: &'static str,
        fn_name: &'static str,
        args: Vec<u8>,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str> {
        let call = self.assemble_call(module_name, fn_name, args)?;

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
        args: Vec<u8>,
    ) -> Result<Call, &'static str> {
        let (module_index, function_index) = self
            .metadata
            .lookup_module_and_call_indices(module_name, fn_name)?;
        // e.g. call = ([1, 2], MultiAddress::Id(0101010101010101010101010101010101010101010101010101010101010101 (5C62Ck4U...)), 3, 2)
        Ok(Call {
            module_index,
            function_index,
            args: Args::new(args),
        })
    }

    fn assemble_signed_tx_offline(
        &self,
        call: Call,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str> {
        let extra = GenericExtra::new(sp_runtime::generic::Era::Immortal, nonce);

        let raw_payload = SignedPayload::from_raw(
            call.clone(),
            extra.clone(),
            self.runtime_version.spec_version,
            self.runtime_version.transaction_version,
            self.genesis_hash.clone(),
            self.genesis_hash.clone(),
        );

        let authority = AuthorityId::from_slice(self.submitter.clone().to_raw_vec().as_slice());

        let signed = GenericAddress::from(authority.clone());

        let signature: Signature = raw_payload
            .using_encoded(|payload| authority.sign(&payload))
            .expect("Signature should be valid");

        Ok(UncheckedExtrinsicV4::new_signed(
            call,
            signed,
            MultiSignature::from(signature),
            extra,
        ))
    }

    fn assemble_signed_batch_call(
        &self,
        calls: Vec<Call>,
        nonce: u32,
    ) -> Result<UncheckedExtrinsicV4<Call>, &'static str> {
        // TODO: use compose call with Utility.batch instead of hard coded index
        let call = Call {
            module_index: 21,
            function_index: 0,
            args: Args::new(calls.encode()),
        };
        self.assemble_signed_tx_offline(call, nonce)
    }
}

#[cfg(test)]
pub mod tests {
    
    use frame_metadata::{
        DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
    };
    use frame_support::assert_err;
    use sp_core::H256;
    use sp_io::TestExternalities;
    use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
    use sp_runtime::generic::Era;
    use sp_runtime::traits::Verify;
    use sp_version::{ApisVec, RuntimeVersion};

    use crate::message_assembly::signer::app::Public;
    use crate::KEY_TYPE;

    use super::*;

    fn create_submitter() -> Public {
        AuthorityId::default()
    }

    pub fn create_test_genesis_hash() -> H256 {
        [
            221, 128, 124, 142, 125, 253, 143, 71, 228, 18, 125, 155, 96, 26, 215, 135, 26, 23,
            181, 82, 173, 18, 254, 62, 220, 69, 220, 133, 187, 229, 24, 113,
        ]
        .into()
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
    fn sga_assemble_call_returns_error_when_module_is_missing_from_metadata() {
        let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
            create_test_metadata_struct(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        );

        assert_err!(
            sga.assemble_call("MissingModuleName", "FnName", vec![0, 1, 2],),
            "Module with a given name doesn't exist as per the current metadata"
        );
    }

    #[test]
    fn sga_assemble_call_returns_error_when_function_is_missing_from_metadata() {
        let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
            create_test_metadata_struct(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        );

        assert_err!(
            sga.assemble_call("ModuleName", "MissingFnName", vec![0, 1, 2],),
            "Call with a given name doesn't exist on that module as per the current metadata"
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

            let actual_call = sga.assemble_call("ModuleName", "FnName2", vec![3, 3, 3]);

            let expected_call = Call {
                module_index: 1_u8,
                function_index: 1_u8,
                args: Args::new(vec![3_u8, 3_u8, 3_u8]),
            };

            assert_eq!(actual_call.unwrap(), expected_call)
        });
    }

    #[test]
    fn sga_signs_encoded_call_correctly() {
        let keystore = KeyStore::new();

        let submitter_pub_key = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate key submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));

        ext.execute_with(|| {
            let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
                create_test_metadata_struct(),
                create_test_runtime_version(),
                create_test_genesis_hash(),
                AuthorityId::from(submitter_pub_key),
            );

            let test_call = sga
                .assemble_call("ModuleName", "FnName3", vec![0, 1, 2])
                .unwrap();

            let actual_call = test_call.clone();

            let actual_tx_signed = sga.assemble_signed_tx_offline(test_call, 0).unwrap();

            let signature = actual_tx_signed.signature.unwrap();

            let raw_payload = SignedPayload::from_raw(
                actual_call.clone(),
                GenericExtra::new(Era::Immortal, 0),
                sga.runtime_version.spec_version,
                sga.runtime_version.transaction_version,
                sga.genesis_hash.clone(),
                sga.genesis_hash.clone(),
            );

            assert_eq!(actual_tx_signed.function, actual_call);
            assert_eq!(
                signature.clone().signer,
                GenericAddress::from(AuthorityId::from(submitter_pub_key))
            );
            assert!(raw_payload.using_encoded(|payload| {
                signature
                    .signature
                    .verify(payload, &submitter_pub_key.0.into())
            }));
            assert_eq!(signature.clone().era, Era::Immortal);
        });
    }

    #[test]
    fn sga_batch_call() {
        let keystore = KeyStore::new();

        let alice = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, Some("//Alice"))
            .expect("Should generate alice key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let sga = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
                create_test_metadata_struct(),
                create_test_runtime_version(),
                create_test_genesis_hash(),
                AuthorityId::from(alice),
            );

            // inner call is a system.remark from alice
            let inner_call = Call {
                module_index: 4,
                function_index: 1,
                args: Args::new(hex::decode("00").expect("must decode").encode()),
            };

            let signed_call = sga
                .assemble_signed_batch_call(vec![inner_call], 1)
                .expect("must be successful");

            println!("0x{}", hex::encode(signed_call.encode()))
        })
    }
}
