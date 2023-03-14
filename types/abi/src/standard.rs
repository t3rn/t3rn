use crate::{
    sfx_abi::{IngressAbiDescriptors, SFXAbi},
    types::Sfx4bId,
};

pub fn standard_sfx_abi() -> Vec<Vec<(Sfx4bId, SFXAbi)>> {
    vec![
        vec![(*b"data", get_data_abi())],
        vec![(*b"tran", get_sfx_transfer_abi())],
        vec![(*b"tass", get_sfx_transfer_asset_abi())],
        vec![(*b"swap", get_swap_abi())],
        vec![(*b"aliq", get_add_liquidity_abi())],
        vec![(*b"rliq", get_remove_liquidity_abi())],
        vec![(*b"cevm", get_call_evm_contract_abi())],
        vec![(*b"wasm", get_call_wasm_contract_abi())],
        vec![(*b"cgen", get_call_generic_abi())],
    ]
}

pub fn standard_sfx_abi_ids() -> Vec<Sfx4bId> {
    vec![
        *b"data", *b"tran", *b"tass", *b"orml", *b"swap", *b"aliq", *b"cevm", *b"wasm", *b"comp",
    ]
}

pub fn get_sfx_transfer_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"amount".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Transfer:Log(from+:Account20,to+:Account20,amount+:Value128)".to_vec(),
            for_scale: b"Transfer:Enum(from:Account32,to:Account32,amount:Value128)".to_vec(),
        },
    }
}

pub fn get_sfx_transfer_asset_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"asset_id".to_vec(), true),
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"amount".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp:
                b"Assets:Log(asset_id+:Account20,from+:Account20,to+:Account20,amount+:Value128)"
                    .to_vec(),
            for_scale:
                b"Assets:Enum(asset_id:Account32,from:Account32,to:Account32,amount:Value128)"
                    .to_vec(),
        },
    }
}

pub fn get_data_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![(b"key".to_vec(), true)],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Tuple(key:Data,value:Data)".to_vec(),
            for_scale: b"Tuple(key:Data,value:Data)".to_vec(),
        },
    }
}

pub fn get_swap_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"amount_from".to_vec(), true),
            (b"amount_to".to_vec(), true),
            (b"asset_from".to_vec(), true),
            (b"asset_to".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Swap:Log(from+:Account20,to+:Account20,amount_from+:Value128,amount_to+:Value128,asset_from+:Account20,asset_to+:Account20)".to_vec(),
            for_scale: b"Swap:Enum(from:Account32,to:Account32,amount_from:Value128,amount_to:Value128,asset_from:Account32,asset_to:Account32)".to_vec(),
        },
    }
}

pub fn get_add_liquidity_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"asset_left".to_vec(), true),
            (b"asset_right".to_vec(), true),
            (b"liquidity_token".to_vec(), true),
            (b"amount_left".to_vec(), true),
            (b"amount_right".to_vec(), true),
            (b"amount_liquidity_token".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"AddLiquidity:Log(from+:Account20,to+:Account20,amount_from+:Value128,amount_to+:Value128,asset_from+:Account20,asset_to+:Account20)".to_vec(),
            for_scale: b"AddLiquidity:Enum(from:Account32,to:Account32,amount_from:Value128,amount_to:Value128,asset_from:Account32,asset_to:Account32)".to_vec(),
        },
    }
}

pub fn get_remove_liquidity_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"asset_left".to_vec(), true),
            (b"asset_right".to_vec(), true),
            (b"liquidity_token".to_vec(), true),
            (b"amount_left".to_vec(), true),
            (b"amount_right".to_vec(), true),
            (b"amount_liquidity_token".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"RemoveLiquidity:Log(from+:Account20,to+:Account20,amount_from+:Value128,amount_to+:Value128,asset_from+:Account20,asset_to+:Account20)".to_vec(),
            for_scale: b"RemoveLiquidity:Enum(from:Account32,to:Account32,amount_from:Value128,amount_to:Value128,asset_from:Account32,asset_to:Account32)".to_vec(),
        },
    }
}

