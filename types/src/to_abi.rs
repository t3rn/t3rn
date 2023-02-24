use crate::{recode::trim_bytes, types::*};
use codec::{Decode, Encode};
use primitive_types::{H160, H256};

use crate::to_filled_abi::FilledAbi;
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum Abi {
    Struct(Option<Name>, Vec<Box<Abi>>),
    Log(Option<Name>, Vec<Box<Abi>>),
    Enum(Option<Name>, Vec<Box<Abi>>),
    Option(Option<Name>, Box<Abi>),
    Account20(Option<Name>),
    Account32(Option<Name>),
    H256(Option<Name>),
    Bytes(Option<Name>),
    Value256(Option<Name>),
    Value128(Option<Name>),
    Value64(Option<Name>),
    Value32(Option<Name>),
    Byte(Option<Name>),
    Bool(Option<Name>),
    Vec(Option<Name>, Box<Abi>),
    Tuple(Option<Name>, (Box<Abi>, Box<Abi>)),
}

impl Abi {
    pub fn get_name(&self) -> Option<Name> {
        match self {
            Abi::Struct(name, _) => name.clone(),
            Abi::Log(name, _) => name.clone(),
            Abi::Enum(name, _) => name.clone(),
            Abi::Option(name, _) => name.clone(),
            Abi::Account20(name) => name.clone(),
            Abi::Account32(name) => name.clone(),
            Abi::H256(name) => name.clone(),
            Abi::Bytes(name) => name.clone(),
            Abi::Value256(name) => name.clone(),
            Abi::Value128(name) => name.clone(),
            Abi::Value64(name) => name.clone(),
            Abi::Value32(name) => name.clone(),
            Abi::Byte(name) => name.clone(),
            Abi::Bool(name) => name.clone(),
            Abi::Vec(name, _) => name.clone(),
            Abi::Tuple(name, _) => name.clone(),
        }
    }

    pub fn get_type_size(&self) -> usize {
        match self {
            Abi::Struct(_, _fields) => 1,
            Abi::Log(_, _fields) => 1,
            Abi::Enum(_, _fields) => 1,
            Abi::Option(_, _field) => 1,
            Abi::Account20(_) => 20,
            Abi::Account32(_) => 32,
            Abi::H256(_name) => 32,
            Abi::Bytes(_) => 32,
            Abi::Value256(_) => 32,
            Abi::Value128(_) => 16,
            Abi::Value64(_) => 8,
            Abi::Value32(_) => 4,
            Abi::Byte(_) => 1,
            Abi::Bool(_) => 1,
            Abi::Vec(_, _field) => 1,
            Abi::Tuple(_, (_field1, _field2)) => 1,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Abi::Struct(_, fields) => fields.iter().map(|f| f.get_size()).sum::<usize>(),
            Abi::Log(_, fields) => fields.iter().map(|f| f.get_size()).sum::<usize>(),
            Abi::Enum(_, fields) => fields.iter().map(|f| f.get_size()).sum::<usize>(),
            Abi::Option(_, field) => 1 + field.get_size(),
            Abi::Account20(_) => 20,
            Abi::Account32(_) => 32,
            Abi::H256(_) => 32,
            Abi::Bytes(_) => 32,
            Abi::Value256(_) => 32,
            Abi::Value128(_) => 16,
            Abi::Value64(_) => 8,
            Abi::Value32(_) => 4,
            Abi::Byte(_) => 1,
            Abi::Bool(_) => 1,
            // this needs to be multiplied by the length of the vec
            Abi::Vec(_, field) => 1usize + field.get_size(),
            Abi::Tuple(_, (field1, field2)) => field1.get_size() + field2.get_size(),
        }
    }

