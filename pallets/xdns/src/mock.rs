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
// Reexport crate as its pallet name for construct_runtime.
use crate as pallet_xdns;
use frame_support::pallet_prelude::GenesisBuild;
use t3rn_primitives::{EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor};

use sp_keystore::testing::KeyStore;
use sp_keystore::{KeystoreExt, SyncCryptoStore};

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
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Call = Call;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
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
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_sudo::Config for Test {
    type Event = Event;
    type Call = Call;
}

parameter_types! {
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
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

impl Config for Test {
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
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![*b"tran", *b"swap"],
        );
        let gateway_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"gate",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
        );
        let polkadot_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"pdot",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 0,
                token_symbol: Encode::encode("DOT"),
                token_decimals: 10,
            },
            vec![],
        );
        let kusama_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"ksma",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 2,
                token_symbol: Encode::encode("KSM"),
                token_decimals: 12,
            },
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
        let transfer_side_effect = SideEffectInterface {
            id: *b"tran",
            name: b"transfer".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: from
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: value
                Type::OptionalInsurance, // argument_3: insurance
            ],
            argument_to_state_mapper: vec![
                b"from".to_vec(),
                b"to".to_vec(),
                b"value".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"Transfer(from,to,value)".to_vec()],
            escrowed_events: vec![b"EscrowTransfer(from,to,value)".to_vec()],
            commit_events: vec![b"Transfer(executor,to,value)".to_vec()],
            revert_events: vec![b"Transfer(executor,from,value)".to_vec()],
        };

        let swap_side_effect = SideEffectInterface {
            id: *b"swap",
            name: b"swap".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: caller
                Type::DynamicAddress,    // argument_1: to
                Type::Value,             // argument_2: amount_from
                Type::Value,             // argument_3: amount_to
                Type::DynamicBytes,      // argument_4: asset_from
                Type::DynamicBytes,      // argument_5: asset_to
                Type::OptionalInsurance, // argument_6: insurance
            ],
            argument_to_state_mapper: vec![
                b"caller".to_vec(),
                b"to".to_vec(),
                b"amount_from".to_vec(),
                b"amount_to".to_vec(),
                b"asset_from".to_vec(),
                b"asset_to".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
            escrowed_events: vec![b"ExecuteToken(_executor,to,asset_to,amount_to)".to_vec()],
            commit_events: vec![b"MultiTransfer(executor,to,asset_to,amount_to)".to_vec()],
            revert_events: vec![b"MultiTransfer(executor,caller,asset_from,amount_from)".to_vec()],
        };

        let add_liquidity_side_effect = SideEffectInterface {
            id: *b"aliq",
            name: b"add_liquidity".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress,    // argument_0: caller
                Type::DynamicAddress,    // argument_1: to
                Type::DynamicBytes,      // argument_2: asset_left
                Type::DynamicBytes,      // argument_3: asset_right
                Type::DynamicBytes,      // argument_4: liquidity_token
                Type::Value,             // argument_5: amount_left
                Type::Value,             // argument_6: amount_right
                Type::Value,             // argument_7: amount_liquidity_token
                Type::OptionalInsurance, // argument_8: insurance
            ],
            argument_to_state_mapper: vec![
                b"caller".to_vec(),
                b"to".to_vec(),
                b"asset_left".to_vec(),
                b"assert_right".to_vec(),
                b"liquidity_token".to_vec(),
                b"amount_left".to_vec(),
                b"amount_right".to_vec(),
                b"amount_liquidity_token".to_vec(),
                b"insurance".to_vec(),
            ],
            confirm_events: vec![
                b"ExecuteToken(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            escrowed_events: vec![
                b"ExecuteToken(xtx_id,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            commit_events: vec![
                b"MultiTransfer(executor,to,liquidity_token,amount_liquidity_token)".to_vec(),
            ],
            revert_events: vec![
                b"MultiTransfer(executor,caller,asset_left,amount_left)".to_vec(),
                b"MultiTransfer(executor,caller,asset_right,amount_right)".to_vec(),
            ],
        };

        let call_evm_side_effect = SideEffectInterface {
            id: *b"call",
            name: b"call:generic".to_vec(),
            argument_abi: vec![
                Type::DynamicAddress, // argument_0: source
                Type::DynamicAddress, // argument_1: target
                Type::DynamicBytes,   // argument_2: target
                Type::Value,          // argument_3: value
                Type::Uint(64),       // argument_4: gas_limit
                Type::Value,          // argument_5: max_fee_per_gas
                Type::Value,          // argument_6: max_priority_fee_per_gas
                Type::Value,          // argument_7: nonce
                Type::DynamicBytes,   // argument_8: access_list (since HF Berlin?)
            ],
            argument_to_state_mapper: vec![
                b"source".to_vec(),
                b"target".to_vec(),
                b"input".to_vec(),
                b"value".to_vec(),
                b"gas_limit".to_vec(),
                b"max_fee_per_gas".to_vec(),
                b"max_priority_fee_per_gas".to_vec(),
                b"nonce".to_vec(),
                b"access_list".to_vec(),
            ],
            confirm_events: vec![
                b"TransactCall(Append<caller>,source,value,input,gas_limit)".to_vec()
            ],
            escrowed_events: vec![],
            commit_events: vec![],
            revert_events: vec![],
        };

        let get_data_side_effect = SideEffectInterface {
            id: *b"data",
            name: b"data:get".to_vec(),
            argument_abi: vec![
                Type::DynamicBytes, // argument_0: key
            ],
            argument_to_state_mapper: vec![b"key".to_vec()],
            confirm_events: vec![b"<InclusionOnly>".to_vec()],
            escrowed_events: vec![],
            commit_events: vec![],
            revert_events: vec![],
        };

        // map side_effects to id, keeping lib.rs clean
        self.standard_side_effects = vec![
           transfer_side_effect,
           swap_side_effect,
           add_liquidity_side_effect,
           call_evm_side_effect,
           get_data_side_effect,
        ];
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
