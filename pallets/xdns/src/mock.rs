// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

//! Mock runtime for pallet-xdns.

use crate::*;
use frame_support::{parameter_types, traits::Everything};
use sp_core::{sr25519, Pair, H256};
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    KeyTypeId,
};
use sp_std::convert::{TryFrom, TryInto};
// Reexport crate as its pallet name for construct_runtime.
use crate as pallet_xdns;
use frame_support::pallet_prelude::GenesisBuild;
use t3rn_primitives::{EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor};

use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use t3rn_primitives::{side_effect::interface::SideEffectInterface, xdns::XdnsRecord};

type AccountId = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct a mock runtime.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
    }
);

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub const TransactionByteFee: u64 = 1;
}

impl pallet_timestamp::Config for Test {
    type MinimumPeriod = MinimumPeriod;
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl EscrowTrait<Test> for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = u64;
    type BaseCallFilter = Everything;
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
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_sudo::Config for Test {
    type Call = Call;
    type Event = Event;
}

parameter_types! {
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl Config for Test {
    type Balances = Balances;
    type Escrowed = Self;
    type Event = Event;
    type WeightInfo = ();
}

pub(crate) struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_side_effects: Vec<SideEffectInterface>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            known_xdns_records: vec![],
            standard_side_effects: vec![],
        }
    }
}

impl ExtBuilder {
    pub(crate) fn with_default_xdns_records(mut self) -> ExtBuilder {
        let circuit_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"circ",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
            vec![*b"tran", *b"swap"],
        );
        let gateway_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"gate",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
            vec![],
        );
        let polkadot_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"pdot",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 0,
                token_symbol: Encode::encode("DOT"),
                token_decimals: 10,
            },
            vec![],
            vec![],
        );
        let kusama_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"ksma",
            None,
            Default::default(),
            GatewayVendor::PolkadotLike,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 2,
                token_symbol: Encode::encode("KSM"),
                token_decimals: 12,
            },
            vec![],
            vec![],
        );
        self.known_xdns_records = vec![
            circuit_xdns_record,
            gateway_xdns_record,
            polkadot_xdns_record,
            kusama_xdns_record,
        ];
        self
    }

    pub(crate) fn with_standard_side_effects(mut self) -> ExtBuilder {
        // map side_effects to id, keeping lib.rs clean
        self.standard_side_effects =
            t3rn_protocol::side_effects::standards::standard_side_effects();

        self
    }

    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .expect("Pallet balances storage can be assimilated");

        pallet_xdns::GenesisConfig::<Test> {
            known_xdns_records: self.known_xdns_records,
            standard_side_effects: self.standard_side_effects,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet contracts registry can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub trait WithAuthorities {
    fn with_authorities(self, authorities_suri: Vec<&str>) -> sp_io::TestExternalities;
}

impl WithAuthorities for sp_io::TestExternalities {
    fn with_authorities(mut self, authorities_suri: Vec<&str>) -> sp_io::TestExternalities {
        let keystore = KeyStore::new();

        // Insert authorities' keys
        for suri in authorities_suri {
            let keypair = sr25519::Pair::from_string(suri, None).expect("Generates key pair");
            SyncCryptoStore::insert_unknown(
                &keystore,
                KeyTypeId(*b"circ"),
                suri,
                keypair.public().as_ref(),
            )
            .expect("Inserts unknown key");
        }

        self.register_extension(KeystoreExt(keystore.into()));
        self
    }
}