    pub fn decode_topics_as_rlp(
        &self,
        input: Vec<u8>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        frame_support::ensure!(
            input.len() >= 32,
            "decode_topics_as_rlp -- Invalid input length lesser than 32"
        );
        match self {
            Abi::Account20(name) => {
                frame_support::ensure!(input.len() >= 20, "Decode Abi::Account20 too short");
                let data: H160 = H160::from_slice(&input[input.len() - 20..input.len()]);
                Ok((
                    FilledAbi::Account20(name.clone(), data.as_bytes().to_vec()),
                    32usize,
                ))
            },
            Abi::H256(name) | Abi::Account32(name) => {
                frame_support::ensure!(
                    input.len() == 32,
                    "Decode Abi::Account32 size mismatches 32 bytes"
                );
                let data: H256 = H256::from_slice(input.as_slice());
                Ok((
                    FilledAbi::H256(name.clone(), data.as_bytes().to_vec()),
                    32usize,
                ))
            },
            Abi::Bytes(name) => Ok((FilledAbi::Bytes(name.clone(), input), 32usize)),
            Abi::Value256(name) => {
                frame_support::ensure!(
                    input.len() == 32,
                    "Value256InvalidInput size mismatches 32 bytes"
                );
                let trimmed = trim_bytes(input.as_slice(), 32);
                Ok((FilledAbi::Value256(name.clone(), trimmed.to_vec()), 32usize))
            },
            Abi::Value128(name) => {
                let data: u128 = rlp::decode(input.as_slice()).unwrap();
                let recoded = rlp::encode(&data);
                Ok((FilledAbi::Value128(name.clone(), recoded.to_vec()), 32usize))
            },
            Abi::Value64(name) => {
                let data: u64 = rlp::decode(input.as_slice()).unwrap();
                let recoded = rlp::encode(&data);
                Ok((FilledAbi::Value64(name.clone(), recoded.to_vec()), 32usize))
            },
            Abi::Value32(name) => {
                let data: u32 = rlp::decode(input.as_slice()).unwrap();
                let recoded = rlp::encode(&data);
                Ok((FilledAbi::Value32(name.clone(), recoded.to_vec()), 32usize))
            },
            Abi::Byte(name) | Abi::Bool(name) => {
                frame_support::ensure!(
                    input.len() == 32,
                    "Decode Abi::Byte size mismatches 1 byte"
                );
                Ok((FilledAbi::Byte(name.clone(), vec![input[31]]), 32usize))
            },

            Abi::Tuple(name, (field1, field2)) => {
                let filled_1 = field1.decode_topics_as_rlp(input.clone())?;
                let filled_2 = field2.decode_topics_as_rlp(input[32..].to_vec())?;
                Ok((
                    FilledAbi::Tuple(name.clone(), (Box::new(filled_1.0), Box::new(filled_2.0))),
                    64usize,
                ))
            },
            Abi::Vec(name, field) => {
                let mut filled_vec = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;
                loop {
                    if input.is_empty() {
                        break
                    }
                    let filled = field.decode_topics_as_rlp(input.clone())?;
                    filled_vec.push(filled.0);
                    consumed += filled.1;
                    input = input[32..].to_vec();
                }
                Ok((
                    FilledAbi::Vec(name.clone(), Box::new(filled_vec), 0u8),
                    consumed,
                ))
            },
            Abi::Option(name, field) => {
                let filled = field.decode_topics_as_rlp(input)?;
                Ok((FilledAbi::Option(name.clone(), Box::new(filled.0)), 33usize))
            },
            Abi::Struct(name, fields) => {
                let mut filled_fields = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;
                for field in fields {
                    let filled = field.decode_topics_as_rlp(input.clone())?;
                    filled_fields.push(Box::new(filled.0));
                    consumed += filled.1;
                    input = input[32..].to_vec();
                }
                Ok((
                    FilledAbi::Struct(name.clone(), filled_fields, 0u8),
                    consumed,
                ))
            },
            _ => {
                unreachable!("decode_topics_as_rlp -- Invalid type")
            },
        }
    }
}

impl TryFrom<Data> for Abi {
    type Error = DispatchError;

