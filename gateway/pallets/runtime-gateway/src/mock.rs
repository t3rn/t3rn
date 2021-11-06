// Creating mock runtime here

use crate::Config;

use frame_support::{parameter_types, traits::Get, weights::Weight};

use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Convert, IdentityLookup},
    DispatchResult, Perbill,
};
use std::cell::RefCell;
use t3rn_primitives::{transfers::BalanceOf, EscrowTrait};

use sp_runtime::AccountId32;

use frame_support::dispatch::DispatchErrorWithPostInfo;
use frame_support::pallet_prelude::*;
use frame_support::weights::PostDispatchInfo;

use versatile_wasm::{DispatchRuntimeCall, VersatileWasm};

use crate as pallet_runtime_gateway;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type WeightsCall = weights::Call<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
        VersatileWasmVM: versatile_wasm::{Pallet, Call, Event<T>},
        EscrowGateway: pallet_runtime_gateway::{Pallet, Call, Storage, Event<T>},
        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},

        Flipper: flipper::{Pallet, Call},
        Weights: weights::{Pallet, Call},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u64 = 16;
    pub const StorageSizeOffset: u32 = 8;
    pub const RentByteFee: u64 = 4;
    pub const RentDepositOffset: u64 = 10_000;
    pub const SurchargeReward: u64 = 150;
    pub const MaxDepth: u32 = 100;
    pub const MaxValueSize: u32 = 16_384;
}

pub struct ExampleDispatchRuntimeCall;

impl DispatchRuntimeCall<Test> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        input: &[u8],
        escrow_account: &<Test as frame_system::Config>::AccountId,
        _requested: &<Test as frame_system::Config>::AccountId,
        _callee: &<Test as frame_system::Config>::AccountId,
        _value: BalanceOf<Test>,
        gas_meter: &mut versatile_wasm::gas::GasMeter<Test>,
    ) -> DispatchResult {
        let res = match (module_name, fn_name) {
            ("Flipper", "flip") => Flipper::flip(Origin::signed(escrow_account.clone())),
            ("Weights", "store_value") => {
                let decoded_input: u32 = match Decode::decode(&mut input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
                    WeightsCall::store_value(decoded_input),
                )))?;
                // Alternatively use the call - call.dispatch((Origin::signed(*escrow_account))).map_err(|e| e.error)?;
                Weights::store_value(Origin::signed(escrow_account.clone()), decoded_input)
            }
            ("Weights", "double") => {
                let decoded_input: u32 = match Decode::decode(&mut input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(WeightsCall::double(
                    decoded_input,
                ))))?;
                Weights::double(Origin::signed(escrow_account.clone()), decoded_input)
            }
            ("Weights", "complex_calculations") => {
                let (decoded_x, decoded_y): (u32, u32) = match Decode::decode(&mut input.clone()) {
                    Ok(dec) => dec,
                    Err(_) => {
                        return Err(DispatchError::Other(
                            "Can't decode input for Weights::store_value. Expected u32.",
                        ));
                    }
                };
                gas_meter.charge_runtime_dispatch(Box::new(Call::Weights(
                    WeightsCall::complex_calculations(decoded_x, decoded_y),
                )))?;
                Weights::complex_calculations(
                    Origin::signed(escrow_account.clone()),
                    decoded_x,
                    decoded_y,
                )
            }
            (_, _) => Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo {
                    actual_weight: Some(0),
                    pays_fee: Default::default(),
                },
                error: "Call to unrecognized runtime function".into(),
            }),
        };

        match res {
            Ok(_res_with_post_info) => Ok(()),
            Err(err_with_post_info) => Err(err_with_post_info.error.into()),
        }
        // Ok(())
    }
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}

thread_local! {
    static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
    fn get() -> u64 {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
    }
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const TransactionByteFee: u64 = 1;
}

