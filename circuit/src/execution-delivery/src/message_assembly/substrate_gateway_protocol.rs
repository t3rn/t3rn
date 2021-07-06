#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use sp_runtime::RuntimeAppPublic;
use sp_std::vec;
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::GatewayType;

use crate::message_assembly::chain_generic_metadata::Metadata;

use super::circuit_outbound::{CircuitOutboundMessage, GatewayExpectedOutput, MessagePayload};
use super::gateway_inbound_protocol::GatewayInboundProtocol;
use super::substrate_gateway_assembly::SubstrateGatewayAssembly;

pub struct SubstrateGatewayProtocol<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + sp_std::fmt::Debug,
{
    pub assembly: SubstrateGatewayAssembly<Authority, Hash>,
}

impl<Authority, Hash> SubstrateGatewayProtocol<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + sp_std::fmt::Debug,
{
    pub fn new(
        metadata: Metadata,
        runtime_version: RuntimeVersion,
        genesis_hash: Hash,
        submitter: Authority,
    ) -> Self {
        SubstrateGatewayProtocol {
            assembly: SubstrateGatewayAssembly::<Authority, Hash>::new(
                metadata,
                runtime_version,
                genesis_hash,
                submitter,
            ),
        }
    }

    pub fn produce_signed_payload(
        &self,
        namespace: Vec<u8>,
        name: Vec<u8>,
        _arguments: Vec<Vec<u8>>,
    ) -> MessagePayload {
        // let call_bytes = compose_call!(name_str, name_str, arguments).to_vec();
        // let tx = compose_call!(namespace, name);

        MessagePayload::Signed {
            // ToDo: get public key from AuthorityId instead
            signer: vec![],
            module_name: namespace,
            method_name: name,
            call_bytes: vec![],
            signature: vec![],
            extra: vec![],
        }
    }
}