    fn try_from(descriptor: Data) -> Result<Self, Self::Error> {
        let parsed_descriptor: Vec<(Data, Option<Data>, usize)> =
            parse_descriptor_flat(descriptor)?;

        const MAX_DEPTH: usize = 10;
        fn from_parsed_descriptor_recursive(
            fields_iter: &mut IntoIter<(Data, Option<Data>, usize)>,
            current_depth: usize,
        ) -> Result<Abi, DispatchError> {
            if current_depth > MAX_DEPTH {
                return Err("CrossCodec::from_parsed_descriptor_recursive: max depth reached".into())
            }

            let (next_field, maybe_name, _lvl) = match fields_iter.next() {
                Some(next) => next,
                None =>
                    return Err(
                        "CrossCodec::from_parsed_descriptor_recursive: no more fields".into(),
                    ),
            };
            let field_str = sp_std::str::from_utf8(next_field.as_slice())
                .map_err(|_e| "CrossCodec::failed to stringify field descriptor")?;

            match field_str {
                "Option" => {
                    let next_field_descriptor =
                        from_parsed_descriptor_recursive(fields_iter, current_depth + 1)?;
                    Ok(Abi::Option(maybe_name, Box::new(next_field_descriptor)))
                },
                "Struct" | "Enum" | "Log" => {
                    let mut fields = Vec::new();
                    let mut next_peek = fields_iter.clone().peekable();
                    while let Some((_next_field_str, _maybe_next_name, lvl)) = next_peek.peek() {
                        if lvl > &(current_depth + 1) {
                            break
                        }
                        fields.push(Box::new(from_parsed_descriptor_recursive(
                            &mut fields_iter.clone(),
                            current_depth + 1,
                        )?));
                        fields_iter.next();
                        next_peek.next();
                    }
                    match field_str {
                        "Struct" => Ok(Abi::Struct(maybe_name, fields)),
                        "Enum" => Ok(Abi::Enum(maybe_name, fields)),
                        "Log" => Ok(Abi::Log(maybe_name, fields)),
                        _ => unreachable!(),
                    }
                },
                "Bytes" => Ok(Abi::Bytes(maybe_name)),
                "Account20" => Ok(Abi::Account20(maybe_name)),
                "Account32" => Ok(Abi::Account32(maybe_name)),
                "Value256" => Ok(Abi::Value256(maybe_name)),
                "Value128" => Ok(Abi::Value128(maybe_name)),
                "Value64" => Ok(Abi::Value64(maybe_name)),
                "Value32" => Ok(Abi::Value32(maybe_name)),
                "Byte" => Ok(Abi::Byte(maybe_name)),
                "Bool" => Ok(Abi::Bool(maybe_name)),
                "Vec" => {
                    let next_field_descriptor =
                        from_parsed_descriptor_recursive(fields_iter, current_depth + 1)?;
                    Ok(Abi::Vec(maybe_name, Box::new(next_field_descriptor)))
                },
                "Tuple" => {
                    let _next_field = fields_iter.next().unwrap();
                    let next_field_descriptor =
                        from_parsed_descriptor_recursive(fields_iter, current_depth + 1)?;
                    let _next_field = fields_iter.next().unwrap();
                    let next_field_descriptor_2 =
                        from_parsed_descriptor_recursive(fields_iter, current_depth + 1)?;
                    Ok(Abi::Tuple(
                        maybe_name,
                        (
                            Box::new(next_field_descriptor),
                            Box::new(next_field_descriptor_2),
                        ),
                    ))
                },
                _ => Err("CrossCodec::failed to parse field descriptor".into()),
            }
        }

        from_parsed_descriptor_recursive(&mut parsed_descriptor.into_iter(), 0)
    }
}

#[cfg(test)]
mod test_abi {
    use super::*;

