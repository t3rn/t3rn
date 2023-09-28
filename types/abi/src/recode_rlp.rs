use crate::{recode::Recode, to_abi::Abi, to_filled_abi::FilledAbi, types::Name};
use codec::{Decode, Encode};

use sp_core::{H160, H256};
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eth2IngressEventLog {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

impl Decodable for Eth2IngressEventLog {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let address = rlp.val_at::<H160>(0)?;
        let topics = rlp.list_at::<H256>(1).map_err(|e| {
            log::error!("Error decoding Eth2IngressEventLog topics: {:?}", e);
            e
        })?;
        let data = rlp.val_at::<Vec<u8>>(2).map_err(|e| {
            log::error!("Error decoding Eth2IngressEventLog data: {:?}", e);
            e
        })?;
        Ok(Eth2IngressEventLog {
            address,
            topics,
            data,
        })
    }
}

impl Encodable for Eth2IngressEventLog {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list::<H256, _>(&self.topics);
        // s.append_list::<H256, _>(&self.data);
        s.append(&self.data);
    }
}

impl Eth2IngressEventLog {
    pub fn encode(&self) -> Vec<u8> {
        rlp::encode(self).to_vec()
    }
}

#[test]
fn decodes_eth2_ingress_event_log_out_of_usdt_erc20_transfer() {
    let rlp_encoded_usdt_erc20: Vec<u8> = hex_literal::hex!("f89b947169d38820dfd117c3fa1f22a697dba58d90ba06f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bfa00000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8cea00000000000000000000000000000000000000000000000000000000003473bc0").into();

    let decoded: Eth2IngressEventLog = rlp::decode(&rlp_encoded_usdt_erc20.as_slice()).unwrap();

    assert_eq!(
        decoded,
        Eth2IngressEventLog {
            address: H160::from(hex_literal::hex!(
                "7169d38820dfd117c3fa1f22a697dba58d90ba06"
            )),
            topics: vec![
                H256::from(hex_literal::hex!(
                    "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                )),
                H256::from(hex_literal::hex!(
                    "000000000000000000000000b12713bfa9d1de339ca14b01f8f14f092ffe75bf"
                )), // from
                H256::from(hex_literal::hex!(
                    "0000000000000000000000000e8eb8efdb38c216f2ec7185b1f54855ac50a8ce"
                )), // to
            ],
            data: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                3, 71, 59, 192
            ], // amount
        }
    );
}

use bytes::{Buf, Bytes};
use frame_support::{ensure, log, log::error};
use rlp::{Decodable, Encodable};

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
        let eth_ingress_event_log: Eth2IngressEventLog = rlp::decode(&mut &field_data[..])
            .map_err(|_e| "Eth2IngressEventLog::decode can't be derived with provided data")?;

        let (topics, data) = (eth_ingress_event_log.topics, eth_ingress_event_log.data);

        let mut flat_topics: Vec<u8> = topics
            .into_iter()
            .skip(1)
            .flat_map(|t| t.as_ref().to_vec())
            .collect::<Vec<u8>>();

        let mut total_size = 0usize;
        let fields_iter = fields_iter_clone.peekable();
        let mut flat_data = data.clone();

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
                        field_descriptor.decode_topics_as_rlp(flat_data.clone())?;
                    flat_data.truncate(flat_data.len() - chopped_size);
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
            Abi::Bytes4(name) => Ok((
                FilledAbi::Bytes4(name.clone(), last_32b[0..4].to_vec()),
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
