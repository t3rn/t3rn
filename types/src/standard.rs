use crate::{
    abi::Type,
    interface::SideEffectInterface,
    side_effect::{
        Bytes, ADD_LIQUIDITY_SIDE_EFFECT_ID, ASSETS_TRANSFER_SIDE_EFFECT_ID, CALL_SIDE_EFFECT_ID,
        COMPOSABLE_CALL_SIDE_EFFECT_ID, DATA_SIDE_EFFECT_ID, EVM_CALL_SIDE_EFFECT_ID,
        ORML_TRANSFER_SIDE_EFFECT_ID, SWAP_SIDE_EFFECT_ID, TRANSFER_SIDE_EFFECT_ID,
        WASM_CALL_SIDE_EFFECT_ID,
    },
};
use scale_info::prelude::{boxed::Box, vec, vec::Vec};

pub fn standard_side_effects() -> Vec<SideEffectInterface> {
    vec![
        get_data_interface(),
        get_transfer_interface(),
        get_transfer_assets_interface(),
        get_transfer_orml_interface(),
        get_swap_interface(),
        get_add_liquidity_interface(),
        get_call_evm_interface(),
        get_call_wasm_interface(),
        get_call_composable_interface(),
    ]
}

pub fn standard_side_effects_ids() -> Vec<[u8; 4]> {
    vec![
        *b"data", *b"tran", *b"tass", *b"orml", *b"swap", *b"aliq", *b"cevm", *b"wasm", *b"comp",
    ]
}

/// All of events providers should provide conversion from target-native events
/// to the following events nomenclature
/// Note:
/// a) Appearing extra _executor field to the emitted event allows
/// for t3rn protocol to detect the origin of event emitter and whether
/// was it authorised Executors entity eligible for the optional reversibility
/// as per rules of the fail-safe execution with t3rn
/// a) Overloading event with extra _source field enables the escrowed security
/// by ensuring that the source emitting the event is an authorised t3rn escrow
/// smart contract / logic / XBI dest on the remote target
pub fn known_events() -> Vec<Bytes> {
    vec![
        b"Transfer(_executor,to,value)".to_vec(),
        // Used as a side effect for both swaps and add_liquidity
        b"MultiTransfer(_executor,to,asset_id,value)".to_vec(),
        b"CallEvm(_executor,to,asset_id,value)".to_vec(),
        b"CallWasm(_executor,to,asset_id,value)".to_vec(),
        b"CallComposable(_executor,to,asset_id,value)".to_vec(),
        b"CallCustom(_executor,to,asset_id,value)".to_vec(),
        // Overloaded events with extra _source field enabling escrow security
        b"MultiTransfer(_source,_executor,to,asset_id,value)".to_vec(),
        b"CallEvm(_source,_executor,to,asset_id,value)".to_vec(),
        b"CallWasm(_source,_executor,to,asset_id,value)".to_vec(),
        b"CallComposable(_source,_executor,to,asset_id,value)".to_vec(),
        b"CallCustom(_source,_executor,to,asset_id,value)".to_vec(),
    ]
}

pub fn get_data_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *DATA_SIDE_EFFECT_ID,
        name: b"data:get".to_vec(),
        argument_abi: vec![
            Type::DynamicBytes, // argument_0: key
        ],
        argument_to_state_mapper: vec![b"key".to_vec()],
        confirm_events: vec![b"<InclusionOnly>".to_vec()],
        escrowed_events: vec![],
        commit_events: vec![],
        revert_events: vec![],
    }
}

pub fn get_transfer_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *TRANSFER_SIDE_EFFECT_ID,
        name: b"transfer".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: from
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: value
            Type::OptionalInsurance, // argument_3: insurance
        ],
        argument_to_state_mapper: vec![
            b"from".to_vec(),
            b"to".to_vec(),
            b"value".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"Transfer(_executor,to,value)".to_vec()],
        escrowed_events: vec![b"Transfer(_source,_executor,to,value)".to_vec()],
        commit_events: vec![b"Transfer(_executor,to,value)".to_vec()],
        revert_events: vec![b"Transfer(_executor,from,value)".to_vec()],
    }
}

