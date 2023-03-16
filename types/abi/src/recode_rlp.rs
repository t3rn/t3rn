use crate::{recode::Recode, to_abi::Abi, to_filled_abi::FilledAbi, types::Name};
use codec::{Decode, Encode};

use sp_core::{H160, H256};
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct EthIngressEventLog(pub Vec<H256>, pub Vec<u8>);
use bytes::{Buf, Bytes};
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
}

impl Abi {
    // assumes that the input is already padded to 32 bytes
    pub fn decode_topics_as_rlp(
        &self,
        input: Vec<u8>,
    ) -> Result<(FilledAbi, usize), DispatchError> {
        const MINIMUM_INPUT_LENGTH: usize = 32;
        frame_support::ensure!(
            input.len() >= MINIMUM_INPUT_LENGTH,
            "decode_topics_as_rlp -- Invalid input length lesser than 32"
        );
        match self {
            Abi::Account20(name) => {
                const ACCOUNT20_SIZE: usize = 20;
                frame_support::ensure!(
                    input.len() >= ACCOUNT20_SIZE,
                    "Decode Abi::Account20 too short"
                );
                let data: H160 = H160::from_slice(&input[input.len() - ACCOUNT20_SIZE..]);
                Ok((
                    FilledAbi::Account20(name.clone(), data.as_bytes().to_vec()),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::H256(name) | Abi::Account32(name) => {
                let data: H256 = H256::from_slice(&input[input.len() - MINIMUM_INPUT_LENGTH..]);
                Ok((
                    FilledAbi::H256(name.clone(), data.as_bytes().to_vec()),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Bytes(name) => Ok((FilledAbi::Bytes(name.clone(), input), MINIMUM_INPUT_LENGTH)),
            Abi::Value256(name) => {
                let data = input[input.len() - MINIMUM_INPUT_LENGTH..].to_vec();
                Ok((
                    FilledAbi::Value256(name.clone(), data),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Value128(name) | Abi::Value64(name) | Abi::Value32(name) => {
                let trimmed_32b = &input[input.len() - MINIMUM_INPUT_LENGTH..];
                let as_u256 = sp_core::U256::from_big_endian(trimmed_32b);

                let filled_abi = match self {
                    Abi::Value128(_) => {
                        let as_val: u128 = as_u256.as_u128();
                        FilledAbi::Value128(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                    Abi::Value64(_) => {
                        let as_val: u64 = as_u256.as_u64();
                        FilledAbi::Value64(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                    _ => {
                        let as_val: u32 = as_u256.as_u32();
                        FilledAbi::Value32(name.clone(), rlp::encode(&as_val).to_vec())
                    },
                };

                Ok((filled_abi, MINIMUM_INPUT_LENGTH))
            },
            Abi::Byte(name) | Abi::Bool(name) => {
                const BYTE_INDEX: usize = 31;
                Ok((
                    FilledAbi::Byte(name.clone(), vec![input[BYTE_INDEX]]),
                    MINIMUM_INPUT_LENGTH,
                ))
            },
            Abi::Tuple(name, (field1, field2)) => {
                let filled_1 = field1.decode_topics_as_rlp(input.clone())?;
                let filled_2 =
                    field2.decode_topics_as_rlp(input[MINIMUM_INPUT_LENGTH..].to_vec())?;
                Ok((
                    FilledAbi::Tuple(name.clone(), (Box::new(filled_1.0), Box::new(filled_2.0))),
                    MINIMUM_INPUT_LENGTH * 2,
                ))
            },
            Abi::Vec(name, field) => {
                let mut filled_vec = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;

                while !input.is_empty() {
                    let filled = field.decode_topics_as_rlp(input.clone())?;
                    filled_vec.push(filled.0);
                    consumed += filled.1;
                    input = input[MINIMUM_INPUT_LENGTH..].to_vec();
                }

                Ok((
                    FilledAbi::Vec(name.clone(), Box::new(filled_vec), 0u8),
                    consumed,
                ))
            },
            Abi::Option(name, field) => {
                let filled = field.decode_topics_as_rlp(input)?;
                Ok((
                    FilledAbi::Option(name.clone(), Box::new(filled.0)),
                    MINIMUM_INPUT_LENGTH + 1,
                ))
            },
            Abi::Struct(name, fields) => {
                let mut filled_fields = Vec::new();
                let mut input = input;
                let mut consumed = 0usize;

                for field in fields {
                    let filled = field.decode_topics_as_rlp(input.clone())?;
                    filled_fields.push(Box::new(filled.0));
                    consumed += filled.1;
                    input = input[MINIMUM_INPUT_LENGTH..].to_vec();
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
