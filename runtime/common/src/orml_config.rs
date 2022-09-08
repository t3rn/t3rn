use super::{Amount, Balance, Event, Runtime};

use frame_support::traits::Nothing;

// ORML Tokens
use orml_traits::parameter_type_with_key;
pub type CurrencyId = u32;
parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        Default::default()
    };
}

impl orml_tokens::Config for Runtime {
    type Amount = Amount;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type DustRemovalWhitelist = Nothing;
    type Event = Event;
    type ExistentialDeposits = ExistentialDeposits;
    type MaxLocks = ();
    type OnDust = ();
    type WeightInfo = ();
}
