// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

// From construct_runtime macro
#![allow(clippy::from_over_into)]

use frame_support::{construct_runtime, parameter_types, traits::Everything, weights::Weight};
use sp_runtime::{
    generic,
    testing::H256,
    traits::{BlakeTwo256, ConstU32, IdentityLookup},
    Perbill,
};

type Header = generic::Header<u32, BlakeTwo256>;
use sp_std::convert::{TryFrom, TryInto};

use crate::bridges::runtime::Chain;
pub type AccountId = u64;
pub type TestHeader = crate::BridgedHeader<TestRuntime, ()>;
pub type TestNumber = crate::BridgedBlockNumber<TestRuntime, ()>;

type Block = frame_system::mocking::MockBlock<TestRuntime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

use crate::{
    light_clients::{KusamaInstance, PolkadotInstance, RococoInstance},
    BestFinalizedHash, Config, ImportedHeaders,
};

construct_runtime! {
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        GrandpaFinalityVerifier: crate,
        PolkadotBridge: crate::<Instance1>,
        KusamaBridge: crate::<Instance2>,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Sudo: pallet_sudo,
    }
}

parameter_types! {
    pub const BlockHashCount: u32 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for TestRuntime {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = AccountId;
    type BaseCallFilter = Everything;
    type BlockHashCount = BlockHashCount;
    type BlockLength = ();
    type BlockNumber = u32;
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub const TransactionByteFee: u64 = 1;
}

impl pallet_timestamp::Config for TestRuntime {
    type MinimumPeriod = MinimumPeriod;
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl pallet_sudo::Config for TestRuntime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
    pub const HeadersToStore: u32 = 5;
    pub const SessionLength: u64 = 5;
    pub const NumValidators: u32 = 5;
}

parameter_types! {
    pub const MaxReserves: u32 = 50;
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for TestRuntime {
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl Config<RococoInstance> for TestRuntime {
    type BridgedChain = TestCircuitLikeChain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl Config<KusamaInstance> for TestRuntime {
    type BridgedChain = TestCircuitLikeChain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
impl Config<PolkadotInstance> for TestRuntime {
    type BridgedChain = TestCircuitLikeChain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<3u32>;
    type FinalizedConfirmationOffset = ConstU32<10u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<10u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

#[derive(Debug)]
pub struct TestCircuitLikeChain;

impl Chain for TestCircuitLikeChain {
    type BlockNumber = <TestRuntime as frame_system::Config>::BlockNumber;
    type Hash = <TestRuntime as frame_system::Config>::Hash;
    type Hasher = <TestRuntime as frame_system::Config>::Hashing;
    type Header = <TestRuntime as frame_system::Config>::Header;
}

pub fn run_test<T>(test: impl FnOnce() -> T) -> T {
    sp_io::TestExternalities::new(Default::default()).execute_with(test)
}

#[cfg(all(feature = "testing", test))]
pub fn test_header(num: TestNumber) -> TestHeader {
    // We wrap the call to avoid explicit type annotations in our tests
    crate::bridges::test_utils::test_header(num)
}

#[cfg(all(feature = "testing", test))]
pub fn test_header_with_correct_parent(num: TestNumber, parent_hash: Option<H256>) -> TestHeader {
    crate::bridges::test_utils::test_header_with_correct_parent(num, parent_hash)
}

#[cfg(all(feature = "testing", test))]
pub fn test_header_range(to: u32) -> Vec<TestHeader> {
    let mut headers: Vec<TestHeader> = vec![];
    let mut parent_hash = None;
    for (i, block) in (0..=to).enumerate() {
        let header = test_header_with_correct_parent(block, parent_hash);
        headers.push(header);
        parent_hash = Some(headers[i].hash());
    }
    headers
}

#[cfg(feature = "testing")]
pub fn brute_seed_block_1(_gateway_id: [u8; 4]) {
    // Brute update storage of MFV::MultiImportedHeaders to blockA = 1 and BestAvailable -> blockA

    let header_1 = crate::bridges::test_utils::test_header::<TestHeader>(1);
    let block_hash_1 = header_1.hash();

    <ImportedHeaders<TestRuntime>>::insert::<H256, TestHeader>(block_hash_1, header_1);

    <BestFinalizedHash<TestRuntime>>::put(block_hash_1);
}
