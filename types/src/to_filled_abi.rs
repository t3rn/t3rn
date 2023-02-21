use crate::{
    recode::{split_bytes, trim_bytes, Codec},
    to_abi::Abi,
    types::*,
};
use codec::{Decode, Encode};
use frame_support::ensure;

use primitive_types::H160;
use sp_core::{crypto::AccountId32, ByteArray};
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};
use std::u32;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum FilledAbi {
    Struct(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Enum(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Log(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Option(Option<Name>, Box<FilledAbi>),
    Bytes(Option<Name>, Data),
    Account20(Option<Name>, Data),
    Account32(Option<Name>, Data),
    Value256(Option<Name>, Data),
    Value128(Option<Name>, Data),
    Value64(Option<Name>, Data),
    Value32(Option<Name>, Data),
    Byte(Option<Name>, Data),
    Bool(Option<Name>, Data),
    Vec(Option<Name>, Box<Vec<FilledAbi>>, u8),
    Tuple(Option<Name>, (Box<FilledAbi>, Box<FilledAbi>)),
}

impl FilledAbi {
    pub fn get_data(&self) -> Data {
        match self {
            FilledAbi::Struct(_, fields, prefix_memo)
            | FilledAbi::Enum(_, fields, prefix_memo)
            | FilledAbi::Log(_, fields, prefix_memo) => {
                let mut data = vec![*prefix_memo];
                for field in fields {
                    data.extend_from_slice(field.get_data().as_slice());
                }
                data.clone()
            },
            FilledAbi::Option(_, field) => {
                let val_data = field.get_data();
                match val_data.is_empty() {
                    true => vec![0],
                    false => {
                        let mut data = vec![1];
                        data.extend_from_slice(val_data.as_slice());
                        data.clone()
                    },
                }
            },
            FilledAbi::Bytes(_, data) => data.clone(),
            FilledAbi::Account20(_, data) => data.clone(),
            FilledAbi::Account32(_, data) => data.clone(),
            FilledAbi::Value256(_, data) => data.clone(),
            FilledAbi::Value128(_, data) => data.clone(),
            FilledAbi::Value64(_, data) => data.clone(),
            FilledAbi::Value32(_, data) => data.clone(),
            FilledAbi::Byte(_, data) => data.clone(),
            FilledAbi::Bool(_, data) => data.clone(),
            FilledAbi::Vec(_, fields, prefix_memo) => {
                let mut data = vec![*prefix_memo];
                for field in *fields.clone() {
                    data.extend_from_slice(field.get_data().as_slice());
                }
                data.clone()
            },
            FilledAbi::Tuple(_, (field1, field2)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.clone()
            },
        }
    }

    pub fn get_by_name(&self, by_name: &Name) -> Option<FilledAbi> {
        fn recursive_get_by_name(abi: &FilledAbi, by_name: &Name) -> Option<FilledAbi> {
            match abi {
                FilledAbi::Struct(name, fields, _)
                | FilledAbi::Enum(name, fields, _)
                | FilledAbi::Log(name, fields, _) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(abi.clone())
                    }

                    for field in fields {
                        if let Some(data) = recursive_get_by_name(field, by_name) {
                            return Some(data)
                        }
                    }

                    None
                },
                FilledAbi::Option(name, field) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(abi.clone())
                    }

                    recursive_get_by_name(field, by_name)
                },
                | FilledAbi::Bytes(name, _data)
                | FilledAbi::Account20(name, _data)
                | FilledAbi::Account32(name, _data)
                | FilledAbi::Value256(name, _data)
                | FilledAbi::Value128(name, _data)
                | FilledAbi::Value64(name, _data)
                | FilledAbi::Value32(name, _data)
                | FilledAbi::Bool(name, _data)
                | FilledAbi::Byte(name, _data) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(abi.clone())
                    }

                    None
                },
                FilledAbi::Vec(_name, field, _) =>
                    recursive_get_by_name(field.get(0).unwrap(), by_name),
                FilledAbi::Tuple(_name, (field1, field2)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field2, by_name)
                },
            }
        }

        recursive_get_by_name(self, by_name)
    }

    pub fn get_data_by_name(&self, by_name: &Name) -> Option<Data> {
        fn recursive_get_data_by_name(abi: &FilledAbi, by_name: &Name) -> Option<Data> {
            match abi {
                FilledAbi::Struct(name, fields, _)
                | FilledAbi::Enum(name, fields, _)
                | FilledAbi::Log(name, fields, _) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(abi.encode())
                    }

                    for field in fields {
                        if let Some(data) = recursive_get_data_by_name(field, by_name) {
                            return Some(data)
                        }
                    }

                    None
                },
                FilledAbi::Option(name, field) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(abi.encode())
                    }

                    recursive_get_data_by_name(field, by_name)
                },
                | FilledAbi::Bytes(name, data)
                | FilledAbi::Account20(name, data)
                | FilledAbi::Account32(name, data)
                | FilledAbi::Value256(name, data)
                | FilledAbi::Value128(name, data)
                | FilledAbi::Value64(name, data)
                | FilledAbi::Value32(name, data)
                | FilledAbi::Byte(name, data) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(data.clone())
                    }

                    None
                },
                FilledAbi::Bool(name, data) => {
                    if name.as_ref() == Some(by_name) {
                        return Some(data.clone())
                    }

                    None
                },
                FilledAbi::Vec(_name, field, _) =>
                    recursive_get_data_by_name(field.get(0).unwrap(), by_name),
                FilledAbi::Tuple(_name, (field1, field2)) => {
                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field2, by_name)
                },
            }
        }

        recursive_get_data_by_name(self, by_name)
    }
}

