use crate::{
    sfx_abi::{PerCodecAbiDescriptors, SFXAbi},
    types::Sfx4bId,
};
use sp_std::prelude::*;

pub fn standard_sfx_abi() -> Vec<(Sfx4bId, SFXAbi)> {
    vec![
        (*b"data", get_data_abi()),
        (*b"tran", get_sfx_transfer_abi()),
        (*b"tass", get_sfx_transfer_asset_abi()),
        (*b"swap", get_swap_abi()),
        (*b"aliq", get_add_liquidity_abi()),
        (*b"rliq", get_remove_liquidity_abi()),
        (*b"cevm", get_call_evm_contract_abi()),
        (*b"wasm", get_call_wasm_contract_abi()),
        (*b"cgen", get_call_generic_abi()),
    ]
}

impl SFXAbi {
    pub fn get_standard_interface(sfx_4b_id: Sfx4bId) -> Option<SFXAbi> {
        for (id, abi) in standard_sfx_abi() {
            if id == sfx_4b_id {
                return Some(abi)
            }
        }
        None
    }
}

pub fn standard_sfx_abi_ids() -> Vec<Sfx4bId> {
    vec![
        *b"data", *b"tran", *b"tass", *b"orml", *b"swap", *b"aliq", *b"cevm", *b"wasm", *b"comp",
    ]
}

