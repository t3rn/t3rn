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

use frame_support::{
    parameter_types,
    traits::{ConstU32, FindAuthor},
    weights::Weight,
    ConsensusEngineId,
};
use sp_core::{H160, H256, U256};
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_std::{boxed::Box, prelude::*, str::FromStr};

use fp_evm::{ExitError, ExitReason, Transfer};
use pallet_evm::{
    Context, EnsureAddressNever, EnsureAddressRoot, FeeCalculator, IdentityAddressMapping,
    PrecompileHandle,
};

frame_support::construct_runtime! {
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        EVM: pallet_evm::{Pallet, Call, Storage, Config<T>, Event<T>},
        Utility: pallet_utility::{Pallet, Call, Event},
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 0));
}
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = H160;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = frame_system::mocking::MockBlock<Self>;
    type BlockHashCount = BlockHashCount;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
    type Nonce = u64;
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

impl pallet_utility::Config for Test {
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxHolds = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = ();
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

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> (U256, Weight) {
        // Return some meaningful gas price and weight
        (1_000_000_000u128.into(), Weight::from_parts(7u64, 0))
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
parameter_types! {
    pub BlockGasLimit: U256 = U256::max_value();
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
}
impl pallet_evm::Config for Test {
    type AddressMapping = IdentityAddressMapping;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressRoot<Self::AccountId>;
    type ChainId = ();
    type Currency = Balances;
    type FeeCalculator = FixedGasPrice;
    type FindAuthor = FindAuthorTruncated;
    type GasLimitPovSizeRatio = ();
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type PrecompilesType = ();
    type PrecompilesValue = ();
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type RuntimeEvent = RuntimeEvent;
    type Timestamp = Timestamp;
    type WeightInfo = ();
    type WeightPerGas = WeightPerGas;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
}

pub(crate) struct MockHandle {
    pub input: Vec<u8>,
    pub context: Context,
}

impl PrecompileHandle for MockHandle {
    fn call(
        &mut self,
        _: H160,
        _: Option<Transfer>,
        _: Vec<u8>,
        _: Option<u64>,
        _: bool,
        _: &Context,
    ) -> (ExitReason, Vec<u8>) {
        unimplemented!()
    }

    fn record_cost(&mut self, _: u64) -> Result<(), ExitError> {
        Ok(())
    }

    fn record_external_cost(
        &mut self,
        _ref_time: Option<u64>,
        _proof_size: Option<u64>,
    ) -> Result<(), ExitError> {
        Ok(())
    }

    fn refund_external_cost(&mut self, _ref_time: Option<u64>, _proof_size: Option<u64>) {}

    fn remaining_gas(&self) -> u64 {
        unimplemented!()
    }

    fn log(&mut self, _: H160, _: Vec<H256>, _: Vec<u8>) -> Result<(), ExitError> {
        unimplemented!()
    }

    fn code_address(&self) -> H160 {
        unimplemented!()
    }

    fn input(&self) -> &[u8] {
        &self.input
    }

    fn context(&self) -> &Context {
        &self.context
    }

    fn is_static(&self) -> bool {
        unimplemented!()
    }

    fn gas_limit(&self) -> Option<u64> {
        None
    }
}
