#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::*;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait VendorSideEffectsParser {
    fn parse_event<T: pallet_balances::Config + orml_tokens::Config>(
        name: &'static str,
        event_encoded: Vec<u8>,
        signature: &'static str,
    ) -> Result<Arguments, &'static str>;
}