pub fn get_sfx_transfer_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![(b"to".to_vec(), true), (b"amount".to_vec(), true)],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Transfer:Log(from+:Account20,to+:Account20,amount+:Value256)".to_vec(),
            for_scale:
                b"Balances:Struct(Transfer:Event(from:Account32,to:Account32,amount:Value128))"
                    .to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Transfer:Struct(to+:Account20,amount+:Value256)".to_vec(),
            for_scale: b"Transfer:Struct(to:Account32,amount:Value128)".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_sfx_transfer_asset_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"asset_id".to_vec(), false),
            (b"to".to_vec(), true),
            (b"amount".to_vec(), true),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+") //  event Transfer(address indexed from, address indexed to, uint tokens);
            for_rlp:
                b"Transfer:Log(from+:Account20,to+:Account20,amount:Value256)"
                    .to_vec(),
            for_scale:
                b"Assets:Struct(Transferred:Event(asset_id:Value32,from:Account32,to:Account32,amount:Value128))"
                    .to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Assets:Struct(asset_id:Value32,to:Account32,amount:Value128)".to_vec(),
            for_scale: b"Assets:Struct(asset_id:Value32,to:Account32,amount:Value128)".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_data_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![(b"key".to_vec(), true)],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Tuple(key:Data,value:Data)".to_vec(),
            for_scale: b"Tuple(key:Data,value:Data)".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"H256".to_vec(),
            for_scale: b"H256".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_swap_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"to".to_vec(), true),
            (b"amount_from".to_vec(), true),
            (b"amount_to".to_vec(), true),
            (b"asset_from".to_vec(), true),
            (b"asset_to".to_vec(), true),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Swap:Log(from+:Account20,to+:Account20,amount_from+:Value128,amount_to+:Value128,asset_from+:Account20,asset_to+:Account20)".to_vec(),
            for_scale: b"Assets:Struct(Swap:Event(from:Account32,to:Account32,amount_from:Value128,amount_to:Value128,asset_from:Account32,asset_to:Account32))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Swap:Struct(to:Account20,amount_from:Value128,amount_to:Value128,asset_from:Account20,asset_to:Account20)".to_vec(),
            for_scale: b"Swap:Struct(to:Account32,amount_from:Value128,amount_to:Value128,asset_from:Account32,asset_to:Account32)".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_add_liquidity_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"to".to_vec(), true),
            (b"asset_left".to_vec(), true),
            (b"asset_right".to_vec(), true),
            (b"liquidity_token".to_vec(), true),
            (b"amount_left".to_vec(), true),
            (b"amount_right".to_vec(), true),
            (b"amount_liquidity_token".to_vec(), true),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"AddLiquidity:Log(from+:Account20,to+:Account20,amount_left+:Value256,amount_right+:Value256,asset_right+:Account20,asset_left+:Account20,liquidity_token+:Account20,amount_liquidity_token+:Value256)".to_vec(),
            for_scale: b"Assets:Struct(AddLiquidity:Event(from:Account32,to:Account32,amount_left:Value128,amount_right:Value128,asset_right+:Account32,asset_left:Account32,liquidity_token:Account32,amount_liquidity_token:Value128))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            for_rlp: b"AddLiquidity:Struct(to:Account20,amount_left:Value256,amount_right:Value256,asset_right:Account20,asset_left:Account20,liquidity_token:Account20,amount_liquidity_token:Value256)".to_vec(),
            for_scale: b"AddLiquidity:Struct(to:Account32,amount_left:Value128,amount_right:Value128,asset_right:Account32,asset_left:Account32,liquidity_token:Account32,amount_liquidity_token:Value128)".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_remove_liquidity_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"to".to_vec(), true),
            (b"asset_left".to_vec(), true),
            (b"asset_right".to_vec(), true),
            (b"liquidity_token".to_vec(), true),
            (b"amount_left".to_vec(), true),
            (b"amount_right".to_vec(), true),
            (b"amount_liquidity_token".to_vec(), true),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"RemoveLiquidity:Log(from+:Account20,to+:Account20,amount_left+:Value256,amount_right+:Value256,asset_right+:Account20,asset_left+:Account20,liquidity_token+:Account20,amount_liquidity_token+:Value256)".to_vec(),
            for_scale: b"Assets:Struct(RemoveLiquidity:Event(from:Account32,to:Account32,amount_left:Value128,amount_right:Value128,asset_right+:Account32,asset_left:Account32,liquidity_token:Account32,amount_liquidity_token:Value128))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            for_rlp: b"RemoveLiquidity:Struct(to:Account20,amount_left:Value256,amount_right:Value256,asset_right:Account20,asset_left:Account20,liquidity_token:Account20,amount_liquidity_token:Value256)".to_vec(),
            for_scale: b"RemoveLiquidity:Struct(to:Account32,amount_left:Value128,amount_right:Value128,asset_right:Account32,asset_left:Account32,liquidity_token:Account32,amount_liquidity_token:Value128)".to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_call_evm_contract_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"target".to_vec(), true),
            (b"value".to_vec(), false),
            (b"input".to_vec(), true),
            (b"gas_limit".to_vec(), false),
            (b"max_fee_per_gas".to_vec(), false),
            (b"max_priority_fee_per_gas".to_vec(), false),
            (b"nonce".to_vec(), false),
            (b"access_list".to_vec(), false),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"CallEvm:Log(target+:Account20,source+:Account20,tx_hash+:H256,input-:Bytes)"
                .to_vec(),
            for_scale: b"Evm:Struct(Call:Event(source+:Account20,target+:Account20))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            for_rlp: b"CallEvm:Struct(target:Account20,value:Value256,input:Bytes,gas_limit:Value256,max_fee_per_gas:Value256,max_priority_fee_per_gas:Value256,nonce:Value256,access_list:Bytes)"
                .to_vec(),
            for_scale: b"CallEvm:Struct(target:Account20,value:Value128,input:Bytes,gas_limit:Value128,max_fee_per_gas:Value128,max_priority_fee_per_gas:Value128,nonce:Value128,access_list:Bytes)"
                .to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_call_wasm_contract_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"contract".to_vec(), true),
            (b"value".to_vec(), false),
            (b"gas_limit".to_vec(), false),
            (b"storage_deposit_limit".to_vec(), false),
            (b"input".to_vec(), false),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"CallWasm:Log(caller+:Account32,contract+:Account32)".to_vec(),
            for_scale: b"Contracts:Struct(Call:Event(caller:Account32,contract:Account32))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            for_rlp: b"CallWasm:Struct(contract:Account32,value:Value128,gas_limit:Value128,storage_deposit_limit:Value128,input:Bytes)"
                .to_vec(),
            for_scale: b"CallWasm:Struct(contract:Account32,value:Value128,gas_limit:Value128,storage_deposit_limit:Value128,input:Bytes)"
                .to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

pub fn get_call_generic_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"target".to_vec(), false),
            (b"value".to_vec(), false),
            (b"input".to_vec(), true),
            (b"limit".to_vec(), false),
            (b"additional_params".to_vec(), false),
        ],
        ingress_abi_descriptors: PerCodecAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Call:Log(source+:Account20,target+:Account20,value+:Value128,input-:Bytes)".to_vec(),
            for_scale: b"Pallet:Struct(Call:Event(source+:Account32,target+:Account32,value+:Value128,input+:Bytes,limit+:Value128))".to_vec(),
        },
        egress_abi_descriptors: PerCodecAbiDescriptors {
            for_rlp: b"Call:Struct(target:Account20,value:Value128,input:Bytes,limit:Value128,additional_params:Bytes)"
                .to_vec(),
            for_scale: b"Call:Struct(target:Account32,value:Value128,input:Bytes,limit:Value128,additional_params:Bytes)"
                .to_vec(),
        },
        maybe_prefix_memo: None,
    }
}

