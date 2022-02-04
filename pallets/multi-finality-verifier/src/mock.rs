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
    testing::{Header, H256},
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use t3rn_primitives::bridges::runtime::Chain;

use t3rn_primitives::bridges::polkadot_core::PolkadotLike;
use t3rn_primitives::EscrowTrait;

pub type AccountId = u64;
pub type TestHeader = crate::BridgedHeader<TestRuntime, ()>;
pub type TestNumber = crate::BridgedBlockNumber<TestRuntime, ()>;
pub type TestHash = crate::BridgedBlockHash<TestRuntime, ()>;

type Block = frame_system::mocking::MockBlock<TestRuntime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

use crate as multi_finality_verifier;

construct_runtime! {
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        MultiFinalityVerifier: multi_finality_verifier::{Pallet, Call, Storage, Config<T, I>},
        MultiFinalityVerifierPolkadotLike: multi_finality_verifier::<Instance1>::{Pallet, Call, Storage, Config<T, I>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub const TransactionByteFee: u64 = 1;
}

impl pallet_timestamp::Config for TestRuntime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_sudo::Config for TestRuntime {
    type Event = Event;
    type Call = Call;
}

impl EscrowTrait for TestRuntime {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub const MaxRequests: u32 = 2;
    pub const HeadersToKeep: u32 = 5;
    pub const SessionLength: u64 = 5;
    pub const NumValidators: u32 = 5;
}

parameter_types! {
    pub const MaxReserves: u32 = 50;
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for TestRuntime {
    type MaxLocks = ();
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

impl pallet_xdns::Config for TestRuntime {
    type Event = Event;
    type WeightInfo = ();
}

impl multi_finality_verifier::Config for TestRuntime {
    type BridgedChain = TestCircuitLikeChain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

pub type PolkadotLikeFinalityVerifierInstance = multi_finality_verifier::Instance1;
impl multi_finality_verifier::Config<PolkadotLikeFinalityVerifierInstance> for TestRuntime {
    type BridgedChain = PolkadotLike;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
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

#[derive(Debug)]
pub struct TestKeccak256U64Chain;

impl Chain for TestKeccak256U64Chain {
    type BlockNumber = <TestRuntime as frame_system::Config>::BlockNumber;
    type Hash = <TestRuntime as frame_system::Config>::Hash;
    type Hasher = <TestRuntime as frame_system::Config>::Hashing;
    type Header = <TestRuntime as frame_system::Config>::Header;
}

pub fn run_test<T>(test: impl FnOnce() -> T) -> T {
    sp_io::TestExternalities::new(Default::default()).execute_with(test)
}

pub fn test_header(num: TestNumber) -> TestHeader {
    // We wrap the call to avoid explicit type annotations in our tests
    t3rn_primitives::bridges::test_utils::test_header(num)
}

pub fn test_header_with_correct_parent(num: TestNumber, parent_hash: Option<H256>) -> TestHeader {
    t3rn_primitives::bridges::test_utils::test_header_with_correct_parent(num, parent_hash)
}

pub fn test_header_range(from: u64, to: u64, mut parent_hash: Option<H256>) -> Vec<TestHeader> {
    let mut headers: Vec<TestHeader> = vec![];
    for (i, block) in (from..to).enumerate() {
        let header = test_header_with_correct_parent(block.into(), parent_hash);
        headers.push(header);
        parent_hash = Some(headers[i].hash());
    }

    return headers;
}