pub fn get_swap_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *SWAP_SIDE_EFFECT_ID,
        name: b"swap".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::Value,             // argument_2: amount_from
            Type::Value,             // argument_3: amount_to
            Type::DynamicBytes,      // argument_4: asset_from
            Type::DynamicBytes,      // argument_5: asset_to
            Type::OptionalInsurance, // argument_6: insurance
        ],
        argument_to_state_mapper: vec![
            b"caller".to_vec(),
            b"to".to_vec(),
            b"amount_from".to_vec(),
            b"amount_to".to_vec(),
            b"asset_from".to_vec(),
            b"asset_to".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"MultiTransfer(_executor,to,asset_to,amount_to)".to_vec()],
        escrowed_events: vec![b"MultiTransfer(_source,_executor,to,asset_to,amount_to)".to_vec()],
        commit_events: vec![b"MultiTransfer(_executor,to,asset_to,amount_to)".to_vec()],
        revert_events: vec![b"MultiTransfer(_executor,caller,asset_from,amount_from)".to_vec()],
    }
}

pub fn get_add_liquidity_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *ADD_LIQUIDITY_SIDE_EFFECT_ID,
        name: b"add_liquidity".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: to
            Type::DynamicBytes,      // argument_2: asset_left
            Type::DynamicBytes,      // argument_3: asset_right
            Type::DynamicBytes,      // argument_4: liquidity_token
            Type::Value,             // argument_5: amount_left
            Type::Value,             // argument_6: amount_right
            Type::Value,             // argument_7: amount_liquidity_token
            Type::OptionalInsurance, // argument_8: insurance
        ],
        argument_to_state_mapper: vec![
            b"caller".to_vec(),
            b"to".to_vec(),
            b"asset_left".to_vec(),
            b"asset_right".to_vec(),
            b"liquidity_token".to_vec(),
            b"amount_left".to_vec(),
            b"amount_right".to_vec(),
            b"amount_liquidity_token".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![
            b"MultiTransfer(_executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        escrowed_events: vec![
            // _xtx_id?
            b"MultiTransfer(_source,_executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        commit_events: vec![
            b"MultiTransfer(_executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
        ],
        revert_events: vec![
            b"MultiTransfer(_executor,caller,asset_left,amount_left)".to_vec(),
            b"MultiTransfer(_executor,caller,asset_right,amount_right)".to_vec(),
        ],
    }
}

pub fn get_transfer_assets_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *ASSETS_TRANSFER_SIDE_EFFECT_ID,
        name: b"transfer:assets".to_vec(),
        argument_abi: vec![
            Type::DynamicBytes,      // argument_0: asset_id
            Type::DynamicAddress,    // argument_1: from
            Type::DynamicAddress,    // argument_2: to
            Type::Value,             // argument_3: value
            Type::OptionalInsurance, // argument_4: insurance
        ],
        argument_to_state_mapper: vec![
            b"asset_id".to_vec(),
            b"from".to_vec(),
            b"to".to_vec(),
            b"value".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"MultiTransfer(_executor,asset_id,to,value)".to_vec()],
        escrowed_events: vec![b"MultiTransfer(_source,_executor,asset_id,to,value)".to_vec()],
        commit_events: vec![b"MultiTransfer(_executor,asset_id,to,value)".to_vec()],
        revert_events: vec![b"MultiTransfer(_executor,asset_id,from,value)".to_vec()],
    }
}

pub fn get_transfer_orml_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *ORML_TRANSFER_SIDE_EFFECT_ID,
        name: b"transfer:orml".to_vec(),
        argument_abi: vec![
            Type::DynamicBytes,      // argument_0: asset_id
            Type::DynamicAddress,    // argument_1: from
            Type::DynamicAddress,    // argument_2: to
            Type::Value,             // argument_3: value
            Type::OptionalInsurance, // argument_4: insurance
        ],
        argument_to_state_mapper: vec![
            b"asset_id".to_vec(),
            b"from".to_vec(),
            b"to".to_vec(),
            b"value".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"MultiTransfer(_executor,asset_id,to,value)".to_vec()],
        escrowed_events: vec![b"MultiTransfer(_source,_executor,asset_id,to,value)".to_vec()],
        commit_events: vec![b"MultiTransfer(_executor,asset_id,to,value)".to_vec()],
        revert_events: vec![b"MultiTransfer(_executor,asset_id,from,value)".to_vec()],
    }
}

pub fn get_call_generic_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *CALL_SIDE_EFFECT_ID,
        name: b"call:generic".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,    // argument_0: caller
            Type::DynamicAddress,    // argument_1: dest
            Type::Value,             // argument_2: value
            Type::DynamicBytes,      // argument_3: input
            Type::Value,             // argument_4: limit
            Type::DynamicBytes,      // argument_5: additional_params
            Type::OptionalInsurance, // argument_6: insurance
        ],
        argument_to_state_mapper: vec![
            b"caller".to_vec(),
            b"dest".to_vec(),
            b"value".to_vec(),
            b"input".to_vec(),
            b"limit".to_vec(),
            b"additional_params".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"CallGeneric(_executor,dest,value,input,limit)".to_vec()],
        escrowed_events: vec![b"CallGeneric(_source,_executor,dest,value,input,limit)".to_vec()],
        commit_events: vec![b"CallGeneric(_executor,dest,value,input,limit)".to_vec()],
        revert_events: vec![b"CallGeneric(_executor,dest,value,input,limit)".to_vec()],
    }
}

