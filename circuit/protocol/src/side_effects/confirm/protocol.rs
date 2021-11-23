#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use crate::side_effects::confirm::parser::VendorSideEffectsParser;
use crate::side_effects::protocol::SideEffectProtocol;
use crate::side_effects::volatile::LocalState;
use crate::side_effects::volatile::Volatile;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait SideEffectConfirmationProtocol: SideEffectProtocol {
    // Use CONFIRMING_EVENTS now to confirm that the content received events follows the protocol
    //  1. Decode each event following it's Vendor decoding implementation (substrate events vs eth events)
    //  2. Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args" step before the SideEffect was emitted for execution
    //  3. Check each argument of decoded "encoded_remote_events" against the values from STATE
    //  4. Return error that will potentially be a subject for a punishment of the executioner - up to the misbehaviour manager
    // confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm<T: pallet_balances::Config, VendorParser: VendorSideEffectsParser>(
        &self,
        encoded_remote_events: Vec<Vec<u8>>,
        local_state: &mut LocalState,
    ) -> Result<(), &'static str> {
        // 0. Check incoming args with protocol requirements
        assert!(encoded_remote_events.len() == Self::get_confirming_events(self).len());
        // 1. Decode event as relying on Vendor-specific decoding/parsing

        for (i, encoded_event) in encoded_remote_events.iter().enumerate() {
            let expected_event_signature = Self::get_confirming_events(self)[i];
            let decoded_events = VendorParser::parse_event::<T>(
                Self::get_name(self),
                encoded_event.clone(),
                expected_event_signature,
            )?;
            // 2.  Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args"
            // ToDo: It will work for transfer for now without analyzing the signature
            //  since the args names are the same as expected confirmation events params.
            //  the signature, but here there should be a lookup now for
            //  arg_names = get_arg_names_from_signature(self.get_confirmation_event()[0])
            let mapper = self.get_arguments_2_state_mapper();
            assert!(mapper.len() == decoded_events.len());
            for (j, arg_name) in mapper.iter().enumerate() {
                //  3. Check each argument of decoded "encoded_remote_events" against the values from State
                if !local_state.cmp(arg_name, decoded_events[j].clone()) {
                    return Err("Confirmation Failed - received event arguments differ from expected by state");
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::side_effects::confirm::substrate::tests::Test;
    use crate::side_effects::confirm::substrate::SubstrateSideEffectsParser;
    use crate::side_effects::protocol::TransferSideEffectProtocol;
    use codec::Encode;

    use hex_literal::hex;

    #[test]
    fn successfully_confirms_transfer_side_effect() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer(
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1u64,
        )
        .encode();
        let _encoded_transfer_args_input = vec![
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1u64.encode(),
        ];

        let mut local_state = LocalState::new();
        // Preload state by with the arguments and their names first
        local_state
            .insert(
                "from",
                hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            )
            .unwrap();
        local_state
            .insert(
                "to",
                hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            )
            .unwrap();
        local_state
            .insert("value", hex!("0100000000000000").into())
            .unwrap();

        let transfer_protocol = TransferSideEffectProtocol {};
        let res = transfer_protocol.confirm::<Test, SubstrateSideEffectsParser>(
            vec![encoded_balance_transfer_event],
            &mut local_state,
        );
        assert_eq!(res, Ok(()));
    }
}
