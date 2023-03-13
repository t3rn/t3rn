use codec::Encode;
use hex_literal::hex;
use sp_std::{vec, vec::Vec};
use t3rn_types::{abi::Type, interface::SideEffectInterface, sfx::SideEffect, Bytes};

pub use circuit_runtime_types::{AccountId, Balance as BalanceOf, BlockNumber};

pub type Arguments = Vec<Bytes>;
pub type Hashing = sp_runtime::traits::BlakeTwo256;

pub const FIRST_REQUESTER_NONCE: u32 = 0;

/// Assumes BlockNumber, BalanceOf = u64, AccountId = AccountId32
pub fn produce_test_side_effect(
    name: [u8; 4],
    arguments: Arguments,
    signature: Vec<u8>,
) -> SideEffect<AccountId, BalanceOf> {
    let last_arg = arguments
        .last()
        .expect("there always should be at least one arg ");

    let insurance_and_reward_u128: [u128; 2] = codec::Decode::decode(&mut &last_arg.to_vec()[..])
        .expect("Each SDX should have its insurance and max_fee which always decode to [u128; 2]");

    SideEffect::<AccountId, BalanceOf> {
        target: [0, 0, 0, 0],
        max_reward: insurance_and_reward_u128[0],
        action: name,
        encoded_args: arguments,
        signature,
        insurance: insurance_and_reward_u128[1],
        enforce_executor: None,
        reward_asset_id: None,
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
                for m in 0..n {
                    bytes.push(m);
                }
                args.push(bytes);
            },
            Type::Address(n) => {
                args.push(match n {
                    32 => match variant {
                        ArgVariant::A =>
                            hex!("0909090909090909090909090909090909090909090909090909090909090909")
                                .into(),
                        ArgVariant::B =>
                            hex!("0606060606060606060606060606060606060606060606060606060606060606")
                                .into(),
                        ArgVariant::C =>
                            hex!("0303030303030303030303030303030303030303030303030303030303030303")
                                .into(),
                    },
                    _ => unimplemented!(),
                });
            },
            Type::OptionalInsurance => {
                args.push(match variant {
                    ArgVariant::A =>
                        hex!("0100000000000000000000000000000001000000000000000000000000000000")
                            .into(),
                    ArgVariant::B =>
                        hex!("0200000000000000000000000000000002000000000000000000000000000000")
                            .into(),
                    ArgVariant::C =>
                        hex!("0300000000000000000000000000000003000000000000000000000000000000")
                            .into(),
                });
            },
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
            },
            _ => unimplemented!(),
        }
    }
    args
}

pub fn produce_and_validate_side_effect(
    sfx_interface: SideEffectInterface,
    args_variants: Vec<(Type, ArgVariant)>,
) -> SideEffect<AccountId, BalanceOf> {
    produce_test_side_effect(
        sfx_interface.get_id(),
        produce_test_args(args_variants),
        vec![],
    )
}

pub fn generate_xtx_id<Hasher: sp_core::Hasher>(
    requester: AccountId,
    requester_nonce: u32,
) -> <Hasher as sp_core::Hasher>::Out {
    let mut requester_and_nonce = requester.encode();
    requester_and_nonce.extend_from_slice(&requester_nonce.to_be_bytes());
    Hasher::hash(&requester_and_nonce)
}
