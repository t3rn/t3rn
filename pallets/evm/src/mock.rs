// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test mock for unit tests and benchmarking

use fp_evm::FeeCalculator;
use frame_support::{
    parameter_types,
    traits::{ConstU32, FindAuthor},
    ConsensusEngineId,
};
use sp_core::{crypto::AccountId32, H160, H256, U256};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::{boxed::Box, prelude::*, str::FromStr};

use crate::{EnsureAddressNever, EnsureSigned, StoredHashAddressMapping, ThreeVMCurrencyAdapter};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime! {
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        EVM: crate::{Pallet, Call, Storage, Config, Event<T>},
        ThreeVm: pallet_3vm::{Pallet, Call, Storage, Event<T>},
        // Utility: pallet_utility::{Pallet, Call, Storage, Event},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Storage, Config<T>, Event<T>},
        Sudo: pallet_sudo,
        Assets: pallet_assets,
        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>},
        CircuitPortal: pallet_portal,
        Xdns: pallet_xdns,
        AccountManager: pallet_account_manager,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,
    }
}

parameter_types! {
    pub const BlockHashCount: u32 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = AccountId32;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = BlockHashCount;
    type BlockLength = ();
    type BlockNumber = u32;
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<u32, BlakeTwo256>;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
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
    pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Test {
    type MinimumPeriod = MinimumPeriod;
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

/// Fixed gas price of `0`.
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        1u128.into()
    }
}

pub struct FindAuthorTruncated;
impl FindAuthor<H160> for FindAuthorTruncated {
    fn find_author<'a, I>(_digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        Some(H160::from_str("1234500000000000000000000000000000000000").unwrap())
    }
}

impl crate::Config for Test {
    type AddressMapping = StoredHashAddressMapping<Test>;
    type BlockGasLimit = ();
    type BlockHashMapping = crate::SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureSigned<Self>;
    type ChainId = ();
    type Currency = Balances;
    type FeeCalculator = FixedGasPrice;
    type FindAuthor = FindAuthorTruncated;
    type GasWeightMapping = ();
    type OnChargeTransaction = ThreeVMCurrencyAdapter<Balances, ()>;
    type PrecompilesType = ();
    type PrecompilesValue = ();
    type Runner = crate::runner::stack::Runner<Self>;
    type RuntimeEvent = RuntimeEvent;
    type ThreeVm = ThreeVm;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
}
