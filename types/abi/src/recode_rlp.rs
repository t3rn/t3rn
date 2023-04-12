use crate::{recode::Recode, to_abi::Abi, to_filled_abi::FilledAbi, types::Name, Codec};
use codec::{Decode, Encode};

use sp_core::{H160, H256};
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct EthIngressEventLog(pub Vec<H256>, pub Vec<u8>);
use bytes::{Buf, Bytes, BytesMut};
use frame_support::ensure;

pub struct RecodeRlp;

impl Recode for RecodeRlp {
    // For RLP relies on the RLP library to do the chopping, since RLP carries the type and size information within the data.
    fn chop_encoded(
        field_data: &[u8],
        _fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(IntoIter<Vec<u8>>, u8), DispatchError> {
        let memo_prefix = field_data.first().copied().ok_or_else(|| {
            DispatchError::from(
                "RecodeRlp::chop_encoded - memo byte cannot be empty for RLP structs",
            )
        })?;

        let rlp = rlp::Rlp::new(field_data);
        let chopped_field_data: Vec<Vec<u8>> =
            rlp.into_iter().map(|rlp| rlp.as_raw().to_vec()).collect();

        Ok((chopped_field_data.into_iter(), memo_prefix))
    }

    fn event_to_filled(
        field_data: &[u8],
        name: Option<Name>,
        fields_iter_clone: IntoIter<Box<Abi>>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        let eth_ingress_event_log: EthIngressEventLog =
            EthIngressEventLog::decode(&mut &field_data[..])
                .map_err(|_e| "EthIngressEventLog::decode can't be derived with provided data")?;

        let (topics, data) = (eth_ingress_event_log.0, eth_ingress_event_log.1);

        let mut flat_topics: Vec<u8> = topics
            .into_iter()
            .skip(1)
            .flat_map(|t| t.as_ref().to_vec())
            .collect::<Vec<u8>>();

        let mut total_size = 0usize;
        let fields_iter = fields_iter_clone.peekable();
        let mut data_buf = Bytes::copy_from_slice(&data);

        let filled_abi_content = fields_iter
            .rev()
            .map(|field_descriptor| {
                let name = field_descriptor.get_name().unwrap_or(b"+".to_vec());
                let next_filled_abi = if name.last() == Some(&b'+') {
                    let (filled_abi, chopped_size) =
                        field_descriptor.decode_topics_as_rlp(flat_topics.clone())?;
                    flat_topics.truncate(flat_topics.len() - chopped_size);
                    total_size += chopped_size;
                    filled_abi
                } else {
                    let (filled_abi, chopped_size) =
                        field_descriptor.decode_topics_as_rlp(data_buf.to_vec())?;
                    data_buf.advance(chopped_size);
                    total_size += chopped_size;

                    filled_abi
                };
                Ok(Box::new(next_filled_abi))
            })
            .collect::<Result<Vec<Box<FilledAbi>>, DispatchError>>()?
            .into_iter()
            .rev()
            .collect::<Vec<Box<FilledAbi>>>();

        Ok((FilledAbi::Log(name, filled_abi_content, 0u8), total_size))
    }

    fn fill_abi(abi: Abi, field_data: Vec<u8>) -> Result<FilledAbi, DispatchError> {
        println!("Rlp::fill_abi - abi: {:?}", abi);
        println!("Rlp::fill_abi - field_data: {:?}", field_data);
        let rlp = rlp::Rlp::new(&*field_data);
        println!("Rlp::fill_abi - rlp: {:?}", rlp);

        for field in rlp.iter() {
            println!("Rlp::fill_abi - field: {:?}", field);
        }

        // if rlp isn't able to decode the bytes, give it a shot with raw fillers
        let chopped_field_data: Vec<Vec<u8>> =
            rlp.into_iter().map(|rlp| rlp.as_raw().to_vec()).collect();

        println!(
            "Rlp::fill_abi - chopped_field_data: {:?}",
            chopped_field_data
        );

        let mut filled_abi_vec = vec![];
        let mut total_struct_size = 0usize;

        for field in chopped_field_data.iter() {
            let (filled_abi, size) =
                FilledAbi::recursive_fill_abi(abi.clone(), field.as_slice(), Codec::Rlp)?;
            total_struct_size += size;
            filled_abi_vec.push(Box::new(filled_abi));
        }

        println!("Rlp::fill_abi - filled_abi_vec: {:?}", filled_abi_vec);

        match filled_abi_vec.len() {
            0 => Err(DispatchError::from("RecodeRlp::fill_abi - empty struct")),
            1 => Ok(*filled_abi_vec.pop().unwrap()),
            2 => Ok(FilledAbi::Tuple(
                abi.get_name().map(|name| name),
                (filled_abi_vec[0].clone(), filled_abi_vec[1].clone()),
            )),
            3 => Ok(FilledAbi::Triple(
                abi.get_name().map(|name| name),
                (
                    filled_abi_vec[0].clone(),
                    filled_abi_vec[1].clone(),
                    filled_abi_vec[2].clone(),
                ),
            )),
            4 => Ok(FilledAbi::Quadruple(
                abi.get_name().map(|name| name),
                (
                    filled_abi_vec[0].clone(),
                    filled_abi_vec[1].clone(),
                    filled_abi_vec[2].clone(),
                    filled_abi_vec[3].clone(),
                ),
            )),
            5 => Ok(FilledAbi::Quintuple(
                abi.get_name().map(|name| name),
                (
                    filled_abi_vec[0].clone(),
                    filled_abi_vec[1].clone(),
                    filled_abi_vec[2].clone(),
                    filled_abi_vec[3].clone(),
                    filled_abi_vec[4].clone(),
                ),
            )),
            6 => Ok(FilledAbi::Sextuple(
                abi.get_name().map(|name| name),
                (
                    filled_abi_vec[0].clone(),
                    filled_abi_vec[1].clone(),
                    filled_abi_vec[2].clone(),
                    filled_abi_vec[3].clone(),
                    filled_abi_vec[4].clone(),
                    filled_abi_vec[5].clone(),
                ),
            )),
            _ => Err(DispatchError::from(
                "RecodeRlp::fill_abi - unsupported args with more than 7 fields",
            )),
        }
    }
}

impl Abi {
    // assumes that the input is already padded to 32 bytes
    pub fn decode_topics_as_rlp(
        &self,
        input: Vec<u8>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        const MINIMUM_INPUT_LENGTH: usize = 32;
        ensure!(
            input.len() >= MINIMUM_INPUT_LENGTH,
            "decode_topics_as_rlp -- Invalid input length lesser than 32"
        );
        let input = Bytes::from(input);
        let input_len = input.len();
        let last_32b = &input[input_len - MINIMUM_INPUT_LENGTH..];

        match self {
            Abi::Account20(name) => {
                const ACCOUNT20_SIZE: usize = 20;
                let data: H160 =
                    H160::from_slice(&last_32b[MINIMUM_INPUT_LENGTH - ACCOUNT20_SIZE..]);
                Ok((
                    FilledAbi::Account20(name.clone(), data.as_bytes().to_vec()),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::H256(name) | Abi::Account32(name) => {
                let data: H256 = H256::from_slice(last_32b);
                Ok((
                    FilledAbi::H256(name.clone(), data.as_bytes().to_vec()),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Bytes(name) => Ok((
                FilledAbi::Bytes(name.clone(), input.to_vec()),
                MINIMUM_INPUT_LENGTH,
            )),
            Abi::Value256(name) => Ok((
                FilledAbi::Value256(name.clone(), last_32b.to_vec()),
                MINIMUM_INPUT_LENGTH,
            )),
            Abi::Value128(name) | Abi::Value64(name) | Abi::Value32(name) => {
                let as_u256 = sp_core::U256::from_big_endian(last_32b);
                let filled_abi = match self {
                    Abi::Value128(_) => {
                        let as_val: u128 = as_u256.try_into()?;
                        FilledAbi::Value128(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                    Abi::Value64(_) => {
                        let as_val: u64 = as_u256.try_into()?;
                        FilledAbi::Value64(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                    _ => {
                        let as_val: u32 = as_u256.try_into()?;
                        FilledAbi::Value32(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                };

                Ok((filled_abi, MINIMUM_INPUT_LENGTH))
            },
            Abi::Byte(name) | Abi::Bool(name) => {
                const BYTE_INDEX: usize = 31;
                Ok((
                    FilledAbi::Byte(name.clone(), vec![last_32b[BYTE_INDEX]]),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Tuple(name, (field1, field2)) => {
                let mut input = input;
                let (filled_2, filled_2_size) = field2.decode_topics_as_rlp(input.to_vec())?;
                ensure!(
                    filled_2_size <= input.len(),
                    "decode_topics_as_rlp -- Invalid input length for Tuple"
                );
                let input1 = input.split_to(filled_2_size);
                let (filled_1, filled_1_size) = field1.decode_topics_as_rlp(input1.to_vec())?;
                Ok((
                    FilledAbi::Tuple(name.clone(), (Box::new(filled_1), Box::new(filled_2))),
                    filled_2_size + filled_1_size,
                ))
            },
            Abi::Vec(name, field) => {
                let mut filled_vec = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;

                while !input.is_empty() {
                    let filled = field.decode_topics_as_rlp(input.to_vec())?;
                    filled_vec.push(filled.0);
                    consumed += MINIMUM_INPUT_LENGTH;
                    input = input.split_to(input.len() - MINIMUM_INPUT_LENGTH);
                }
                Ok((
                    FilledAbi::Vec(name.clone(), Box::new(filled_vec), 0u8),
                    consumed,
                ))
            },
            Abi::Option(name, field) => {
                let filled = field.decode_topics_as_rlp(input.to_vec())?;
                Ok((
                    FilledAbi::Option(name.clone(), Box::new(filled.0)),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Struct(name, fields) => {
                let mut filled_fields = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;

                for field in fields {
                    let filled = field.decode_topics_as_rlp(input.to_vec())?;
                    filled_fields.push(Box::new(filled.0));
                    consumed += MINIMUM_INPUT_LENGTH;
                    input = input.split_to(input.len() - MINIMUM_INPUT_LENGTH);
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

pub fn rlp_encode(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend(rlp::encode_list(&input[..]).to_vec());
    println!("rlp_encode -- output: {:?}", output);
    output

    // let len = input.len();
    //
    // let mut encoded: Vec<u8> = Vec::new();
    //
    // // If the length is between 0 and 55 bytes and the first byte is greater than or equal to 0x80
    // if len > 0 && len <= 55 && input[0] >= 0x80 {
    //     encoded.push(0x80 + len as u8);
    //     encoded.extend(input);
    // }
    // // If the length is between 1 and 55 bytes and the first byte is less than 0x80
    // else if len > 0 && len <= 55 && input[0] < 0x80 {
    //     encoded = input;
    // }
    // // If the length is more than 55 bytes
    // else if len > 55 {
    //     let len_of_len = length_of_length(len);
    //     encoded.push(0xb7 + len_of_len as u8);
    //     encoded.extend(rlp::encode_length(len, len_of_len));
    //     encoded.extend(input);
    // }
    //
    // encoded
}

pub fn rlp_encode_raw(input: Vec<u8>) -> Vec<u8> {
    println!("rlp_encode start input: {:?}", input);
    let len = input.len();

    let mut encoded: Vec<u8> = Vec::new();

    // // If the length is between 0 and 55 bytes and the first byte is greater than or equal to 0x80
    // if len > 0 && len <= 55 && input[0] >= 0x80 {
    //     encoded.push(0x80 + len as u8);
    //     encoded.extend(input);
    // }
    // // If the length is between 1 and 55 bytes and the first byte is less than 0x80
    // else if len > 0 && len <= 55 && input[0] < 0x80 {
    //     encoded = input;
    // }
    // If the length is between 0 and 55 bytes and the first byte is greater than or equal to 0x80
    if len > 0 && len <= 55 {
        encoded.push(0x80 + len as u8);
        encoded.extend(input);
    }
    // // If the length is between 1 and 55 bytes and the first byte is less than 0x80
    // else if len > 0 && len <= 55 && input[0] < 0x80 {
    //     encoded = input;
    // }
    // If the length is more than 55 bytes
    else if len > 55 {
        let len_of_len = length_of_length(len);
        encoded.push(0xb7 + len_of_len as u8);
        encoded.extend(encode_length(len, len_of_len));
        encoded.extend(input);
    }

    println!("rlp_encode: {:?}", encoded);
    encoded
}

pub fn length_of_length(len: usize) -> usize {
    let mut len_of_len = 0;
    while (len >> (8 * len_of_len)) > 0 {
        len_of_len += 1;
    }
    len_of_len
}

pub fn encode_length(len: usize, len_of_len: usize) -> Vec<u8> {
    let mut encoded_length = vec![0; len_of_len];
    for i in 0..len_of_len {
        encoded_length[len_of_len - 1 - i] = ((len >> (8 * i)) & 0xff) as u8;
    }
    encoded_length
}

#[cfg(test)]
mod test_recode_rlp {
    use super::*;
    use frame_support::assert_err;
    use hex_literal::hex;

    #[test]
    fn test_decode_topics_as_rlp_account20() {
        let abi = Abi::Account20(Some(b"address".to_vec()));
        let input = hex!("000000000000000000000000000102030405060708090A0B0C0D0E0F10111213");
        let result = abi.decode_topics_as_rlp(input.to_vec()).unwrap();
        let expected_output = (
            FilledAbi::Account20(
                Some(b"address".to_vec()),
                hex!("000102030405060708090A0B0C0D0E0F10111213").to_vec(),
            ),
            32,
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_topics_as_rlp_h256() {
        let abi = Abi::H256(Some(b"hash".to_vec()));
        let input = hex!("AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00112233445566778899");
        let result = abi.decode_topics_as_rlp(input.to_vec()).unwrap();
        let expected_output = (
            FilledAbi::H256(
                Some(b"hash".to_vec()),
                hex!("AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00112233445566778899").to_vec(),
            ),
            32,
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_topics_as_rlp_value256() {
        let abi = Abi::Value256(Some(b"value256".to_vec()));
        let input = hex!("0000000000000000000000000000000000000000000000000A0B0C0D0E0F1011");
        let result = abi.decode_topics_as_rlp(input.to_vec()).unwrap();
        let expected_output = (
            FilledAbi::Value256(
                Some(b"value256".to_vec()),
                hex!("0000000000000000000000000000000000000000000000000A0B0C0D0E0F1011").to_vec(),
            ),
            32,
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_topics_as_rlp_value128() {
        let abi = Abi::Value128(Some(b"value128".to_vec()));
        let input = hex!("0000000000000000000000000000000000000000000000000A0B0C0D0E0F1011");
        let result = abi.decode_topics_as_rlp(input.to_vec()).unwrap();
        let expected_output = (
            FilledAbi::Value128(
                Some(b"value128".to_vec()),
                vec![136, 10, 11, 12, 13, 14, 15, 16, 17],
            ),
            32,
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_topics_as_rlp_value64() {
        let abi = Abi::Value64(Some(b"value64".to_vec()));
        let input = hex!("0000000000000000000000000000000000000000000000000A0B0C0D0E0F1011");
        let result = abi.decode_topics_as_rlp(input.to_vec()).unwrap();
        let expected_output = (
            FilledAbi::Value64(
                Some(b"value64".to_vec()),
                vec![136, 10, 11, 12, 13, 14, 15, 16, 17],
            ),
            32,
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_topics_as_rlp_value32_throw_overflow() {
        let abi = Abi::Value32(Some(b"value32".to_vec()));
        let input = hex!("0000000000000000000000000000000000000000000000000A0B0C0D0E0F1011");
        assert_err!(
            abi.decode_topics_as_rlp(input.to_vec()),
            "integer overflow when casting to u32"
        );
    }

    // Helper function for H256 data
    fn h256_data(input: &[u8; 32]) -> Vec<u8> {
        let mut data = vec![0; 32];
        data.copy_from_slice(input);
        data
    }

    #[test]
    fn test_decode_topics_as_rlp_h256_tuple() {
        let abi = Abi::Tuple(
            Some(b"tuple".to_vec()),
            (
                Box::new(Abi::H256(Some(b"h256_1".to_vec()))),
                Box::new(Abi::H256(Some(b"h256_2".to_vec()))),
            ),
        );
        let input1 = h256_data(&[1; 32]);
        let input2 = h256_data(&[2; 32]);
        let input = [&input1[..], &input2[..]].concat();

        let result = abi.decode_topics_as_rlp(input).unwrap();
        match result.0 {
            FilledAbi::Tuple(_, (field1, field2)) => {
                assert_eq!(field1.get_name(), Some(b"h256_1".to_vec()));
                assert_eq!(field2.get_name(), Some(b"h256_2".to_vec()));
            },
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_decode_topics_as_rlp_h256_option() {
        let abi = Abi::Option(
            Some(b"option".to_vec()),
            Box::new(Abi::H256(Some(b"h256".to_vec()))),
        );
        let input = h256_data(&[1; 32]);

        let result = abi.decode_topics_as_rlp(input).unwrap();
        match result.0 {
            FilledAbi::Option(_, inner) => {
                assert_eq!(inner.get_name(), Some(b"h256".to_vec()));
            },
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_decode_topics_as_rlp_h256_vector() {
        let abi = Abi::Vec(
            Some(b"vector".to_vec()),
            Box::new(Abi::H256(Some(b"h256".to_vec()))),
        );
        let input1 = h256_data(&[1; 32]);
        let input2 = h256_data(&[2; 32]);
        let input = [&input1[..], &input2[..]].concat();

        let result = abi.decode_topics_as_rlp(input).unwrap();
        match result.0 {
            FilledAbi::Vec(_, inner, _) => {
                assert_eq!(inner.len(), 2);
                assert_eq!(inner[0].get_name(), Some(b"h256".to_vec()));
                assert_eq!(inner[1].get_name(), Some(b"h256".to_vec()));
            },
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_decode_topics_as_rlp_struct() {
        let field1 = Box::new(Abi::H256(Some(b"field1".to_vec())));
        let field2 = Box::new(Abi::H256(Some(b"field2".to_vec())));
        let struct_abi = Abi::Struct(Some(b"test_struct".to_vec()), vec![field1, field2]);

        let input1 = hex!("1111111111111111111111111111111111111111111111111111111111111111");
        let input2 = hex!("2222222222222222222222222222222222222222222222222222222222222222");
        let combined_input = [&input2[..], &input1[..]].concat();

        let (filled_abi, consumed) = struct_abi.decode_topics_as_rlp(combined_input).unwrap();

        assert_eq!(consumed, 64);
        if let FilledAbi::Struct(name, fields, _) = filled_abi {
            assert_eq!(name, Some(b"test_struct".to_vec()));
            assert_eq!(fields.len(), 2);

            if let FilledAbi::H256(field_name, data) = &*fields[0] {
                assert_eq!(field_name, &Some(b"field1".to_vec()));
                assert_eq!(data, &input1);
            } else {
                panic!("Unexpected FilledAbi variant for field1");
            }

            if let FilledAbi::H256(field_name, data) = &*fields[1] {
                assert_eq!(field_name, &Some(b"field2".to_vec()));
                assert_eq!(data, &input2);
            } else {
                panic!("Unexpected FilledAbi variant for field2");
            }
        } else {
            panic!("Unexpected FilledAbi variant");
        }
    }
}
