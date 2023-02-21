use crate::{to_abi::Abi, to_filled_abi::FilledAbi};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
pub enum Codec {
    #[default]
    Scale,
    Rlp,
}

pub fn trim_bytes(input: &[u8], n: usize) -> &[u8] {
    let len = input.len();
    if n >= len {
        &[]
    } else {
        &input[n..]
    }
}

pub fn split_bytes(input: &[u8], n: usize) -> Result<(&[u8], &[u8]), DispatchError> {
    let len = input.len();
    println!("split_bytes::len vs input size {} {}", n, input.len());

    if n > len {
        Err(DispatchError::Other("split_bytes::Invalid size of input"))
    } else {
        Ok(input.split_at(n))
    }
}

pub fn trim_bytes_to(input: &[u8], n: usize) -> Result<Vec<u8>, DispatchError> {
    let len = input.len();
    println!("trim_bytes_to::len vs input size {} {}", len, input.len());
    if n > len {
        Err(DispatchError::Other("trim_bytes_mut::Invalid input"))
    } else {
        let trimmed = input[..n].to_vec();
        Ok(trimmed)
    }
}

pub fn try_trim_bytes(input: &[u8], n: usize) -> Result<&[u8], DispatchError> {
    match trim_bytes(input, n) {
        [] => Err(DispatchError::Other("try_trim_bytes::Invalid input")),
        x => Ok(x),
    }
}

pub fn recode_bytes_with_descriptor(
    encoded_bytes: Vec<u8>,
    abi_descriptor: Vec<u8>,
    in_codec: Codec,
    out_codec: Codec,
) -> Result<Vec<u8>, DispatchError> {
    let abi: Abi = abi_descriptor.try_into()?;
    let filled_abi = FilledAbi::try_fill_abi(abi, encoded_bytes, in_codec.clone())?;
    filled_abi.recode_as(&in_codec, &out_codec)
}
