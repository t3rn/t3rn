#![cfg_attr(not(feature = "std"), no_std)]
use crate::side_effects::standards::SideEffectProtocol;
use codec::Encode;
use hex_literal::hex;
use sp_std::vec;
use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    side_effect::SideEffect,
};

pub use t3rn_primitives::volatile::{
    LocalState, Volatile, FROM_2XX_32B_HASH, TO_2XX_32B_HASH, VALUE_2XX_32B_HASH,
};

pub type Bytes = Vec<u8>;
pub type Arguments = Vec<Bytes>;
pub type BlockNumber = u64;
pub type BalanceOf = u64;
pub type AccountId = u64;
pub type Hashing = sp_runtime::traits::BlakeTwo256;

// Below constants are for arguments with 32 insurance bytes (u128 insurance + u128 reward)
pub const FROM_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    104, 233, 34, 225, 243, 183, 245, 115, 238, 249, 50, 31, 27, 119, 67, 7, 185, 171, 247, 181,
    79, 165, 70, 60, 115, 240, 129, 166, 48, 60, 13, 175,
];
pub const TO_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    31, 120, 12, 224, 199, 234, 123, 29, 60, 196, 117, 123, 175, 65, 138, 109, 153, 238, 57, 163,
    170, 175, 165, 116, 45, 240, 115, 5, 58, 71, 98, 176,
];
pub const VALUE_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    120, 169, 120, 229, 75, 105, 236, 96, 175, 216, 249, 235, 193, 172, 52, 91, 176, 211, 196, 113,
    67, 187, 56, 79, 246, 44, 157, 211, 241, 118, 129, 136,
];
pub const INSURANCE_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    229, 90, 225, 72, 45, 216, 144, 112, 54, 62, 210, 104, 127, 236, 151, 243, 106, 148, 239, 220,
    214, 209, 16, 10, 183, 87, 121, 232, 123, 131, 3, 98,
];

// Below constants are for arguments without (with empty) insurance bytes
pub const NO_INSURANCE_FROM_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    135, 186, 74, 120, 142, 62, 53, 142, 138, 172, 35, 115, 53, 35, 112, 132, 70, 21, 133, 206,
    214, 241, 158, 240, 232, 14, 103, 230, 2, 138, 48, 249,
];
pub const NO_INSURANCE_TO_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    199, 72, 192, 11, 32, 51, 108, 91, 105, 147, 25, 20, 138, 202, 3, 90, 98, 199, 206, 235, 234,
    196, 116, 232, 121, 58, 192, 209, 103, 203, 6, 42,
];
pub const NO_INSURANCE_VALUE_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    193, 177, 115, 243, 35, 135, 13, 128, 231, 110, 19, 241, 46, 211, 177, 112, 16, 238, 55, 182,
    59, 50, 248, 132, 229, 193, 142, 71, 149, 2, 177, 241,
];
pub const NO_INSURANCE_INSURANCE_PLUS_PREFIX_2XX_32B_HASH: [u8; 32] = [
    75, 151, 74, 133, 143, 67, 19, 230, 105, 140, 226, 120, 58, 208, 39, 135, 144, 132, 212, 138,
    213, 107, 157, 73, 99, 211, 237, 206, 137, 158, 242, 99,
];

pub fn produce_test_side_effect(
    name: [u8; 4],
    arguments: Arguments,
    signature: Vec<u8>,
) -> SideEffect<AccountId, BlockNumber, BalanceOf> {
    SideEffect::<AccountId, BlockNumber, BalanceOf> {
        target: [0, 0, 0, 0],
        prize: 0,
        ordered_at: 0,
        encoded_action: name.to_vec(),
        encoded_args: arguments,
        signature,
        enforce_executioner: None,
    }
}

pub enum ArgVariant {
    A,
    B,
    C,
}