impl<Authority, Hash> GatewayInboundProtocol for SubstrateGatewayProtocol<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + sp_std::fmt::Debug,
{
    // Get storage key directly to foreign storage system
    // For substrate that follows the following key formats:
    // key[0..16].copy_from_slice(&Twox128::hash(module_prefix));
    // key[16..32].copy_from_slice(&Twox128::hash(storage_prefix));
    fn get_storage(&self, key: [u8; 32], _gateway_type: GatewayType) -> CircuitOutboundMessage {
        // events
        // storage
        let expected_storage = GatewayExpectedOutput::Storage {
            key: vec![key],
            value: vec![None],
        };
        let arguments = vec![key.to_vec()];

        CircuitOutboundMessage::Read {
            arguments,
            expected_output: vec![expected_storage],
            payload: MessagePayload::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"getStorage".to_vec(),
            },
        }
    }

    // Set storage key directly to foreign storage system
    // For substrate that follows the following key formats:
    // key[0..16].copy_from_slice(&Twox128::hash(module_prefix));
    // key[16..32].copy_from_slice(&Twox128::hash(storage_prefix));
    fn set_storage(
        &self,
        key: [u8; 32], //sp_core::storage
        value: Option<Vec<u8>>,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        // storage
        let expected_storage = GatewayExpectedOutput::Storage {
            key: vec![key],
            value: vec![value],
        };

        let arguments = vec![key.to_vec()];

        CircuitOutboundMessage::Write {
            arguments: arguments.clone(),
            expected_output: vec![expected_storage],
            payload: self.produce_signed_payload(
                b"state".to_vec(),
                b"setStorage".to_vec(),
                arguments,
            ),
        }
    }

    /// Call pallet's method in a read-only manner (static)
    fn call_static(
        &self,
        _module_name: &str,
        _fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let arguments = vec![to.encode(), value.encode(), gas.encode(), data];
                CircuitOutboundMessage::Write {
                    arguments: arguments.clone(),
                    expected_output: vec![GatewayExpectedOutput::Events {
                        signatures: vec![
                            // dest, value, gas_limit, data
                            b"CallStatic(address,value,uint64,dynamic_bytes)".to_vec(),
                        ],
                    }],
                    payload: self.produce_signed_payload(
                        b"gatewayEscrowed".to_vec(),
                        b"callStatic".to_vec(),
                        arguments,
                    ),
                }
            }
            // Don't think there is a way of enforcing calls to be static on via external dispatch?
            GatewayType::ProgrammableExternal | GatewayType::TxOnly => {
                unimplemented!();
            }
        }
    }

    fn call(
        &self,
        module_name: Vec<u8>,
        fn_name: Vec<u8>,
        data: Vec<u8>,
        _escrow_account: [u8; 32],
        _requester: [u8; 32],
        _to: [u8; 32],
        _value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        // For state::call first argument is PalletName_MethodName
        const UNDERSCORE_BYTE: &[u8] = b"_";
        let method_enc = [module_name.as_slice(), fn_name.as_slice()].join(UNDERSCORE_BYTE);

        let expected_output = vec![
            GatewayExpectedOutput::Events {
                signatures: vec![
                    // dest, value, gas_limit, data
                    b"Call(address,value,uint64,dynamic_bytes)".to_vec(),
                ],
            },
            GatewayExpectedOutput::Output {
                output: b"dynamic_bytes".to_vec(),
            },
        ];

        // ToDo: Sign message payload passed through state call
        CircuitOutboundMessage::Write {
            arguments: vec![method_enc.encode(), data],
            expected_output,
            payload: MessagePayload::Rpc {
                module_name: b"State".to_vec(),
                method_name: b"Call".to_vec(),
            },
        }
    }

    fn call_escrow(
        &self,
        _module_name: &str,
        _fn_name: &str,
        data: Vec<u8>,
        to: [u8; 32],
        value: u128,
        gas: u64,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // dest, value, gas_limit, data
                        b"CallEscrowed(address,value,uint64,dynamic_bytes)".to_vec(),
                    ],
                }];
                let arguments = vec![to.encode(), value.encode(), gas.encode(), data];
                CircuitOutboundMessage::Write {
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload(
                        b"gatewayEscrowed".to_vec(),
                        b"callEscrowed".to_vec(),
                        arguments,
                    ),
                }
            }
            // Don't think there is a way of enforcing calls to be static on via external dispatch?
            GatewayType::ProgrammableExternal | GatewayType::TxOnly => {
                unimplemented!();
            }
        }
    }

    fn custom_call_static(
        &self,
        _module_name: &str,
        _fn_name: &str,
        _data: Vec<u8>,
        _to: [u8; 32],
        _value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn custom_call_dirty(
        &self,
        _module_name: &str,
        _fn_name: &str,
        _data: Vec<u8>,
        _to: [u8; 32],
        _value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn custom_call_escrow(
        &self,
        _module_name: &str,
        _fn_name: &str,
        _data: Vec<u8>,
        _to: [u8; 32],
        _value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn transfer(
        &self,
        to: [u8; 32],
        value: u128,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        let expected_output = vec![GatewayExpectedOutput::Events {
            signatures: vec![
                // from, to, value
                b"Transfer(address,address,value)".to_vec(),
            ],
        }];

        let arguments = vec![to.encode(), value.encode(), vec![]];

        CircuitOutboundMessage::Write {
            arguments: arguments.clone(),
            expected_output,
            payload: self.produce_signed_payload(
                b"Balances".to_vec(),
                b"Transfer".to_vec(),
                arguments,
            ),
        }
    }

    fn transfer_escrow(
        &self,
        escrow_account: Vec<u8>,
        _requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        _transfers: &mut Vec<TransferEntry>,
        gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                    ],
                }];

                let arguments = vec![to, value, vec![]];

                CircuitOutboundMessage::Write {
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload(
                        b"Gateway".to_vec(),
                        b"EscrowTransfer".to_vec(),
                        arguments,
                    ),
                }
            }
            GatewayType::ProgrammableExternal | GatewayType::TxOnly => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"Transfer(address,address,value)".to_vec(),
                        b"Transfer(address,address,value)".to_vec(),
                    ],
                }];

                let arguments = vec![to.encode(), value.encode(), vec![]];

                let transfers = vec![
                    self.produce_signed_payload(
                        b"Balances".to_vec(),
                        b"Transfer".to_vec(),
                        vec![
                            // ToDo: change to dummy vector to self.assembly.submitter_pair.public()
                            vec![],
                            escrow_account.clone(),
                            value.clone(),
                        ],
                    )
                    .encode(),
                    self.produce_signed_payload(
                        b"Balances".to_vec(),
                        b"Transfer".to_vec(),
                        vec![escrow_account, to, value],
                    )
                    .encode(),
                ];

                CircuitOutboundMessage::Write {
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload(
                        b"Utility".to_vec(),
                        b"BatchAll".to_vec(),
                        transfers,
                    ),
                }
            }
        }
    }

    fn swap_dirty(
        &self,
        _to: [u8; 32],
        _value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        unimplemented!()
    }

    fn swap_escrow(
        &self,
        _from: [u8; 32],
        _x_token: [u8; 32],
        _y_token: [u8; 32],
        _x_value: u128,
        _y_value: u128,
        _gas: u64,
        _gateway_type: GatewayType,
    ) -> CircuitOutboundMessage {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
