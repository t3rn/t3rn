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

use frame_support::{assert_ok, assert_err, parameter_types};
use frame_support::weights::Weight;
use sp_core::{Public, sr25519::Signature, H256};

use sp_runtime::{
    testing::{Header as SubstrateHeader, TestXt},
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify, Convert},
    FixedU128,
    DispatchResult, DispatchError
};

use bp_runtime::Size;
use bp_messages::{
    source_chain::{
        LaneMessageVerifier, MessageDeliveryAndDispatchPayment, RelayersRewards, Sender, TargetHeaderChain,
    },
    target_chain::{DispatchMessage, MessageDispatch, ProvedLaneMessages, ProvedMessages, SourceHeaderChain},
    InboundLaneData, LaneId, Message, MessageData, MessageKey, MessageNonce, OutboundLaneData,
    Parameter as MessagesParameter,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use t3rn_primitives::transfers::BalanceOf;

use versatile_wasm::{VersatileWasm, DispatchRuntimeCall};

use std::collections::BTreeMap;

// For testing the module, we construct a mock runtime.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        Balances: pallet_balances::{Pallet, Call, Event<T>},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ExecDelivery: example_offchain_worker::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
        Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},
        Timestamp: pallet_timestamp::{Pallet},
        VersatileWasmVM: versatile_wasm::{Pallet, Call, Event<T>},
        Randomness: pallet_randomness_collective_flip::{Pallet, Call, Storage},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Storage, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Event<T>},
    }
);


impl pallet_contracts_registry::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}

impl pallet_xdns::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}

pub type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

pub struct ExampleDispatchRuntimeCall;

impl DispatchRuntimeCall<Test> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        _input: &[u8],
        _escrow_account: &<Test as frame_system::Config>::AccountId,
        _requested: &<Test as frame_system::Config>::AccountId,
        _callee: &<Test as frame_system::Config>::AccountId,
        _value: BalanceOf<Test>,
        _gas_meter: &mut versatile_wasm::gas::GasMeter<Test>,
    ) -> DispatchResult {
        match (module_name, fn_name) {
            ("Weights", "complex_calculations") => {
                let (_decoded_x, _decoded_y): (u32, u32) = match Decode::decode(&mut _input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };

                Ok(())
            }
            (_, _) => Err(DispatchError::Other(
                "Call to unrecognized runtime function",
            )),
        }
    }
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_sudo::Config for Test {
    type Event = Event;
    type Call = Call;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Test>;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub const TransactionByteFee: u64 = 1;
}
use frame_support::weights::IdentityFee;
impl pallet_transaction_payment::Config for Test {
    // type OnChargeTransaction = ();
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

impl VersatileWasm for Test {
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type Event = Event;
    type Call = Call;
    type Randomness = Randomness;
}


parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

// type AccountId = sp_core::sr25519::Public;

impl frame_system::Config for Test {
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_core::sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = SubstrateHeader;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TestMessagesParameter {
    TokenConversionRate(FixedU128),
}

impl MessagesParameter for TestMessagesParameter {
    fn save(&self) {
        match *self {
            TestMessagesParameter::TokenConversionRate(conversion_rate) => TokenConversionRate::set(&conversion_rate),
        }
    }
}

parameter_types! {
	pub const MaxMessagesToPruneAtOnce: u64 = 10;
	pub const MaxUnrewardedRelayerEntriesAtInboundLane: u64 = 16;
	pub const MaxUnconfirmedMessagesAtInboundLane: u64 = 32;
	pub storage TokenConversionRate: FixedU128 = 1.into();
}

#[derive(Decode, Encode, Clone, Debug, PartialEq, Eq)]
pub struct TestPayload(pub u64, pub Weight);
impl Size for TestPayload {
    fn size_hint(&self) -> u32 {
        16
    }
}

pub type TestMessageFee = u64;
pub type TestRelayer = AccountId;

pub struct AccountIdConverter;

impl sp_runtime::traits::Convert<H256, AccountId> for AccountIdConverter {
    fn convert(hash: H256) -> AccountId {
        sp_core::sr25519::Public::decode(&mut &hash.as_bytes()[..]).unwrap_or_default()
    }
}

// /// Account that has balance to use in tests.
// pub const ENDOWED_ACCOUNT: AccountId = 0xDEAD;
//
// /// Account id of test relayer.
// pub const TEST_RELAYER_A: AccountId = 100;
//
// /// Account id of additional test relayer - B.
// pub const TEST_RELAYER_B: AccountId = 101;
//
// /// Account id of additional test relayer - C.
// pub const TEST_RELAYER_C: AccountId = 102;

/// Error that is returned by all test implementations.
pub const TEST_ERROR: &str = "Test error";

/// Lane that we're using in tests.
pub const TEST_LANE_ID: LaneId = [0, 0, 0, 1];

/// Regular message payload.
pub const REGULAR_PAYLOAD: TestPayload = TestPayload(0, 50);

/// Payload that is rejected by `TestTargetHeaderChain`.
pub const PAYLOAD_REJECTED_BY_TARGET_CHAIN: TestPayload = TestPayload(1, 50);

/// Vec of proved messages, grouped by lane.
pub type MessagesByLaneVec = Vec<(LaneId, ProvedLaneMessages<Message<TestMessageFee>>)>;

/// Test messages proof.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct TestMessagesProof {
    pub result: Result<MessagesByLaneVec, ()>,
}

impl Size for TestMessagesProof {
    fn size_hint(&self) -> u32 {
        0
    }
}

impl From<Result<Vec<Message<TestMessageFee>>, ()>> for TestMessagesProof {
    fn from(result: Result<Vec<Message<TestMessageFee>>, ()>) -> Self {
        Self {
            result: result.map(|messages| {
                let mut messages_by_lane: BTreeMap<LaneId, ProvedLaneMessages<Message<TestMessageFee>>> =
                    BTreeMap::new();
                for message in messages {
                    messages_by_lane
                        .entry(message.key.lane_id)
                        .or_default()
                        .messages
                        .push(message);
                }
                messages_by_lane.into_iter().collect()
            }),
        }
    }
}

/// Messages delivery proof used in tests.
#[derive(Debug, Encode, Decode, Eq, Clone, PartialEq)]
pub struct TestMessagesDeliveryProof(pub Result<(LaneId, InboundLaneData<TestRelayer>), ()>);

impl Size for TestMessagesDeliveryProof {
    fn size_hint(&self) -> u32 {
        0
    }
}

/// Target header chain that is used in tests.
#[derive(Debug, Default)]
pub struct TestTargetHeaderChain;

impl TargetHeaderChain<TestPayload, TestRelayer> for TestTargetHeaderChain {
    type Error = &'static str;

