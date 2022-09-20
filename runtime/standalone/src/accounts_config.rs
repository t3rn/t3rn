use super::*;
use frame_support::parameter_types;
use sp_core::crypto::AccountId32;

parameter_types! {
    // TODO: update me to be better
    pub EscrowAccount: AccountId32 = AccountId32::new([55_u8; 32]);
}

impl pallet_account_manager::Config for Runtime {
    type Clock = Clock;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Time = Timestamp;
    type WeightInfo = ();
}

pallet_account_manager::setup_currency_adapter!();
