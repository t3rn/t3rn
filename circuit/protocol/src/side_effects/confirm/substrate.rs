#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec;
use sp_std::vec::*;

use crate::side_effects::parser::VendorSideEffectsParser;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

// Parser would come to the SideEffects as a parameter that implements the parser of events best suited for each vendor:
// Substrate - probably based on scale let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
// Ethereum - probably based on events decode that uses a signature as a string like Transfer(address,address,value)
pub struct SubstrateSideEffectsParser {}

impl VendorSideEffectsParser for SubstrateSideEffectsParser {
    fn parse_event(
        name: &'static str,
        _event_encoded: Vec<u8>,
        // If we go with decoding events based on the pallet-inherited Event encoder we won't need the signature to decode from Substrate
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {
        let output_args = vec![];

        match name {
            "transfer:dirty" => {
                // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
                // let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
            }
            &_ => {}
        }

        Ok(output_args)
    }
}
