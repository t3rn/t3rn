use crate::{
    recode::Codec,
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::{Data, Name},
};
use codec::{Decode, Encode};
use primitive_types::U256;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct IngressAbiDescriptors {
    pub for_rlp: Vec<u8>,
    pub for_scale: Vec<u8>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SFXAbi {
    // must match encoded args order. bool is for optional validation
    pub args_names: Vec<(Data, bool)>,
    pub ingress_abi_descriptors: IngressAbiDescriptors,
}

impl SFXAbi {
    pub fn get_args_names(&self) -> Vec<(Name, bool)> {
        self.args_names.clone()
    }

    pub fn get_expected_optimistic_descriptor(&self, codec: Codec) -> Name {
        match codec {
            Codec::Scale => self.ingress_abi_descriptors.for_scale.clone(),
            Codec::Rlp => self.ingress_abi_descriptors.for_rlp.clone(),
        }
    }

    pub fn check_args_equal(
        ordered_arg: Data,
        filled_abi: FilledAbi,
        output_codec: &Codec,
        in_codec: &Codec,
    ) -> Result<bool, DispatchError> {
        match filled_abi.clone() {
            FilledAbi::Value256(_name, encoded_value) => {
                let output_val_u256 = match output_codec {
                    Codec::Scale => U256::from_little_endian(&encoded_value),
                    Codec::Rlp => U256::from_big_endian(&encoded_value),
                };
                let input_val_u256 = match in_codec {
                    Codec::Scale => U256::from_little_endian(ordered_arg.as_slice()),
                    Codec::Rlp => U256::from_big_endian(ordered_arg.as_slice()),
                };

                Ok(output_val_u256 == input_val_u256)
            },
            FilledAbi::Value128(_name, encoded_value) => {
                let output_val_u128: u128 = match output_codec {
                    Codec::Scale => u128::decode(&mut &encoded_value[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&encoded_value).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                let input_val_u128: u128 = match in_codec {
                    Codec::Scale => u128::decode(&mut &ordered_arg[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&ordered_arg).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                Ok(output_val_u128 == input_val_u128)
            },
            FilledAbi::Value64(_name, encoded_value) => {
                let output_val_u64 = match output_codec {
                    Codec::Scale => u64::decode(&mut &encoded_value[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&encoded_value).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                let input_val_u64 = match in_codec {
                    Codec::Scale => u64::decode(&mut &ordered_arg[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&ordered_arg).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                Ok(output_val_u64 == input_val_u64)
            },
            FilledAbi::Value32(_name, encoded_value) => {
                let output_val_u32: u32 = match output_codec {
                    Codec::Scale => u32::decode(&mut &encoded_value[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&encoded_value).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                let input_val_u32: u32 = match in_codec {
                    Codec::Scale => u32::decode(&mut &ordered_arg[..]).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                    Codec::Rlp => rlp::decode(&ordered_arg).map_err(|_| {
                        DispatchError::Other("SFXAbi::check args equal - Invalid output value")
                    }),
                }?;

                Ok(output_val_u32 == input_val_u32)
            },
            _ => {
                let received_arg = filled_abi.get_data();
                Ok(received_arg == ordered_arg)
            },
        }
    }

    pub fn ensure_arguments_order(&self, ordered_args: &Vec<Data>) -> Result<(), DispatchError> {
        if ordered_args.len() != self.args_names.len() {
            return Err(DispatchError::Other(
                "SFXAbi::ensure args order - Invalid number of arguments",
            ))
        }
        for (i, arg) in ordered_args.iter().enumerate() {
            if self.args_names[i].1 && arg.is_empty() {
                return Err(DispatchError::Other(
                    "SFXAbi::ensure args order - Invalid argument",
                ))
            }
        }
        Ok(())
    }

    pub fn validate_arguments_against_received(
        &self,
        ordered_args: &Vec<Data>,
        received_payload: Data,
        payload_codec: Codec,
    ) -> Result<(), DispatchError> {
        self.ensure_arguments_order(ordered_args)?;
        let abi: Abi = self
            .get_expected_optimistic_descriptor(payload_codec.clone())
            .try_into()?;

        let filled_named_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, received_payload, payload_codec.clone())?;

        for (i, ordered_arg) in ordered_args.iter().enumerate() {
            let (current_arg_name, is_to_verify) = self.args_names.get(i).ok_or(
                DispatchError::Other("SFXAbi::Invalid argument - check ensure arguments order"),
            )?;
            if !is_to_verify {
                continue
            }
            let filled_abi_matched_by_name =
                filled_named_abi
                    .get_by_name(current_arg_name)
                    .ok_or(DispatchError::Other(
                        "SFXAbi::Cannot find payload argument by name {current_arg_name:?}",
                    ))?;

            if !SFXAbi::check_args_equal(
                ordered_arg.clone(),
                filled_abi_matched_by_name,
                &payload_codec,
                &Codec::Scale,
            )? {
                return Err(DispatchError::Other("SFXAbi:: Invalid argument"))
            }
        }

        Ok(())
    }
}
