use crate::pallet as pallet_staking;
use frame_support::{
    parameter_types,
    traits::{GenesisBuild, OnFinalize, OnInitialize},
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill, Percent,
};
use t3rn_primitives::{
    common::{Range, BLOCKS_PER_HOUR, BLOCKS_PER_DAY},
    monetary::T3RN,
};

pub(crate) fn last_event() -> Event {
    System::events().pop().expect("event expected").event
}

pub(crate) fn last_n_events(n: usize) -> Vec<pallet_staking::Event<Test>> {
    let events = System::events();
    let len = events.len();
    if len > 0 {
        events[len - n..]
            .into_iter()
            .map(|r| r.event.clone())
            .filter_map(|e| {
                if let Event::Staking(inner) = e {
                    Some(inner)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

/// Assert input equal to the last event emitted
#[macro_export]
macro_rules! assert_last_event {
    ($event:expr) => {
        match &$event {
            e => assert_eq!(crate::mock::last_event(), *e),
        }
    };
}

/// Assert input equal to the last n events emitted
#[macro_export]
macro_rules! assert_last_n_events {
    ($n:expr, $event:expr) => {
        match &$event {
            e => similar_asserts::assert_eq!(crate::mock::last_n_events($n), *e),
        }
    };
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Treasury: pallet_treasury::{Pallet, Call,Config<T>, Storage, Event<T>},
        Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = u32;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = BlockHashCount;
    type BlockLength = ();
    type BlockNumber = u64;
    type BlockWeights = ();
    type Call = Call;
    type DbWeight = ();
    type Event = Event;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type Origin = Origin;
    type PalletInfo = PalletInfo;
    type SS58Prefix = SS58Prefix;
    type SystemWeightInfo = ();
    type Version = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1u64;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

parameter_types! {
    pub const TreasuryAccount: u32 = 0;
    pub const ReserveAccount: u32 = 1;
    pub const AuctionFund: u32 = 2;
    pub const ContractFund: u32 = 3;
    pub const MinRoundTerm: u32 = 20; // TODO
    pub const DefaultRoundTerm: u32 = 6 * BLOCKS_PER_HOUR; // TODO
    pub const GenesisIssuance: u32 = 20_000_000; // TODO
    pub const IdealPerpetualInflation: Perbill = Perbill::from_percent(1);
    pub const InflationRegressionMonths: u32 = 72;
}

impl pallet_treasury::Config for Test {
    type AuctionFund = AuctionFund;
    type ContractFund = ContractFund;
    type Currency = Balances;
    type DefaultRoundTerm = DefaultRoundTerm;
    type Event = Event;
    type GenesisIssuance = GenesisIssuance;
    type IdealPerpetualInflation = IdealPerpetualInflation;
    type InflationRegressionMonths = InflationRegressionMonths;
    type MinRoundTerm = MinRoundTerm;
    type ReserveAccount = ReserveAccount;
    type TreasuryAccount = TreasuryAccount;
    type WeightInfo = ();
}

// // LeaveCandidatesDelay=28 (=14d) assuming round_term=6h
// parameter_types! {
//     pub const ActiveSetSize: Range<u32> = Range {
//         min: 1_u32,
//         ideal: 32_u32,
//         max: 128_u32,
//     };
//     pub const MaxCommission: Percent = Percent::from_percent(50);
//     pub const MaxRisk: Percent = Percent::from_percent(50);
//     pub const MinExecutorBond: u64 = 1000 * T3RN;
//     pub const MinCandidateBond: u64 = 1000 * T3RN;
//     pub const MinAtomicStake:u64 = 500 * T3RN;
//     pub const MinTotalStake: u64 = 500 * T3RN;
//     pub const MaxTopStakesPerCandidate: u32 = 300;
//     pub const MaxBottomStakesPerCandidate: u32 = 50;
//     pub const MaxStakesPerStaker: u32 = 100;
//     pub const ConfigureExecutorDelay: u32 = 14 * BLOCKS_PER_DAY;
//     pub const LeaveCandidatesDelay: u32 = 28;
//     pub const LeaveStakersDelay: u32 = 28;
//     pub const CandidateBondLessDelay: u32 =28;
//     pub const RevokeStakeDelay: u32 = 28;
// }

    // type ActiveSetSize = ActiveSetSize;
    // type CandidateBondLessDelay = CandidateBondLessDelay;
    // type ConfigureExecutorDelay = ConfigureExecutorDelay;
    // type LeaveCandidatesDelay = LeaveCandidatesDelay;
    // type LeaveStakersDelay = LeaveStakersDelay;
    // type MaxBottomStakesPerCandidate = MaxBottomStakesPerCandidate;
    // type MaxCommission = MaxCommission;
    // type MaxRisk = MaxRisk;
    // type MaxStakesPerStaker = MaxStakesPerStaker;
    // type MaxTopStakesPerCandidate = MaxTopStakesPerCandidate;
    // type MinCandidateBond = MinCandidateBond;
    // type MinExecutorBond = MinExecutorBond;
    // type MinAtomicStake = MinAtomicStake;
    // type MinTotalStake = MinTotalStake;
    // type RevokeStakeDelay = RevokeStakeDelay;

impl pallet_staking::Config for Test {
    type Currency = Balances;
    type Event = Event;
    type Treasury = Treasury;
    type WeightInfo = ();
}

pub(crate) fn fast_forward_to(n: u64) {
    while System::block_number() < n {
        Staking::on_finalize(System::block_number());
        Balances::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Balances::on_initialize(System::block_number());
        Staking::on_initialize(System::block_number());
    }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .expect("mock pallet-staking genesis storage");

    pallet_staking::GenesisConfig::<Test>::default()
        .assimilate_storage(&mut storage)
        .expect("mock pallet-staking genesis storage assimilation");

    let mut ext = sp_io::TestExternalities::from(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