    type MessagesDeliveryProof = TestMessagesDeliveryProof;

    fn verify_message(payload: &TestPayload) -> Result<(), Self::Error> {
        if *payload == PAYLOAD_REJECTED_BY_TARGET_CHAIN {
            Err(TEST_ERROR)
        } else {
            Ok(())
        }
    }

    fn verify_messages_delivery_proof(
        proof: Self::MessagesDeliveryProof,
    ) -> Result<(LaneId, InboundLaneData<TestRelayer>), Self::Error> {
        proof.0.map_err(|_| TEST_ERROR)
    }
}

/// Lane message verifier that is used in tests.
#[derive(Debug, Default)]
pub struct TestLaneMessageVerifier;

impl LaneMessageVerifier<AccountId, TestPayload, TestMessageFee> for TestLaneMessageVerifier {
    type Error = &'static str;

    fn verify_message(
        _submitter: &Sender<AccountId>,
        delivery_and_dispatch_fee: &TestMessageFee,
        _lane: &LaneId,
        _lane_outbound_data: &OutboundLaneData,
        _payload: &TestPayload,
    ) -> Result<(), Self::Error> {
        if *delivery_and_dispatch_fee != 0 {
            Ok(())
        } else {
            Err(TEST_ERROR)
        }
    }
}

/// Message fee payment system that is used in tests.
#[derive(Debug, Default)]
pub struct TestMessageDeliveryAndDispatchPayment;

impl TestMessageDeliveryAndDispatchPayment {
    /// Reject all payments.
    pub fn reject_payments() {
        frame_support::storage::unhashed::put(b":reject-message-fee:", &true);
    }

    /// Returns true if given fee has been paid by given submitter.
    pub fn is_fee_paid(submitter: AccountId, fee: TestMessageFee) -> bool {
        frame_support::storage::unhashed::get(b":message-fee:") == Some((Sender::Signed(submitter), fee))
    }

    /// Returns true if given relayer has been rewarded with given balance. The reward-paid flag is
    /// cleared after the call.
    pub fn is_reward_paid(relayer: AccountId, fee: TestMessageFee) -> bool {
        let key = (b":relayer-reward:", relayer, fee).encode();
        frame_support::storage::unhashed::take::<bool>(&key).is_some()
    }
}

impl MessageDeliveryAndDispatchPayment<AccountId, TestMessageFee> for TestMessageDeliveryAndDispatchPayment {
    type Error = &'static str;

