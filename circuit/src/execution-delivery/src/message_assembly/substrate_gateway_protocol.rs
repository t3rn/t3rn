#![cfg_attr(not(feature = "std"), no_std)]



use codec::{Compact, Encode, Decode, alloc::collections::HashMap};
use substrate_api_client::{
    compose_call, compose_extrinsic, compose_extrinsic_offline, Api, GenericAddress, Metadata, UncheckedExtrinsicV4, XtStatus, Hash,
    extrinsic::xt_primitives::*
};

use sp_version::RuntimeVersion;


use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::{GatewayVendor, GatewayType, GatewayPointer};

use super::substrate_gateway_assembly::{SubstrateGatewayAssembly};
use super::gateway_inbound_protocol::{GatewayInboundProtocol};
use super::circuit_outbound::{CircuitOutboundMessage, CircuitInboundResult, InboundStepProofTypes, MessageTransmissionMedium};

pub struct SubstrateGatewayProtocol<Pair: substrate_api_client::sp_core::Pair> {
    pub assembly: SubstrateGatewayAssembly<Pair>,
}

// :( should be using self::sp_runtime
// ToDo: Use the same sp_core library as rest of crate instead of accessing on from ext sub_api_client :(
impl <Pair: substrate_api_client::sp_core::Pair> SubstrateGatewayProtocol<Pair> {
    pub fn new(
        metadata: Metadata,
        runtime_version: RuntimeVersion,
        genesis_hash: Hash,
        submitter_pair: Pair,
        submitter_pair_multisig: substrate_api_client::sp_core::sr25519::Pair,
    ) -> Self {
        SubstrateGatewayProtocol {
            assembly: SubstrateGatewayAssembly::<Pair>::new(
                metadata, runtime_version, genesis_hash, submitter_pair, submitter_pair_multisig
            )
        }
    }
}

impl <Pair: substrate_api_client::sp_core::Pair> GatewayInboundProtocol for SubstrateGatewayProtocol<Pair> {

    fn get_storage(&self, key: &[u8; 32], _gateway_type: GatewayType) -> CircuitOutboundMessage {
        println!("SubstrateGatewayProtocol get storage {:?}", key);

        CircuitOutboundMessage::Read {
            arguments: vec![key.to_vec()],
            inbound_results: CircuitInboundResult {
                result_format: b"Option<Vec<u8>>".to_vec(),
                proof_type: InboundStepProofTypes::State,
            },
            transmission_medium: MessageTransmissionMedium::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"getStorage".to_vec(),
            }
        }
    }

    fn set_storage(&self, _key: &[u8; 32], _value: Option<Vec<u8>>, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn call_static(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn call(
        &self,
        _module_name: Vec<u8>,
        _fn_name: Vec<u8>,
        _data: Vec<u8>,
        escrow_account: [u8; 32],
        requester: [u8; 32],
        to: [u8; 32],
        value: u128,
        _gas: u64,
        _gateway_type: GatewayType
    ) -> CircuitOutboundMessage {

        // Dummy RPC to state call now
        CircuitOutboundMessage::Write {
            sender: escrow_account.to_vec(),
            arguments: vec![
                escrow_account.to_vec(),
                Encode::encode(&value),
                to.to_vec(),
                requester.to_vec(),
            ],
            inbound_results: CircuitInboundResult {
                result_format: b"None".to_vec(),
                proof_type: InboundStepProofTypes::State,
            },
            transmission_medium: MessageTransmissionMedium::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"call".to_vec(),
            }
        }
    }

    fn call_dirty(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn call_escrow(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn custom_call_static(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn custom_call_dirty(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn custom_call_escrow(&self, _module_name: &str, _fn_name: &str, _data: Vec<u8>, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn transfer(
        &self,
        escrow_account: [u8; 32],
        requester: [u8; 32],
        to: [u8; 32],
        value: u128,
        _transfers: &mut Vec<TransferEntry>,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {


        /*
        let msg = match gateway_type {

            GatewayType::ProgrammableInternal => {
                // &self, module_name: &str, fn_name: &str, data: Vec<u8>, to: [u8; 32], value: u128, gas: u64



                let call_bytes = Self::GatewayAssembly.assemble_call(
                    "EscrowGateway", "escrow_transfer", vec![
                        escrow_account.to_vec()
                        // extend(requester.to_vec())
                    ], to.clone(), value, gas
                );

                let signed_tx = Self::GatewayAssembly.assemble_signed_tx_offline(
                    call_bytes,
                    &pair.into(),
                    0
                );

                CircuitOutboundMessage::Escrowed {
                    sender: escrow_account,
                    target: to,
                    arguments: vec![key.to_vec()],
                    inbound_results: CircuitInboundResult {
                        result_format: b"None".to_vec(),
                        proof_type: InboundStepProofTypes::Transaction,
                    },
                    transmission_medium: MessageTransmissionMedium::TransactionDispatch {
                        call_bytes,
                        signature: signed_tx.signature,
                        extra: signed_tx.extra
                    }
                }
            },
            GatewayType::ProgrammableExternal => {

            },
            GatewayType::TxOnly => {

            },
        };
        */
        // Dummy for now
        CircuitOutboundMessage::Write {
            sender: escrow_account.to_vec(),
            arguments: vec![
                escrow_account.to_vec(),
                Encode::encode(&value),
                to.to_vec(),
                requester.to_vec(),
            ],
            inbound_results: CircuitInboundResult {
                result_format: b"None".to_vec(),
                proof_type: InboundStepProofTypes::State,
            },
            transmission_medium: MessageTransmissionMedium::Rpc {
                module_name: b"author".to_vec(),
                method_name: b"submitAndWatchExtrinsic".to_vec(),
            }
        }
    }

    fn transfer_dirty(&self, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn transfer_escrow(&self, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn swap_dirty(&self, _to: [u8; 32], _value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn swap_escrow(&self, _from: [u8; 32], _x_token: [u8; 32], _y_token: [u8; 32], _x_value: u128, _y_value: u128, _gas: u64, _gateway_type: GatewayType) -> CircuitOutboundMessage {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    use crate::message_assembly::substrate_gateway_assembly::tests::{create_test_metadata_struct, create_test_genesis_hash, create_test_runtime_version};

    

    
    use substrate_api_client::sp_core::Pair;
    
    
    
    
    

    #[test]
    fn sap_creates_substrate_gateway_protocol() {

        let pair = substrate_api_client::sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();

        let sgp = SubstrateGatewayProtocol::<substrate_api_client::sp_core::sr25519::Pair>::new(
            create_test_metadata_struct(), create_test_runtime_version(), create_test_genesis_hash(), pair.clone(), pair
        );

        sgp.get_storage(b"kikikikikikikikikikikikikikikiki", GatewayType::ProgrammableExternal);


        //
        // println!("sag = {:?}", sag);
        // let test_a_pk = [1 as u8; 32];
        // let test_b_pk = [0 as u8; 32];
        // let test_call_bytes = sag.assemble_call(
        //     "ModuleName", "FnName", vec![0, 1, 2], [1 as u8; 32], 3, 2
        // );
        //
        // let secret_uri = "//Alice";
        //
        // let pair = substrate_api_client::sp_core::sr25519::Pair::from_string(secret_uri, None).unwrap();
        //
        // let test_tx_signed = sag.assemble_signed_tx_offline(
        //     test_call_bytes,
        //     &pair.into(),
        //     0
        // );
    }
}
