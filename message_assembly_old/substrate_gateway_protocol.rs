#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Compact, Encode};

use sp_runtime::generic::Era;
use sp_runtime::RuntimeAppPublic;
use sp_std::vec;
use sp_std::vec::*;
use sp_version::RuntimeVersion;

use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::*;

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

    // ToDo: Accept additional argument to differentiate between mutlisig and single sig produced by submitter / submitters.
    pub fn produce_signed_payload(
        &self,
        namespace: &'static str,
        name: &'static str,
        arguments: Vec<u8>,
        nonce: u32,
    ) -> Result<ExtraMessagePayload, &'static str> {
        let extrinsic = self
            .assembly
            .assemble_signed_call(namespace, name, arguments, nonce)?;

        let signature = extrinsic
            .signature
            .clone()
            .expect("Signature of extrinsic should be valid if assemble_signed_tx was successful")
            .signature;

        Ok(ExtraMessagePayload {
            signer: self.assembly.submitter.to_raw_vec(),
            module_name: namespace.encode(),
            method_name: name.encode(),
            call_bytes: extrinsic.function.encode(),
            signature: signature.encode(),
            extra: GenericExtra::new(Era::Immortal, nonce).encode(),
            tx_signed: extrinsic.encode(),
            custom_payload: None,
        })
    }

    fn collect_args(args: Vec<Vec<u8>>) -> Vec<u8> {
        args.iter().fold(vec![], |mut a, b| {
            a.extend(b);
            a
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

        Ok(CircuitOutboundMessage {
            name: b"get_storage".to_vec(),
            module_name: b"state".to_vec(),
            method_name: b"getStorage".to_vec(),
            arguments,
            expected_output: vec![expected_storage],
            extra_payload: None,
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
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
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        // storage
        let expected_storage = GatewayExpectedOutput::Storage {
            key: vec![key.clone()],
            value: vec![value],
        };

        let arguments = vec![key];

        Ok(CircuitOutboundMessage {
            name: b"set_storage".to_vec(),
            module_name: b"state".to_vec(),
            method_name: b"setStorage".to_vec(),
            arguments: arguments.clone(),
            expected_output: vec![expected_storage],
            extra_payload: Some(self.produce_signed_payload(
                "state",
                "setStorage",
                Self::collect_args(arguments),
                gateway_type.fetch_nonce(),
            )?),
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
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
            GatewayType::ProgrammableInternal(nonce) => {
                let arguments = vec![to, value, gas, data];
                Ok(CircuitOutboundMessage {
                    name: b"call_static".to_vec(),
                    method_name: b"callStatic".to_vec(),
                    module_name: b"gateway".to_vec(),
                    arguments: arguments.clone(),
                    expected_output: vec![GatewayExpectedOutput::Events {
                        signatures: vec![
                            // dest, value, gas_limit, data
                            b"CallStatic(address,value,uint64,dynamic_bytes)".to_vec(),
                        ],
                    }],
                    extra_payload: Some(self.produce_signed_payload(
                        "gatewayEscrowed",
                        "callStatic",
                        Self::collect_args(arguments),
                        nonce,
                    )?),
                    sender: None,
                    target: None,
                    gateway_vendor: GatewayVendor::Substrate,
                })
            }
            // Don't think there is a way of enforcing calls to be static on via external dispatch?
            GatewayType::ProgrammableExternal(_) | GatewayType::TxOnly(_) => {
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
        gateway_type: GatewayType,
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

        let arguments = vec![method_enc.encode(), data];
        // ToDo: Sign message payload passed through state call
        Ok(CircuitOutboundMessage {
            name: b"call".to_vec(),
            module_name: b"state".to_vec(),
            method_name: b"call".to_vec(),
            arguments: arguments.clone(),
            expected_output,
            extra_payload: Some(self.produce_signed_payload(
                "state",
                "call",
                Self::collect_args(arguments),
                gateway_type.fetch_nonce(),
            )?),
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
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
            GatewayType::ProgrammableInternal(nonce) => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // dest, value, gas_limit, data
                        b"CallEscrowed(address,value,uint64,dynamic_bytes)".to_vec(),
                    ],
                }];
                let arguments = vec![to, value, gas, data];
                Ok(CircuitOutboundMessage {
                    name: b"call_escrow".to_vec(),
                    module_name: b"gateway".to_vec(),
                    method_name: b"callEscrow".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    extra_payload: Some(self.produce_signed_payload(
                        "gatewayEscrowed",
                        "callEscrowed",
                        Self::collect_args(arguments),
                        nonce,
                    )?),
                    sender: None,
                    target: None,
                    gateway_vendor: GatewayVendor::Substrate,
                })
            }
            // Don't think there is a way of enforcing calls to be static on via external dispatch?
            GatewayType::ProgrammableExternal(_) | GatewayType::TxOnly(_) => {
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
        to: GenericAddress,
        value: Compact<u128>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        let expected_output = vec![GatewayExpectedOutput::Events {
            signatures: vec![
                // from, to, value
                b"Transfer(address,address,value)".to_vec(),
            ],
        }];

        let arguments = vec![to.encode(), value.encode()];

        Ok(CircuitOutboundMessage {
            name: b"transfer".to_vec(),
            module_name: b"Balances".to_vec(),
            method_name: b"transfer".to_vec(),
            arguments: arguments.clone(),
            expected_output,
            extra_payload: Some(self.produce_signed_payload(
                "Balances",
                "transfer",
                Self::collect_args(arguments),
                gateway_type.fetch_nonce(),
            )?),
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
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
            GatewayType::ProgrammableInternal(nonce) => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                    ],
                }];

                Ok(CircuitOutboundMessage {
                    name: b"transfer_escrow".to_vec(),
                    module_name: b"gateway".to_vec(),
                    method_name: b"transfer".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    extra_payload: Some(self.produce_signed_payload(
                        "gateway",
                        "transfer",
                        Self::collect_args(arguments),
                        nonce,
                    )?),
                    sender: None,
                    target: None,
                    gateway_vendor: GatewayVendor::Substrate,
                })
            }
            GatewayType::ProgrammableExternal(nonce) | GatewayType::TxOnly(nonce) => {
                let expected_output = vec![GatewayExpectedOutput::Events {
                    signatures: vec![
                        // from, to, value
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                        b"EscrowedTransfer(address,address,value)".to_vec(),
                    ],
                }];

                let transfers = vec![
                    self.assembly
                        .assemble_call(
                            "Balances",
                            "transfer",
                            Self::collect_args(vec![
                                self.assembly.submitter.to_raw_vec(),
                                escrow_account.clone(),
                                value.clone(),
                            ]),
                        )?
                        .encode(),
                    self.assembly
                        .assemble_call(
                            "Balances",
                            "transfer",
                            Self::collect_args(vec![escrow_account, to, value]),
                        )?
                        .encode(),
                ];

                Ok(CircuitOutboundMessage {
                    name: b"transfer_escrow".to_vec(),
                    module_name: b"utility".to_vec(),
                    method_name: b"batchAll".to_vec(),
                    arguments: arguments.clone(),
                    expected_output,
                    extra_payload: Some(self.produce_signed_payload(
                        "Utility",
                        "batchAll",
                        Self::collect_args(transfers),
                        nonce,
                    )?),
                    sender: None,
                    target: None,
                    gateway_vendor: GatewayVendor::Substrate,
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
pub mod tests {
    use codec::{Compact, Encode};

    use sp_application_crypto::RuntimePublic;
    use sp_core::sr25519::Signature;

    use sp_io::TestExternalities;
    use sp_keystore::testing::KeyStore;
    use sp_keystore::{KeystoreExt, SyncCryptoStore};

    use t3rn_primitives::transfers::TransferEntry;
    use t3rn_primitives::{GatewayType, GatewayVendor};

    use crate::KEY_TYPE;

    use crate::message_assembly::test_utils::*;

    use super::{
        CircuitOutboundMessage, ExtraMessagePayload, GatewayExpectedOutput, GatewayInboundProtocol,
        Vec,
    };
    use crate::message_assembly::signer::app::GenericAddress;

    pub fn assert_signed_payload(
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
            CircuitOutboundMessage {
                name: _,
                module_name: _,
                method_name: _,
                arguments,
                expected_output,
                sender: _,
                target: _,
                extra_payload,
                gateway_vendor: gateway_chain,
            } => {
                assert_eq!(arguments, exp_arguments);
                assert_eq!(expected_output, exp_output);

                match extra_payload {
                    Some(ExtraMessagePayload {
                        signer,
                        module_name,
                        method_name,
                        call_bytes,
                        signature,
                        extra,
                        custom_payload: _,
                        tx_signed: _,
                    }) => {
                        assert_eq!(signer, submitter.encode());
                        assert_eq!(module_name, expected_module.encode());
                        assert_eq!(method_name, expected_fn.encode());
                        assert_eq!(call_bytes, exp_call_bytes);
                        assert_eq!(gateway_chain, GatewayVendor::Substrate);

                        let expected_message = expected_payload;
                        assert!(submitter.verify(
                            &expected_message,
                            &Signature::from_slice(&signature.as_slice()[1..65]).into(),
                        ));
                        assert_eq!(extra, vec![0, 0, 0]);
                    }
                    _ => assert!(false),
                }
            }
        }
    }

    #[test]
    fn get_storage_should_create_outbound_messages_correctly() {
        let test_protocol = create_default_test_gateway_protocol();
        let test_key = [1_u8; 32].to_vec();

        let expected_message = CircuitOutboundMessage {
            name: b"get_storage".to_vec(),
            module_name: b"state".to_vec(),
            method_name: b"getStorage".to_vec(),
            arguments: vec![test_key.clone()],
            expected_output: vec![GatewayExpectedOutput::Storage {
                key: vec![test_key.clone()],
                value: vec![None],
            }],
            extra_payload: None,
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
        };
        // TODO: update these tests as soon implementation takes the gateway type into account
        assert_eq!(
            test_protocol
                .get_storage(test_key.clone(), GatewayType::ProgrammableInternal(0))
                .unwrap(),
            expected_message
        );
        assert_eq!(
            test_protocol
                .get_storage(test_key.clone(), GatewayType::ProgrammableExternal(0))
                .unwrap(),
            expected_message
        );
        assert_eq!(
            test_protocol
                .get_storage(test_key, GatewayType::TxOnly(0))
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
                GatewayType::ProgrammableInternal(0),
                GatewayType::ProgrammableExternal(0),
                GatewayType::TxOnly(0),
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
                        0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1, 1,
                    ],
                    vec![
                        0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54,
                        23, 242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225,
                        49, 11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23,
                        242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49,
                        11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240,
                    ],
                    "state",
                    "setStorage",
                );
            }
        });
    }

    #[test]
    #[ignore]
    fn call_should_create_outbound_messages_correctly() {
        let test_protocol = create_default_test_gateway_protocol();
        let from = [1_u8; 32].to_vec();
        let to = [2_u8; 32].to_vec();
        let escrow = [3_u8; 32].to_vec();
        let value = 1_u128.encode();
        let gas = 1_u64.encode();
        let data = vec![1_u8];

        let expected_message = CircuitOutboundMessage {
            name: b"call".to_vec(),
            module_name: b"state".to_vec(),
            method_name: b"call".to_vec(),
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
            // ToDo: Expected Payload = [0, 0, 76, 40, 77, 111, 100, 117, 108, 101, 78, 97, 109, 101, 95, 24, 70, 110, 78, 97, 109, 101, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54, 23, 242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23, 242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240]
            extra_payload: None,
            sender: None,
            target: None,
            gateway_vendor: GatewayVendor::Substrate,
        };
        let keystore = KeyStore::new();

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
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
                        GatewayType::ProgrammableInternal(0),
                        None,
                    )
                    .unwrap(),
                expected_message
            )
        });
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
                vec![
                    ("gatewayEscrowed", vec!["callEscrowed"]),
                    ("ModuleName", vec!["FnName"]),
                ],
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
                    GatewayType::ProgrammableInternal(0),
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
                    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
                    0, 0, 0, 0, 0, 0, 2,
                ],
                vec![
                    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
                    0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54, 23, 242,
                    175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192,
                    245, 48, 220, 24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23, 242, 175, 145, 3,
                    62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220,
                    24, 125, 95, 95, 230, 28, 240,
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
                GatewayType::ProgrammableExternal(0),
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
                GatewayType::TxOnly(0),
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
            let test_protocol = create_test_stuffed_gateway_protocol(submitter.into());

            let actual = test_protocol
                .call_static(
                    "ModuleName",
                    "FnName",
                    data.clone(),
                    to.clone(),
                    value.clone(),
                    gas.clone(),
                    GatewayType::ProgrammableInternal(0),
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
                    16, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                    0, 0, 0, 0, 0, 0, 3,
                ],
                vec![
                    16, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54, 23, 242,
                    175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192,
                    245, 48, 220, 24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23, 242, 175, 145, 3,
                    62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220,
                    24, 125, 95, 95, 230, 28, 240,
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
                GatewayType::ProgrammableExternal(0),
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
                GatewayType::TxOnly(0),
                None,
            )
            .unwrap();
    }

    #[test]
    fn transfer_should_create_outbound_messages_correctly() {
        let keystore = KeyStore::new();
        let to = [4_u8; 32];
        let value = Compact::from(4_u128);

        let submitter = SyncCryptoStore::sr25519_generate_new(&keystore, KEY_TYPE, None)
            .expect("Should generate submitter key");

        let mut ext = TestExternalities::new_empty();
        ext.register_extension(KeystoreExt(keystore.into()));
        ext.execute_with(|| {
            println!(
                "transfer_should_create_outbound_messages_correctly - expect this submitter {:?}",
                submitter
            );
            let test_protocol = create_test_stuffed_gateway_protocol(submitter.into());

            let actual = test_protocol
                .transfer(
                    GenericAddress::Id(to.into()),
                    value.clone(),
                    GatewayType::ProgrammableInternal(0),
                )
                .unwrap();

            assert_signed_payload(
                actual,
                submitter.into(),
                vec![
                    vec![
                        0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                        4, 4, 4, 4, 4, 4, 4, 4,
                    ],
                    value.encode(),
                ],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"Transfer(address,address,value)".to_vec()],
                }],
                vec![
                    8, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 16,
                ],
                vec![
                    8, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 16, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54,
                    23, 242, 175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49,
                    11, 192, 245, 48, 220, 24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23, 242,
                    175, 145, 3, 62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192,
                    245, 48, 220, 24, 125, 95, 95, 230, 28, 240,
                ],
                "Balances",
                "transfer",
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
            let test_protocol =
                create_test_gateway_protocol(vec![("gateway", vec!["transfer"])], submitter.into());
            let actual = test_protocol
                .transfer_escrow(
                    escrow.to_vec(),
                    from.to_vec(),
                    to.to_vec(),
                    value.clone(),
                    &mut _transfers,
                    GatewayType::ProgrammableInternal(0),
                )
                .unwrap();

            assert_signed_payload(
                actual,
                submitter,
                vec![to, value, vec![]],
                vec![GatewayExpectedOutput::Events {
                    signatures: vec![b"EscrowedTransfer(address,address,value)".to_vec()],
                }],
                vec![0, 0, 6, 1],
                vec![
                    0, 0, 6, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 228, 91, 54, 23, 242, 175, 145, 3,
                    62, 53, 1, 176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220,
                    24, 125, 95, 95, 230, 28, 240, 228, 91, 54, 23, 242, 175, 145, 3, 62, 53, 1,
                    176, 110, 242, 112, 238, 216, 163, 225, 49, 11, 192, 245, 48, 220, 24, 125, 95,
                    95, 230, 28, 240,
                ],
                "gateway",
                "transfer",
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
                    ("Balances", vec!["transfer"]),
                    ("Utility", vec!["batchAll"]),
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
                    GatewayType::ProgrammableExternal(0),
                )
                .unwrap();

            match actual {
                CircuitOutboundMessage {
                    name,
                    module_name: _,
                    method_name: _,
                    arguments,
                    expected_output,
                    extra_payload,
                    sender: _,
                    target: _,
                    gateway_vendor: gateway_chain,
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
                    assert_eq!(gateway_chain, GatewayVendor::Substrate);
                    match extra_payload {
                        Some(ExtraMessagePayload {
                            signer,
                            module_name,
                            method_name,
                            call_bytes: _,
                            signature: _,
                            extra,
                            tx_signed: _,
                            custom_payload: _,
                        }) => {
                            assert_eq!(signer, submitter.encode());
                            assert_eq!(module_name, "Utility".encode());
                            assert_eq!(method_name, "batchAll".encode());
                            assert_eq!(extra, vec![0, 0, 0]);
                        }
                        _ => assert!(false),
                    }
                }
            };
        });
    }
}