    fn pay_delivery_and_dispatch_fee(
        submitter: &Sender<AccountId>,
        fee: &TestMessageFee,
        _relayer_fund_account: &AccountId,
    ) -> Result<(), Self::Error> {
        if frame_support::storage::unhashed::get(b":reject-message-fee:") == Some(true) {
            return Err(TEST_ERROR);
        }

        frame_support::storage::unhashed::put(b":message-fee:", &(submitter, fee));
        Ok(())
    }

    fn pay_relayers_rewards(
        _confirmation_relayer: &AccountId,
        relayers_rewards: RelayersRewards<AccountId, TestMessageFee>,
        _relayer_fund_account: &AccountId,
    ) {
        for (relayer, reward) in relayers_rewards {
            let key = (b":relayer-reward:", relayer, reward.reward).encode();
            frame_support::storage::unhashed::put(&key, &true);
        }
    }
}

/// Source header chain that is used in tests.
#[derive(Debug)]
pub struct TestSourceHeaderChain;

impl SourceHeaderChain<TestMessageFee> for TestSourceHeaderChain {
    type Error = &'static str;

    type MessagesProof = TestMessagesProof;

    fn verify_messages_proof(
        proof: Self::MessagesProof,
        _messages_count: u32,
    ) -> Result<ProvedMessages<Message<TestMessageFee>>, Self::Error> {
        proof
            .result
            .map(|proof| proof.into_iter().collect())
            .map_err(|_| TEST_ERROR)
    }
}

/// Source header chain that is used in tests.
#[derive(Debug)]
pub struct TestMessageDispatch;

impl MessageDispatch<TestMessageFee> for TestMessageDispatch {
    type DispatchPayload = TestPayload;

    fn dispatch_weight(message: &DispatchMessage<TestPayload, TestMessageFee>) -> Weight {
        match message.data.payload.as_ref() {
            Ok(payload) => payload.1,
            Err(_) => 0,
        }
    }

    fn dispatch(_message: DispatchMessage<TestPayload, TestMessageFee>) {}
}

/// Return test lane message with given nonce and payload.
pub fn message(nonce: MessageNonce, payload: TestPayload) -> Message<TestMessageFee> {
    Message {
        key: MessageKey {
            lane_id: TEST_LANE_ID,
            nonce,
        },
        data: message_data(payload),
    }
}

/// Return message data with valid fee for given payload.
pub fn message_data(payload: TestPayload) -> MessageData<TestMessageFee> {
    MessageData {
        payload: payload.encode(),
        fee: 1,
    }
}

pub(crate) type DefaultMessagesInstance = pallet_bridge_messages::DefaultInstance;

impl pallet_bridge_messages::Config<DefaultMessagesInstance> for Test {
    type Event = Event;
    type WeightInfo = ();
    type Parameter = TestMessagesParameter;
    type MaxMessagesToPruneAtOnce = MaxMessagesToPruneAtOnce;
    type MaxUnrewardedRelayerEntriesAtInboundLane = MaxUnrewardedRelayerEntriesAtInboundLane;
    type MaxUnconfirmedMessagesAtInboundLane = MaxUnconfirmedMessagesAtInboundLane;

    type OutboundPayload = TestPayload;
    type OutboundMessageFee = TestMessageFee;

    type InboundPayload = TestPayload;
    type InboundMessageFee = TestMessageFee;
    type InboundRelayer = TestRelayer;

    type AccountIdConverter = AccountIdConverter;

    type TargetHeaderChain = TestTargetHeaderChain;
    type LaneMessageVerifier = TestLaneMessageVerifier;
    type MessageDeliveryAndDispatchPayment = TestMessageDeliveryAndDispatchPayment;

    type SourceHeaderChain = TestSourceHeaderChain;
    type MessageDispatch = TestMessageDispatch;
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


pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

pub struct CircuitToGateway;
impl Convert<Balance, u128> for CircuitToGateway {
    fn convert(val: Balance) -> u128 {
        val.into()
    }
}

impl Config for Test {
    type Event = Event;
    type AuthorityId = crypto::TestAuthId;
    type Call = Call;
    type GracePeriod = GracePeriod;
    type UnsignedInterval = UnsignedInterval;
    type UnsignedPriority = UnsignedPriority;
    type AccountId32Converter = AccountId32Converter;
    type ToStandardizedGatewayBalance = CircuitToGateway;
}


#[test]
fn it_submits_empty_composable_exec_request() {
    sp_io::TestExternalities::default().execute_with(|| {
        assert_err!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            vec![],
            vec![]
        ),
        "empty parameters submitted for execution order");
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

        assert_eq!(
            "component1, component2 | component3;",
            std::str::from_utf8(&io_bytes[..]).unwrap()
        );
        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            io_bytes,
            vec![
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        ));
    });
}

