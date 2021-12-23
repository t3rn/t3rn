#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::side_effects::confirm::protocol::SideEffectConfirmationProtocol;
pub use crate::side_effects::standards::{CallSideEffectProtocol, TransferSideEffectProtocol};

use codec::Encode;
use sp_std::vec::*;
use t3rn_primitives::abi::{GatewayABIConfig, Type};
use t3rn_primitives::volatile::{LocalState, Volatile};

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait SideEffectProtocol {
    fn get_name(&self) -> &'static str;
    fn get_id(&self) -> [u8; 4];
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
        key_prefix: Option<Bytes>,
    ) -> Result<(), &'static str> {
        let mapper = self.get_arguments_2_state_mapper();
        assert!(mapper.len() == encoded_args.len());

        // ToDo: Mark the key prefix in side effects entries
        // let known_side_effects = local_state.get(b"SIDE_EFFECTS".to_vec())?;
        // match known_side_effects.find(|x| key_prefix == x) { Some(_) return Err("known already" }
        // local_state.insert(b"SIDE_EFFECTS".to_vec(), arg.to_vec())?;
        for (i, arg) in encoded_args.iter().enumerate() {
            let key = LocalState::stick_key_with_prefix(mapper[i].encode(), key_prefix.clone());
            match local_state.insert(key, arg.to_vec()) {
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
        id: Option<Bytes>,
    ) -> Result<(), &'static str> {
        // Args number must match with the args number in the protocol
        // assert!(Self::get_arguments_abi(self).len() == args.len());
        // ToDo: Extract to a separate function
        // Validate that the input arguments set by a user follow the protocol for get_storage side effect
        // Evaluate each input argument against strictly defined type for that gateway.
        // ToDo: Dig now to self.gateway_abi and recover the length of values, addresses to check
        for (i, type_n) in Self::get_arguments_abi(self).iter().enumerate() {
            let arg = match args.get(i) {
                Some(bytes) => Ok(bytes.clone()),
                None => match type_n {
                    Type::OptionalInsurance => Ok(vec![]),
                    _ => Err("Side Effect Validation - Incorrect arguments length"),
                },
            }?;

            type_n.clone().eval(arg.clone())?;
        }
        self.populate_state(args, local_state, id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use codec::Encode;
    use hex_literal::hex;
    use sp_std::vec;
    use t3rn_primitives::volatile::{FROM_2XX_32B_HASH, TO_2XX_32B_HASH, VALUE_2XX_32B_HASH};

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
        let res =
            transfer_protocol.populate_state(encoded_transfer_args_input, &mut local_state, None);

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
