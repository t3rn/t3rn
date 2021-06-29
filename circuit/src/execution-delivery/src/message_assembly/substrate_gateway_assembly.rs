#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Compact, Encode};
use sp_runtime::RuntimeAppPublic;
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use crate::message_assembly::chain_generic_metadata::Metadata;
use crate::message_assembly::signer::app::{
    GenericAddress, GenericExtra, MultiSig, SignedPayload, UncheckedExtrinsicV4,
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
    crate::message_assembly::signer::app::Public: std::convert::From<Authority>,
    sp_runtime::MultiSignature:
        std::convert::From<<Authority as sp_runtime::RuntimeAppPublic>::Signature>,
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
        _data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
    ) -> Vec<u8> {
        let call = compose_call!(
            self.metadata,
            module_name,
            fn_name,
            // GenericAddress::Id(to.into()),
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

        let signed = GenericAddress::from(AuthorityId::from(self.submitter.clone()));

        let signature = raw_payload
            .using_encoded(|payload| self.submitter.sign(&payload.encode()))
            .expect("Signature should be valid");

        UncheckedExtrinsicV4::new_signed(call_bytes, signed, MultiSig::from(signature), extra)
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use frame_metadata::{
        DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
    };
    use frame_support::assert_err;
    use sp_core::H256;
    use sp_io::TestExternalities;
    use sp_version::{ApisVec, RuntimeVersion};

    use super::*;
    use crate::message_assembly::signer::app::Public;
    use frame_support::sp_runtime::Storage;

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
        let call_fn_index = 2;

        let mut call_map = HashMap::new();
        call_map.insert(fn_name, call_fn_index as u8);

        let function_metadata = FunctionMetadata {
            name: DecodeDifferent::Encode(fn_name),
            arguments: DecodeDifferent::Decoded(vec![]),
            documentation: DecodeDifferent::Decoded(vec![]),
        };

        let module_metadata = ModuleMetadata {
            index: module_index,
            name: DecodeDifferent::Encode(module_name),
            storage: None,
            calls: Some(DecodeDifferent::Decoded(vec![function_metadata])),
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
    fn sap_panics_when_module_is_missing_from_metadata() {
        let sag = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
            create_test_metadata_struct(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        );
    }

    #[test]
    fn sap_prints_polkadot_metadata() {
        let externalities = TestExternalities::new_empty();
        TestExternalities::new_empty().execute_with(|| {
            let sag = SubstrateGatewayAssembly::<AuthorityId, H256>::new(
                create_test_metadata_struct(),
                create_test_runtime_version(),
                create_test_genesis_hash(),
                create_submitter(),
            );

            let test_call_bytes =
                sag.assemble_call("ModuleName", "FnName", vec![0, 1, 2], [1_u8; 32], 3, 2);

            let test_tx_signed = sag.assemble_signed_tx_offline(test_call_bytes, 0);
        });
    }
}
