#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use codec::{Compact, Encode};

use crate::message_assembly::chain_generic_metadata::Metadata;

use sp_version::RuntimeVersion;

use super::gateway_inbound_assembly::GatewayInboundAssembly;

// #[macro_use]
use crate::compose_call;

use sp_runtime::RuntimeAppPublic;
use t3rn_primitives::{GenericExtra, SignedPayload, UncheckedExtrinsicV4};

pub struct SubstrateGatewayAssembly<Pair, Hash>
where
    Pair: RuntimeAppPublic,
    Hash: Clone,
{
    pub metadata: Metadata,
    pub runtime_version: RuntimeVersion,
    pub genesis_hash: Hash,
    pub submitter_pair: Pair,
}

impl<Pair, Hash> SubstrateGatewayAssembly<Pair, Hash>
where
    Pair: RuntimeAppPublic,
    Hash: Clone,
{
    pub fn new(
        metadata: Metadata,
        runtime_version: RuntimeVersion,
        genesis_hash: Hash,
        submitter_pair: Pair,
    ) -> Self {
        SubstrateGatewayAssembly {
            metadata,
            runtime_version,
            genesis_hash,
            submitter_pair,
        }
    }
}

impl<Pair, Hash> GatewayInboundAssembly for SubstrateGatewayAssembly<Pair, Hash>
where
    Pair: RuntimeAppPublic,
    Hash: Clone,
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
        let _signed_tx = &self
            .submitter_pair
            .sign(&call_bytes)
            .expect("Signature should be there");

        let extra = GenericExtra::new(sp_runtime::generic::Era::Immortal, nonce.clone());

        let raw_payload = SignedPayload::from_raw(
            call_bytes.clone(),
            extra.clone(),
            (
                nonce,
                self.runtime_version.transaction_version,
                &self.genesis_hash, //dropped the clone here and below due to missing trait, not sure if correct
                &self.genesis_hash,
                (),
                (),
                (),
            ),
        );

        // this one is failing cause &[u8] is not Sized
        let _signature =
            raw_payload.using_encoded(|payload| self.submitter_pair.sign(&payload.encode()));

        //     let mut arr = Default::default();
        //     arr.clone_from_slice($signer.public().as_ref());
        //
        // UncheckedExtrinsicV4::new_signed(
        //     call_bytes,
        //     GenericAddress::from(AccountId::from(arr)),
        //     signature,
        //     extra,
        // )

        // dummy return
        UncheckedExtrinsicV4 {
            function: call_bytes,
            signature: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    // use super::*;
    //
    // use std::collections::{HashMap};
    // use sp_version::RuntimeVersion;
    //
    // pub fn create_test_genesis_hash() -> Hash {
    //     [9 as u8; 32].into()
    // }
    //
    // pub fn create_test_runtime_version() -> sp_version::RuntimeVersion {
    //     RuntimeVersion {
    //         spec_name: "circuit-runtime".into(),
    //         impl_name: "circuit-runtime".into(),
    //         authoring_version: 1,
    //         impl_version: 1,
    //         apis: sp_version::create_apis_vec![[]],
    //         transaction_version: 4,
    //         spec_version: 13,
    //     }
    // }
    //
    // pub fn create_test_metadata_struct() -> Metadata {
    //     let mut modules = HashMap::new();
    //     let mut modules_with_calls = HashMap::new();
    //     let mut modules_with_events = HashMap::new();
    //
    //     let module_name: String = "ModuleName".to_string();
    //     let fn_name: String = "FnName".to_string();
    //     let module_index = 1;
    //     let call_fn_index = 2;
    //     let storage_map = HashMap::new();
    //     let mut call_map = HashMap::new();
    //     let event_map = HashMap::new();
    //
    //     call_map.insert(fn_name, call_fn_index as u8);
    //
    //     modules.insert(
    //         module_name.clone(),
    //         ModuleMetadata {
    //             index: module_index,
    //             name: module_name.clone(),
    //             storage: storage_map,
    //         },
    //     );
    //
    //     modules_with_calls.insert(
    //         module_name.clone(),
    //         ModuleWithCalls {
    //             index: module_index,
    //             name: module_name.clone(),
    //             calls: call_map,
    //         },
    //     );
    //
    //     modules_with_events.insert(
    //         module_name.clone(),
    //         ModuleWithEvents {
    //             index: module_index,
    //             name: module_name.clone(),
    //             events: event_map,
    //         },
    //     );
    //
    //     Metadata {
    //         modules,
    //         modules_with_calls,
    //         modules_with_events,
    //     }
    // }
    //
    // #[test]
    // fn sap_prints_polkadot_metadata() {
    //
    //     let pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
    //
    //     let sag = SubstrateGatewayAssembly::<sp_core::sr25519::Pair>::new(
    //         create_test_metadata_struct(),
    //         create_test_runtime_version(),
    //         create_test_genesis_hash(),
    //         pair.clone(),
    //         pair,
    //     );
    //
    //     let _test_a_pk = [1 as u8; 32];
    //     let _test_b_pk = [0 as u8; 32];
    //
    //     let test_call_bytes = sag.assemble_call(
    //         "ModuleName", "FnName", vec![0, 1, 2], [1 as u8; 32], 3, 2
    //     );
    //
    //     let _test_tx_signed = sag.assemble_signed_tx_offline(
    //         test_call_bytes,
    //         0
    //     );
    // }
}
