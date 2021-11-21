#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec;
use sp_std::vec::*;

use crate::side_effects::parser::VendorSideEffectsParser;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

struct EthereumSideEffectsParser {}

impl VendorSideEffectsParser for EthereumSideEffectsParser {
    fn parse_event(
        name: &'static str,
        _event_encoded: Vec<u8>,
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {
        let output_args = vec![];

        match name {
            "transfer:dirty" => {
                // Use the similar tricks that are currently used in eth_outbound that use
                //         let signature: &str =
                //             sp_std::str::from_utf8(&name[..]).map_err(|_| "`Can't decode argument to &str")?;
                //
                //         let event = EthAbiEvent {
                //             signature,
                //             inputs: expected_arg_types_eth.as_slice(),
                //             anonymous: false,
                //         };
                //
                //         let args_decoded = event
                //             .decode(self.topics.clone(), self.data.to_vec())
                //             .map_err(|_| "Error decoding native eth event using ethabi-decoder")?;
            }
            &_ => {}
        }

        Ok(output_args)
    }
}
