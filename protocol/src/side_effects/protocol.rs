#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::side_effects::confirm::protocol::SideEffectConfirmationProtocol;
pub use crate::side_effects::standards::{
    CallSideEffectProtocol, GetDataSideEffectProtocol, TransferSideEffectProtocol,
};

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
        // let known_side_effects = local_state.get(b"SIDE_EFFECTS".to_vec())?;
        // match known_side_effects.find(|x| key_prefix == x) { Some(_) return Err("known already" }
        // local_state.insert(b"SIDE_EFFECTS".to_vec(), arg.to_vec())?;
        for (i, arg) in encoded_args.iter().enumerate() {
            let insert_res = match key_prefix {
                None => local_state.insert(mapper[i], arg.to_vec()),
                Some(ref prefix) => {
                    let key =
                        LocalState::stick_key_with_prefix(mapper[i].encode(), prefix.to_vec());
                    local_state.insert(key, arg.to_vec())
                }
            };
            match insert_res {
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
        gateway_abi: &GatewayABIConfig,
        local_state: &mut LocalState,
        id: Option<Bytes>,
    ) -> Result<(), &'static str> {
        // Args number must match with the args number in the protocol
        // assert!(Self::get_arguments_abi(self).len() == args.len());
        // ToDo: Extract to a separate function
        // Validate that the input arguments set by a user follow the protocol for get_storage side effect
        // Evaluate each input argument against strictly defined type for that gateway.

        let mut validated_args: Arguments = vec![];
        // ToDo: Dig now to self.gateway_abi and recover the length of values, addresses to check
        for (i, type_n) in Self::get_arguments_abi(self).iter().enumerate() {
            let arg = match args.get(i) {
                Some(bytes) => Ok(bytes.clone()),
                None => match type_n {
                    Type::OptionalInsurance => Ok(vec![]),
                    _ => Err("Side Effect Validation - Incorrect arguments length"),
                },
            }?;
            let res = type_n.clone().eval_abi(arg.clone(), gateway_abi)?;
            validated_args.push(res);
        }

        self.populate_state(validated_args, local_state, id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::side_effects::test_utils::*;

    use sp_std::vec;

    #[test]
    fn successfully_populates_state_for_transfer_arguments() {
        let encoded_transfer_args_input = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A),
        ]);

        let mut local_state = LocalState::new();
        let transfer_protocol = TransferSideEffectProtocol {};
        let res =
            transfer_protocol.populate_state(encoded_transfer_args_input, &mut local_state, None);

        assert_eq!(res, Ok(()));

        assert_populated_state(
            local_state,
            produce_test_args(vec![
                (Type::Address(32), ArgVariant::A),
                (Type::Address(32), ArgVariant::B),
                (Type::Uint(64), ArgVariant::A),
            ]),
            vec![FROM_2XX_32B_HASH, TO_2XX_32B_HASH, VALUE_2XX_32B_HASH],
        );
    }

    #[test]
    fn successfully_populates_state_for_transfer_arguments_with_prefix() {
        let encoded_transfer_args_with_empty_insurance = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A),
        ]);
        let encoded_transfer_args_no_insurance = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
        ]);

        let valid_transfer_side_effect_no_signature =
            produce_test_side_effect(*b"tran", encoded_transfer_args_no_insurance.clone(), vec![]);

        let valid_side_effect_id = valid_transfer_side_effect_no_signature.generate_id::<Hashing>();

        let mut local_state = LocalState::new();
        let transfer_protocol = TransferSideEffectProtocol {};

        let res = transfer_protocol.populate_state(
            encoded_transfer_args_with_empty_insurance,
            &mut local_state,
            Some(valid_side_effect_id.as_ref().to_vec()),
        );

        assert_eq!(res, Ok(()));

        assert_populated_state(
            local_state,
            encoded_transfer_args_no_insurance,
            vec![
                NO_INSURANCE_FROM_PLUS_PREFIX_2XX_32B_HASH,
                NO_INSURANCE_TO_PLUS_PREFIX_2XX_32B_HASH,
                NO_INSURANCE_VALUE_PLUS_PREFIX_2XX_32B_HASH,
            ],
        );
    }

    #[test]
    fn successfully_validates_transfer_arguments_with_prefix_no_insurance() {
        let encoded_transfer_args_no_insurance = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
        ]);

        let valid_transfer_side_effect_no_signature =
            produce_test_side_effect(*b"tran", encoded_transfer_args_no_insurance.clone(), vec![]);

        let valid_side_effect_id = valid_transfer_side_effect_no_signature.generate_id::<Hashing>();

        let mut local_state = LocalState::new();
        let transfer_protocol = TransferSideEffectProtocol {};

        let abi: GatewayABIConfig = Default::default();
        let res = transfer_protocol.validate_args(
            encoded_transfer_args_no_insurance,
            &abi,
            &mut local_state,
            Some(valid_side_effect_id.as_ref().to_vec()),
        );

        assert_eq!(res, Ok(()));

        let encoded_transfer_args_with_empty_insurance = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::Bytes(0), ArgVariant::A),
        ]);

        assert_populated_state(
            local_state,
            encoded_transfer_args_with_empty_insurance,
            vec![
                NO_INSURANCE_FROM_PLUS_PREFIX_2XX_32B_HASH,
                NO_INSURANCE_TO_PLUS_PREFIX_2XX_32B_HASH,
                NO_INSURANCE_VALUE_PLUS_PREFIX_2XX_32B_HASH,
                NO_INSURANCE_INSURANCE_PLUS_PREFIX_2XX_32B_HASH,
            ],
        );
    }

    #[test]
    fn successfully_validates_transfer_arguments_with_prefix_with_insurance() {
        let encoded_transfer_args_input = produce_test_args(vec![
            (Type::Address(32), ArgVariant::A),
            (Type::Address(32), ArgVariant::B),
            (Type::Uint(64), ArgVariant::A),
            (Type::OptionalInsurance, ArgVariant::A),
        ]);

        let valid_transfer_side_effect_no_signature =
            produce_test_side_effect(*b"tran", encoded_transfer_args_input.clone(), vec![]);

        let valid_side_effect_id = valid_transfer_side_effect_no_signature.generate_id::<Hashing>();
        let mut local_state = LocalState::new();
        let transfer_protocol = TransferSideEffectProtocol {};
        let abi: GatewayABIConfig = Default::default();
        let res = transfer_protocol.validate_args(
            encoded_transfer_args_input.clone(),
            &abi,
            &mut local_state,
            Some(valid_side_effect_id.as_ref().to_vec()),
        );
        assert_eq!(res, Ok(()));

        // Make sure there's no error in test_utils between assert_populated_state vs assert_populated_state_auto_key_derive
        assert_populated_state(
            local_state.clone(),
            encoded_transfer_args_input.clone(),
            vec![
                FROM_PLUS_PREFIX_2XX_32B_HASH,
                TO_PLUS_PREFIX_2XX_32B_HASH,
                VALUE_PLUS_PREFIX_2XX_32B_HASH,
                INSURANCE_PLUS_PREFIX_2XX_32B_HASH,
            ],
        );

        assert_populated_state_auto_key_derive(
            local_state,
            encoded_transfer_args_input,
            Box::new(transfer_protocol),
            valid_side_effect_id.as_ref().to_vec(),
        );
    }

    #[test]
    fn successfully_validates_transfer_arguments_with_prefix_with_insurance_for_more_variants() {
        let encoded_transfer_args_input = produce_test_args(vec![
            (Type::Address(32), ArgVariant::B),
            (Type::Address(32), ArgVariant::C),
            (Type::Uint(64), ArgVariant::C),
            (Type::OptionalInsurance, ArgVariant::C),
        ]);

        let valid_transfer_side_effect =
            produce_test_side_effect(*b"tran", encoded_transfer_args_input.clone(), vec![]);

        let mut local_state = LocalState::new();
        assert_correct_validation_and_populated_state(
            &mut local_state,
            valid_transfer_side_effect,
            encoded_transfer_args_input,
            Box::new(TransferSideEffectProtocol {}),
        )
    }

    #[test]
    fn successfully_validates_get_data_arguments_with_prefix() {
        let encoded_get_data_args_input = produce_test_args(vec![(Type::Bytes(32), ArgVariant::A)]);

        let valid_get_data_side_effect =
            produce_test_side_effect(*b"data", encoded_get_data_args_input.clone(), vec![]);

        let mut local_state = LocalState::new();
        assert_correct_validation_and_populated_state(
            &mut local_state,
            valid_get_data_side_effect,
            encoded_get_data_args_input,
            Box::new(GetDataSideEffectProtocol {}),
        )
    }
}
