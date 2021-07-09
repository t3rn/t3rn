// Creating mock runtime here

use crate::Config;

use frame_support::{parameter_types, traits::Get, weights::Weight};

use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Convert, IdentityLookup},
    DispatchResult, Perbill,
};
use std::cell::RefCell;
use t3rn_primitives::{transfers::BalanceOf, EscrowTrait};

use sp_runtime::AccountId32;

use frame_support::pallet_prelude::*;
use versatile_wasm::{DispatchRuntimeCall, VersatileWasm};

use crate as pallet_runtime_gateway;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        // Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
        VersatileWasmVM: versatile_wasm::{Pallet, Call, Event<T>},
        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        EscrowGateway: pallet_runtime_gateway::{Pallet, Call, Storage, Event<T>},

        Flipper: flipper::{Pallet, Call},
        Weights: weights::{Pallet, Call},
    }
);

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

pub struct ExampleDispatchRuntimeCall;

impl DispatchRuntimeCall<Test> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        _module_name: &str,
        _fn_name: &str,
        _input: &[u8],
        _escrow_account: &<Test as frame_system::Config>::AccountId,
        _requested: &<Test as frame_system::Config>::AccountId,
        _callee: &<Test as frame_system::Config>::AccountId,
        _value: BalanceOf<Test>,
        _gas_meter: &mut versatile_wasm::gas::GasMeter<Test>,
    ) -> DispatchResult {
        Ok(())
        // match (module_name, fn_name) {
        //     ("Flipper", "flip") => Flipper::flip(Origin::signed(*escrow_account)),
        //     ("Weights", "store_value") => {
        //         let decoded_input: u32 = match Decode::decode(&mut _input.clone()) {
        //             Ok(dec) => dec,
        //             Err(_) => {
        //                 return Err(DispatchError::Other(
        //                     "Can't decode input for Weights::store_value. Expected u32.",
        //                 ));
        //             }
        //         };
        //         gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
        //             WeightsCall::store_value(decoded_input),
        //         )))?;
        //         // Alternatively use the call - call.dispatch((Origin::signed(*escrow_account))).map_err(|e| e.error)?;
        //         Weights::store_value(Origin::signed(*escrow_account), decoded_input)
        //     }
        //     ("Weights", "double") => {
        //         let decoded_input: u32 = match Decode::decode(&mut _input.clone()) {
        //             Ok(dec) => dec,
        //             Err(_) => {
        //                 return Err(DispatchError::Other(
        //                     "Can't decode input for Weights::store_value. Expected u32.",
        //                 ));
        //             }
        //         };
        //         gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(WeightsCall::double(
        //             decoded_input,
        //         ))))?;
        //         Weights::double(Origin::signed(*escrow_account), decoded_input)
        //     }
        //     ("Weights", "complex_calculations") => {
        //         let (decoded_x, decoded_y): (u32, u32) = match Decode::decode(&mut _input.clone()) {
        //             Ok(dec) => dec,
        //             Err(_) => {
        //                 return Err(DispatchError::Other(
        //                     "Can't decode input for Weights::store_value. Expected u32.",
        //                 ));
        //             }
        //         };
        //         gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
        //             WeightsCall::complex_calculations(decoded_x, decoded_y),
        //         )))?;
        //         Weights::complex_calculations(Origin::signed(*escrow_account), decoded_x, decoded_y)
        //     }
        //     (_, _) => Err(DispatchError::Other(
        //         "Call to unrecognized runtime function",
        //     )),
        // }
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

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const TransactionByteFee: u64 = 1;
}

use frame_support::weights::IdentityFee;
impl pallet_transaction_payment::Config for Test {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

/** Balances -- end **/
impl pallet_timestamp::Config for Test {
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

impl weights::Config for Test {}

impl flipper::Config for Test {}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Version = ();
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl pallet_sudo::Config for Test {
    type Event = Event;
    type Call = Call;
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub MyVVMSchedule: versatile_wasm::Schedule = <versatile_wasm::simple_schedule_v2::Schedule>::default();
}

impl VersatileWasm for Test {
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type Event = Event;
    type Call = Call;
    type Randomness = Randomness;
    type CallStack = [versatile_wasm::call_stack::Frame<Self>; 31];
    type WeightPrice = Self;
    type Schedule = MyVVMSchedule;
}

impl Config for Test {
    type Event = Event;
}

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
    pub fn build(self, escrow_account: AccountId32) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();
        pallet_sudo::GenesisConfig::<Test> {
            key: escrow_account,
        }
        .assimilate_storage(&mut t)
        .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

// pub struct ExtBuilder {
//     existential_deposit: u64,
// }
// impl Default for ExtBuilder {
//     fn default() -> Self {
//         Self {
//             existential_deposit: 1,
//         }
//     }
// }
// impl ExtBuilder {
//     pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
//         self.existential_deposit = existential_deposit;
//         self
//     }
//     pub fn set_associated_consts(&self) {
//         EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
//     }
//     pub fn build(self) -> sp_io::TestExternalities {
//         self.set_associated_consts();
//         let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
//         pallet_balances::GenesisConfig::<Test> {
//             balances: vec![],
//         }.assimilate_storage(&mut t).unwrap();
//         let mut ext = sp_io::TestExternalities::new(t);
//         ext.execute_with(|| System::set_block_number(1));
//         ext
//     }
// }

pub fn new_test_ext_builder(deposit: u64, escrow_account: AccountId32) -> sp_io::TestExternalities {
    ExtBuilder::default()
        .existential_deposit(deposit)
        .build(escrow_account)
}
