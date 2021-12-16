#![cfg_attr(not(feature = "std"), no_std)]

use crate::abi::{GatewayABIConfig, Type};
use crate::match_format::{ensure_str_err, StrLike};
use codec::{Decode, Encode};
use sp_std::vec;
use sp_std::vec::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub struct Signature {
    topic: StrLike,
    data: Vec<Vec<u8>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub enum Surrounding {
    Local,
    Remote,
}

pub trait HasXDNSAccess {
    fn get_abi(target: [u8; 4]) -> Result<GatewayABIConfig, &'static str>;
    fn get_my_target_id() -> [u8; 4];
    fn get_surrounding(target: [u8; 4]) -> Surrounding {
        if Self::get_my_target_id() == target {
            return Surrounding::Local;
        }
        Surrounding::Remote
    }
}

pub fn validate_next_arg(
    input: &Vec<u8>,
    abi_type: Type,
    start_pos: usize,
    end_pos: usize,
) -> Result<Vec<u8>, &'static str> {
    let next_arg_bytes: Vec<u8> = input[start_pos..end_pos].to_vec();
    abi_type.eval(next_arg_bytes.clone())?;
    Ok(next_arg_bytes)
}

pub fn validate_next_args(
    input: &Vec<u8>,
    args_abi: Vec<(Type, u16)>,
) -> Result<Vec<Vec<u8>>, &'static str> {
    let mut start_pos: usize = SIDE_EFFECT_HEADER_SIZE;
    let mut end_pos: usize = SIDE_EFFECT_HEADER_SIZE;
    let mut args_bytes: Vec<Vec<u8>> = vec![];

    for (arg_type, arg_size) in args_abi {
        println!("curr arg size {:?} ", arg_size);
        // ToDo: Eliminate possible casting err since as takes only lower bytes - ABI should be in usize
        end_pos += arg_size as usize;
        args_bytes.push(validate_next_arg(input, arg_type, start_pos, end_pos)?);
        start_pos = end_pos;
    }

    Ok(args_bytes)
}

pub const SIDE_EFFECT_HEADER_SIZE: usize = 12;
pub const TARGET_ID_SIZE: usize = 4;
pub const SIDE_EFFECT_NAME_SIZE: usize = 4;
pub const SIDE_EFFECT_TYPE_SIZE: usize = 4;
const SIDE_EFFECT_NAME_START_POS: usize = TARGET_ID_SIZE;
const SIDE_EFFECT_NAME_END_POS: usize = TARGET_ID_SIZE + SIDE_EFFECT_NAME_SIZE;
const SIDE_EFFECT_TYPE_START_POS: usize = SIDE_EFFECT_NAME_END_POS;

pub type RawSideEffectHeader = [[u8; 4]; 3];

pub fn read_raw_side_effect_header(input: Vec<u8>) -> Result<RawSideEffectHeader, &'static str> {
    ensure_str_err(
        input.len() >= SIDE_EFFECT_HEADER_SIZE,
        "Side Effect Raw Header can't be shorter than 12 bytes",
    )?;

    fn cut_out_4_bytes(input: &Vec<u8>, start_pos: usize) -> [u8; 4] {
        let mut tmp_4b: [u8; 4] = [0, 0, 0, 0];
        tmp_4b.copy_from_slice(&input[start_pos..start_pos + 4]);
        tmp_4b
    }

    // Expect target to be on the first 4 bytes
    let target_4b = cut_out_4_bytes(&input, 0);
    // Expect Side Effect Name to be on the next 4 bytes
    let se_name_4b = cut_out_4_bytes(&input, SIDE_EFFECT_NAME_START_POS);
    // Expect Side Effect Type to be on the next 4 bytes
    let se_type_4b = cut_out_4_bytes(&input, SIDE_EFFECT_TYPE_START_POS);

    Ok([target_4b, se_name_4b, se_type_4b])
}
