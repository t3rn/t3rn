pub use circuit_runtime_types::{AccountId, Balance as BalanceOf, BlockNumber};
use codec::Encode;
use hex_literal::hex;
use sp_core::U256;
use sp_std::{convert::TryInto, vec, vec::Vec};
use t3rn_types::{sfx::SideEffect, types::Bytes};

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

pub fn recursive_produce_test_args_for_abi(abi: Abi, args_variant: ArgVariant) -> Arguments {
    match abi {
        Abi::Struct(_, abi_vec) | Abi::Log(_, abi_vec) | Abi::Enum(_, abi_vec) => {
            let mut args = vec![];
            for field in abi_vec {
                args.append(&mut recursive_produce_test_args_for_abi(
                    *field,
                    args_variant.clone(),
                ));
            }
            args
        },
        Abi::Tuple(_, (abi1, abi2)) => match args_variant {
            ArgVariant::A | ArgVariant::B | ArgVariant::C =>
                recursive_produce_test_args_for_abi(*abi1, args_variant.clone())
                    .into_iter()
                    .chain(recursive_produce_test_args_for_abi(*abi2, args_variant).into_iter())
                    .collect(),
        },
        Abi::Vec(_, _abi_vec) => {
            unimplemented!("Vec is not supported yet")
        },
        Abi::Option(_, abi) => match args_variant {
            ArgVariant::A => vec![0u8.encode()],
            ArgVariant::B => recursive_produce_test_args_for_abi(*abi, args_variant),
            ArgVariant::C => recursive_produce_test_args_for_abi(*abi, args_variant),
        },
        Abi::Account20(_) => match args_variant {
            ArgVariant::A => vec![hex!("0909090909090909090909090909090909090909").into()],
            ArgVariant::B => vec![hex!("0606060606060606060606060606060606060606").into()],
            ArgVariant::C => vec![hex!("0303030303030303030303030303030303030303").into()],
        },
        Abi::Account32(_) => match args_variant {
            ArgVariant::A =>
                vec![
                    hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
                ],
            ArgVariant::B =>
                vec![
                    hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
                ],
            ArgVariant::C =>
                vec![
                    hex!("0303030303030303030303030303030303030303030303030303030303030303").into(),
                ],
        },
        Abi::H256(_) => match args_variant {
            ArgVariant::A =>
                vec![
                    hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
                ],
            ArgVariant::B =>
                vec![
                    hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
                ],
            ArgVariant::C =>
                vec![
                    hex!("0303030303030303030303030303030303030303030303030303030303030303").into(),
                ],
        },
        Abi::Bytes(_) => match args_variant {
            ArgVariant::A => vec![hex!("0909090909090909090909090909090909").into()],
            ArgVariant::B => vec![hex!("0606060606060606").into()],
            ArgVariant::C =>
                vec![
                    hex!("0303030303030303030303030303030303030303030303030303030303030303").into(),
                ],
        },
        Abi::Value256(_) => match args_variant {
            ArgVariant::A => vec![U256::from(30000).encode()],
            ArgVariant::B => vec![U256::from(40000000).encode()],
            ArgVariant::C => vec![U256::from(0).encode()],
        },
        Abi::Value128(_) => match args_variant {
            ArgVariant::A => vec![30000u128.encode()],
            ArgVariant::B => vec![40000000u128.encode()],
            ArgVariant::C => vec![0u128.encode()],
        },
        Abi::Value64(_) => match args_variant {
            ArgVariant::A => vec![30000u64.encode()],
            ArgVariant::B => vec![40000000u64.encode()],
            ArgVariant::C => vec![0u64.encode()],
        },
        Abi::Value32(_) => match args_variant {
            ArgVariant::A => vec![30000u32.encode()],
            ArgVariant::B => vec![40000000u32.encode()],
            ArgVariant::C => vec![0u32.encode()],
        },
        Abi::Byte(_) => match args_variant {
            ArgVariant::A => vec![1u8.encode()],
            ArgVariant::B => vec![255u8.encode()],
            ArgVariant::C => vec![0u8.encode()],
        },
        Abi::Bool(_) => match args_variant {
            ArgVariant::A => vec![true.encode()],
            ArgVariant::B => vec![false.encode()],
            ArgVariant::C => vec![0u8.encode()],
        },
    }
}

use t3rn_abi::{Abi, SFXAbi};
use t3rn_types::sfx::Sfx4bId;

pub fn produce_and_validate_side_effect(
    action: Sfx4bId,
    insurance: BalanceOf,
    max_reward: BalanceOf,
    codec: t3rn_abi::recode::Codec,
    args_variant: ArgVariant,
) -> SideEffect<AccountId, BalanceOf> {
    let sfx_interface = SFXAbi::get_standard_interface(action).expect("SFX should be registered");

    let abi: Abi = sfx_interface
        .get_expected_egress_descriptor(codec)
        .try_into()
        .expect("ABI should be valid for standard registered SFX");

    SideEffect::<AccountId, BalanceOf> {
        target: [0, 0, 0, 0],
        max_reward,
        action,
        encoded_args: recursive_produce_test_args_for_abi(abi, args_variant),
        signature: vec![],
        insurance,
        enforce_executor: None,
        reward_asset_id: None,
    }
}

pub fn generate_xtx_id<Hasher: sp_core::Hasher>(
    requester: AccountId,
    requester_nonce: u32,
) -> <Hasher as sp_core::Hasher>::Out {
    let mut requester_and_nonce = requester.encode();
    requester_and_nonce.extend_from_slice(&requester_nonce.to_be_bytes());
    Hasher::hash(&requester_and_nonce)
}
