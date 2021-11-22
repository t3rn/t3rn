#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

use crate::side_effects::confirm::parser::VendorSideEffectsParser;
use crate::side_effects::protocol::SideEffectProtocol;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait SideEffectConfirmationProtocol: SideEffectProtocol {
    // Use CONFIRMING_EVENTS now to
    //  1. Decode each event following it's Vendor decoding implementation (substrate events vs eth events)
    //  2. Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args" step before the SideEffect was emitted for execution
    //  3. Check each argument of decoded "encoded_remote_events" against the values from STATE
    //  4. Return error that will potentially be a subject for a punishment of the executioner - up to the misbehaviour manager
    // confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm<VendorParser: VendorSideEffectsParser>(
        &self,
        encoded_remote_events: Vec<Vec<u8>>,
    ) -> Result<(), &'static str> {
        // 0. Check incoming args with protocol requirements
        assert!(encoded_remote_events.len() == Self::get_arguments_abi(self).len());

        // 1. Decode event as relying on Vendor-specific decoding/parsing
        let _decoded_events = encoded_remote_events
            .iter()
            .enumerate()
            .map(|(i, encoded_event)| {
                let expected_event_signature = Self::get_confirming_events(self)[i];
                VendorParser::parse_event(
                    Self::get_name(self),
                    encoded_event.clone(),
                    expected_event_signature,
                )
            });

        // ToDo: 2. 3. 4. missing
        Ok(())
    }
}