#[test]
fn decodes_external_substrate_dispatches() {
    sp_io::TestExternalities::default().execute_with(|| {
        // That's IO schedule.
        // We can have several parallel steps (e.g. separated with comma) and several sequential phases (e.g. separated with |)
        // ToDo: Find best way to format and process the IO schedule of cross-chain execution.
        // In the future each component will be following XCM MultiLocation format https://github.com/paritytech/polkadot/blob/master/xcm/src/v0/junction.rs
        let io_bytes: Vec<u8> = b"component1 | component2;".to_vec();

        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            io_bytes,
            vec![
                Compose {
                    name: b"component1".to_vec(),
                    code_txt: r#"
                        promise_auto(
                            call(System, remark)
                            call("TransferMultiAsset", (asset, escrow_dest)
                        )
                        promise_dispatch([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ]);
                        promise_rpc([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                        promise_xcmp([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                        calls.xcmp.send_message(xcmp( call( "TransferMultiAsset", (asset, escrow_dest) ) )
                        promise_vm_exec([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                    "#.as_bytes().to_vec(),
                    gateway_id: [0 as u8; 4],
                    exec_type: b"xt_prog".to_vec(),
                    dest: sp_core::sr25519::Public::from_slice(&[1 as u8; 32]),
                    value: 0,
                    bytes: vec![],
                    input_data: vec![],
                },
                Compose {
                    name: b"component2".to_vec(),
                    code_txt: r#"

                    "#.as_bytes().to_vec(),
                    gateway_id: [0 as u8; 4],
                    exec_type: b"xt_prog".to_vec(),
                    dest: sp_core::sr25519::Public::from_slice(&[1 as u8; 32]),
                    value: 0,
                    bytes: vec![],
                    input_data: vec![],
                },
            ]
        ));
    });
}

#[test]
fn decodes_external_substrate_dispatches_with_imports() {
    sp_io::TestExternalities::default().execute_with(|| {
        // That's IO schedule.
        // We can have several parallel steps (e.g. separated with comma) and several sequential phases (e.g. separated with |)
        // ToDo: Find best way to format and process the IO schedule of cross-chain execution.
        // In the future each component will be following XCM MultiLocation format https://github.com/paritytech/polkadot/blob/master/xcm/src/v0/junction.rs
        let io_bytes: Vec<u8> = b"component1;".to_vec();

        assert_ok!(ExecDelivery::submit_composable_exec_order(
            Origin::signed(Default::default()),
            io_bytes,
            vec![
                Compose {
                    name: b"component1".to_vec(),
                    code_txt: r#"

                    t3rn::xdns::chain::{Acala};

                    acala = t3rn_sdk::prog_gateway::ext(Acala::id);


                    #[cfg(paid=”10% fees”, name=bob_btc_uni_swap)]
                    Circuit::BTC::tx_only::on_escrowed_transfer(&self, | user, x_btc, pair| {
                        let price = cmc100::ext::call_static::get_price(‘BTC’ + pair);
                        let bob = self.owner;
                            let btc = price * btc_swap;
                        let eth = acala::ext::escrow_swap(‘BTC/ETH’, btc, bob);
                        let amount= eth::ext::uniswap::swap(‘ETH + ‘pair’, eth, bob);
                        eth::ext::transfer_dirty(bob, user, amount, pair)
                        Ok()
                    });

                        promise_auto(
                            call(System, remark)
                            call("TransferMultiAsset", (asset, escrow_dest)
                        )
                        promise_dispatch([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ]);
                        promise_rpc([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                        promise_xcmp([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                        calls.xcmp.send_message(xcmp( call( "TransferMultiAsset", (asset, escrow_dest) ) )
                        promise_vm_exec([
                            call("System", "remark", [1]),
                            call( "TransferMultiAsset", (asset, escrow_dest) ),
                        ])
                    "#.as_bytes().to_vec(),
                    gateway_id: [0 as u8; 4],
                    exec_type: b"exec_escrow".to_vec(),
                    dest: sp_core::sr25519::Public::from_slice(&[1 as u8; 32]),
                    value: 0,
                    bytes: vec![],
                    input_data: vec![],
                }
            ]
        ));
    });
}


#[test]
fn can_say_hello_from_pallet_implementation() {
    assert_eq!("hello", ExecDelivery::say_hello());
}
