use crate::{
    recode::Codec,
    types::{Data, Name},
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

use crate::{to_abi::Abi, to_filled_abi::FilledAbi};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct SFXAbi {
    // must match encoded args order. bool is for optional validation
    pub args_names: Vec<(Name, bool)>,
    pub expected_optimistic_descriptor: Name,
}

impl SFXAbi {
    pub fn get_args_names(&self) -> Vec<(Name, bool)> {
        self.args_names.clone()
    }

    pub fn get_expected_optimistic_descriptor(&self) -> Name {
        self.expected_optimistic_descriptor.clone()
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
        let abi: Abi = self.get_expected_optimistic_descriptor().try_into()?;
        let filled_named_abi: FilledAbi =
            FilledAbi::try_fill_abi(abi, received_payload, payload_codec)?;

        for (i, get_data_by_name) in ordered_args.iter().enumerate() {
            let (current_arg_name, is_to_verify) = self.args_names.get(i).ok_or(
                DispatchError::Other("SFXAbi::Invalid argument - check ensure arguments order"),
            )?;
            if !is_to_verify {
                continue
            }
            println!("SFXAbi:: Validating argument {current_arg_name:?}");

            let received_arg =
                filled_named_abi
                    .get_data_by_name(current_arg_name)
                    .ok_or(DispatchError::Other(
                        "SFXAbi::Cannot find payload argument by name {current_arg_name:?}",
                    ))?;

            if received_arg != *get_data_by_name {
                println!(
                    "SFXAbi:: Received argument {received_arg:?} != {get_data_by_name:?} does not match for field name {current_arg_name:?}");
                return Err(DispatchError::Other("SFXAbi:: Invalid argument"))
            }
        }

        Ok(())
    }
}

pub fn get_default_evm_transfer_interface() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), false),
            (b"to".to_vec(), true),
            (b"amount".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        expected_optimistic_descriptor: b"Transfer:Log(to:Account32,amount:Value256)".to_vec(),
    }
}

pub fn get_default_substrate_transfer_interface() -> SFXAbi {
    SFXAbi {
        args_names: vec![
            (b"from".to_vec(), true),
            (b"to".to_vec(), true),
            (b"amount".to_vec(), true),
            (b"insurance".to_vec(), false),
        ],
        expected_optimistic_descriptor:
            b"Transfer:Struct(from:Account32,to:Account32,amount:Value128)".to_vec(),
    }
}

#[cfg(test)]
mod test_sfx_abi {
    use super::*;
    use crate::mini_mock::MiniRuntime;

    use sp_runtime::AccountId32;

    #[test]
    fn test_transfer_validate_arguments_against_received_substrate_balances_event() {
        let transfer_interface = get_default_substrate_transfer_interface();
        let ordered_args = vec![
            AccountId32::new([2; 32]).encode(),
            AccountId32::new([1; 32]).encode(),
            100u128.encode(),
            50u128.encode(),
        ];
        let scale_encoded_transfer_event = pallet_balances::Event::<MiniRuntime>::Transfer {
            from: AccountId32::new([2; 32]),
            to: AccountId32::new([1; 32]),
            amount: 100u128,
        }
        .encode();

        let res = transfer_interface.validate_arguments_against_received(
            &ordered_args,
            scale_encoded_transfer_event,
            Codec::Scale,
        );

        println!("{res:?}");
        assert!(res.is_ok());
    }

    #[test]
    fn test_transfer_interface() {
        let transfer_interface = get_default_substrate_transfer_interface();
        assert_eq!(
            transfer_interface.args_names,
            vec![
                (b"from".to_vec(), true),
                (b"to".to_vec(), true),
                (b"amount".to_vec(), true),
                (b"insurance".to_vec(), false),
            ]
        );
        assert_eq!(
            transfer_interface.expected_optimistic_descriptor,
            b"Transfer:Struct(from:Account32,to:Account32,amount:Value128)".to_vec()
        );
    }
}