#[cfg(test)]
mod test_abi_standards {
    use super::*;
    use crate::recode::Codec;
    use codec::Encode;
    use frame_support::assert_err;

    use crate::mini_mock::MiniRuntime;
    use hex_literal::hex;
    use sp_core::{H160, U256};

    use crate::{to_filled_abi::Eth2IngressEventLog, Abi, FilledAbi};
    use sp_core::H256;
    use sp_runtime::AccountId32;

    #[test]
    fn test_transfer_validate_arguments_against_received_substrate_balances_event() {
        let transfer_interface = get_sfx_transfer_abi();
        let ordered_args = vec![
            AccountId32::new([1; 32]).encode(), // to
            100u128.encode(),                   // amount
        ];

        // Pallet index byte
        let scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([4; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();

        // append an extra pallet event index byte as the second byte
        let mut scale_encoded_transfer_event = scale_encoded_transfer_event;
        scale_encoded_transfer_event.insert(1, 1u8);

        let res = transfer_interface.validate_arguments_against_received(
            &ordered_args,
            scale_encoded_transfer_event,
            &Codec::Scale,
            &Codec::Scale,
        );

        println!("{res:?}");
        assert!(res.is_ok());
    }

    #[test]
    fn test_transfer_validate_arguments_against_received_substrate_balances_event_with_prefix_memo()
    {
        let mut transfer_interface = get_sfx_transfer_abi();
        transfer_interface.set_prefix_memo(2u8);
        assert_eq!(transfer_interface.maybe_prefix_memo, Some(2u8));

        let ordered_args = vec![
            AccountId32::new([1; 32]).encode(), // to
            100u128.encode(),                   // amount
        ];
        let mut scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([2; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();
        // append an extra pallet event index byte as the second byte
        scale_encoded_transfer_event.insert(1, 1u8);

        let abi: Abi = transfer_interface
            .get_expected_ingress_descriptor(Codec::Scale)
            .try_into()
            .unwrap();

        let filled_transfer_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, scale_encoded_transfer_event.clone(), Codec::Scale)
                .unwrap();

        assert_eq!(filled_transfer_abi.get_prefix_memo(), Some(2u8));

        let res = transfer_interface.validate_arguments_against_received(
            &ordered_args,
            scale_encoded_transfer_event,
            &Codec::Scale,
            &Codec::Scale,
        );

        println!("{res:?}");
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_data_returns_scale_encoded_bytes_for_transfer() {
        let mut transfer_interface = get_sfx_transfer_abi();
        transfer_interface.set_prefix_memo(99u8);
        assert_eq!(transfer_interface.maybe_prefix_memo, Some(99u8));

        let _ordered_args = vec![
            AccountId32::new([1; 32]).encode(), // to
            100u128.encode(),                   // amount
        ];
        let mut scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([4; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();

        scale_encoded_transfer_event.insert(1, 1u8);

        let abi: Abi = transfer_interface
            .get_expected_ingress_descriptor(Codec::Scale)
            .try_into()
            .unwrap();

        let filled_transfer_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, scale_encoded_transfer_event.clone(), Codec::Scale)
                .unwrap();

        assert_eq!(filled_transfer_abi.get_data(), scale_encoded_transfer_event);
    }

    #[test]
    fn test_transfer_rejects_arguments_against_received_substrate_balances_with_wrong_prefix_memo()
    {
        let mut transfer_interface = get_sfx_transfer_abi();
        transfer_interface.set_prefix_memo(99u8);
        assert_eq!(transfer_interface.maybe_prefix_memo, Some(99u8));

        let ordered_args = vec![
            AccountId32::new([1; 32]).encode(), // to
            100u128.encode(),                   // amount
        ];
        let mut scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([2; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();

        scale_encoded_transfer_event.insert(1, 1u8);

        let abi: Abi = transfer_interface
            .get_expected_ingress_descriptor(Codec::Scale)
            .try_into()
            .unwrap();

        let filled_transfer_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, scale_encoded_transfer_event.clone(), Codec::Scale)
                .unwrap();

        assert_eq!(filled_transfer_abi.get_prefix_memo(), Some(2u8));

        let res = transfer_interface.validate_arguments_against_received(
            &ordered_args,
            scale_encoded_transfer_event,
            &Codec::Scale,
            &Codec::Scale,
        );

        assert_err!(
            res,
            "SFXAbi::invalid prefix memo for -- expected: doesn't match received"
        );
    }

    #[test]
    fn test_transfer_validate_arguments_against_received_evm_balances_event() {
        let transfer_interface = get_sfx_transfer_abi();
        const HUNDRED: u128 = 100;

        let ordered_args = vec![
            AccountId32::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000054321"
            ))
            .encode(), // to
            HUNDRED.encode(), // amount
        ];

        let hundred_u256: U256 = U256::from(HUNDRED);
        let mut hundred_u256_bytes = [0u8; 32];
        hundred_u256.to_big_endian(&mut hundred_u256_bytes);

        let rlp_raw_log_bytes = Eth2IngressEventLog {
            address: H160::from_slice(&hex!("0909090909090909090909090909090909090909")),
            topics: vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000012321").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000054321").into(),
                hundred_u256_bytes.into(),
            ],
            data: hex!(
                "
			"
            )
            .into(),
        };

        println!("{:?}", rlp_raw_log_bytes.encode());
        println!("{:?}", rlp_raw_log_bytes);

        let res = transfer_interface.validate_arguments_against_received(
            &ordered_args,
            rlp_raw_log_bytes.encode(),
            &Codec::Scale,
            &Codec::Rlp,
        );

        println!("{res:?}");
        assert!(res.is_ok());
    }

    #[test]
    fn test_call_validate_arguments_against_received_evm_call_contract_event() {
        let call_interface = get_call_evm_contract_abi();
        const HUNDRED: u128 = 100;

        let ordered_args = vec![
            // target
            AccountId32::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000054321"
            ))
            .encode(),
            // value
            HUNDRED.encode(),
            // input
            H256::from([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8,
            ])
            .encode(),
            // limit
            HUNDRED.encode(),
            // max_fee_per_gas
            HUNDRED.encode(),
            // max_priority_fee_per_gas
            1u128.encode(),
            // nonce
            1u128.encode(),
            // access_list
            vec![],
        ];

        let rlp_raw_log_bytes = Eth2IngressEventLog {
            address: H160::from_slice(&hex!("0909090909090909090909090909090909090909")),
            topics: vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                // address of the smart contract -- target
                hex!("0000000000000000000000000000000000000000000000000000000000054321").into(),
                // address of the caller -- source
                hex!("0000000000000000000000000000000000000000000000000000000000012321").into(),
                //transaction hash
                hex!("3b9aca00e23c7ca8e3976f71de69e0be0e9c6f16b02a052f7d52fb1c39c7a8d3").into(),
            ],
            // encoded function parameters
            data: hex!(
                "
                   0000000000000000000000000000000000000000000000000000000102030405
			"
            )
            .into(),
        };

