use crate::{
    recode_rlp::RecodeRlp, recode_scale::RecodeScale, to_abi::Abi, to_filled_abi::FilledAbi,
    types::Name,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
pub enum Codec {
    #[default]
    Scale,
    Rlp,
}
// Implementable Recode trait for each codec.
pub trait Recode {
    fn chop_encoded(
        field_data: &[u8],
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError>;

    fn event_to_filled(
        field_data: &[u8],
        name: Option<Name>,
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(FilledAbi, usize), DispatchError>;
}

pub struct CrossRecode;

impl CrossRecode {
    pub fn chop_encoded(
        codec: Codec,
        field_data: &[u8],
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError> {
        match codec {
            Codec::Scale => RecodeScale::chop_encoded(field_data, fields_iter_clone),
            Codec::Rlp => RecodeRlp::chop_encoded(field_data, fields_iter_clone),
        }
    }

    pub fn event_to_filled(
        codec: Codec,
        field_data: &[u8],
        name: Option<Name>,
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        match codec {
            Codec::Scale => RecodeScale::event_to_filled(field_data, name, fields_iter_clone),
            Codec::Rlp => RecodeRlp::event_to_filled(field_data, name, fields_iter_clone),
        }
    }
}

pub fn trim_bytes(input: &[u8], n: usize) -> &[u8] {
    let len = input.len();
    if n >= len {
        &[]
    } else {
        &input[n..]
    }
}

pub fn take_last_n(input: &[u8], n: usize) -> Result<&[u8], DispatchError> {
    let len = input.len();
    if n > len {
        Err(DispatchError::Other("take_last_n::Invalid size of input"))
    } else {
        Ok(&input[(len - n)..len])
    }
}

pub fn split_bytes(input: &[u8], n: usize) -> Result<(&[u8], &[u8]), DispatchError> {
    let len = input.len();
    if n > len {
        Err(DispatchError::Other("split_bytes::Invalid size of input"))
    } else {
        Ok(input.split_at(n))
    }
}

pub fn trim_till_non_zero(input: &[u8], _n: usize) -> Result<(&[u8], &[u8]), DispatchError> {
    let len = input.len();
    let mut pos: usize = 0;
    while pos < len {
        if input[pos] != 0u8 {
            break
        }
        pos += 1;
    }
    split_bytes(input, pos)
}

pub fn trim_bytes_to(input: &[u8], n: usize) -> Result<Vec<u8>, DispatchError> {
    let len = input.len();
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
