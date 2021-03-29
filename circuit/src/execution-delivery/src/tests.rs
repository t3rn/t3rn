// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

use crate::*;

use crate as example_offchain_worker;

use frame_support::{assert_ok, parameter_types};
use sp_core::{sr25519::Signature, H256};

use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the module, we construct a mock runtime.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ExecDelivery: example_offchain_worker::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_core::sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

type Extrinsic = TestXt<Call, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        _public: <Signature as Verify>::Signer,
        _account: AccountId,
        nonce: u64,
    ) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
        Some((call, (nonce, ())))
    }
}

parameter_types! {
    pub const GracePeriod: u64 = 5;
    pub const UnsignedInterval: u64 = 128;
    pub const UnsignedPriority: u64 = 1 << 20;
}

impl Config for Test {
    type Event = Event;
    type AuthorityId = crypto::TestAuthId;
    type Call = Call;
    type GracePeriod = GracePeriod;
    type UnsignedInterval = UnsignedInterval;
    type UnsignedPriority = UnsignedPriority;
}

#[test]
fn it_submits_empty_composable_exec_request() {
    sp_io::TestExternalities::default().execute_with(|| {
        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            vec![],
            vec![]
        ));
    });
}

#[test]
fn it_submits_correct_io_schedule_to_composable_exec_request() {
    sp_io::TestExternalities::default().execute_with(|| {
        // That's IO schedule.
        // We can have several parallel steps (e.g. separated with comma) and several sequential phases (e.g. separated with |)
        // ToDo: Find best way to format and process the IO schedule of cross-chain execution.
        // In the future each component will be following XCM MultiLocation format https://github.com/paritytech/polkadot/blob/master/xcm/src/v0/junction.rs
        let io_bytes: Vec<u8> = b"component1, component2 | component3;".to_vec();

        println!("mimi: {:?}", std::str::from_utf8(&io_bytes[..]).unwrap());
        assert_eq!(
            "component1, component2 | component3;",
            std::str::from_utf8(&io_bytes[..]).unwrap()
        );
        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            vec![],
            vec![]
        ));
    });
}

#[test]
fn can_say_hello_from_pallet_implementation() {
    assert_eq!("hello", ExecDelivery::say_hello());
}
