use crate::{recode::Codec, to_abi::Abi, types::*};
use bytes::{Buf, Bytes};
use codec::{Decode, Encode};
use frame_support::ensure;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

pub use crate::recode_rlp::EthIngressEventLog;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum FilledAbi {
    Struct(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Enum(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Event(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Log(Option<Name>, Vec<Box<FilledAbi>>, u8),
    Option(Option<Name>, Box<FilledAbi>),
    Bytes(Option<Name>, Data),
    Bytes4(Option<Name>, Data),
    Account20(Option<Name>, Data),
    Account32(Option<Name>, Data),
    H256(Option<Name>, Data),
    Value256(Option<Name>, Data),
    Value128(Option<Name>, Data),
    Value64(Option<Name>, Data),
    Value32(Option<Name>, Data),
    Byte(Option<Name>, Data),
    Codec(Option<Name>, Data),
    Bool(Option<Name>, Data),
    Vec(Option<Name>, Box<Vec<FilledAbi>>, u8),
    Uniple(Option<Name>, Box<FilledAbi>),
    Tuple(Option<Name>, (Box<FilledAbi>, Box<FilledAbi>)),
    Triple(
        Option<Name>,
        (Box<FilledAbi>, Box<FilledAbi>, Box<FilledAbi>),
    ),
    Quadruple(
        Option<Name>,
        (
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
        ),
    ),
    Quintuple(
        Option<Name>,
        (
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
        ),
    ),
    Sextuple(
        Option<Name>,
        (
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
            Box<FilledAbi>,
        ),
    ),
}

pub fn matches_name(field_name: Option<&Name>, by_name: &Name) -> bool {
    match field_name {
        Some(field_name) => {
            match field_name.last() {
                Some(last_char) => {
                    if (*last_char == b"+"[0] || *last_char == b"-"[0])
                        && &field_name[..field_name.len() - 1].to_vec() == by_name
                    {
                        return true
                    }
                },
                None => return false,
            }
            field_name == by_name
        },
        None => false,
    }
}

impl FilledAbi {
    pub fn get_prefix_memo(&self) -> Option<u8> {
        match self {
            FilledAbi::Struct(_name, _, prefix_memo) => Some(*prefix_memo),
            FilledAbi::Enum(_name, _, prefix_memo) => Some(*prefix_memo),
            FilledAbi::Event(_name, _, prefix_memo) => Some(*prefix_memo),
            FilledAbi::Log(_name, _, prefix_memo) => Some(*prefix_memo),
            FilledAbi::Option(_name, _) => None,
            FilledAbi::Codec(_name, _) => None,
            FilledAbi::Bytes(_name, _) => None,
            FilledAbi::Bytes4(_name, _) => None,
            FilledAbi::Account20(_name, _) => None,
            FilledAbi::Account32(_name, _) => None,
            FilledAbi::H256(_name, _) => None,
            FilledAbi::Value256(_name, _) => None,
            FilledAbi::Value128(_name, _) => None,
            FilledAbi::Value64(_name, _) => None,
            FilledAbi::Value32(_name, _) => None,
            FilledAbi::Byte(_name, _) => None,
            FilledAbi::Bool(_name, _) => None,
            FilledAbi::Vec(_name, _, _) => None,
            FilledAbi::Tuple(_name, _) => None,
            FilledAbi::Uniple(_name, _) => None,
            FilledAbi::Triple(_name, _) => None,
            FilledAbi::Quadruple(_name, _) => None,
            FilledAbi::Quintuple(_name, _) => None,
            FilledAbi::Sextuple(_name, _) => None,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            FilledAbi::Struct(_name, _, _) => "Struct",
            FilledAbi::Enum(_name, _, _) => "Enum",
            FilledAbi::Log(_name, _, _) => "Log",
            FilledAbi::Option(_name, _) => "Option",
            FilledAbi::Bytes(_name, _) => "Bytes",
            FilledAbi::Account20(_name, _) => "Account20",
            FilledAbi::Account32(_name, _) => "Account32",
            FilledAbi::H256(_name, _) => "H256",
            FilledAbi::Value256(_name, _) => "Value256",
            FilledAbi::Value128(_name, _) => "Value128",
            FilledAbi::Value64(_name, _) => "Value64",
            FilledAbi::Value32(_name, _) => "Value32",
            FilledAbi::Byte(_name, _) => "Byte",
            FilledAbi::Bool(_name, _) => "Bool",
            FilledAbi::Vec(_name, _, _) => "Vec",
            FilledAbi::Tuple(_name, _) => "Tuple",
            FilledAbi::Event(_, _, _) => "Event",
            FilledAbi::Bytes4(_, _) => "Bytes4",
            FilledAbi::Codec(_, _) => "Codec",
            FilledAbi::Uniple(_, _) => "Uniple",
            FilledAbi::Triple(_, _) => "Triple",
            FilledAbi::Quadruple(_, _) => "Quadruple",
            FilledAbi::Quintuple(_, _) => "Quintuple",
            FilledAbi::Sextuple(_, _) => "Sextuple",
        }
    }

    pub fn get_data(&self) -> Data {
        match self {
            FilledAbi::Struct(_, fields, prefix_memo)
            | FilledAbi::Enum(_, fields, prefix_memo)
            | FilledAbi::Event(_, fields, prefix_memo)
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
            FilledAbi::Bytes4(_, data) => data.clone(),
            FilledAbi::Account20(_, data) => data.clone(),
            FilledAbi::Account32(_, data) => data.clone(),
            FilledAbi::H256(_, data) => data.clone(),
            FilledAbi::Value256(_, data) => data.clone(),
            FilledAbi::Value128(_, data) => data.clone(),
            FilledAbi::Value64(_, data) => data.clone(),
            FilledAbi::Value32(_, data) => data.clone(),
            FilledAbi::Byte(_, data) => data.clone(),
            FilledAbi::Codec(_, data) => data.clone(),
            FilledAbi::Bool(_, data) => data.clone(),
            FilledAbi::Vec(_, fields, prefix_memo) => {
                let mut data = vec![*prefix_memo];
                for field in *fields.clone() {
                    data.extend_from_slice(field.get_data().as_slice());
                }
                data.clone()
            },
            FilledAbi::Uniple(_, field1) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.clone()
            },
            FilledAbi::Tuple(_, (field1, field2)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.clone()
            },
            FilledAbi::Triple(_, (field1, field2, field3)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.extend_from_slice(field3.get_data().as_slice());
                data.clone()
            },
            FilledAbi::Quadruple(_, (field1, field2, field3, field4)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.extend_from_slice(field3.get_data().as_slice());
                data.extend_from_slice(field4.get_data().as_slice());
                data.clone()
            },
            FilledAbi::Quintuple(_, (field1, field2, field3, field4, field5)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.extend_from_slice(field3.get_data().as_slice());
                data.extend_from_slice(field4.get_data().as_slice());
                data.extend_from_slice(field5.get_data().as_slice());
                data.clone()
            },
            FilledAbi::Sextuple(_, (field1, field2, field3, field4, field5, field6)) => {
                let mut data = vec![];
                data.extend_from_slice(field1.get_data().as_slice());
                data.extend_from_slice(field2.get_data().as_slice());
                data.extend_from_slice(field3.get_data().as_slice());
                data.extend_from_slice(field4.get_data().as_slice());
                data.extend_from_slice(field5.get_data().as_slice());
                data.extend_from_slice(field6.get_data().as_slice());
                data.clone()
            },
        }
    }

    pub fn get_name(&self) -> Option<Name> {
        match self {
            FilledAbi::Struct(name, _, _)
            | FilledAbi::Enum(name, _, _)
            | FilledAbi::Event(name, _, _)
            | FilledAbi::Log(name, _, _)
            | FilledAbi::Option(name, _)
            | FilledAbi::Bytes(name, _)
            | FilledAbi::Bytes4(name, _)
            | FilledAbi::Account20(name, _)
            | FilledAbi::Account32(name, _)
            | FilledAbi::H256(name, _)
            | FilledAbi::Value256(name, _)
            | FilledAbi::Value128(name, _)
            | FilledAbi::Value64(name, _)
            | FilledAbi::Value32(name, _)
            | FilledAbi::Codec(name, _)
            | FilledAbi::Byte(name, _)
            | FilledAbi::Bool(name, _)
            | FilledAbi::Vec(name, _, _)
            | FilledAbi::Uniple(name, _)
            | FilledAbi::Triple(name, _)
            | FilledAbi::Quadruple(name, _)
            | FilledAbi::Quintuple(name, _)
            | FilledAbi::Sextuple(name, _)
            | FilledAbi::Tuple(name, _) => name.clone(),
        }
    }

    pub fn get_by_name(&self, by_name: &Name) -> Option<FilledAbi> {
        fn recursive_get_by_name(abi: &FilledAbi, by_name: &Name) -> Option<FilledAbi> {
            match abi {
                FilledAbi::Struct(name, fields, _)
                | FilledAbi::Event(name, fields, _)
                | FilledAbi::Enum(name, fields, _)
                | FilledAbi::Log(name, fields, _) => {
                    if matches_name(name.as_ref(), by_name) {
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
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.clone())
                    }

                    recursive_get_by_name(field, by_name)
                },
                | FilledAbi::Bytes(name, _data)
                | FilledAbi::Bytes4(name, _data)
                | FilledAbi::Codec(name, _data)
                | FilledAbi::Account20(name, _data)
                | FilledAbi::Account32(name, _data)
                | FilledAbi::H256(name, _data)
                | FilledAbi::Value256(name, _data)
                | FilledAbi::Value128(name, _data)
                | FilledAbi::Value64(name, _data)
                | FilledAbi::Value32(name, _data)
                | FilledAbi::Bool(name, _data)
                | FilledAbi::Byte(name, _data) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.clone())
                    }

                    None
                },
                FilledAbi::Vec(_name, field, _) => {
                    let vec_abi_content = match field.get(0) {
                        Some(vec_abi_content) => vec_abi_content,
                        None => return None,
                    };
                    recursive_get_by_name(vec_abi_content, by_name)
                },
                FilledAbi::Tuple(_name, (field1, field2)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field2, by_name)
                },
                FilledAbi::Uniple(_name, field) => recursive_get_by_name(field, by_name),
                FilledAbi::Triple(_name, (field1, field2, field3)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field2, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field3, by_name)
                },
                FilledAbi::Quadruple(_name, (field1, field2, field3, field4)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field3, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field4, by_name)
                },
                FilledAbi::Quintuple(_name, (field1, field2, field3, field4, field5)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field3, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field4, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field5, by_name)
                },
                FilledAbi::Sextuple(_name, (field1, field2, field3, field4, field5, field6)) => {
                    if let Some(data) = recursive_get_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field3, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field4, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_by_name(field5, by_name) {
                        return Some(data)
                    }

                    recursive_get_by_name(field6, by_name)
                },
            }
        }

        recursive_get_by_name(self, by_name)
    }

    pub fn get_data_by_name(&self, by_name: &Name) -> Option<Data> {
        fn recursive_get_data_by_name(abi: &FilledAbi, by_name: &Name) -> Option<Data> {
            match abi {
                FilledAbi::Struct(name, fields, _)
                | FilledAbi::Event(name, fields, _)
                | FilledAbi::Enum(name, fields, _)
                | FilledAbi::Log(name, fields, _) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    for field in fields {
                        if let Some(data) = recursive_get_data_by_name(field, by_name) {
                            return Some(data)
                        }
                    }

                    None
                },
                FilledAbi::Option(name, field) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    recursive_get_data_by_name(field, by_name)
                },
                | FilledAbi::Bytes(name, data)
                | FilledAbi::Codec(name, data)
                | FilledAbi::Bytes4(name, data)
                | FilledAbi::Account20(name, data)
                | FilledAbi::Account32(name, data)
                | FilledAbi::H256(name, data)
                | FilledAbi::Value256(name, data)
                | FilledAbi::Value128(name, data)
                | FilledAbi::Value64(name, data)
                | FilledAbi::Value32(name, data)
                | FilledAbi::Byte(name, data) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(data.clone())
                    }

                    None
                },
                FilledAbi::Bool(name, data) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(data.clone())
                    }

                    None
                },
                FilledAbi::Vec(_name, field, _) => {
                    let vec_abi_content = match field.get(0) {
                        Some(vec_abi_content) => vec_abi_content,
                        None => return None,
                    };
                    recursive_get_data_by_name(vec_abi_content, by_name)
                },
                FilledAbi::Tuple(name, (field1, field2)) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field2, by_name)
                },
                FilledAbi::Uniple(name, field1) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    recursive_get_data_by_name(field1, by_name)
                },
                FilledAbi::Triple(name, (field1, field2, field3)) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field2, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field3, by_name)
                },
                FilledAbi::Quadruple(name, (field1, field2, field3, field4)) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field3, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field4, by_name)
                },
                FilledAbi::Quintuple(name, (field1, field2, field3, field4, field5)) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field3, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field4, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field5, by_name)
                },
                FilledAbi::Sextuple(name, (field1, field2, field3, field4, field5, field6)) => {
                    if matches_name(name.as_ref(), by_name) {
                        return Some(abi.get_data())
                    }

                    if let Some(data) = recursive_get_data_by_name(field1, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field2, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field3, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field4, by_name) {
                        return Some(data)
                    }

                    if let Some(data) = recursive_get_data_by_name(field5, by_name) {
                        return Some(data)
                    }

                    recursive_get_data_by_name(field6, by_name)
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
            let mut data_buf = Bytes::copy_from_slice(data);
            data_buf.advance(1);
            ensure!(
                !data_buf.is_empty(),
                "recode_as_vector::Scale::InvalidDataSize"
            );
            Ok(data_buf.to_vec())
        },
        Codec::Rlp => {
            let rlp = rlp::Rlp::new(data);
            let rlp_encoded = rlp.as_raw();
            let mut rlp_buf = Bytes::copy_from_slice(rlp_encoded);
            rlp_buf.advance(1);
            ensure!(
                !rlp_buf.is_empty(),
                "recode_as_vector::Rlp::InvalidDataSize"
            );
            Ok(rlp_buf.to_vec())
        },
    }
}

