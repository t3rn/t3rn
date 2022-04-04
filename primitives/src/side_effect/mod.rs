#![cfg_attr(not(feature = "std"), no_std)]
use crate::{
    abi::extract_property_names_from_signature_as_bytes,
    protocol::SideEffectProtocol,
    volatile::{LocalState, Volatile},
};
use codec::{Decode, Encode};
use parser::VendorSideEffectsParser;
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::vec::Vec;

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub mod interface;
pub mod parser;

type Bytes = Vec<u8>;
pub type SideEffectId<T> = <T as frame_system::Config>::Hash;
pub type TargetId = [u8; 4];
pub type EventSignature = Vec<u8>;
pub type SideEffectName = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Encode, Default, Decode, Debug, TypeInfo)]
pub struct SideEffect<AccountId, BlockNumber, BalanceOf> {
    pub target: TargetId,
    pub prize: BalanceOf,
    pub ordered_at: BlockNumber,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executioner: Option<AccountId>,
}

impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > SideEffect<AccountId, BlockNumber, BalanceOf>
{
    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }
}

pub trait SideEffectConfirmationProtocol: SideEffectProtocol {
    // Use CONFIRMING_EVENTS now to confirm that the content received events follows the protocol
    //  1. Decode each event following it's Vendor decoding implementation (substrate events vs eth events)
    //  2. Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args" step before the SideEffect was emitted for execution
    //  3. Check each argument of decoded "encoded_remote_events" against the values from STATE
    //  4. Return error that will potentially be a subject for a punishment of the executioner - up to the misbehaviour manager
    // confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm<T: frame_system::Config, VendorParser: VendorSideEffectsParser>(
        &self,
        encoded_remote_events: Vec<Bytes>,
        local_state: &mut LocalState,
        side_effect_id: Option<Bytes>,
    ) -> Result<(), &'static str> {
        // 0. Check incoming args with protocol requirements
        assert!(encoded_remote_events.len() == Self::get_confirming_events(self).len());
        // 1. Decode event as relying on Vendor-specific decoding/parsing

        for (i, encoded_event) in encoded_remote_events.iter().enumerate() {
            let expected_event_signature = &Self::get_confirming_events(self)[i];
            let decoded_events = VendorParser::parse_event::<T>(
                &Self::get_id(self),
                encoded_event.clone(),
                &expected_event_signature.to_vec(),
            )?;
            // 2.  Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args"
            // ToDo: It will work for transfer for now without analyzing the signature
            //  since the args names are the same as expected confirmation events params.
            //  the signature, but here there should be a lookup now for
            //  arg_names = get_arg_names_from_signature(self.get_confirmation_event()[0])
            let (_, property_names) =
                extract_property_names_from_signature_as_bytes(expected_event_signature.encode())?;

            // let mapper = side_effect_protocol.get_arguments_2_state_mapper();
            // assert!(mapper.len() == decoded_events.len());
            for (j, property_name) in property_names.iter().enumerate() {
                //  3. Check each argument of decoded "encoded_remote_events" against the values from State

                let key = match side_effect_id.clone() {
                    None => property_name.clone(),
                    Some(ref prefix) => LocalState::stick_key_with_prefix(
                        property_name.clone().encode(),
                        prefix.to_vec(),
                    ),
                };

                if !local_state.cmp(key, decoded_events[j].clone()) {
                    return Err("Confirmation Failed - received event arguments differ from expected by state");
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<Bytes>,
    pub output: Option<Bytes>,
    pub encoded_effect: Bytes,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BlockNumber, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sp_runtime::testing::H256;

    type BlockNumber = u64;
    type BalanceOf = u64;
    type AccountId = u64;
    type Hashing = sp_runtime::traits::BlakeTwo256;

    #[test]
    fn successfully_creates_empty_side_effect() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            empty_side_effect,
            SideEffect {
                target: [0, 0, 0, 0],
                prize: 0,
                ordered_at: 0,
                encoded_action: vec![],
                encoded_args: vec![],
                signature: vec![],
                enforce_executioner: None,
            }
        );
    }

    #[test]
    fn successfully_generates_id_for_side_empty_effect() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(
            empty_side_effect.generate_id::<Hashing>(),
            H256::from_slice(&hex!(
                "19ea4a516c66775ea1f648d71f6b8fa227e8b0c1a0c9203f82c33b89c4e759b5"
            ))
        );
    }

    #[test]
    fn successfully_defaults_side_effect_to_an_empty_one() {
        let empty_side_effect = SideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: [0, 0, 0, 0],
            prize: 0,
            ordered_at: 0,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            enforce_executioner: None,
        };

        assert_eq!(empty_side_effect, SideEffect::default(),);
    }
}
