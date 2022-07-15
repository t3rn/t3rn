use crate::{abi::Type, side_effect::EventSignature};
use sp_std::vec::*;

pub type Arguments = Vec<Vec<u8>>;
pub type Source = Vec<u8>;

pub trait VendorSideEffectsParser {
    fn parse_event<T: frame_system::Config>(
        name: &[u8; 4],
        event_encoded: Vec<u8>,
        signature: &EventSignature,
        value_abi_unsigned_type: Type,
    ) -> Result<(Arguments, Source), &'static str>;
}