    #[test]
    fn having_descriptor_with_all_named_struct_with_2_fields_as_bytes_derives_abi() {
        let descriptor = Data::from(r#"record_name:Struct<name:Bytes,age:Value32>"#.as_bytes());
        let abi = Abi::try_from(descriptor).unwrap();
        assert_eq!(
            abi,
            Abi::Struct(
                Some(Data::from("record_name".as_bytes())),
                vec![
                    Box::new(Abi::Bytes(Some(Data::from("name".as_bytes())))),
                    Box::new(Abi::Value32(Some(Data::from("age".as_bytes())))),
                ]
            )
        )
    }
    #[test]
    fn having_descriptor_with_partially_named_struct_with_2_fields_as_bytes_derives_abi() {
        let descriptor = Data::from(r#"Struct<name:Bytes,Value32>"#.as_bytes());
        let abi = Abi::try_from(descriptor).unwrap();
        assert_eq!(
            abi,
            Abi::Struct(
                None,
                vec![
                    Box::new(Abi::Bytes(Some(Data::from("name".as_bytes())))),
                    Box::new(Abi::Value32(None)),
                ]
            )
        )
    }

    #[test]
    fn having_descriptor_with_unnamed_struct_with_2_fields_as_bytes_derives_abi() {
        let descriptor = Data::from(r#"Struct<Bytes,Value32>"#.as_bytes());
        let abi = Abi::try_from(descriptor).unwrap();
        assert_eq!(
            abi,
            Abi::Struct(
                None,
                vec![Box::new(Abi::Bytes(None)), Box::new(Abi::Value32(None)),]
            )
        )
    }
}

pub fn parse_descriptor_flat(
    descriptor: Data,
) -> Result<Vec<(Data, Option<Data>, usize)>, DispatchError> {
    let descriptor_str = sp_std::str::from_utf8(descriptor.as_slice())
        .map_err(|_e| "CrossCodec::failed to stringify field descriptor")?;

    let mut current_lvl = 0usize;
    let mut descriptors: Vec<(String, Option<String>, usize)> = vec![];
    let mut maybe_name_field: Option<String> = None;
    let mut maybe_indexed: Option<bool> = None;
    let mut current_field: String = "".into();

    for x in descriptor_str.chars() {
        match x {
            '<' | '(' => {
                descriptors.push((current_field, maybe_name_field.clone(), current_lvl));
                current_lvl += 1;
                current_field = "".into();
                maybe_name_field = None;
                maybe_indexed = None;
            },
            '>' | ')' => {
                descriptors.push((current_field, maybe_name_field.clone(), current_lvl));
                current_field = "".into();
                maybe_name_field = None;
                maybe_indexed = None;
                current_lvl -= 1;
            },
            ',' => {
                descriptors.push((current_field, maybe_name_field.clone(), current_lvl));
                current_field = "".into();
                maybe_name_field = None;
                maybe_indexed = None;
            },
            ':' => {
                maybe_name_field = Some(current_field.clone());
                current_field = "".into();
            },
            '+' => {
                maybe_indexed = Some(true);
            },
            '-' => {
                maybe_indexed = Some(false);
            },
            _ => {
                current_field.push(x);
            },
        }
    }

    let res = descriptors
        .into_iter()
        .map(|(x, maybe_y, lvl)| match maybe_y {
            Some(y) => (x.as_bytes().to_vec(), Some(y.as_bytes().to_vec()), lvl),
            None => (x.as_bytes().to_vec(), None, lvl),
        })
        .collect();

    Ok(res)
}

#[test]
pub fn test_parse_descriptor_flat_for_struct_with_all_named_fields() {
    let descriptor = b"record:Struct<name:Account20,age:Value32>".to_vec();
    let parsed = parse_descriptor_flat(descriptor).unwrap();
    assert_eq!(
        parsed,
        vec![
            (b"Struct".to_vec(), Some(b"record".to_vec()), 0),
            (b"Account20".to_vec(), Some(b"name".to_vec()), 1),
            (b"Value32".to_vec(), Some(b"age".to_vec()), 1),
        ]
    );
}

#[test]
pub fn test_parse_descriptor_flat_for_struct_with_2_out_of_3_named_fields() {
    let descriptor = b"Struct<name:Account20,age:Value32>".to_vec();
    let parsed = parse_descriptor_flat(descriptor).unwrap();
    assert_eq!(
        parsed,
        vec![
            (b"Struct".to_vec(), None, 0),
            (b"Account20".to_vec(), Some(b"name".to_vec()), 1),
            (b"Value32".to_vec(), Some(b"age".to_vec()), 1),
        ]
    );
}

#[test]
pub fn test_parse_descriptor_flat_for_struct_with_unnamed_fields() {
    let descriptor = b"Struct<Account20,Value32>".to_vec();
    let parsed = parse_descriptor_flat(descriptor).unwrap();
    assert_eq!(
        parsed,
        vec![
            (b"Struct".to_vec(), None, 0),
            (b"Account20".to_vec(), None, 1),
            (b"Value32".to_vec(), None, 1),
        ]
    );
}