pub fn produce_test_args(types: Vec<(Type, ArgVariant)>) -> Arguments {
    let mut args: Arguments = vec![];
    for (t, variant) in types {
        match t {
            Type::Bytes(n) => {
                let mut bytes: Vec<u8> = vec![];
                for _ in 0..n {
                    bytes.push(0u8);
                }
                args.push(bytes);
            }
            Type::Address(n) => {
                args.push(match n {
                    32 => match variant {
                        ArgVariant::A => {
                            hex!("0909090909090909090909090909090909090909090909090909090909090909")
                                .into()
                        }
                        ArgVariant::B => {
                            hex!("0606060606060606060606060606060606060606060606060606060606060606")
                                .into()
                        }
                        ArgVariant::C => {
                            hex!("0303030303030303030303030303030303030303030303030303030303030303")
                                .into()
                        }
                    },
                    _ => unimplemented!(),
                });
            }
            Type::OptionalInsurance => {
                args.push(match variant {
                    ArgVariant::A => {
                        hex!("0100000000000000000000000000000002000000000000000000000000000000")
                            .into()
                    }
                    ArgVariant::B => {
                        hex!("0200000000000000000000000000000004000000000000000000000000000000")
                            .into()
                    }
                    ArgVariant::C => {
                        hex!("0300000000000000000000000000000001000000000000000000000000000000")
                            .into()
                    }
                });
            }
            Type::Uint(n) => {
                args.push(match n {
                    32 => match variant {
                        ArgVariant::A => 1u32.encode(),
                        ArgVariant::B => 2u32.encode(),
                        ArgVariant::C => 3u32.encode(),
                    },
                    64 => match variant {
                        ArgVariant::A => 1u64.encode(),
                        ArgVariant::B => 2u64.encode(),
                        ArgVariant::C => 3u64.encode(),
                    },
                    128 => match variant {
                        ArgVariant::A => 1u128.encode(),
                        ArgVariant::B => 2u128.encode(),
                        ArgVariant::C => 3u128.encode(),
                    },
                    _ => unimplemented!(),
                });
            }
            _ => unimplemented!(),
        }
    }
    args
}

pub fn assert_populated_state(
    local_state: LocalState,
    expected_arguments: Arguments,
    keys: Vec<[u8; 32]>,
) {
    for (i, key) in keys.iter().enumerate() {
        println!("ASSERT {:?}", key);
        assert_eq!(local_state.state.get(key), Some(&expected_arguments[i]));
    }
}

pub fn assert_populated_state_auto_key_derive(
    local_state: LocalState,
    expected_arguments: Arguments,
    protocol: Box<dyn SideEffectProtocol>,
    side_effect_id: Bytes,
) {
    let derive_for_properties = protocol.get_arguments_2_state_mapper();
    let mut keys: Vec<Bytes> = vec![];
    for prop in derive_for_properties {
        let key = LocalState::stick_key_with_prefix(prop.clone().encode(), side_effect_id.clone());
        println!("DERIVED KEYS = {:?} FOR PROP {:?}", key, prop);

        keys.push(key);
    }
    println!("DERIVED KEYS = {:?}", keys);

    for (i, key) in keys.iter().enumerate() {
        println!(
            "AUTO ASSERT GET FOR KEY {:?} {:?} VAL = {:?}, HASH_KEY {:?}",
            i,
            key,
            local_state.get(key.clone()),
            LocalState::key_2_state_key(key),
        );

        assert_eq!(local_state.get(key), Some(&expected_arguments[i]));
    }
}

pub fn assert_correct_validation_and_populated_state(
    local_state: &mut LocalState,
    side_effect: SideEffect<AccountId, BlockNumber, BalanceOf>,
    populate_state_args: Arguments,
    protocol: Box<dyn SideEffectProtocol>,
) {
    // let encoded_args_input = produce_test_args(args_variants);
    //
    // let valid_side_effect_no_signature =
    //     produce_test_side_effect(side_effect_name, encoded_args_input.clone(), vec![]);

    let valid_side_effect_id = side_effect.generate_id::<Hashing>();
    let abi: GatewayABIConfig = Default::default();
    let res = protocol.validate_args(
        side_effect.encoded_args.clone(),
        &abi,
        local_state,
        Some(valid_side_effect_id.as_ref().to_vec()),
    );

    assert_eq!(res, Ok(()));

    assert_populated_state_auto_key_derive(
        local_state.clone(),
        populate_state_args, // Consider going back to populate_state_args since when re-using args from SE emtpy values aren't checked
        protocol,
        valid_side_effect_id.as_ref().to_vec(),
    );
}

pub fn produce_and_validate_side_effect(
    args_variants: Vec<(Type, ArgVariant)>,
    local_state: &mut LocalState,
    protocol: Box<dyn SideEffectProtocol>,
) -> SideEffect<AccountId, BlockNumber, BalanceOf> {
    let encoded_transfer_args_input = produce_test_args(args_variants);

    let valid_transfer_side_effect = produce_test_side_effect(
        protocol.get_id(),
        encoded_transfer_args_input.clone(),
        vec![],
    );

    assert_correct_validation_and_populated_state(
        local_state,
        valid_transfer_side_effect.clone(),
        encoded_transfer_args_input,
        protocol,
    );

    valid_transfer_side_effect
}
