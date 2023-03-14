use crate::{
    recode::Codec,
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::{Data, Name},
};
use codec::{Decode, Encode};

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
        ordered_args_codec: &Codec,
        payload_codec: &Codec,
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

            let current_arg_name_str = sp_std::str::from_utf8(current_arg_name.as_slice())
                .map_err(|_e| "CrossCodec::failed to stringify current_arg_name_str, it's useful for debug message")?;

            if !is_to_verify {
                continue
            }

            let filled_abi_matched_by_name =
                filled_named_abi
                    .get_by_name(current_arg_name)
                    .ok_or(DispatchError::Other(Box::leak(
                        format!(
                            "SFXAbi::Cannot find payload argument by name {current_arg_name_str:?}"
                        )
                        .into_boxed_str(),
                    )))?;

            // Check if arguments are equal after recoding to ordered_args_codec
            let recoded_payload: Data =
                filled_abi_matched_by_name.recode_as(payload_codec, ordered_args_codec)?;

            match recoded_payload == *ordered_arg {
                true => continue,
                false =>
                    return Err(DispatchError::Other(
                        Box::leak(format!("SFXAbi::invalid payload argument for: '{current_arg_name_str:?}'; expected: {ordered_arg:?}; received and recoded: {recoded_payload:?}").into_boxed_str()),
                    )),
            }
        }

        Ok(())
    }
}