pub fn get_call_evm_contract_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"source".to_vec(), false),
            (b"target".to_vec(), true),
            (b"value".to_vec(), false),
            (b"input".to_vec(), true),
            (b"gas_limit".to_vec(), false),
            (b"max_fee_per_gas".to_vec(), false),
            (b"max_priority_fee_per_gas".to_vec(), false),
            (b"nonce".to_vec(), false),
            (b"access_list".to_vec(), false),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"CallEvm:Log(target+:Account20,source+:Account20,tx_hash+:H256,input-:Bytes)"
                .to_vec(),
            for_scale: b"CallEvm:Log(source+:Account32,target+:Account32)".to_vec(),
        },
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
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"CallWasm:Log(caller+:Account32,contract+:Account32)".to_vec(),
            for_scale: b"CallWasm:Enum(caller:Account32,contract:Account32)".to_vec(),
        },
    }
}

pub fn get_call_generic_abi() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"source".to_vec(), true),
            (b"target".to_vec(), false),
            (b"value".to_vec(), false),
            (b"input".to_vec(), true),
            (b"limit".to_vec(), false),
            (b"additional_params".to_vec(), false),
            (b"insurance".to_vec(), false),
        ],
        ingress_abi_descriptors: IngressAbiDescriptors {
            // assume all indexed in topics ("+")
            for_rlp: b"Swap:Log(source+:Account20,target+:Account20,value+:Value128,input-:Bytes)".to_vec(),
            for_scale: b"Swap:Log(source+:Account32,target+:Account32,value+:Value128,input+:Bytes,limit+:Value128)".to_vec(),
        },
    }
}

#[cfg(test)]
mod test_abi_standards {
    use super::*;
    use crate::recode::Codec;
    use codec::Encode;

    use crate::mini_mock::MiniRuntime;
    use hex_literal::hex;
    use sp_core::{H160, U256};

    use crate::to_filled_abi::EthIngressEventLog;
    use sp_core::H256;
    use sp_runtime::AccountId32;

    #[test]
    fn test_transfer_validate_arguments_against_received_substrate_balances_event() {
        let transfer_interface = get_sfx_transfer_abi();
        let ordered_args = vec![
            AccountId32::new([2; 32]).encode(),
            AccountId32::new([1; 32]).encode(),
            100u128.encode(),
            50u128.encode(),
        ];
        let scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([2; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();

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
    fn test_transfer_validate_arguments_against_received_evm_balances_event() {
        let transfer_interface = get_sfx_transfer_abi();
        const HUNDRED: u128 = 100;
        const FIFTY: u128 = 50;

        let ordered_args = vec![
            H160::from(hex!("0000000000000000000000000000000000012321")).encode(),
            H160::from(hex!("0000000000000000000000000000000000054321")).encode(),
            HUNDRED.encode(),
            FIFTY.encode(),
        ];

        let hundred_u256: U256 = U256::from(HUNDRED);
        let mut hundred_u256_bytes = [0u8; 32];
        hundred_u256.to_big_endian(&mut hundred_u256_bytes);
        let rlp_raw_log_bytes = EthIngressEventLog(
            vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000012321").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000054321").into(),
                hundred_u256_bytes.into(),
            ],
            hex!(
                "
			"
            )
            .into(),
        );

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
        const FIFTY: u128 = 50;

        let ordered_args = vec![
            // source
            H160::from(hex!("0000000000000000000000000000000000012321")).encode(),
            // target
            H160::from(hex!("0000000000000000000000000000000000054321")).encode(),
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
            // insurance
            FIFTY.encode(),
        ];

        let rlp_raw_log_bytes = EthIngressEventLog(
            vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                // address of the smart contract -- target
                hex!("0000000000000000000000000000000000000000000000000000000000054321").into(),
                // address of the caller -- source
                hex!("0000000000000000000000000000000000000000000000000000000000012321").into(),
                //transaction hash
                hex!("3b9aca00e23c7ca8e3976f71de69e0be0e9c6f16b02a052f7d52fb1c39c7a8d3").into(),
            ],
            // encoded function parameters
            hex!(
                "
                   0000000000000000000000000000000000000000000000000000000102030405
			"
            )
            .into(),
        );

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
            // insurance
            FIFTY.encode(),
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

        let res = call_interface.validate_arguments_against_received(
            &ordered_args,
            wasm_contracts_called_event_mock.encode(),
            &Codec::Scale,
            &Codec::Scale,
        );

        println!("{res:?}");

        assert!(res.is_ok());
    }
}