impl FilledAbi {
    pub fn recursive_fill_abi(
        abi: Abi,
        field_data: &[u8],
        in_codec: Codec,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        match abi {
            Abi::Log(name, fields_descriptors) => crate::recode::CrossRecode::event_to_filled(
                in_codec,
                field_data,
                name,
                fields_descriptors.into_iter(),
            ),
            Abi::Struct(name, fields_descriptors) | Abi::Event(name, fields_descriptors) => {
                let mut fields = Vec::new();

                let (mut chopped_field_data_iter, memo_prefix) =
                    crate::recode::CrossRecode::chop_encoded(
                        in_codec.clone(),
                        field_data,
                        fields_descriptors.clone().into_iter(),
                    )?;

                let mut total_struct_size = 0usize;

                for field_descriptor in fields_descriptors {
                    let (field, size) = Self::recursive_fill_abi(
                        *field_descriptor,
                        chopped_field_data_iter
                            .next()
                            .ok_or::<DispatchError>("Abi::Struct - Not enough data".into())?
                            .as_slice(),
                        in_codec.clone(),
                    )?;
                    total_struct_size += size;
                    fields.push(Box::new(field));
                }

                Ok((
                    FilledAbi::Struct(name, fields, memo_prefix),
                    total_struct_size,
                ))
            },
            Abi::Enum(_name, field_descriptor) => {
                let mut data_buf = Bytes::copy_from_slice(field_data);
                let first_byte = data_buf
                    .first()
                    .ok_or::<DispatchError>("FilledAbi::Enum - Not enough data".into())?;

                let selected_field_descriptor =
                    field_descriptor
                        .get(*first_byte as usize)
                        .ok_or::<DispatchError>("FilledAbi::Enum - Invalid data".into())?;

                data_buf.advance(1);
                Self::recursive_fill_abi(*selected_field_descriptor.clone(), &data_buf, in_codec)
            },
            Abi::Option(name, field_descriptor) => {
                let mut data_buf = Bytes::copy_from_slice(field_data);
                data_buf.advance(1);
                let no_option_prefix_data = data_buf;
                let (field, size) =
                    Self::recursive_fill_abi(*field_descriptor, &no_option_prefix_data, in_codec)?;
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
                // strip the prefix memo if present
                let account_20_bytes = if field_data.len() == 21 {
                    &field_data[1..]
                } else {
                    field_data
                };
                let account_20: [u8; 20] = account_20_bytes
                    .try_into()
                    .map_err(|_| "Account20::InvalidDataSize: expected 20 bytes")?;

                Ok((
                    FilledAbi::Account20(name, account_20.to_vec()),
                    field_data.len(),
                ))
            },
            Abi::Account32(name) | Abi::H256(name) => {
                // strip the prefix memo if present
                let data_maybe_stripped_prefix = if field_data.len() == 33 {
                    &field_data[1..]
                } else {
                    field_data
                };

                let data_32b: [u8; 32] = data_maybe_stripped_prefix
                    .try_into()
                    .map_err(|_| "Account20::InvalidDataSize: expected 20 bytes")?;

                Ok((
                    FilledAbi::Account32(name, data_32b.to_vec()),
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
            Abi::Bytes4(name) => {
                let bytes4: [u8; 4] = field_data[0..4]
                    .try_into()
                    .map_err(|_| "Bytes4::InvalidDataSize: expected 4 bytes")?;
                Ok((FilledAbi::Bytes4(name, bytes4.to_vec()), 4))
            },
            Abi::Byte(name) => {
                let byte = field_data
                    .first()
                    .ok_or::<DispatchError>("Byte::InvalidDataSize".into())?;
                Ok((FilledAbi::Byte(name, vec![*byte]), 1))
            },
            Abi::Bool(name) => {
                let byte = field_data
                    .first()
                    .ok_or::<DispatchError>("Bool::InvalidDataSize".into())?;
                Ok((FilledAbi::Bool(name, vec![*byte]), 1))
            },
            Abi::Codec(name) => {
                let byte = field_data
                    .first()
                    .ok_or::<DispatchError>("Codec::InvalidDataSize".into())?;
                Ok((FilledAbi::Codec(name, vec![*byte]), 1))
            },
            Abi::Vec(name, field_descriptor) => {
                if in_codec == Codec::Rlp {
                    return Err(
                        "Abi::Vec::NotImplemented for RLP - undeterministc size of vec items with RLP makes it diff to predict the size".into(),
                    );
                }

                let recoded_vector_data = ensure_vector_and_trim_prefix(field_data, &in_codec)?;

                let mut vec = Vec::new();
                let mut offset = 0;
                let max_size_of_current_field = field_descriptor.get_size();

                while offset + max_size_of_current_field <= recoded_vector_data.len() {
                    let (field, size) = Self::recursive_fill_abi(
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
            Abi::Uniple(name, field1) => {
                let (field1, size1) = Self::recursive_fill_abi(*field1, field_data, in_codec)?;

                Ok((FilledAbi::Uniple(name, Box::new(field1)), size1))
            },
            Abi::Tuple(name, (field1, field2)) => {
                let (field1, size1) =
                    Self::recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                let (field2, size2) =
                    Self::recursive_fill_abi(*field2, &field_data[size1..], in_codec)?;

                Ok((
                    FilledAbi::Tuple(name, (Box::new(field1), Box::new(field2))),
                    size1 + size2,
                ))
            },
            Abi::Triple(name, (field1, field2, field3)) => {
                let (field1, size1) =
                    Self::recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                let (field2, size2) =
                    Self::recursive_fill_abi(*field2, &field_data[size1..], in_codec.clone())?;

                let (field3, size3) =
                    Self::recursive_fill_abi(*field3, &field_data[size1 + size2..], in_codec)?;

                Ok((
                    FilledAbi::Triple(name, (Box::new(field1), Box::new(field2), Box::new(field3))),
                    size1 + size2 + size3,
                ))
            },
            Abi::Quadruple(name, (field1, field2, field3, field4)) => {
                let (field1, size1) =
                    Self::recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                let (field2, size2) =
                    Self::recursive_fill_abi(*field2, &field_data[size1..], in_codec.clone())?;

                let (field3, size3) = Self::recursive_fill_abi(
                    *field3,
                    &field_data[size1 + size2..],
                    in_codec.clone(),
                )?;

                let (field4, size4) = Self::recursive_fill_abi(
                    *field4,
                    &field_data[size1 + size2 + size3..],
                    in_codec,
                )?;

                Ok((
                    FilledAbi::Quadruple(
                        name,
                        (
                            Box::new(field1),
                            Box::new(field2),
                            Box::new(field3),
                            Box::new(field4),
                        ),
                    ),
                    size1 + size2 + size3 + size4,
                ))
            },
            Abi::Quintuple(name, (field1, field2, field3, field4, field5)) => {
                let (field1, size1) =
                    Self::recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                let (field2, size2) =
                    Self::recursive_fill_abi(*field2, &field_data[size1..], in_codec.clone())?;

                let (field3, size3) = Self::recursive_fill_abi(
                    *field3,
                    &field_data[size1 + size2..],
                    in_codec.clone(),
                )?;

                let (field4, size4) = Self::recursive_fill_abi(
                    *field4,
                    &field_data[size1 + size2 + size3..],
                    in_codec.clone(),
                )?;

                let (field5, size5) = Self::recursive_fill_abi(
                    *field5,
                    &field_data[size1 + size2 + size3 + size4..],
                    in_codec,
                )?;

                Ok((
                    FilledAbi::Quintuple(
                        name,
                        (
                            Box::new(field1),
                            Box::new(field2),
                            Box::new(field3),
                            Box::new(field4),
                            Box::new(field5),
                        ),
                    ),
                    size1 + size2 + size3 + size4 + size5,
                ))
            },
            Abi::Sextuple(name, (field1, field2, field3, field4, field5, field6)) => {
                let (field1, size1) =
                    Self::recursive_fill_abi(*field1, field_data, in_codec.clone())?;

                let (field2, size2) =
                    Self::recursive_fill_abi(*field2, &field_data[size1..], in_codec.clone())?;

                let (field3, size3) = Self::recursive_fill_abi(
                    *field3,
                    &field_data[size1 + size2..],
                    in_codec.clone(),
                )?;

                let (field4, size4) = Self::recursive_fill_abi(
                    *field4,
                    &field_data[size1 + size2 + size3..],
                    in_codec.clone(),
                )?;

                let (field5, size5) = Self::recursive_fill_abi(
                    *field5,
                    &field_data[size1 + size2 + size3 + size4..],
                    in_codec.clone(),
                )?;

                let (field6, size6) = Self::recursive_fill_abi(
                    *field6,
                    &field_data[size1 + size2 + size3 + size4 + size5..],
                    in_codec,
                )?;

                Ok((
                    FilledAbi::Sextuple(
                        name,
                        (
                            Box::new(field1),
                            Box::new(field2),
                            Box::new(field3),
                            Box::new(field4),
                            Box::new(field5),
                            Box::new(field6),
                        ),
                    ),
                    size1 + size2 + size3 + size4 + size5 + size6,
                ))
            },
        }
    }

    // Fills the ABI with raw data, only assuming the type size of input codec
    pub fn try_fill_abi(abi: Abi, data: Data, in_codec: Codec) -> Result<FilledAbi, DispatchError> {
        match Self::recursive_fill_abi(abi, data.as_slice(), in_codec) {
            Ok((filled_abi, _)) => Ok(filled_abi),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test_fill_abi {
    use super::*;
    use crate::mini_mock::MiniRuntime;
    use frame_support::assert_ok;
    use hex_literal::hex;
    use std::vec;

    use rlp_derive::{RlpDecodable, RlpEncodable};
    use sp_core::{crypto::AccountId32, ByteArray};

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
                hex!("0909090909090909090909090909090909090909090909090909090909090909").to_vec()
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
    fn fills_abi_enum_with_3_u8_options_no_args() {
        let abi = Abi::Enum(
            Some(b"address".to_vec()),
            vec![
                Box::new(Abi::Byte(Some(b"option_a".to_vec()))),
                Box::new(Abi::Byte(Some(b"option_b".to_vec()))),
                Box::new(Abi::Byte(Some(b"option_c".to_vec()))),
            ],
        );

        let filled_abi = FilledAbi::try_fill_abi(abi, vec![1u8, 2u8], Codec::Scale).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Byte(Some(b"option_b".to_vec()), vec![2u8])
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
    fn decodes_eth_events_with_mocked_data() {
        use ethabi::{Event, EventParam, ParamType, RawLog};

        let correct_event = Event {
            name: "Test".into(),
            inputs: vec![
                EventParam {
                    name: "tuple".into(),
                    kind: ParamType::Tuple(vec![ParamType::Address, ParamType::Address]),
                    indexed: false,
                },
                EventParam {
                    name: "addr".into(),
                    kind: ParamType::Address,
                    indexed: true,
                },
            ],
            anonymous: false,
        };
        // swap indexed params
        let mut wrong_event = correct_event.clone();
        wrong_event.inputs[0].indexed = true;
        wrong_event.inputs[1].indexed = false;

        let abi = Abi::Log(
            Some(b"test".to_vec()),
            vec![
                Box::new(Abi::Tuple(
                    Some(b"tuple-".to_vec()),
                    (
                        Box::new(Abi::Account20(Some(b"A-".to_vec()))),
                        Box::new(Abi::Account20(Some(b"B-".to_vec()))),
                    ),
                )),
                Box::new(Abi::Account20(Some(b"A+".to_vec()))),
            ],
        );

        let log = RawLog {
            topics: vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000012321").into(),
            ],
            data: hex!(
                "
			0000000000000000000000000000000000000000000000000000000000012345
			0000000000000000000000000000000000000000000000000000000000054321
			"
            )
            .into(),
        };

        // write parse_rlp_log function that returns the content of the log as per defined in the abi
        let _corr_res = correct_event.parse_log(log.clone());

        assert!(wrong_event.parse_log(log.clone()).is_ok());
        assert!(correct_event.parse_log(log).is_ok());

        let rlp_raw_log_bytes = EthIngressEventLog(
            vec![
                hex!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000012345").into(),
            ],
            hex!(
                "
			0000000000000000000000000000000000000000000000000000000000012345
			0000000000000000000000000000000000000000000000000000000000054321
			"
            )
            .into(),
        );

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_raw_log_bytes.encode(), Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Log(
                Some(b"test".to_vec()),
                vec![
                    Box::new(FilledAbi::Tuple(
                        Some(b"tuple-".to_vec()),
                        (
                            Box::new(FilledAbi::Account20(
                                Some(b"A-".to_vec()),
                                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 35, 69]
                            )),
                            Box::new(FilledAbi::Account20(
                                Some(b"B-".to_vec()),
                                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 67, 33]
                            )),
                        ),
                    )),
                    Box::new(FilledAbi::Account20(
                        Some(b"A+".to_vec()),
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 35, 69]
                    )),
                ],
                0
            )
        );
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
                                "0606060606060606060606060606060606060606060606060606060606060606"
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
    fn fills_abi_for_32b_rlp_encoded_word_representing_bytes4() {
        let abi = Abi::Bytes4(Some(b"word".to_vec()));

        let rlp_encoded_work =
            &hex!("0102030400000000000000000000000000000000000000000000000000000000");

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_work.to_vec(), Codec::Rlp).unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"word".to_vec()), hex!("01020304").to_vec())
        )
    }

    #[test]
    fn fills_abi_for_32b_rlp_encoded_word_representing_bytes4_and_recodes_to_scale_on_4_bytes() {
        let abi = Abi::Bytes4(Some(b"word".to_vec()));

        let rlp_encoded_work =
            &hex!("0102030400000000000000000000000000000000000000000000000000000000");

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_work.to_vec(), Codec::Rlp).unwrap();

        let recoded_bytes4 = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale);

        assert_ok!(recoded_bytes4.clone());

        assert_eq!(recoded_bytes4.unwrap(), hex!("01020304").to_vec())
    }

