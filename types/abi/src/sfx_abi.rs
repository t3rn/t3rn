use crate::{
    recode::Codec,
    to_abi::Abi,
    to_filled_abi::FilledAbi,
    types::{Data, Name},
};
use codec::{Decode, Encode};
use frame_support::log;

use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct PerCodecAbiDescriptors {
    pub for_rlp: Vec<u8>,
    pub for_scale: Vec<u8>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SFXAbi {
    // must match encoded args order. bool is for optional validation
    pub args_names: Vec<(Data, bool)>,
    // Must be set for secure decoding of ingress emitted by Substrate events
    pub maybe_prefix_memo: Option<u8>,
    pub egress_abi_descriptors: PerCodecAbiDescriptors,
    pub ingress_abi_descriptors: PerCodecAbiDescriptors,
}

impl SFXAbi {
    pub fn set_prefix_memo(&mut self, prefix_memo: u8) {
        self.maybe_prefix_memo = Some(prefix_memo);
    }

    pub fn get_args_names(&self) -> Vec<(Name, bool)> {
        self.args_names.clone()
    }

    pub fn get_expected_ingress_descriptor(&self, codec: Codec) -> Name {
        match codec {
            Codec::Scale => self.ingress_abi_descriptors.for_scale.clone(),
            Codec::Rlp => self.ingress_abi_descriptors.for_rlp.clone(),
        }
    }

    pub fn get_expected_egress_descriptor(&self, codec: Codec) -> Name {
        match codec {
            Codec::Scale => self.egress_abi_descriptors.for_scale.clone(),
            Codec::Rlp => self.egress_abi_descriptors.for_rlp.clone(),
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

    pub fn validate_ordered_arguments(
        &self,
        ordered_args: &Vec<Data>,
        ordered_args_codec: &Codec,
    ) -> Result<FilledAbi, DispatchError> {
        self.ensure_arguments_order(ordered_args)?;

        let abi: Abi = self
            .get_expected_egress_descriptor(ordered_args_codec.clone())
            .try_into()?;

        let maybe_prefix_vec = match self.maybe_prefix_memo {
            Some(prefix_memo) => vec![prefix_memo],
            None => vec![],
        };

        let ordered_args_flatten: Data =
            // Extend with 0u8 assumed as prefix memo for struct Codec
            ordered_args.iter().fold(maybe_prefix_vec, |mut acc, arg| {
                acc.append(&mut arg.clone());
                acc
            });

        FilledAbi::try_fill_abi(abi, ordered_args_flatten, ordered_args_codec.clone())
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
            .get_expected_ingress_descriptor(payload_codec.clone())
            .try_into()?;

        let filled_named_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, received_payload, payload_codec.clone())?;

        // Check prefix memo if it's set - it's optional since not required for any Event decoding besides Substrate Events
        // At the same time imposes security risk by attacker faking events sent out of unauthorized pallets
        if payload_codec != &Codec::Rlp
            && self.maybe_prefix_memo.is_some()
            && filled_named_abi.get_prefix_memo() != self.maybe_prefix_memo
        {
            log::error!(
                "SFXAbi::invalid prefix memo for: '{:?}'; expected: {:?}; received: {:?}",
                self.get_expected_ingress_descriptor(payload_codec.clone()),
                self.maybe_prefix_memo,
                filled_named_abi.get_prefix_memo()
            );
            return Err(DispatchError::Other(
                "SFXAbi::invalid prefix memo for -- expected: doesn't match received",
            ))
        }

        for (i, ordered_arg) in ordered_args.iter().enumerate() {
            let (current_arg_name, is_to_verify) = self.args_names.get(i).ok_or(
                DispatchError::Other("SFXAbi::Invalid argument - check ensure arguments order"),
            )?;

            let current_arg_name_str = sp_std::str::from_utf8(current_arg_name.as_slice())
                .map_err(|_e| "CrossCodec::failed to stringify current_arg_name_str, it's useful for debug message")?;

            if !is_to_verify {
                continue
            }

            let filled_abi_matched_by_name = filled_named_abi
                .get_by_name(current_arg_name)
                .ok_or_else(|| {
                    log::error!(
                        "SFXAbi::Cannot find payload argument by name {:?}",
                        current_arg_name_str
                    );
                    DispatchError::Other("SFXAbi::Cannot find payload argument by name")
                })?;

            // Check if arguments are equal after recoding to ordered_args_codec
            let recoded_payload: Data =
                filled_abi_matched_by_name.recode_as(payload_codec, ordered_args_codec, true)?;

            match recoded_payload == *ordered_arg {
                true => continue,
                false => {
                    log::error!(
                        "SFXAbi::invalid payload argument for: '{:?}'; expected: {:?}; received and recoded: {:?}", current_arg_name_str, ordered_arg, recoded_payload
                    );
                    return Err(DispatchError::Other(
                        "SFXAbi::invalid payload argument for -- expected: doesn't match received and recoded",
                    ));
                },
            }
        }

        Ok(())
    }
}