pub fn get_call_evm_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *EVM_CALL_SIDE_EFFECT_ID,
        name: b"call:evm".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,                 // argument_0: source
            Type::DynamicAddress,                 // argument_1: dest
            Type::Value,                          // argument_2: value
            Type::DynamicBytes, // argument_3: input // TODO: try to put this at the end so we don't have complicated chunks for arbitrary length input
            Type::Uint(64),     // argument_4: gas_limit
            Type::Value,        // argument_5: max_fee_per_gas
            Type::Option(Box::from(Type::Value)), // argument_6: max_priority_fee_per_gas
            Type::Option(Box::from(Type::Value)), // argument_7: nonce
            Type::DynamicBytes, // argument_8: access_list (since HF Berlin?)
            Type::OptionalInsurance, // argument_9: insurance
        ],
        argument_to_state_mapper: vec![
            b"source".to_vec(),
            b"target".to_vec(),
            b"value".to_vec(),
            b"input".to_vec(),
            b"gas_limit".to_vec(),
            b"max_fee_per_gas".to_vec(),
            b"max_priority_fee_per_gas".to_vec(),
            b"nonce".to_vec(),
            b"access_list".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"CallEvm(_executor,dest,value,input,gas_limit)".to_vec()],
        escrowed_events: vec![b"CallEvm(_source,_executor,dest,value,input,gas_limit)".to_vec()],
        commit_events: vec![b"CallEvm(_executor,dest,value,input,gas_limit)".to_vec()],
        revert_events: vec![b"CallEvm(_executor,dest,value,input,gas_limit)".to_vec()],
    }
}

pub fn get_call_wasm_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *WASM_CALL_SIDE_EFFECT_ID,
        name: b"call:wasm".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,                // argument_1: dest
            Type::Value,                         // argument_2: value
            Type::Value,                         // argument_3: gas_limit
            Type::Option(Box::new(Type::Value)), // argument_4: storage_deposit_limit
            Type::DynamicBytes,                  // argument_5: data
            Type::OptionalInsurance,             // argument_6: insurance
        ],
        argument_to_state_mapper: vec![
            b"dest".to_vec(),
            b"value".to_vec(),
            b"gas_limit".to_vec(),
            b"storage_deposit_limit".to_vec(),
            b"data".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"CallWasm(_executor,dest,data,input,gas_limit)".to_vec()],
        escrowed_events: vec![b"CallWasm(_source,_executor,dest,value,data,gas_limit)".to_vec()],
        commit_events: vec![b"CallWasm(_executor,dest,value,data,gas_limit)".to_vec()],
        revert_events: vec![b"CallWasm(_executor,dest,value,data,gas_limit)".to_vec()],
    }
}

pub fn get_call_composable_interface() -> SideEffectInterface {
    SideEffectInterface {
        id: *COMPOSABLE_CALL_SIDE_EFFECT_ID,
        name: b"call:comp".to_vec(),
        argument_abi: vec![
            Type::DynamicAddress,                // argument_1: dest
            Type::Value,                         // argument_2: value
            Type::Value,                         // argument_3: gas_limit
            Type::Option(Box::new(Type::Value)), // argument_4: storage_deposit_limit
            Type::DynamicBytes,                  // argument_5: data
            Type::OptionalInsurance,             // argument_6: insurance
        ],
        argument_to_state_mapper: vec![
            b"dest".to_vec(),
            b"value".to_vec(),
            b"gas_limit".to_vec(),
            b"storage_deposit_limit".to_vec(),
            b"data".to_vec(),
            b"insurance".to_vec(),
        ],
        confirm_events: vec![b"CallComposable(_executor,dest,data,input,gas_limit)".to_vec()],
        escrowed_events: vec![
            b"CallComposable(_source,_executor,dest,value,data,gas_limit)".to_vec(),
        ],
        commit_events: vec![b"CallComposable(_executor,dest,value,data,gas_limit)".to_vec()],
        revert_events: vec![b"CallComposable(_executor,dest,value,data,gas_limit)".to_vec()],
    }
}