use frame_support::weights::IdentityFee;
impl pallet_transaction_payment::Config for Test {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

/** Bridge Messages - start **/
use bp_messages::{
    source_chain::{
        LaneMessageVerifier, MessageDeliveryAndDispatchPayment, RelayersRewards, Sender,
        TargetHeaderChain,
    },
    target_chain::{
        DispatchMessage, MessageDispatch, ProvedLaneMessages, ProvedMessages, SourceHeaderChain,
    },
    InboundLaneData, LaneId, Message, MessageData, MessageKey, MessageNonce, OutboundLaneData,
    Parameter as MessagesParameter,
};
use sp_std::collections::btree_map::BTreeMap;

pub type AccountId = sp_runtime::AccountId32;
use bp_runtime::Size;
use sp_runtime::FixedU128;
// start of bridge messages impl parameters
parameter_types! {
    pub const MaxMessagesToPruneAtOnce: u64 = 10;
    pub const MaxUnrewardedRelayerEntriesAtInboundLane: u64 = 16;
    pub const MaxUnconfirmedMessagesAtInboundLane: u64 = 32;
    pub storage TokenConversionRate: FixedU128 = 1.into();
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum TestMessagesParameter {
    TokenConversionRate(FixedU128),
}

impl MessagesParameter for TestMessagesParameter {
    fn save(&self) {
        match *self {
            TestMessagesParameter::TokenConversionRate(conversion_rate) => {
                TokenConversionRate::set(&conversion_rate)
            }
        }
    }
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
        AccountId::decode(&mut &hash.as_bytes()[..]).unwrap_or_default()
    }
}

/// Error that is returned by all test implementations.
pub const TEST_ERROR: &str = "Test error";

/// Lane that we're using in tests.
pub const _TEST_LANE_ID: LaneId = [0, 0, 0, 1];

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
                let mut messages_by_lane: BTreeMap<
                    LaneId,
                    ProvedLaneMessages<Message<TestMessageFee>>,
                > = BTreeMap::new();
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
        frame_support::storage::unhashed::get(b":message-fee:")
            == Some((Sender::Signed(submitter), fee))
    }

    /// Returns true if given relayer has been rewarded with given balance. The reward-paid flag is
    /// cleared after the call.
    pub fn is_reward_paid(relayer: AccountId, fee: TestMessageFee) -> bool {
        let key = (b":relayer-reward:", relayer, fee).encode();
        frame_support::storage::unhashed::take::<bool>(&key).is_some()
    }
}

impl MessageDeliveryAndDispatchPayment<AccountId, TestMessageFee>
    for TestMessageDeliveryAndDispatchPayment
{
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
pub fn _message(nonce: MessageNonce, payload: TestPayload) -> Message<TestMessageFee> {
    Message {
        key: MessageKey {
            lane_id: _TEST_LANE_ID,
            nonce,
        },
        data: _message_data(payload),
    }
}

/// Return message data with valid fee for given payload.
pub fn _message_data(payload: TestPayload) -> MessageData<TestMessageFee> {
    MessageData {
        payload: payload.encode(),
        fee: 1,
    }
}

impl pallet_bridge_messages::Config for Test {
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

/** Bridge Messages - end **/

/** Balances -- end **/
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w
    }
}

impl weights::Config for Test {}

impl flipper::Config for Test {}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Version = ();
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl pallet_sudo::Config for Test {
    type Event = Event;
    type Call = Call;
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub MyVVMSchedule: versatile_wasm::Schedule = <versatile_wasm::simple_schedule_v2::Schedule>::default();
}

impl VersatileWasm for Test {
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type Event = Event;
    type Call = Call;
    type Randomness = Randomness;
    type CallStack = [versatile_wasm::call_stack::Frame<Self>; 31];
    type WeightPrice = Self;
    type Schedule = MyVVMSchedule;
}

impl Config for Test {
    type Event = Event;
}

pub struct ExtBuilder {
    existential_deposit: u64,
}
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
        }
    }
}
impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }
    pub fn build(self, escrow_account: AccountId32) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();
        pallet_sudo::GenesisConfig::<Test> {
            key: escrow_account,
        }
        .assimilate_storage(&mut t)
        .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn new_test_ext_builder(deposit: u64, escrow_account: AccountId32) -> sp_io::TestExternalities {
    ExtBuilder::default()
        .existential_deposit(deposit)
        .build(escrow_account)
}