pub fn ensure_vector_and_trim_prefix(
    data: &[u8],
    in_codec: &Codec,
) -> Result<Vec<u8>, DispatchError> {
    match in_codec {
        Codec::Scale => {
            let vector_val: Vec<u8> = data.to_vec();
            let _size_hint = vector_val.size_hint();
            let val = trim_bytes(data, 1);
            ensure!(!val.is_empty(), "recode_as_vector::Scale::InvalidDataSize");
            Ok(val.to_vec())
        },
        Codec::Rlp => {
            let rlp = rlp::Rlp::new(data);
            let rlp_encoded = rlp.as_raw();
            let val = trim_bytes(rlp_encoded, 1);
            ensure!(!val.is_empty(), "recode_as_vector::Rlp::InvalidDataSize");
            Ok(val.to_vec())
        },
    }
}

// Removes the first byte of the input data for SCALE encoded data and chops the data into fields by the given ABI size.
// For RLP relies on the RLP library to do the chopping, since RLP carries the type and size information within the data.
pub fn encoded_struct_chopper(
    field_data: &[u8],
    in_codec: &Codec,
    fields_iter_clone: IntoIter<Box<Abi>>,
) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError> {
    let (memo_prefix, right) = split_bytes(field_data, 1)?;

    let chopped_field_data: Vec<Vec<u8>> = match in_codec {
        Codec::Scale => {
            let mut no_strut_prefix_data = right;
            // Make sure original fields iterator won't be consumed
            // let fields_iter_clone = fields_descriptors.iter().cloned();
            let fields_iter = fields_iter_clone.peekable();
            fields_iter
                .map(|field_descriptor| {
                    let field_size = field_descriptor.get_size();
                    println!("abi: {field_descriptor:?}  + field_size: {field_size:?}");
                    let (left, right) = split_bytes(no_strut_prefix_data, field_size)?;
                    no_strut_prefix_data = right;
                    Ok(left.to_vec())
                })
                .collect::<Result<Vec<Vec<u8>>, DispatchError>>()?
        },
        Codec::Rlp => {
            let rlp = rlp::Rlp::new(field_data);
            rlp.into_iter().map(|rlp| rlp.as_raw().to_vec()).collect()
        },
    };
    println!("chopped_field_data: {chopped_field_data:?}");
    ensure!(
        memo_prefix.len() == 1,
        "encoded_struct_chopper - Memo cannot be empty for structs"
    );
    Ok((
        chopped_field_data.into_iter(),
        *memo_prefix
            .first()
            .expect("encoded_struct_chopper - Memo cannot be empty for structs"),
    ))
}
impl FilledAbi {
    pub fn recode_as(&self, in_codec: &Codec, out_codec: &Codec) -> Result<Data, DispatchError> {
        match self {
            FilledAbi::Struct(_name, fields, struct_prefix_memo)
            | FilledAbi::Enum(_name, fields, struct_prefix_memo)
            | FilledAbi::Log(_name, fields, struct_prefix_memo) => {
                // will prolly have to remove and re-add the struct prefix at the end
                let mut encoded_fields: Vec<u8> = vec![];
                for field in fields {
                    encoded_fields.extend_from_slice(&field.recode_as(in_codec, out_codec)?[..]);
                }

                match (in_codec, out_codec) {
                    (Codec::Scale, Codec::Scale) => Ok(encoded_fields),
                    (Codec::Rlp, Codec::Rlp) => Ok(encoded_fields),
                    (Codec::Rlp, Codec::Scale) => Ok({
                        let mut scale_encoded_struct = vec![*struct_prefix_memo]; // how to calculate the prefix for a struct in SCALE?
                        scale_encoded_struct.extend_from_slice(&encoded_fields);
                        scale_encoded_struct
                    }),
                    (Codec::Scale, Codec::Rlp) => {
                        let mut rlp_encoded_struct = vec![*struct_prefix_memo]; // assume 0xc8 is the code for a struct
                        rlp_encoded_struct.extend_from_slice(&encoded_fields);
                        Ok(rlp_encoded_struct)
                    },
                }
            },
            FilledAbi::Option(_name, field) => {
                // Option Prefix
                let mut encoded_fields: Vec<u8> = vec![];
                encoded_fields.extend_from_slice(&field.recode_as(in_codec, out_codec)?[..]);
                match (in_codec, out_codec) {
                    (_, Codec::Scale) => Ok({
                        let mut scale_encoded_option = match encoded_fields.is_empty() {
                            false => vec![0x01],
                            true => vec![0x00],
                        };
                        scale_encoded_option.extend_from_slice(&encoded_fields);
                        scale_encoded_option
                    }),
                    (_, Codec::Rlp) => Ok({
                        let mut rlp_encoded_list = vec![0xc3]; // assume 0xc3 is the code for an option
                        rlp_encoded_list.extend_from_slice(&encoded_fields);
                        rlp_encoded_list
                    }),
                }
            },
            FilledAbi::Vec(_name, fields, _prefix_memo) => {
                let mut encoded_fields: Vec<u8> = vec![];
                let encoded_data: Data = fields
                    .iter()
                    .map(|field| field.recode_as(in_codec, out_codec))
                    .collect::<Result<Vec<Data>, DispatchError>>()?
                    .concat();
                encoded_fields.extend_from_slice(&encoded_data);

                match (in_codec, out_codec) {
                    (_, Codec::Scale) => Ok(encoded_fields),
                    (_, Codec::Rlp) => Ok(rlp::encode_list(&encoded_fields).to_vec()),
                }
            },
            FilledAbi::Byte(_name, data) | FilledAbi::Bool(_name, data) => Ok(data.clone()),
            FilledAbi::Account32(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let decoded_account: AccountId32 = AccountId32::decode(&mut &data[..])
                        .map_err(|_e| "Account32 error at recoding back to Scale")?;

                    Ok(rlp::encode(&decoded_account.to_raw_vec()).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    // In RLP the account is encoded as a list of 33 bytes.
                    ensure!(
                        data.len() == 33,
                        "RLP encoded account should be 33 bytes long"
                    );
                    let no_prefix_data: [u8; 32] = data[1..33]
                        .try_into()
                        .map_err(|_e| "Account32 error at recoding back to [u8; 32]")?;

                    let account_id = AccountId32::new(no_prefix_data);
                    Ok(account_id.encode())
                },
            },
            FilledAbi::Account20(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let decoded_account: H160 = H160::decode(&mut &data[..])
                        .map_err(|_e| "Account20 error at recoding back to Scale")?;

                    Ok(rlp::encode(&decoded_account.as_bytes()).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    // In RLP the account is encoded as a list of 21 bytes.
                    ensure!(
                        data.len() == 21,
                        "RLP encoded account should be 21 bytes long"
                    );
                    let account_id_20: H160 = H160::from_slice(&data[1..21]);
                    Ok(account_id_20.encode())
                },
            },
            FilledAbi::Value32(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u32 = Decode::decode(&mut &data[..]).unwrap();
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u32 = rlp::decode(&data[..]).unwrap();
                    Ok(value.encode())
                },
            },
            FilledAbi::Value64(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u64 = Decode::decode(&mut &data[..]).unwrap();
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u64 = rlp::decode(&data[..]).unwrap();
                    Ok(value.encode())
                },
            },
            FilledAbi::Value128(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u128 = Decode::decode(&mut &data[..]).unwrap();
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u128 = rlp::decode(&data[..]).unwrap();
                    Ok(value.encode())
                },
            },
            FilledAbi::Value256(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value_256: sp_core::U256 = Decode::decode(&mut &data[..]).unwrap();
                    //rlp doesn't recognize u256, so we convert to u128
                    let value: u128 = value_256.low_u128();
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u128 = rlp::decode(&data[..]).unwrap();
                    let value_256: sp_core::U256 = value.into();
                    Ok(value_256.encode())
                },
            },
            // FilledAbi::Tuple(_name, (field_a, field_b)) => {
            //     let mut encoded_fields: Vec<u8> = vec![];
            //     encoded_fields.extend_from_slice(
            //         &mut &field_a.recode_as(in_codec.clone(), out_codec.clone())?[..],
            //     );
            //     encoded_fields.extend_from_slice(
            //         &mut &field_b.recode_as(in_codec.clone(), out_codec.clone())?[..],
            //     );
            //
            //     match (in_codec, out_codec) {
            //         (_, Codec::Scale) => Ok(encoded_fields),
            //         (_, Codec::Rlp) => Ok(rlp::encode_list(&encoded_fields).to_vec()),
            //     }
            // },
            // FilledAbi::Vec(name, field, _) => {
            //     // Option Prefix
            //     let mut encoded_fields: Vec<u8> = vec![];
            //     encoded_fields.extend_from_slice(
            //         &mut &Self::recursive_encode_filled_abi(*field, out_codec.clone())?[..],
            //     );
            //     match out_codec {
            //         Recode::Scale(_) => Ok(encoded_fields),
            //         Recode::Rlp(_) => Ok({
            //             let mut rlp_encoded_list = vec![0xc3]; // assume 0xc3 is the code for a list
            //             rlp_encoded_list.extend_from_slice(&mut &encoded_fields);
            //             rlp_encoded_list
            //         }),
            //     }
            // },
            _ => Err(DispatchError::Other("Not implemented yet")),
        }
    }

    // Fills the ABI with raw data, only assuming the type size of input codec
    pub fn try_fill_abi(abi: Abi, data: Data, in_codec: Codec) -> Result<FilledAbi, DispatchError> {
        fn recursive_fill_abi(
            abi: Abi,
            field_data: &[u8],
            in_codec: Codec,
        ) -> Result<(FilledAbi, usize), DispatchError> {
            let l = field_data.len();
            println!("recursive_fill_abi - field_data: {field_data:?} + length: {l}");
            match abi {
                Abi::Struct(name, fields_descriptors)
                | Abi::Log(name, fields_descriptors)
                | Abi::Enum(name, fields_descriptors) => {
                    let mut fields = Vec::new();

                    let (mut chopped_field_data_iter, memo_prefix) = encoded_struct_chopper(
                        field_data,
                        &in_codec,
                        fields_descriptors.clone().into_iter(),
                    )?;

                    for chopped_piece in chopped_field_data_iter.clone() {
                        println!("Abi::Struct - chopped_piece: {chopped_piece:?}");
                    }

                    let mut total_struct_size = 0usize;

                    for field_descriptor in fields_descriptors {
                        println!("Abi::Struct - next field_descriptor: {field_descriptor:?}");
                        let (field, size) = recursive_fill_abi(
                            *field_descriptor,
                            chopped_field_data_iter
                                .next()
                                .ok_or::<DispatchError>("Abi::Struct - Not enough data".into())?
                                .as_slice(),
                            in_codec.clone(),
                        )?;
                        println!("Abi::Struct - size: {size:?}");
                        total_struct_size += size;
                        fields.push(Box::new(field));
                    }

                    Ok((
                        FilledAbi::Struct(name, fields, memo_prefix),
                        total_struct_size,
                    ))
                },
                Abi::Option(name, field_descriptor) => {
                    println!("Abi::Option - field_data: {field_data:?}");
                    let no_option_prefix_data = trim_bytes(field_data, 1);
                    println!("Abi::Option - no_option_prefix_data: {no_option_prefix_data:?}");
                    let (field, size) =
                        recursive_fill_abi(*field_descriptor, no_option_prefix_data, in_codec)?;
                    Ok((FilledAbi::Option(name, Box::new(field)), size + 1))
                },
                Abi::Bytes(name) => {
                    let recoded_bytes = field_data.to_vec();
                    Ok((
                        FilledAbi::Bytes(name, recoded_bytes.to_vec()),
                        recoded_bytes.len(),
                    ))
                },
                Abi::Account20(name) => {
                    let len = match in_codec {
                        Codec::Scale => 20usize,
                        // Rlp encoding of AccountId20 is 21 bytes
                        // since doesn't support the array encoding
                        // and therefore encodes it as a list (+with the prefix)
                        Codec::Rlp => 21usize,
                    };

                    ensure!(
                        field_data.len() == len,
                        "Account20::InvalidDataSize: expected {len:?}",
                    );

                    Ok((
                        FilledAbi::Account20(name, field_data.to_vec()),
                        field_data.len(),
                    ))
                },
                Abi::Account32(name) => {
                    println!(
                        "Abi::Account32 - field_data: {:?}, len: {:?}",
                        field_data,
                        field_data.len()
                    );
                    let len = match in_codec {
                        // Expect AccountId32 to be 32 bytes
                        Codec::Scale => 32usize,
                        // Rlp encoding of AccountId32 is 33 bytes
                        // since doesn't support the array encoding
                        // and therefore encodes it as a list (+with the prefix)
                        Codec::Rlp => 33usize,
                    };
                    ensure!(
                        field_data.len() == len,
                        "Account32::InvalidDataSize: expected {len:?}",
                    );

                    Ok((
                        FilledAbi::Account32(name, field_data.to_vec()),
                        field_data.len(),
                    ))
                },
                Abi::Value256(name) => Ok((
                    FilledAbi::Value256(name, field_data.to_vec()),
                    field_data.len(),
                )),
                Abi::Value128(name) => Ok((
                    FilledAbi::Value128(name, field_data.to_vec()),
                    field_data.len(),
                )),
                Abi::Value64(name) => Ok((
                    FilledAbi::Value64(name, field_data.to_vec()),
                    field_data.len(),
                )),
                Abi::Value32(name) => Ok((
                    FilledAbi::Value32(name, field_data.to_vec()),
                    field_data.len(),
                )),
                Abi::Byte(name) => {
                    ensure!(field_data.len() == 1, "Byte::InvalidDataSize");
                    Ok((FilledAbi::Byte(name, field_data.to_vec()), 1))
                },
                Abi::Bool(name) => {
                    ensure!(field_data.len() == 1, "Bool::InvalidDataSize");
                    Ok((FilledAbi::Bool(name, field_data.to_vec()), 1))
                },
                Abi::Vec(name, field_descriptor) => {
                    if in_codec == Codec::Rlp {
                        return Err(
                            "Abi::Vec::NotImplemented for RLP - undeterministc size of vec items with RLP makes it diff to predict the size".into(),
                        );
                    }

                    let recoded_vector_data = ensure_vector_and_trim_prefix(field_data, &in_codec)?;
                    println!("Abi::Vec - recoded_vector_data: {recoded_vector_data:?}");

                    let mut vec = Vec::new();
                    let mut offset = 0;

                    let max_size_of_current_field = field_descriptor.get_size();
                    println!("Abi::Vec - max_size_of_current_field: {max_size_of_current_field:?}");

                    while offset + max_size_of_current_field <= recoded_vector_data.len() {
                        let (field, size) = recursive_fill_abi(
                            *field_descriptor.clone(),
                            &recoded_vector_data[offset..offset + max_size_of_current_field],
                            in_codec.clone(),
                        )?;
                        vec.push(field);
                        offset += size;
                    }

                    Ok((
                        FilledAbi::Vec(name, Box::new(vec), 0u8),
                        recoded_vector_data.len(),
                    ))
                },
                Abi::Tuple(name, (field1, field2)) => {
                    let (field1, size1) =
                        recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                    let (field2, size2) =
                        recursive_fill_abi(*field2, &field_data[size1..], in_codec)?;

                    Ok((
                        FilledAbi::Tuple(name, (Box::new(field1), Box::new(field2))),
                        size1 + size2,
                    ))
                },
                // Abi::Enum(name, (field1, field2)) => {
                //     let (field1, size1) = recursive_fill_abi(*field1, field_data)?;
                //     let (field2, size2) = recursive_fill_abi(*field2, &field_data[size1..])?;
                //
                //     Ok((
                //         FilledAbi::Enum(name, (Box::new(field1), Box::new(field2))),
                //         size1 + size2,
                //     ))
                // },
            }
        }

        match recursive_fill_abi(abi, data.as_slice(), in_codec) {
            Ok((filled_abi, _)) => Ok(filled_abi),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test_fill_abi {
    use super::*;
    use crate::mini_mock::MiniRuntime;
    use hex_literal::hex;
    use rlp_derive::{RlpDecodable, RlpEncodable};
    use sp_core::{crypto::AccountId32, ByteArray};

    #[test]
    fn recodes_account32_from_scale_to_rlp() {
        let abi = Abi::Account32(None);
        let val: AccountId32 =
            hex!("0909090909090909090909090909090906060606060606060606060606060606").into();

        let scale_encoded = val.encode();

        let filled_abi = FilledAbi::try_fill_abi(abi, scale_encoded.clone(), Codec::Scale).unwrap();

        assert_eq!(filled_abi, FilledAbi::Account32(None, scale_encoded));

        let rlp_encoded = filled_abi.recode_as(&Codec::Scale, &Codec::Rlp).unwrap();

        assert_eq!(rlp_encoded, rlp::encode(&val.to_raw_vec()).to_vec());
    }

    #[test]
    fn recodes_account32_from_rlp_to_scale() {
        let abi = Abi::Account32(None);
        let val: AccountId32 =
            hex!("0909090909090909090909090909090906060606060606060606060606060606").into();

        let rlp_encoded = rlp::encode(&val.to_raw_vec()).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, rlp_encoded.clone(), Codec::Rlp).unwrap();

        assert_eq!(filled_abi, FilledAbi::Account32(None, rlp_encoded));

        let scale_encoded = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale).unwrap();

        assert_eq!(scale_encoded, val.encode());
    }

    #[test]
    fn recodes_value32_from_rlp_to_scale() {
        let abi = Abi::Value32(None);
        let val: u32 = 123;

        let rlp_encoded = rlp::encode(&val).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, rlp_encoded.clone(), Codec::Rlp).unwrap();

        assert_eq!(filled_abi, FilledAbi::Value32(None, rlp_encoded));

        let scale_encoded = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale).unwrap();

        assert_eq!(scale_encoded, val.encode());
    }

    #[test]
    fn fills_abi_for_bool_in_rlp() {
        let abi = Abi::Bool(None);
        let val: bool = true;

        let bool_rlp_encoded = rlp::encode(&val).to_vec();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, bool_rlp_encoded.clone(), Codec::Rlp).unwrap();

        assert_eq!(filled_abi, FilledAbi::Bool(None, bool_rlp_encoded));
    }

    #[test]
    fn fills_abi_for_bool_in_scale() {
        let abi = Abi::Bool(None);
        let val: bool = true;

        let bool_scale_encoded = val.encode();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, bool_scale_encoded.clone(), Codec::Scale).unwrap();

        assert_eq!(filled_abi, FilledAbi::Bool(None, bool_scale_encoded));
    }

    #[test]
    fn fills_abi_for_value32_in_scale() {
        let abi = Abi::Value32(Some(b"amount".to_vec()));
        let amount: u32 = 100_000;

        let amount_scale_encoded = amount.encode();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, amount_scale_encoded.clone(), Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Value32(Some(b"amount".to_vec()), amount_scale_encoded)
        );
    }

    #[test]
    fn fills_abi_for_value64_above_255_encoded_in_rlp() {
        let abi = Abi::Value64(Some(b"amount".to_vec()));
        let amount: u64 = 100_000;

        let amount_rlp_encoded = rlp::encode(&amount).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, amount_rlp_encoded, Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Value64(Some(b"amount".to_vec()), vec![131u8, 1u8, 134u8, 160u8])
        );
    }

    #[test]
    fn fills_abi_for_value64_below_255_encoded_in_rlp() {
        let abi = Abi::Value64(Some(b"amount".to_vec()));
        let amount: u64 = 100;

        let amount_rlp_encoded = rlp::encode(&amount).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, amount_rlp_encoded, Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Value64(Some(b"amount".to_vec()), vec![100u8])
        );
    }

    #[test]
    fn fills_abi_for_value64_encoded_in_scale() {
        let abi = Abi::Value64(Some(b"amount".to_vec()));
        let amount: u64 = 100;

        let amount_scale_encoded = amount.encode();

        let filled_abi = FilledAbi::try_fill_abi(abi, amount_scale_encoded, Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Value64(Some(b"amount".to_vec()), hex!("6400000000000000").to_vec())
        );
    }

    #[test]
    fn fills_abi_for_account32_encoded_in_rlp() {
        let abi = Abi::Account32(Some(b"address".to_vec()));
        let address_32b: Vec<u8> = AccountId32::new([0x09; 32]).to_raw_vec();

        let address_32b_rlp_encoded = rlp::encode(&address_32b).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, address_32b_rlp_encoded, Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Account32(
                Some(b"address".to_vec()),
                hex!("A00909090909090909090909090909090909090909090909090909090909090909").to_vec()
            )
        )
    }

    #[test]
    fn fills_abi_for_vector_of_2x_account32_encoded_in_scale() {
        let abi = Abi::Vec(Some(b"addresses".to_vec()), Box::new(Abi::Account32(None)));
        let address_32b_a: AccountId32 = AccountId32::new([0x09; 32]);
        let address_32b_b: AccountId32 = AccountId32::new([0x06; 32]);

        let addresses = vec![address_32b_a, address_32b_b];

        let addresses_scale_encoded = addresses.encode();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, addresses_scale_encoded, Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Vec(
                Some(b"addresses".to_vec()),
                Box::new(vec![
                    FilledAbi::Account32(
                        None,
                        hex!("0909090909090909090909090909090909090909090909090909090909090909")
                            .to_vec()
                    ),
                    FilledAbi::Account32(
                        None,
                        hex!("0606060606060606060606060606060606060606060606060606060606060606")
                            .to_vec()
                    )
                ]),
                0u8
            )
        )
    }

    #[test]
    fn fills_abi_for_account32_encoded_in_scale() {
        let abi = Abi::Account32(Some(b"address".to_vec()));
        let address_32b_scale_encoded: Vec<u8> = AccountId32::new([0x09; 32]).encode();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, address_32b_scale_encoded, Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Account32(
                Some(b"address".to_vec()),
                hex!("0909090909090909090909090909090909090909090909090909090909090909").to_vec()
            )
        )
    }

    #[test]
    fn fills_abi_for_named_struct_and_rlp_encoded_ingress_balance_transfer_event() {
        let abi = Abi::Struct(
            Some(b"IngressBalanceTransferEvent".to_vec()),
            vec![
                Box::new(Abi::Account32(Some(b"from".to_vec()))),
                Box::new(Abi::Account32(Some(b"to".to_vec()))),
                Box::new(Abi::Value128(Some(b"value".to_vec()))),
            ],
        );

        #[derive(Debug, PartialEq, Encode, Decode, RlpEncodable, RlpDecodable)]
        struct Transfer {
            from: Vec<u8>,
            to: Vec<u8>,
            value: u128,
        }

        let transfer = Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").to_vec(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").to_vec(),
            value: 1u128,
        };

        let rlp_encoded_transfer_event = rlp::encode(&transfer);

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_transfer_event.to_vec(), Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Struct(
                Some(b"IngressBalanceTransferEvent".to_vec()),
                vec![
                    Box::new(FilledAbi::Account32(
                        Some(b"from".to_vec()),
                        hex!("E00909090909090909090909090909090909090909090909090909090909090909")
                            .to_vec()
                    )),
                    Box::new(FilledAbi::Account32(
                        Some(b"to".to_vec()),
                        hex!("E00606060606060606060606060606060606060606060606060606060606060606")
                            .to_vec()
                    )),
                    Box::new(FilledAbi::Value128(
                        Some(b"value".to_vec()),
                        hex!("01").to_vec()
                    )),
                ],
                248u8
            )
        );
    }

    #[derive(Debug, PartialEq, Encode, Decode, RlpEncodable, RlpDecodable)]
    struct Donation {
        donor: Vec<u8>,
        amount: u128,
        donation_time: Option<u64>,
        donation_version: u128,
    }

    #[derive(Debug, PartialEq, Encode, Decode, RlpEncodable, RlpDecodable)]
    struct Campaign {
        donations: Donation,
    }

    fn get_test_campaign() -> Campaign {
        Campaign {
            donations: Donation {
                donor: hex!("0606060606060606060606060606060606060606060606060606060606060606")
                    .to_vec(),
                amount: 1u128,
                donation_time: Some(2u64),
                donation_version: 1u128,
            },
        }
    }

    fn get_campaign_abi() -> Abi {
        Abi::Struct(
            Some(b"Campaign".to_vec()),
            vec![Box::new(Abi::Struct(
                Some(b"donations".to_vec()),
                vec![
                    Box::new(Abi::Account32(Some(b"donor".to_vec()))),
                    Box::new(Abi::Value128(Some(b"amount".to_vec()))),
                    Box::new(Abi::Option(
                        Some(b"donation_time".to_vec()),
                        Box::new(Abi::Value64(Some(b"donation_time".to_vec()))),
                    )),
                    Box::new(Abi::Value128(Some(b"donation_version".to_vec()))),
                ],
            ))],
        )
    }

    #[test]
    #[ignore]
    // ffs scale doesn't encode nested structs so encoded campaign is just the encoded donation :(
    fn fills_abi_named_campaign_nested_struct_with_optional_fields_encoded_in_scale() {
        let campaign = get_test_campaign();

        let scale_encoded_campaign: Vec<u8> = campaign.encode();

        println!("scale_encoded_campaign: {scale_encoded_campaign:?}");

        let abi = get_campaign_abi();

        let donation = Donation {
            donor: hex!("0606060606060606060606060606060606060606060606060606060606060606")
                .to_vec(),
            amount: 1u128,
            donation_time: Some(2u64),
            donation_version: 1u128,
        };

        let scale_encoded_donation: Vec<u8> = donation.encode();

        // :< unfortunately below is true
        assert_eq!(scale_encoded_campaign, scale_encoded_donation);

        let filled_abi =
            FilledAbi::try_fill_abi(abi, scale_encoded_campaign.to_vec(), Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Struct(
                Some(b"Campaign".to_vec()),
                vec![Box::new(FilledAbi::Struct(
                    Some(b"donations".to_vec()),
                    vec![
                        Box::new(FilledAbi::Account32(
                            Some(b"donor".to_vec()),
                            hex!(
                                "0606060606060606060606060606060606060606060606060606060606060606"
                            )
                            .to_vec()
                        )),
                        Box::new(FilledAbi::Value128(
                            Some(b"amount".to_vec()),
                            1u128.encode()
                        )),
                        Box::new(FilledAbi::Option(
                            Some(b"donation_time".to_vec()),
                            Box::new(FilledAbi::Value64(
                                Some(b"donation_time".to_vec()),
                                2u64.encode()
                            ))
                        )),
                        Box::new(FilledAbi::Value128(
                            Some(b"donation_version".to_vec()),
                            1u128.encode()
                        )),
                    ],
                    0u8
                ),)],
                0u8
            )
        )
    }
    #[test]
    fn fills_abi_named_campaign_nested_struct_with_optional_fields_encoded_in_rlp() {
        let campaign = get_test_campaign();

        let rlp_encoded_campaign = rlp::encode(&campaign);

        let abi = get_campaign_abi();

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_campaign.to_vec(), Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Struct(
                Some(b"Campaign".to_vec()),
                vec![Box::new(FilledAbi::Struct(
                    Some(b"donations".to_vec()),
                    vec![
                        Box::new(FilledAbi::Account32(
                            Some(b"donor".to_vec()),
                            hex!(
                                "E00606060606060606060606060606060606060606060606060606060606060606"
                            )
                            .to_vec()
                        )),
                        Box::new(FilledAbi::Value128(
                            Some(b"amount".to_vec()),
                            hex!("01").to_vec()
                        )),
                        Box::new(FilledAbi::Option(
                            Some(b"donation_time".to_vec()),
                            Box::new(FilledAbi::Value64(
                                Some(b"donation_time".to_vec()),
                                hex!("02").to_vec()
                            ))
                        )),
                        Box::new(FilledAbi::Value128(
                            Some(b"donation_version".to_vec()),
                            hex!("01").to_vec()
                        )),
                    ],
                    229u8 // saved prefix memo
                ))],
                230u8 // saved prefix memo
            )
        )
    }

    #[test]
    fn fills_abi_for_named_struct_and_scale_encoded_ingress_balance_transfer_event() {
        let abi = Abi::Struct(
            Some(b"IngressBalanceTransferEvent".to_vec()),
            vec![
                Box::new(Abi::Account32(Some(b"from".to_vec()))),
                Box::new(Abi::Account32(Some(b"to".to_vec()))),
                Box::new(Abi::Value128(Some(b"value".to_vec()))),
            ],
        );

        let scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            amount: 1,
        }
        .encode();

        println!("scale_encoded_transfer_event: {scale_encoded_transfer_event:?}");

        let res = FilledAbi::try_fill_abi(abi, scale_encoded_transfer_event, Codec::Scale);

        assert_eq!(
            res.unwrap(),
            FilledAbi::Struct(
                Some(b"IngressBalanceTransferEvent".to_vec()),
                vec![
                    Box::new(FilledAbi::Account32(
                        Some(b"from".to_vec()),
                        hex!("0909090909090909090909090909090909090909090909090909090909090909")
                            .to_vec()
                    )),
                    Box::new(FilledAbi::Account32(
                        Some(b"to".to_vec()),
                        hex!("0606060606060606060606060606060606060606060606060606060606060606")
                            .to_vec()
                    )),
                    Box::new(FilledAbi::Value128(
                        Some(b"value".to_vec()),
                        1u128.to_le_bytes().to_vec()
                    )),
                ],
                2u8 // saved prefix memo - index of Balance::Transfer event
            )
        );
    }
}
