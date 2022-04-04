#![cfg_attr(not(feature = "std"), no_std)]

use crate::side_effect::EventSignature;
use frame_support::traits::fungible::{Inspect, Mutate};
use orml_traits::MultiCurrency;
use sp_std::vec::*;

pub type Arguments = Vec<Vec<u8>>;

pub trait VendorSideEffectsParser {
    fn parse_event<
        T: frame_system::Config,
        Balances: Inspect<T::AccountId> + Mutate<T::AccountId>,
        Tokens: MultiCurrency<T::AccountId>,
    >(
        name: &[u8; 4],
        event_encoded: Vec<u8>,
        signature: &EventSignature,
    ) -> Result<Arguments, &'static str>;
}
