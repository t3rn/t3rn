#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::side_effects::confirm::protocol::SideEffectConfirmationProtocol;
pub use crate::side_effects::volatile::{LocalState, Volatile};
use codec::{Decode, Encode};
use sp_std::vec::*;
use t3rn_primitives::abi::{GatewayABIConfig, Type};

use sp_runtime::RuntimeDebug;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

/// The main idea would be to give a possibility of define the side effects dynamically
/// We'd have the "standard" side effects in the codebase, but for the sake of extensions,
/// the side effects should be made serialized, stored and pre-loaded before the exec pallet starts.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct TransferSideEffectProtocol {}

impl SideEffectProtocol for TransferSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "transfer:dirty"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: from
            Type::DynamicAddress, // argument_1: to
            Type::Value,          // argument_2: value
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "to", "value"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["Transfer(from,to,value)"]
    }
}

impl SideEffectConfirmationProtocol for TransferSideEffectProtocol {}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallSideEffectProtocol {}

impl SideEffectProtocol for CallSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "call:dirty"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: from
            Type::DynamicAddress, // argument_1: to
            Type::Value,          // argument_2: value
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "to", "value"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["Call(from,to,value)"]
    }
}

impl SideEffectConfirmationProtocol for CallSideEffectProtocol {}

pub trait SideEffectProtocol {
    fn get_name(&self) -> &'static str;
    fn get_arguments_abi(&self) -> Vec<Type>;
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str>;
    fn get_confirming_events(&self) -> Vec<&'static str>;
    fn get_escrowed_events(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_exec(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_commit(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        unimplemented!()
    }

    fn populate_state(
        &self,
        encoded_args: Arguments,
        local_state: &mut LocalState,
    ) -> Result<(), &'static str> {
        let mapper = self.get_arguments_2_state_mapper();
        assert!(mapper.len() == encoded_args.len());
        for (i, arg) in encoded_args.iter().enumerate() {
            let arg_name = mapper[i];
            match local_state.insert(arg_name, arg.to_vec()) {
                Ok((_state_key, _state_val)) => continue,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    // For now just assume that State can only be recreated from args? where arg index (usize) will be translated to the arguments name and therefore could be re-used in created expectations in the signature for confirming Events
    fn validate_args(
        &self,
        args: Arguments,
        _gateway_abi: GatewayABIConfig,
        local_state: &mut LocalState,
    ) -> Result<(), &'static str> {
        // Args number must match with the args number in the protocol
        assert!(Self::get_arguments_abi(self).len() == args.len());

        // ToDo: Extract to a separate function
        // Validate that the input arguments set by a user follow the protocol for get_storage side effect
        // Evaluate each input argument against strictly defined type for that gateway.
        // ToDo: Dig now to self.gateway_abi and recover the length of values, addresses to check
        for (i, arg) in args.iter().enumerate() {
            let type_n = &Self::get_arguments_abi(self)[i];
            type_n.eval(arg.clone())?;
        }
        self.populate_state(args, local_state);

        // ToDo: Maybe return a signature assuming it isn't created by a user?
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::side_effects::volatile::tests::{
        FROM_2XX_32B_HASH, TO_2XX_32B_HASH, VALUE_2XX_32B_HASH,
    };
    use hex_literal::hex;

    #[test]
    fn successfully_populates_state_for_transfer_arguments() {
        let _expected_transfer_arg_names_input = vec!["from", "to", "value"];
        let encoded_transfer_args_input = vec![
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1u64.encode(),
        ];
        let mut local_state = LocalState::new();
        let transfer_protocol = TransferSideEffectProtocol {};
        let res = transfer_protocol.populate_state(encoded_transfer_args_input, &mut local_state);

        assert_eq!(res, Ok(()));

        assert_eq!(
            local_state.state.get(&FROM_2XX_32B_HASH),
            Some(&hex!("0909090909090909090909090909090909090909090909090909090909090909").into())
        );

        assert_eq!(
            local_state.state.get(&TO_2XX_32B_HASH),
            Some(&hex!("0606060606060606060606060606060606060606060606060606060606060606").into())
        );

        assert_eq!(
            local_state.state.get(&VALUE_2XX_32B_HASH),
            Some(&hex!("0100000000000000").into())
        );
    }
}
