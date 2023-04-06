use crate::{
    recode_rlp::RecodeRlp,
    recode_scale::RecodeScale,
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::{Data, Name},
};
use codec::{Decode, Encode};
use frame_support::{ensure, log};
use scale_info::TypeInfo;
use sp_core::{
    crypto::{AccountId32, ByteArray},
    H160, U256,
};
use sp_runtime::DispatchError;
use sp_std::{prelude::*, vec::IntoIter};

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum Codec {
    #[default]
    Scale,
    Rlp,
}

impl From<u8> for Codec {
    fn from(value: u8) -> Self {
        match value {
            0 => Codec::Scale,
            1 => Codec::Rlp,
            _ => Codec::default(),
        }
    }
}

impl From<Codec> for u8 {
    fn from(value: Codec) -> Self {
        match value {
            Codec::Scale => 0,
            Codec::Rlp => 1,
        }
    }
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

impl FilledAbi {
    pub fn recode_as(&self, in_codec: &Codec, out_codec: &Codec) -> Result<Data, DispatchError> {
        match self {
            FilledAbi::Struct(_name, fields, struct_prefix_memo)
            | FilledAbi::Event(_name, fields, struct_prefix_memo)
            | FilledAbi::Enum(_name, fields, struct_prefix_memo)
            | FilledAbi::Log(_name, fields, struct_prefix_memo) => {
                // Remove and re-add the struct prefix at the end
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
            FilledAbi::Tuple(_name, (field1, field2)) => {
                let mut encoded_fields: Vec<u8> = vec![];
                for field in &[field1, field2] {
                    encoded_fields.extend_from_slice(&field.recode_as(in_codec, out_codec)?[..]);
                }
                match (in_codec, out_codec) {
                    (_, Codec::Scale) => Ok(encoded_fields),
                    (_, Codec::Rlp) => Ok(rlp::encode_list(&encoded_fields).to_vec()),
                }
            },
            // todo: consider converting between little vs big endian
            FilledAbi::Bytes(_name, data) => Ok(data.clone()),
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
            FilledAbi::Bytes4(_name, data)
            | FilledAbi::Codec(_name, data)
            | FilledAbi::Byte(_name, data)
            | FilledAbi::Bool(_name, data) => Ok(data.clone()),
            FilledAbi::H256(_name, data) | FilledAbi::Account32(_name, data) =>
                match (in_codec, out_codec) {
                    (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                    (Codec::Scale, Codec::Rlp) => {
                        let decoded_account: AccountId32 = AccountId32::decode(&mut &data[..])
                            .map_err(|_e| "Account32 error at recoding back to Scale")?;

                        Ok(rlp::encode(&decoded_account.to_raw_vec()).to_vec())
                    },
                    (Codec::Rlp, Codec::Scale) => {
                        // todo: consider converting between little vs big endian with data.rev()
                        let decoded_account: AccountId32 = AccountId32::decode(&mut &data[..])
                            .map_err(|_e| "Account32 error at recoding back to Scale")?;
                        Ok(decoded_account.encode())
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
                        data.len() == 20,
                        "RLP encoded account should be 20 bytes long"
                    );
                    let account_id_20: H160 = H160::from_slice(&data[0..20]);
                    Ok(account_id_20.encode())
                },
            },
            FilledAbi::Value32(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u32 = Decode::decode(&mut &data[..]).map_err(|_| {
                        DispatchError::Other(
                            "Recode::recode_as failed to decode Value32 from Scale",
                        )
                    })?;
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u32 = rlp::decode(&data[..]).map_err(|_| {
                        DispatchError::Other(
                            "Recode::recode_as failed to decode Value32 from Scale",
                        )
                    })?;
                    Ok(value.encode())
                },
            },
            FilledAbi::Value64(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u64 = Decode::decode(&mut &data[..]).map_err(|_| {
                        DispatchError::Other(
                            "Recode::recode_as failed to decode Value64 from Scale",
                        )
                    })?;
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u64 = rlp::decode(&data[..]).map_err(|_| {
                        DispatchError::Other("Recode::recode_as failed to decode Value64 from Rlp")
                    })?;
                    Ok(value.encode())
                },
            },
            FilledAbi::Value128(_name, data) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) => Ok(data.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value: u128 = Decode::decode(&mut &data[..]).map_err(|_| {
                        DispatchError::Other(
                            "Recode::recode_as failed to decode Value128 from Scale",
                        )
                    })?;
                    Ok(rlp::encode(&value).to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value: u128 = rlp::decode(&data[..]).map_err(|_| {
                        DispatchError::Other(
                            "Recode::recode_as failed to decode Value128 Scale from Rlp",
                        )
                    })?;
                    Ok(value.encode())
                },
            },
            FilledAbi::Value256(_name, encoded_value) => match (in_codec, out_codec) {
                (Codec::Scale, Codec::Scale) | (Codec::Rlp, Codec::Rlp) =>
                    Ok(encoded_value.clone()),
                (Codec::Scale, Codec::Rlp) => {
                    let value_256: U256 = U256::from_little_endian(encoded_value);
                    let mut big_endian_value_32b: [u8; 32] = [0; 32];
                    value_256.to_big_endian(&mut big_endian_value_32b);
                    Ok(big_endian_value_32b.to_vec())
                },
                (Codec::Rlp, Codec::Scale) => {
                    let value_256: U256 = U256::from_big_endian(encoded_value);
                    let mut little_endian_value_32b: [u8; 32] = [0; 32];
                    value_256.to_little_endian(&mut little_endian_value_32b);
                    Ok(little_endian_value_32b.to_vec())
                },
            },
            _ => {
                log::error!(
                    "Recoding filled not implemented for type: {:?}",
                    self.type_name()
                );
                Err(DispatchError::Other(
                    "Recoding filled ABI not implemented for this type",
                ))
            },
        }
    }
}

#[cfg(test)]
mod test_recode {
    use super::*;

    use hex_literal::hex;

    use sp_core::{crypto::AccountId32, ByteArray};

    #[test]
    fn recodes_account20_from_rlp_to_scale() {
        let abi = Abi::Account20(None);
        let val: H160 = hex!("0909090906060606060606060606060606060606").into();
        let rlp_encoded = rlp::encode(&val.0.as_slice()).to_vec();

        let filled_abi = FilledAbi::try_fill_abi(abi, rlp_encoded.clone(), Codec::Rlp).unwrap();
        assert_eq!(
            filled_abi,
            FilledAbi::Account20(None, rlp_encoded[1..].to_vec())
        );

        let scale_recoded = filled_abi.recode_as(&Codec::Rlp, &Codec::Scale).unwrap();

        assert_eq!(scale_recoded, val.encode());
    }

    #[test]
    fn recodes_account20_from_scale_to_rlp() {
        let abi = Abi::Account20(None);
        let val: H160 = hex!("0909090906060606060606060606060606060606").into();
        let scale_encoded = val.encode();

        let filled_abi = FilledAbi::try_fill_abi(abi, scale_encoded.clone(), Codec::Scale).unwrap();

        assert_eq!(filled_abi, FilledAbi::Account20(None, scale_encoded));

        let rlp_encoded = filled_abi.recode_as(&Codec::Scale, &Codec::Rlp).unwrap();

        assert_eq!(rlp_encoded, rlp::encode(&val.0.as_slice()).to_vec());
    }

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

        assert_eq!(
            filled_abi,
            FilledAbi::Account32(None, rlp_encoded[1..].to_vec())
        );

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
}