        let res = call_interface.validate_arguments_against_received(
            &ordered_args,
            rlp_raw_log_bytes.encode(),
            &Codec::Scale,
            &Codec::Rlp,
        );

        println!("{res:?}");

        assert!(res.is_ok());
    }

    #[test]
    fn test_call_validate_arguments_against_mocked_wasm_call_contract_event() {
        let call_interface = get_call_wasm_contract_abi();
        const HUNDRED: u128 = 100;
        const FIFTY: u128 = 50;

        let ordered_args = vec![
            // contract
            AccountId32::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000054321"
            ))
            .encode(),
            // value
            HUNDRED.encode(),
            // gas_limit
            FIFTY.encode(),
            // storage_deposit_limit
            FIFTY.encode(),
            // input
            H256::from([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8,
            ])
            .encode(),
        ];

        let wasm_contracts_called_event_mock =
            crate::mini_mock::MockWasmContractsEvent::<MiniRuntime>::Called {
                /// The account that called the `contract`.
                caller: AccountId32::from(hex!(
                    "0000000000000000000000000000000000000000000000000000000000012321"
                )),
                /// The contract that was called.
                contract: AccountId32::from(hex!(
                    "0000000000000000000000000000000000000000000000000000000000054321"
                )),
            };

        let mut wasm_contracts_called_event_encoded = wasm_contracts_called_event_mock.encode();
        wasm_contracts_called_event_encoded.insert(1, 2);

        let res = call_interface.validate_arguments_against_received(
            &ordered_args,
            wasm_contracts_called_event_encoded,
            &Codec::Scale,
            &Codec::Scale,
        );

        println!("{res:?}");

        assert!(res.is_ok());
    }
}
