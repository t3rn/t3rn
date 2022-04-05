use crate::{
    abi::{GatewayABIConfig, Type},
    side_effect::{EventSignature, SideEffectConfirmationProtocol, SideEffectName},
    volatile::{LocalState, Volatile},
    Bytes,
};
use codec::Encode;
use sp_std::{vec, vec::*};

type Arguments = Vec<Bytes>;

pub trait SideEffectProtocol {
    fn get_id(&self) -> [u8; 4];
    fn get_name(&self) -> SideEffectName;
    fn get_arguments_abi(&self) -> Vec<Type>;
    fn get_arguments_2_state_mapper(&self) -> Vec<EventSignature>;
    fn get_confirming_events(&self) -> Vec<EventSignature>;
    fn get_escrowed_events(&self) -> Vec<EventSignature> {
        unimplemented!()
    }
    fn get_reversible_commit(&self) -> Vec<EventSignature> {
        unimplemented!()
    }
    fn get_reversible_revert(&self) -> Vec<EventSignature> {
        unimplemented!()
    }

    fn populate_state(
        &self,
        encoded_args: Arguments,
        local_state: &mut LocalState,
        key_prefix: Option<Bytes>,
    ) -> Result<(), &'static str> {
        let mapper = &self.get_arguments_2_state_mapper();
        assert_eq!(mapper.len(), encoded_args.len());
        // let known_side_effects = local_state.get(b"SIDE_EFFECTS".to_vec())?;
        // match known_side_effects.find(|x| key_prefix == x) { Some(_) return Err("known already" }
        // local_state.insert(b"SIDE_EFFECTS".to_vec(), arg.to_vec())?;
        for (i, arg) in encoded_args.iter().enumerate() {
            let insert_res = match key_prefix {
                None => local_state.insert(&mapper[i], arg.to_vec()),
                Some(ref prefix) => {
                    let key =
                        LocalState::stick_key_with_prefix(mapper[i].encode(), prefix.to_vec());
                    local_state.insert(key, arg.to_vec())
                },
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
impl SideEffectConfirmationProtocol for dyn SideEffectProtocol {}

// Tests live in protocol
