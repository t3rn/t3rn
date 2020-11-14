// Creating mock runtime here

use crate::{Module, Trait};
use codec::Decode;
use frame_support::{
    impl_outer_dispatch, impl_outer_event, impl_outer_origin, parameter_types, traits::Get,
    weights::Weight,
};

use frame_system as system;
use gateway_escrow_engine::{transfers::BalanceOf, EscrowTrait};
use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Convert, IdentityLookup},
    DispatchError, DispatchResult, Perbill,
};
use std::cell::RefCell;
use sudo;
use versatile_wasm::{DispatchRuntimeCall, VersatileWasm};

mod escrow_gateway {
    // Re-export contents of the root. This basically
    // needs to give a name for the current crate.
    // This hack is required for `impl_outer_event!`.
    pub use super::super::*;
    pub use frame_support::impl_outer_event;
}

impl_outer_event! {
    pub enum MetaEvent for Test {
        system<T>,
        pallet_balances<T>,
        versatile_wasm<T>,
        sudo<T>,
        escrow_gateway<T>,
    }
}

impl_outer_origin! {
    pub enum Origin for Test {}
}

impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        sudo::Sudo,
        weights::Weights,
    }
}

pub type WeightsCall = weights::Call<Test>;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u64 = 16;
    pub const StorageSizeOffset: u32 = 8;
    pub const RentByteFee: u64 = 4;
    pub const RentDepositOffset: u64 = 10_000;
    pub const SurchargeReward: u64 = 150;
    pub const MaxDepth: u32 = 100;
    pub const MaxValueSize: u32 = 16_384;
}

pub type Flipper = flipper::Module<Test>;
pub type Weights = weights::Module<Test>;

pub struct ExampleDispatchRuntimeCall;

impl DispatchRuntimeCall<Test> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        _input: &[u8],
        escrow_account: &<Test as system::Trait>::AccountId,
        _requested: &<Test as system::Trait>::AccountId,
        _callee: &<Test as system::Trait>::AccountId,
        _value: BalanceOf<Test>,
        gas_meter: &mut versatile_wasm::gas::GasMeter<Test>,
    ) -> DispatchResult {
        match (module_name, fn_name) {
            ("Flipper", "flip") => Flipper::flip(Origin::signed(*escrow_account)),
            ("Weights", "store_value") => {
                let decoded_input: u32 = match Decode::decode(&mut _input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
                    WeightsCall::store_value(decoded_input),
                )))?;
                // Alternatively use the call - call.dispatch((Origin::signed(*escrow_account))).map_err(|e| e.error)?;
                Weights::store_value(Origin::signed(*escrow_account), decoded_input)
            }
            ("Weights", "double") => {
                let decoded_input: u32 = match Decode::decode(&mut _input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(WeightsCall::double(
                    decoded_input,
                ))))?;
                Weights::double(Origin::signed(*escrow_account), decoded_input)
            }
            ("Weights", "complex_calculations") => {
                let (decoded_x, decoded_y): (u32, u32) = match Decode::decode(&mut _input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
                    WeightsCall::complex_calculations(decoded_x, decoded_y),
                )))?;
                Weights::complex_calculations(Origin::signed(*escrow_account), decoded_x, decoded_y)
            }
            (_, _) => Err(DispatchError::Other(
                "Call to unrecognized runtime function",
            )),
        }
    }
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}

thread_local! {
    static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
    fn get() -> u64 {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
    }
}

impl pallet_balances::Trait for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = MetaEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const TransactionByteFee: u64 = 1;
}
use frame_support::weights::IdentityFee;
impl pallet_transaction_payment::Trait for Test {
    type Currency = pallet_balances::Module<Test>;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

/** Balances -- end **/

impl pallet_timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w
    }
}

impl weights::Trait for Test {}

impl flipper::Trait for Test {
    type Event = MetaEvent;
}

pub type Balances = pallet_balances::Module<Test>;
type Randomness = pallet_randomness_collective_flip::Module<Test>;
type System = system::Module<Test>;
type Timestamp = pallet_timestamp::Module<Test>;

impl system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Call = Call;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = MetaEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type AvailableBlockRatio = AvailableBlockRatio;
    type MaximumBlockLength = MaximumBlockLength;
    type Version = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type PalletInfo = ();
}

impl sudo::Trait for Test {
    type Event = MetaEvent;
    type Call = Call;
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

impl VersatileWasm for Test {
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type Event = MetaEvent;
    type Call = Call;
    type Randomness = Randomness;
}

impl Trait for Test {
    type Event = MetaEvent;
}

pub type Sudo = sudo::Module<Test>;

pub type EscrowGateway = Module<Test>;

pub struct ExtBuilder {
    existential_deposit: u64,
}
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
        }
    }
}
impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }
    pub fn build(self, escrow_account: u64) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();
        sudo::GenesisConfig::<Test> {
            key: escrow_account,
        }
        .assimilate_storage(&mut t)
        .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn new_test_ext_builder(deposit: u64, escrow_account: u64) -> sp_io::TestExternalities {
    ExtBuilder::default()
        .existential_deposit(deposit)
        .build(escrow_account)
}