    #[test]
    fn fills_abi_for_32b_rlp_encoded_word_representing_enum_with_bytes4_and_recodes_to_scale_on_4_bytes(
    ) {
        let abi = Abi::Enum(None, vec![Box::new(Abi::Bytes4(Some(b"word".to_vec())))]);

        let rlp_encoded_work =
            &hex!("0001020304000000000000000000000000000000000000000000000000000000");

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_work.to_vec(), Codec::Rlp).unwrap();

        let recoded_bytes4 = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale);

        assert_ok!(recoded_bytes4.clone());

        assert_eq!(recoded_bytes4.unwrap(), hex!("01020304").to_vec())
    }

    #[test]
    fn fills_abi_for_32b_rlp_encoded_word_representing_enum_with_bytes4_and_bytes_as_tuple_and_recodes_to_scale_on_4_bytes(
    ) {
        let abi = Abi::Enum(
            None,
            vec![Box::new(Abi::Tuple(
                None,
                (
                    Box::new(Abi::Bytes4(Some(b"word1".to_vec()))),
                    Box::new(Abi::Bytes(Some(b"word2".to_vec()))),
                ),
            ))],
        );

        let rlp_encoded_work =
            &hex!("00010203040000000000000000000000000000000000000000000000000000003333333333333333333333333333333333333333333333333333333333333333");

        let filled_abi =
            FilledAbi::try_fill_abi(abi, rlp_encoded_work.to_vec(), Codec::Rlp).unwrap();

        let recoded_2_words_tuple = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale);

        assert_ok!(recoded_2_words_tuple.clone());

        assert_eq!(
            recoded_2_words_tuple.unwrap(),
            vec![
                1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51,
                51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51
            ]
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
