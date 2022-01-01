#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use crate::side_effects::confirm::parser::VendorSideEffectsParser;
use crate::side_effects::protocol::SideEffectProtocol;

pub use t3rn_primitives::{
    volatile::{LocalState, Volatile},
    GatewayVendor,
};

pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;
pub type Bytes = Vec<u8>;

pub trait SideEffectConfirmationProtocol: SideEffectProtocol {
    // Use CONFIRMING_EVENTS now to confirm that the content received events follows the protocol
    //  1. Decode each event following it's Vendor decoding implementation (substrate events vs eth events)
    //  2. Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args" step before the SideEffect was emitted for execution
    //  3. Check each argument of decoded "encoded_remote_events" against the values from STATE
    //  4. Return error that will potentially be a subject for a punishment of the executioner - up to the misbehaviour manager
    // confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm<T: pallet_balances::Config, VendorParser: VendorSideEffectsParser>(
        &self,
        encoded_remote_events: Vec<Bytes>,
        local_state: &mut LocalState,
        _side_effect_id: Option<Bytes>,
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
            // assert!(mapper.len() == decoded_events.len());
            for (j, arg_name) in mapper.iter().enumerate() {
                // LocalState::stick_key_with_prefix(arg_name.encode(), side_effect_id);
                // if u encode the confimration arg names no collision with insurance anymore
                //  3. Check each argument of decoded "encoded_remote_events" against the values from State
                if !local_state.cmp(arg_name, decoded_events[j].clone()) {
                    return Err("Confirmation Failed - received event arguments differ from expected by state");
                }
            }
        }
        Ok(())
    }
}

pub fn confirmation_plug<T: pallet_balances::Config, VendorParser: VendorSideEffectsParser>(
    side_effect_protocol: Box<dyn SideEffectProtocol>,
    encoded_remote_events: Vec<Bytes>,
    local_state: &mut LocalState,
    _side_effect_id: Option<Bytes>,
) -> Result<(), &'static str> {
    // 0. Check incoming args with protocol requirements
    assert!(encoded_remote_events.len() == side_effect_protocol.get_confirming_events().len());
    // 1. Decode event as relying on Vendor-specific decoding/parsing

    for (i, encoded_event) in encoded_remote_events.iter().enumerate() {
        let expected_event_signature = side_effect_protocol.get_confirming_events()[i];
        let decoded_events = VendorParser::parse_event::<T>(
            side_effect_protocol.get_name(),
            encoded_event.clone(),
            expected_event_signature,
        )?;
        // 2.  Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args"
        // ToDo: It will work for transfer for now without analyzing the signature
        //  since the args names are the same as expected confirmation events params.
        //  the signature, but here there should be a lookup now for
        //  arg_names = get_arg_names_from_signature(self.get_confirmation_event()[0])
        let mapper = side_effect_protocol.get_arguments_2_state_mapper();
        assert!(mapper.len() == decoded_events.len());
        for (j, arg_name) in mapper.iter().enumerate() {
            //  3. Check each argument of decoded "encoded_remote_events" against the values from State
            if !local_state.cmp(arg_name, decoded_events[j].clone()) {
                return Err(
                    "Confirmation Failed - received event arguments differ from expected by state",
                );
            }
        }
    }
    Ok(())
}

pub fn confirm_with_vendor_by_action_id<
    T: pallet_balances::Config,
    SubstrateParser: VendorSideEffectsParser,
    EthParser: VendorSideEffectsParser,
>(
    gateway_vendor: GatewayVendor,
    encoded_action: Bytes,
    encoded_effect: Bytes,
    mut state_copy: &mut LocalState,
    side_effect_id: Option<Bytes>,
) -> Result<(), &'static str> {
    let mut action_id_4b: [u8; 4] = [0, 0, 0, 0];
    action_id_4b.copy_from_slice(&encoded_action[0..4]);
    let side_effect_protocol =
        crate::side_effects::standards::select_side_effect_by_id(action_id_4b)?;

    match gateway_vendor {
        GatewayVendor::Substrate => confirmation_plug::<T, SubstrateParser>(
            side_effect_protocol,
            vec![encoded_effect.clone()],
            &mut state_copy,
            side_effect_id,
        ),
        GatewayVendor::Ethereum => confirmation_plug::<T, EthParser>(
            side_effect_protocol,
            vec![encoded_effect.clone()],
            &mut state_copy,
            side_effect_id,
        ),
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
    fn successfully_confirms_transfer_side_effect_no_prefix_no_insurance() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer(
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1u64,
        )
        .encode();

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
            vec![encoded_balance_transfer_event.clone()],
            &mut local_state,
            None,
        );

        assert_eq!(res, Ok(()));

        let res_vendor = confirm_with_vendor_by_action_id::<
            Test,
            SubstrateSideEffectsParser,
            SubstrateSideEffectsParser,
        >(
            GatewayVendor::Substrate,
            b"tran".to_vec(),
            encoded_balance_transfer_event,
            &mut local_state,
            None,
        );

        assert_eq!(res_vendor, Ok(()));
    }

    #[test]
    fn errors_to_confirm_transfer_side_effect_with_wrong_receiver_no_prefix() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer(
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0505050505050505050505050505050505050505050505050505050505050505").into(),
            1u64,
        )
        .encode();

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
            vec![encoded_balance_transfer_event.clone()],
            &mut local_state,
            None,
        );
        assert_eq!(
            res,
            Err("Confirmation Failed - received event arguments differ from expected by state")
        );

        let res_vendor = confirm_with_vendor_by_action_id::<
            Test,
            SubstrateSideEffectsParser,
            SubstrateSideEffectsParser,
        >(
            GatewayVendor::Substrate,
            b"tran".to_vec(),
            encoded_balance_transfer_event,
            &mut local_state,
            None,
        );

        assert_eq!(
            res_vendor,
            Err("Confirmation Failed - received event arguments differ from expected by state")
        );
    }
}
