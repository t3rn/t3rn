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
    Hash: Clone + Encode + sp_std::fmt::Debug,
{
    pub assembly: SubstrateGatewayAssembly<Authority, Hash>,
}

impl<Authority, Hash> SubstrateGatewayProtocol<Authority, Hash>
where
    Authority: RuntimeAppPublic + Clone,
    Hash: Clone + Encode + sp_std::fmt::Debug,
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
    Hash: Clone + Encode + sp_std::fmt::Debug,
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
        let method_enc = [module_name, fn_name].join(UNDERSCORE_BYTE);

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

                let arguments = vec![to.encode(), value.encode(), vec![]];

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
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                        b"EscrowedTransfer(address,address,value)".to_vec(),
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
mod tests {
    use codec::Encode;
    use sp_core::H256;
    use sp_version::{ApisVec, RuntimeVersion};

    use t3rn_primitives::GatewayType;

    use crate::crypto::Public;

    use super::{
        CircuitOutboundMessage, GatewayExpectedOutput, GatewayInboundProtocol, MessagePayload,
        Metadata, SubstrateGatewayProtocol,
    };
    use t3rn_primitives::transfers::TransferEntry;

    fn create_test_runtime_version() -> RuntimeVersion {
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

    fn create_submitter() -> Public {
        Public::default()
    }

    fn create_test_genesis_hash() -> H256 {
        [0_u8; 32].into()
    }

    fn create_test_gateway_protocol() -> SubstrateGatewayProtocol<Public, H256> {
        SubstrateGatewayProtocol::new(
            Metadata::default(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        )
    }

    #[test]
    fn get_storage_should_create_outbound_messages_correctly() {
        let test_protocol = create_test_gateway_protocol();
        let test_key = [1_u8; 32];

        let expected_message = CircuitOutboundMessage::Read {
            arguments: vec![test_key.to_vec()],
            expected_output: vec![GatewayExpectedOutput::Storage {
                key: vec![test_key],
                value: vec![None],
            }],
            payload: MessagePayload::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"getStorage".to_vec(),
            },
        };
        // TODO: update these tests as soon implementation takes the gateway type into account
        assert_eq!(
            test_protocol.get_storage(test_key, GatewayType::ProgrammableInternal),
            expected_message
        );
        assert_eq!(
            test_protocol.get_storage(test_key, GatewayType::ProgrammableExternal),
            expected_message
        );
        assert_eq!(
            test_protocol.get_storage(test_key, GatewayType::TxOnly),
            expected_message
        );
    }

    #[test]
    fn set_storage_should_create_outbound_messages_correctly() {
        let test_protocol = create_test_gateway_protocol();
        let test_key = [1_u8; 32];
        let test_value = Some(vec![1_u8]);

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![test_key.to_vec()],
            expected_output: vec![GatewayExpectedOutput::Storage {
                key: vec![test_key],
                value: vec![test_value.clone()],
            }],
            payload: MessagePayload::Signed {
                module_name: b"state".to_vec(),
                method_name: b"setStorage".to_vec(),
                signer: vec![],
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        // TODO: update these tests as soon implementation takes the gateway type into account
        assert_eq!(
            test_protocol.set_storage(
                test_key,
                test_value.clone(),
                GatewayType::ProgrammableInternal
            ),
            expected_message
        );
        assert_eq!(
            test_protocol.set_storage(
                test_key,
                test_value.clone(),
                GatewayType::ProgrammableExternal
            ),
            expected_message
        );
        assert_eq!(
            test_protocol.set_storage(test_key, test_value.clone(), GatewayType::TxOnly),
            expected_message
        );
    }

    #[test]
    fn call_should_create_outbound_messages_correctly() {
        let test_protocol = create_test_gateway_protocol();
        let from = [1_u8; 32];
        let to = [2_u8; 32];
        let escrow = [3_u8; 32];
        let value = 1_u128;
        let gas = 1_u64;
        let data = vec![1_u8];

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![
                vec!["ModuleName".encode(), "FnName".encode()]
                    .join(b"_".as_ref())
                    .encode(),
                data.clone(),
            ],
            expected_output: vec![
                GatewayExpectedOutput::Events {
                    signatures: vec![b"Call(address,value,uint64,dynamic_bytes)".to_vec()],
                },
                GatewayExpectedOutput::Output {
                    output: b"dynamic_bytes".to_vec(),
                },
            ],
            payload: MessagePayload::Rpc {
                module_name: b"State".to_vec(),
                method_name: b"Call".to_vec(),
            },
        };
        assert_eq!(
            test_protocol.call(
                "ModuleName".encode(),
                "FnName".encode(),
                data,
                escrow,
                from,
                to,
                value,
                gas,
                GatewayType::ProgrammableInternal
            ),
            expected_message
        )
    }

    #[test]
    fn call_escrow_should_create_outbound_messages_correctly() {
        let test_protocol = create_test_gateway_protocol();
        let to = [1_u8; 32];
        let value = 1_u128;
        let gas = 1_u64;
        let data = vec![1_u8];

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![to.encode(), value.encode(), gas.encode(), data.clone()],
            expected_output: vec![GatewayExpectedOutput::Events {
                signatures: vec![b"CallEscrowed(address,value,uint64,dynamic_bytes)".to_vec()],
            }],
            payload: MessagePayload::Signed {
                module_name: b"gatewayEscrowed".to_vec(),
                method_name: b"callEscrowed".to_vec(),
                signer: vec![],
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        assert_eq!(
            test_protocol.call_escrow(
                "ModuleName",
                "FnName",
                data,
                to,
                value,
                gas,
                GatewayType::ProgrammableInternal
            ),
            expected_message
        )
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_escrow_should_panic_for_external_gateways() {
        let test_protocol = create_test_gateway_protocol();
        test_protocol.call_escrow(
            "ModuleName",
            "FnName",
            vec![1_u8],
            [1_u8; 32],
            1_u128,
            1_u64,
            GatewayType::ProgrammableExternal,
        );
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_escrow_should_panic_for_txonly_gateways() {
        let test_protocol = create_test_gateway_protocol();
        test_protocol.call_escrow(
            "ModuleName",
            "FnName",
            vec![1_u8],
            [1_u8; 32],
            1_u128,
            1_u64,
            GatewayType::TxOnly,
        );
    }

    #[test]
    fn call_static_should_create_outbound_messages_correctly_for_internal_gateways() {
        let test_protocol = create_test_gateway_protocol();
        let to = [1_u8; 32];
        let value = 1_u128;
        let gas = 1_u64;
        let data = vec![1_u8];

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![to.encode(), value.encode(), gas.encode(), data.clone()],
            expected_output: vec![GatewayExpectedOutput::Events {
                signatures: vec![b"CallStatic(address,value,uint64,dynamic_bytes)".to_vec()],
            }],
            payload: MessagePayload::Signed {
                module_name: b"gatewayEscrowed".to_vec(),
                method_name: b"callStatic".to_vec(),
                signer: vec![],
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        assert_eq!(
            test_protocol.call_static(
                "ModuleName",
                "FnName",
                data,
                to,
                value,
                gas,
                GatewayType::ProgrammableInternal
            ),
            expected_message
        )
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_static_should_panic_for_external_gateways() {
        let test_protocol = create_test_gateway_protocol();
        test_protocol.call_static(
            "ModuleName",
            "FnName",
            vec![1_u8],
            [1_u8; 32],
            1_u128,
            1_u64,
            GatewayType::ProgrammableExternal,
        );
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_static_should_panic_for_txonly_gateways() {
        let test_protocol = create_test_gateway_protocol();
        test_protocol.call_static(
            "ModuleName",
            "FnName",
            vec![1_u8],
            [1_u8; 32],
            1_u128,
            1_u64,
            GatewayType::TxOnly,
        );
    }

    #[test]
    fn transfer_should_create_outbound_messages_correctly() {
        let test_protocol = create_test_gateway_protocol();
        let to = [1_u8; 32];
        let value = 1_u128;

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![to.encode(), value.encode(), vec![]],
            expected_output: vec![GatewayExpectedOutput::Events {
                signatures: vec![b"Transfer(address,address,value)".to_vec()],
            }],
            payload: MessagePayload::Signed {
                signer: vec![],
                module_name: b"Balances".to_vec(),
                method_name: b"Transfer".to_vec(),
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        // TODO: update these tests as soon implementation takes the gateway type into account
        assert_eq!(
            test_protocol.transfer(to, value, GatewayType::ProgrammableInternal),
            expected_message
        );
        assert_eq!(
            test_protocol.transfer(to, value, GatewayType::ProgrammableExternal),
            expected_message
        );
        assert_eq!(
            test_protocol.transfer(to, value, GatewayType::TxOnly),
            expected_message
        );
    }

    #[test]
    fn transfer_escrow_should_create_outbound_messages_correctly_for_internal_gateways() {
        let test_protocol = create_test_gateway_protocol();
        let from = vec![1_u8];
        let to = vec![2_u8];
        let escrow = vec![3_u8];
        let _transfers = 1_u128;
        let value = vec![1_u8];
        let mut _transfers = vec![TransferEntry::default()];

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![to.encode(), value.encode(), vec![]],
            expected_output: vec![GatewayExpectedOutput::Events {
                signatures: vec![b"EscrowedTransfer(address,address,value)".to_vec()],
            }],
            payload: MessagePayload::Signed {
                module_name: b"Gateway".to_vec(),
                method_name: b"EscrowTransfer".to_vec(),
                signer: vec![],
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        assert_eq!(
            test_protocol.transfer_escrow(
                escrow.to_vec(),
                from.to_vec(),
                to.to_vec(),
                value,
                &mut _transfers,
                GatewayType::ProgrammableInternal
            ),
            expected_message
        )
    }

    #[test]
    fn transfer_escrow_should_create_outbound_messages_correctly_for_external_and_txonly_gateways()
    {
        let test_protocol = create_test_gateway_protocol();
        let from = vec![1_u8];
        let to = vec![2_u8];
        let escrow = vec![3_u8];
        let _transfers = 1_u128;
        let value = vec![1_u8];
        let mut _transfers = vec![TransferEntry::default()];

        let expected_message = CircuitOutboundMessage::Write {
            arguments: vec![to.encode(), value.encode(), vec![]],
            expected_output: vec![GatewayExpectedOutput::Events {
                signatures: vec![
                    b"EscrowedTransfer(address,address,value)".to_vec(),
                    b"EscrowedTransfer(address,address,value)".to_vec(),
                ],
            }],
            payload: MessagePayload::Signed {
                module_name: b"Utility".to_vec(),
                method_name: b"BatchAll".to_vec(),
                signer: vec![],
                call_bytes: vec![],
                signature: vec![],
                extra: vec![],
            },
        };
        assert_eq!(
            test_protocol.transfer_escrow(
                escrow.to_vec(),
                from.to_vec(),
                to.to_vec(),
                value.clone(),
                &mut _transfers,
                GatewayType::ProgrammableExternal
            ),
            expected_message
        );
        assert_eq!(
            test_protocol.transfer_escrow(
                escrow.to_vec(),
                from.to_vec(),
                to.to_vec(),
                value.clone(),
                &mut _transfers,
                GatewayType::TxOnly
            ),
            expected_message
        );
    }
}
