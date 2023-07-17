pub use circuit_runtime_types::{AccountId, Balance as BalanceOf, BlockNumber};
use codec::Encode;
use hex_literal::hex;
use sp_core::U256;
use sp_std::{convert::TryInto, vec, vec::Vec};
use t3rn_types::{sfx::SideEffect, types::Bytes};

pub type Arguments = Vec<Bytes>;
pub type Hashing = sp_runtime::traits::BlakeTwo256;

pub const FIRST_REQUESTER_NONCE: u32 = 0;

#[derive(Clone)]
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
            // FIXME: for Option arguments, the values need one more element
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
            ArgVariant::A => vec![U256::from(1).encode()],
            ArgVariant::B => vec![U256::from(40000000).encode()],
            ArgVariant::C => vec![U256::from(0).encode()],
        },
        Abi::Value128(_) => match args_variant {
            ArgVariant::A => vec![1u128.encode()],
            ArgVariant::B => vec![40000000u128.encode()],
            ArgVariant::C => vec![0u128.encode()],
        },
        Abi::Value64(_) => match args_variant {
            ArgVariant::A => vec![1u64.encode()],
            ArgVariant::B => vec![40000000u64.encode()],
            ArgVariant::C => vec![0u64.encode()],
        },
        Abi::Value32(_) => match args_variant {
            ArgVariant::A => vec![1u32.encode()],
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
        _ => todo!("produce_test_args_for_abi: {:#?}", abi),
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

    println!("produce_and_validate_side_effect -- ABI: {abi:?}");
    println!(
        "produce_and_validate_side_effect -- test args: {:?}",
        recursive_produce_test_args_for_abi(abi.clone(), args_variant.clone())
    );

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
    let mut requester_on_32b_as_vec = requester.encode();

    let nonce_as_4b_word: [u8; 4] = requester_nonce.to_be_bytes();
    let mut nonce_as_32b_word: [u8; 32];
    nonce_as_32b_word = [0; 32];
    nonce_as_32b_word[28..32].copy_from_slice(&nonce_as_4b_word);
    requester_on_32b_as_vec.extend_from_slice(&nonce_as_32b_word);

    Hasher::hash(requester_on_32b_as_vec.as_slice())
}
