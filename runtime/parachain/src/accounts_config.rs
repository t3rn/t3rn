use crate::*;
use frame_support::parameter_types;
use sp_core::crypto::AccountId32;

parameter_types! {
    // TODO: update me to be better
    pub EscrowAccount: AccountId32 = AccountId32::new([55_u8; 32]);
}

impl pallet_account_manager::Config for Runtime {
    type Clock = t3rn_primitives::clock::ClockMock<Self>;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type Time = Timestamp;
    type WeightInfo = ();
}
