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
};
use sp_core::{H160, H256, U256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    ConsensusEngineId,
};
use sp_std::{boxed::Box, prelude::*, str::FromStr};

use crate::{
    EnsureAddressNever, EnsureAddressRoot, FeeCalculator, IdentityAddressMapping,
    IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};

frame_support::construct_runtime! {
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        EVM: crate::{Pallet, Call, Storage, Config<T>, Event<T>},
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
const BLOCK_GAS_LIMIT: u64 = 150_000_000;
const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
    pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
    pub MockPrecompiles: MockPrecompileSet = MockPrecompileSet;
}
impl crate::Config for Test {
    type AddressMapping = IdentityAddressMapping;
    type BlockGasLimit = BlockGasLimit;
    type BlockHashMapping = crate::SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressRoot<Self::AccountId>;
    type ChainId = ();
    type Currency = Balances;
    type FeeCalculator = FixedGasPrice;
    type FindAuthor = FindAuthorTruncated;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type GasWeightMapping = crate::FixedGasWeightMapping<Self>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type PrecompilesType = MockPrecompileSet;
    type PrecompilesValue = MockPrecompiles;
    type Runner = crate::runner::stack::Runner<Self>;
    type RuntimeEvent = RuntimeEvent;
    type ThreeVm = t3rn_primitives::threevm::NoopThreeVm;
    type Timestamp = Timestamp;
    type WeightInfo = ();
    type WeightPerGas = WeightPerGas;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
}

/// Example PrecompileSet with only Identity precompile.
pub struct MockPrecompileSet;

impl PrecompileSet for MockPrecompileSet {
    /// Tries to execute a precompile in the precompile set.
    /// If the provided address is not a precompile, returns None.
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        let address = handle.code_address();

        if address == H160::from_low_u64_be(1) {
            return Some(pallet_evm_precompile_simple::Identity::execute(handle))
        }

        None
    }

    /// Check if the given address is a precompile. Should only be called to
    /// perform the check while not executing the precompile afterward, since
    /// `execute` already performs a check internally.
    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        IsPrecompileResult::Answer {
            is_precompile: address == H160::from_low_u64_be(1),
            extra_cost: 0,
        }
    }
}
