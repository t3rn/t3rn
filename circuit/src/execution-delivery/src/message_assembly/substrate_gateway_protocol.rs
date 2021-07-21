#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::ensure;
use sp_runtime::generic::Era;
use sp_runtime::RuntimeAppPublic;
use sp_std::vec;
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::*;

use crate::compose_call;
use crate::message_assembly::chain_generic_metadata::Metadata;
use crate::message_assembly::gateway_inbound_assembly::GatewayInboundAssembly;
use crate::message_assembly::signer::app::GenericExtra;

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
        namespace: &'static str,
        name: &'static str,
        arguments: Vec<Vec<u8>>,
    ) -> Result<MessagePayload, &'static str> {
        let call_bytes = compose_call!(self.assembly.metadata, namespace, name, arguments).encode();

        // TODO: use a proper nonce
        let extrinsic = self
            .assembly
            .assemble_signed_tx_offline(call_bytes.clone(), 0)?;
        let signature = extrinsic.signature.unwrap().1;

        Ok(MessagePayload::Signed {
            signer: self.assembly.submitter.to_raw_vec(),
            module_name: namespace.encode(),
            method_name: name.encode(),
            call_bytes: call_bytes.clone(),
            signature: signature.encode(),
            extra: GenericExtra::new(Era::Immortal, 0).encode(),
        })
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
    fn get_storage(
        &self,
        key: Vec<u8>,
        _gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        // events
        // storage
        let expected_storage = GatewayExpectedOutput::Storage {
            key: vec![key.clone()],
            value: vec![None],
        };
        let arguments = vec![key];

        Ok(CircuitOutboundMessage::Read {
            name: b"get_storage".to_vec(),
            arguments,
            expected_output: vec![expected_storage],
            payload: MessagePayload::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"getStorage".to_vec(),
            },
        })
    }

    // Set storage key directly to foreign storage system
    // For substrate that follows the following key formats:
    // key[0..16].copy_from_slice(&Twox128::hash(module_prefix));
    // key[16..32].copy_from_slice(&Twox128::hash(storage_prefix));
    fn set_storage(
        &self,
        key: Vec<u8>, //sp_core::storage
        value: Option<Vec<u8>>,
        _gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        // storage
        let expected_storage = GatewayExpectedOutput::Storage {
            key: vec![key.clone()],
            value: vec![value],
        };

        let arguments = vec![key];

        Ok(CircuitOutboundMessage::Write {
            name: b"set_storage".to_vec(),
            arguments: arguments.clone(),
            expected_output: vec![expected_storage],
            payload: self.produce_signed_payload("state", "setStorage", arguments)?,
        })
    }

    /// Call pallet's method in a read-only manner (static)
    fn call_static(
        &self,
        _module_name: &str,
        _fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        _return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let arguments = vec![to, value, gas, data];
                Ok(CircuitOutboundMessage::Write {
                    name: b"call_static".to_vec(),
                    arguments: arguments.clone(),
                    expected_output: vec![GatewayExpectedOutput::Events {
                        signatures: vec![
                            // dest, value, gas_limit, data
                            b"CallStatic(address,value,uint64,dynamic_bytes)".to_vec(),
                        ],
                    }],
                    payload: self.produce_signed_payload(
                        "gatewayEscrowed",
                        "callStatic",
                        arguments,
                    )?,
                })
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
        _escrow_account: Vec<u8>,
        _requester: Vec<u8>,
        _to: Vec<u8>,
        _value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
        _return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
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
        Ok(CircuitOutboundMessage::Write {
            name: b"call".to_vec(),
            arguments: vec![method_enc.encode(), data],
            expected_output,
            payload: MessagePayload::Rpc {
                module_name: b"State".to_vec(),
                method_name: b"Call".to_vec(),
            },
        })
    }

    fn call_escrow(
        &self,
        _module_name: &str,
        _fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // dest, value, gas_limit, data
                        b"CallEscrowed(address,value,uint64,dynamic_bytes)".to_vec(),
                    ],
                }];
                let arguments = vec![to, value, gas, data];
                Ok(CircuitOutboundMessage::Write {
                    name: b"call_escrow".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload(
                        "gatewayEscrowed",
                        "callEscrowed",
                        arguments,
                    )?,
                })
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
        _to: Vec<u8>,
        _value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
        _return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn custom_call_dirty(
        &self,
        _module_name: &str,
        _fn_name: &str,
        _data: Vec<u8>,
        _to: Vec<u8>,
        _value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
        _return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn custom_call_escrow(
        &self,
        _module_name: &str,
        _fn_name: &str,
        _data: Vec<u8>,
        _to: Vec<u8>,
        _value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
        _return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn transfer(
        &self,
        to: Vec<u8>,
        value: Vec<u8>,
        _gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        let expected_output = vec![GatewayExpectedOutput::Events {
            signatures: vec![
                // from, to, value
                b"Transfer(address,address,value)".to_vec(),
            ],
        }];

        let arguments = vec![to, value, vec![]];

        Ok(CircuitOutboundMessage::Write {
            name: b"transfer".to_vec(),
            arguments: arguments.clone(),
            expected_output,
            payload: self.produce_signed_payload("Balances", "Transfer", arguments)?,
        })
    }

    fn transfer_escrow(
        &self,
        escrow_account: Vec<u8>,
        _requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        _transfers: &mut Vec<TransferEntry>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        let arguments = vec![to.clone(), value.clone(), vec![]];
        match gateway_type {
            GatewayType::ProgrammableInternal => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                    ],
                }];

                Ok(CircuitOutboundMessage::Write {
                    name: b"transfer_escrow".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload("Gateway", "EscrowTransfer", arguments)?,
                })
            }
            GatewayType::ProgrammableExternal | GatewayType::TxOnly => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                    ],
                }];

                let transfers = vec![
                    self.produce_signed_payload(
                        "Balances",
                        "Transfer",
                        vec![
                            self.assembly.submitter.to_raw_vec(),
                            escrow_account.clone(),
                            value.clone(),
                        ],
                    )?
                    .encode(),
                    self.produce_signed_payload(
                        "Balances",
                        "Transfer",
                        vec![escrow_account, to, value],
                    )?
                    .encode(),
                ];

                Ok(CircuitOutboundMessage::Write {
                    name: b"transfer_escrow".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    payload: self.produce_signed_payload("Utility", "BatchAll", transfers)?,
                })
            }
        }
    }

    fn swap_dirty(
        &self,
        _to: Vec<u8>,
        _value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn swap_escrow(
        &self,
        _from: Vec<u8>,
        _x_token: Vec<u8>,
        _y_token: Vec<u8>,
        _x_value: Vec<u8>,
        _y_value: Vec<u8>,
        _gas: Vec<u8>,
        _gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use codec::Encode;
    use frame_metadata::{
        DecodeDifferent, ExtrinsicMetadata, FunctionMetadata, ModuleMetadata, RuntimeMetadataV13,
    };
    use sp_application_crypto::RuntimePublic;
    use sp_core::sr25519::Signature;
    use sp_core::H256;
    use sp_io::TestExternalities;
    use sp_keystore::testing::KeyStore;
    use sp_keystore::{KeystoreExt, SyncCryptoStore};
    use sp_version::{ApisVec, RuntimeVersion};

    use t3rn_primitives::transfers::TransferEntry;
    use t3rn_primitives::GatewayType;

    use crate::crypto::Public;
    use crate::KEY_TYPE;

    use super::{
        CircuitOutboundMessage, GatewayExpectedOutput, GatewayInboundProtocol, MessagePayload,
        Metadata, SubstrateGatewayProtocol,
    };

    fn create_test_metadata(
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

    fn create_default_test_gateway_protocol() -> SubstrateGatewayProtocol<Public, H256> {
        SubstrateGatewayProtocol::new(
            Metadata::default(),
            create_test_runtime_version(),
            create_test_genesis_hash(),
            create_submitter(),
        )
    }

    fn create_test_gateway_protocol(
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

    fn assert_signed_payload(
        actual: CircuitOutboundMessage,
        submitter: sp_core::sr25519::Public,
        exp_arguments: Vec<Vec<u8>>,
        exp_output: Vec<GatewayExpectedOutput>,
        exp_call_bytes: Vec<u8>,
        expected_payload: Vec<u8>,
        expected_module: &str,
        expected_fn: &str,
    ) {
        match actual {
            CircuitOutboundMessage::Write {
                name: _,
                arguments,
                expected_output,
                payload,
            } => {
                assert_eq!(arguments, exp_arguments);
                assert_eq!(expected_output, exp_output);
                match payload {
                    MessagePayload::Signed {
                        signer,
                        module_name,
                        method_name,
                        call_bytes,
                        signature,
                        extra,
                    } => {
                        assert_eq!(signer, submitter.encode());
                        assert_eq!(module_name, expected_module.encode());
                        assert_eq!(method_name, expected_fn.encode());
                        assert_eq!(call_bytes, exp_call_bytes);

                        let expected_message = expected_payload;
                        assert!(submitter.verify(
                            &expected_message,
                            &Signature::from_slice(&signature.as_slice()[1..65]).into()
                        ));
                        assert_eq!(extra, vec![0, 0, 0]);
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn get_storage_should_create_outbound_messages_correctly() {
        let test_protocol = create_default_test_gateway_protocol();
        let test_key = [1_u8; 32].to_vec();

        let expected_message = CircuitOutboundMessage::Read {
            name: b"get_storage".to_vec(),
            arguments: vec![test_key.clone()],
            expected_output: vec![GatewayExpectedOutput::Storage {
                key: vec![test_key.clone()],
                value: vec![None],
            }],
            payload: MessagePayload::Rpc {
                module_name: b"state".to_vec(),
                method_name: b"getStorage".to_vec(),
            },
        };
        // TODO: update these tests as soon implementation takes the gateway type into account
        assert_eq!(
            test_protocol
                .get_storage(test_key.clone(), GatewayType::ProgrammableInternal)
                .unwrap(),
            expected_message
        );
        assert_eq!(
            test_protocol
                .get_storage(test_key.clone(), GatewayType::ProgrammableExternal)
                .unwrap(),
            expected_message
        );
        assert_eq!(
            test_protocol
                .get_storage(test_key, GatewayType::TxOnly)
                .unwrap(),
            expected_message
        );
    }

    #[test]
    fn set_storage_should_create_outbound_messages_correctly() {
        let keystore = KeyStore::new();

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol =
                create_test_gateway_protocol(vec![("state", vec!["setStorage"])], submitter.into());
            let test_key = [1_u8; 32].to_vec();
            let test_value = Some(vec![1_u8]);

            // TODO: update these tests as soon implementation takes the gateway type into account
            for gateway_type in vec![
                GatewayType::ProgrammableInternal,
                GatewayType::ProgrammableExternal,
                GatewayType::TxOnly,
            ] {
                let actual = test_protocol
                    .set_storage(test_key.clone(), test_value.clone(), gateway_type)
                    .unwrap();

                assert_signed_payload(
                    actual,
                    submitter,
                    vec![test_key.clone()],
                    vec![GatewayExpectedOutput::Storage {
                        key: vec![test_key.clone()],
                        value: vec![test_value.clone()],
                    }],
                    vec![
                        1, 0, 4, 128, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    ],
                    vec![
                        144, 1, 0, 4, 128, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    "state",
                    "setStorage",
                );
            }
        });
    }

    #[test]
    fn call_should_create_outbound_messages_correctly() {
        let test_protocol = create_default_test_gateway_protocol();
        let from = [1_u8; 32].to_vec();
        let to = [2_u8; 32].to_vec();
        let escrow = [3_u8; 32].to_vec();
        let value = 1_u128.encode();
        let gas = 1_u64.encode();
        let data = vec![1_u8];

        let expected_message = CircuitOutboundMessage::Write {
            name: b"call".to_vec(),
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
            test_protocol
                .call(
                    "ModuleName".encode(),
                    "FnName".encode(),
                    data,
                    escrow,
                    from,
                    to,
                    value,
                    gas,
                    GatewayType::ProgrammableInternal,
                    None,
                )
                .unwrap(),
            expected_message
        )
    }

    #[test]
    fn call_escrow_should_create_outbound_messages_correctly() {
        let keystore = KeyStore::new();

        let to = [2_u8; 32].to_vec();
        let value = 2_u128.encode();
        let gas = 2_u64.encode();
        let data = vec![2_u8];

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol = create_test_gateway_protocol(
                vec![("gatewayEscrowed", vec!["callEscrowed"])],
                submitter.into(),
            );

            let actual = test_protocol
                .call_escrow(
                    "ModuleName",
                    "FnName",
                    data.clone(),
                    to.clone(),
                    value.clone(),
                    gas.clone(),
                    GatewayType::ProgrammableInternal,
                )
                .unwrap();

            assert_signed_payload(
                actual,
                submitter,
                vec![to, value, gas, data.clone()],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"CallEscrowed(address,value,uint64,dynamic_bytes)".to_vec()],
                }],
                vec![
                    1, 0, 16, 128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 64, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 32, 2, 0, 0, 0, 0, 0, 0, 0, 4, 2,
                ],
                vec![
                    1, 1, 1, 0, 16, 128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 64, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 32, 2, 0, 0, 0, 0, 0, 0, 0, 4, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                "gatewayEscrowed",
                "callEscrowed",
            );
        });
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_escrow_should_panic_for_external_gateways() {
        let test_protocol = create_default_test_gateway_protocol();
        test_protocol
            .call_escrow(
                "ModuleName",
                "FnName",
                vec![1_u8],
                [1_u8; 32].to_vec(),
                1_u128.encode(),
                1_u64.encode(),
                GatewayType::ProgrammableExternal,
            )
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_escrow_should_panic_for_txonly_gateways() {
        let test_protocol = create_default_test_gateway_protocol();
        test_protocol
            .call_escrow(
                "ModuleName",
                "FnName",
                vec![1_u8],
                [1_u8; 32].to_vec(),
                1_u128.encode(),
                1_u64.encode(),
                GatewayType::TxOnly,
            )
            .unwrap();
    }

    #[test]
    fn call_static_should_create_outbound_messages_correctly_for_internal_gateways() {
        let keystore = KeyStore::new();
        let to = [3_u8; 32].to_vec();
        let value = 3_u128.encode();
        let gas = 3_u64.encode();
        let data = vec![3_u8];

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol = create_test_gateway_protocol(
                vec![("gatewayEscrowed", vec!["callStatic"])],
                submitter.into(),
            );

            let actual = test_protocol
                .call_static(
                    "ModuleName",
                    "FnName",
                    data.clone(),
                    to.clone(),
                    value.clone(),
                    gas.clone(),
                    GatewayType::ProgrammableInternal,
                    None,
                )
                .unwrap();

            assert_signed_payload(
                actual,
                submitter,
                vec![to, value, gas, data.clone()],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"CallStatic(address,value,uint64,dynamic_bytes)".to_vec()],
                }],
                vec![
                    1, 0, 16, 128, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 64, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 32, 3, 0, 0, 0, 0, 0, 0, 0, 4, 3,
                ],
                vec![
                    1, 1, 1, 0, 16, 128, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 64, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 32, 3, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                "gatewayEscrowed",
                "callStatic",
            );
        });
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_static_should_panic_for_external_gateways() {
        let test_protocol = create_default_test_gateway_protocol();
        test_protocol
            .call_static(
                "ModuleName",
                "FnName",
                vec![1_u8],
                [1_u8; 32].to_vec(),
                1_u128.encode(),
                1_u64.encode(),
                GatewayType::ProgrammableExternal,
                None,
            )
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn call_static_should_panic_for_txonly_gateways() {
        let test_protocol = create_default_test_gateway_protocol();
        test_protocol
            .call_static(
                "ModuleName",
                "FnName",
                vec![1_u8],
                [1_u8; 32].to_vec(),
                1_u128.encode(),
                1_u64.encode(),
                GatewayType::TxOnly,
                None,
            )
            .unwrap();
    }

    #[test]
    fn transfer_should_create_outbound_messages_correctly() {
        let keystore = KeyStore::new();
        let to = [4_u8; 32].to_vec();
        let value = 4_u128.encode();

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol = create_test_gateway_protocol(
                vec![("Balances", vec!["Transfer"])],
                submitter.into(),
            );

            let actual = test_protocol
                .transfer(to.clone(), value.clone(), GatewayType::ProgrammableInternal)
                .unwrap();

            assert_signed_payload(
                actual,
                submitter,
                vec![to, value, vec![]],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"Transfer(address,address,value)".to_vec()],
                }],
                vec![
                    1, 0, 12, 128, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 64, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0,
                ],
                vec![
                    216, 1, 0, 12, 128, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 64, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0,
                ],
                "Balances",
                "Transfer",
            );
        });
    }

    #[test]
    fn transfer_escrow_should_create_outbound_messages_correctly_for_internal_gateways() {
        let keystore = KeyStore::new();
        let from = vec![5_u8].to_vec();
        let to = vec![6_u8].to_vec();
        let escrow = vec![7_u8];
        let _transfers = 1_u128.encode();
        let value = vec![1_u8];
        let mut _transfers = vec![TransferEntry::default()];

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol = create_test_gateway_protocol(
                vec![("Gateway", vec!["EscrowTransfer"])],
                submitter.into(),
            );

            let actual = test_protocol
                .transfer_escrow(
                    escrow.to_vec(),
                    from.to_vec(),
                    to.to_vec(),
                    value.clone(),
                    &mut _transfers,
                    GatewayType::ProgrammableInternal,
                )
                .unwrap();

            assert_signed_payload(
                actual,
                submitter,
                vec![to, value, vec![]],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"EscrowedTransfer(address,address,value)".to_vec()],
                }],
                vec![1, 0, 12, 4, 6, 4, 1, 0],
                vec![
                    32, 1, 0, 12, 4, 6, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                "Gateway",
                "EscrowTransfer",
            );
        });
    }

    #[test]
    fn transfer_escrow_should_create_outbound_messages_correctly_for_external_and_txonly_gateways()
    {
        let keystore = KeyStore::new();
        let from = vec![5_u8];
        let to = vec![6_u8];
        let escrow = vec![7_u8];
        let value = vec![1_u8];
        let mut transfers = vec![TransferEntry::default()];

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            let test_protocol = create_test_gateway_protocol(
                vec![
                    ("Balances", vec!["Transfer"]),
                    ("Utility", vec!["BatchAll"]),
                ],
                submitter.into(),
            );

            let actual = test_protocol
                .transfer_escrow(
                    escrow,
                    from,
                    to.clone(),
                    value.clone(),
                    &mut transfers,
                    GatewayType::ProgrammableExternal,
                )
                .unwrap();

            match actual {
                CircuitOutboundMessage::Write {
                    name,
                    arguments,
                    expected_output,
                    payload,
                } => {
                    assert_eq!(name, b"transfer_escrow".to_vec());
                    assert_eq!(arguments, vec![to.clone(), value, vec![]]);
                    assert_eq!(
                        expected_output,
                        vec![GatewayExpectedOutput::Events {
                            signatures: vec![
                                b"EscrowedTransfer(address,address,value)".to_vec(),
                                b"EscrowedTransfer(address,address,value)".to_vec(),
                            ],
                        }]
                    );
                    match payload {
                        MessagePayload::Signed {
                            signer,
                            module_name,
                            method_name,
                            call_bytes: _,
                            signature: _,
                            extra,
                        } => {
                            assert_eq!(signer, submitter.encode());
                            assert_eq!(module_name, "Utility".encode());
                            assert_eq!(method_name, "BatchAll".encode());
                            assert_eq!(extra, vec![0, 0, 0]);
                        }
                        _ => assert!(false),
                    }
                }
                _ => assert!(false),
            };
        });
    }
}
