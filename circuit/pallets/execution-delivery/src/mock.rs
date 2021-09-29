//! Test utilities
use crate::{self as pallet_execution_delivery, Config};

use codec::{Decode, Encode};

use pallet_babe::EquivocationHandler;
use pallet_babe::ExternalTrigger;

use bp_runtime::Size;
use sp_runtime::{
    curve::PiecewiseLinear,
    impl_opaque_keys,
    testing::{Header, TestXt},
    traits::{IdentityLookup, OpaqueKeys},
    Perbill,
};
use sp_runtime::{
    testing::UintAuthorityId, traits::Convert, DispatchError, DispatchResult, FixedU128,
};

use frame_support::pallet_prelude::GenesisBuild;
use frame_support::{parameter_types, traits::KeyOwnerProofSystem};

use frame_election_provider_support::onchain;
use pallet_session::historical as pallet_session_historical;
use pallet_staking::EraIndex;
use sp_consensus_babe::AuthorityId;
use sp_staking::SessionIndex;

use frame_support::weights::Weight;
use sp_core::{crypto::KeyTypeId, H160, H256, U256};
use sp_runtime::traits::{BlakeTwo256, Keccak256};

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

use pallet_xdns::XdnsRecord;
use std::collections::BTreeMap;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::EscrowTrait;
use t3rn_primitives::{GatewayType, GatewayVendor};
use volatile_vm::DispatchRuntimeCall;

use pallet_evm::{AddressMapping, FeeCalculator};

pub type AccountId = sp_runtime::AccountId32;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"circ");

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        ExecDelivery: pallet_execution_delivery::{Pallet, Call, Storage, Event<T>},
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Historical: pallet_session_historical::{Pallet},
        Offences: pallet_offences::{Pallet, Storage, Event},
        Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},
        MultiFinalityVerifier: pallet_multi_finality_verifier::{Pallet},

        Babe: pallet_babe::{Pallet, Call, Storage, Config},
        TransactionPayment: pallet_transaction_payment::{Pallet},
        Staking: pallet_staking::{Pallet, Call, Storage, Config<T>, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        ImOnline: pallet_im_online::{Pallet, Call, Storage, Config<T>, Event<T>},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
        VolatileVM: volatile_vm::{Pallet, Call, Event<T>, Storage},
        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Storage, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
        EVM: pallet_evm::{Pallet, Config, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(16);
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

//ToDo: Uncomment when upgrading to v4.0.0 substrate
// impl pallet_randomness_collective_flip::Config for Test {}

/// The hashing algorithm used.
pub type Hashing = BlakeTwo256;

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
    type Hashing = Hashing;
    // type AccountId = DummyValidatorId;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = TestXt<Call, ()>;
}

impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub babe_authority: pallet_babe::Pallet<Test>,
    }
}

impl pallet_sudo::Config for Test {
    type Event = Event;
    type Call = Call;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub const TransactionByteFee: u64 = 1;
}

use frame_support::weights::IdentityFee;
impl pallet_transaction_payment::Config for Test {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub MyScheduleVVM: volatile_vm::Schedule<Test> = <volatile_vm::Schedule<Test>>::default();
}

impl volatile_vm::VolatileVM for Test {
    type Randomness = Randomness;
    type Event = Event;
    type Call = Call;
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type CallStack = [volatile_vm::exec::Frame<Self>; 31];
    type ContractsLazyLoaded = [volatile_vm::wasm::PrefabWasmModule<Self>; 31];
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MyScheduleVVM;
}

impl pallet_contracts_registry::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}

impl pallet_xdns::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

pub type Balance = u128;

// parameter_types! {
// 	pub const ExistentialDeposit: u64 = 1;
// }

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
        _gas_meter: &mut volatile_vm::gas::GasMeter<Test>,
    ) -> DispatchResult {
        match (module_name, fn_name) {
            ("Weights", "complex_calculations") => {
                let (_decoded_x, _decoded_y): (u32, u32) = match Decode::decode(&mut _input.clone())
                {
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

impl pallet_session::Config for Test {
    type Event = Event;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
    type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = MockSessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = ();
}

impl pallet_session::historical::Config for Test {
    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Test>;
}

parameter_types! {
    pub const UncleGenerations: u64 = 0;
}

impl pallet_authorship::Config for Test {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u128;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

pallet_staking_reward_curve::build! {
    const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000u64,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}

parameter_types! {
    pub const SessionsPerEra: SessionIndex = 3;
    pub const BondingDuration: EraIndex = 3;
    pub const SlashDeferDuration: EraIndex = 0;
    pub const AttestationPeriod: u64 = 100;
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
    pub const MaxNominatorRewardedPerValidator: u32 = 64;
    pub const ElectionLookahead: u64 = 0;
    pub const StakingUnsignedPriority: u64 = u64::max_value() / 2;
}

impl onchain::Config for Test {
    type AccountId = <Self as frame_system::Config>::AccountId;
    type BlockNumber = <Self as frame_system::Config>::BlockNumber;
    type BlockWeights = ();
    type Accuracy = Perbill;
    type DataProvider = Staking;
}

impl pallet_staking::Config for Test {
    const MAX_NOMINATIONS: u32 = 16;
    type RewardRemainder = ();
    type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
    type Event = Event;
    type Currency = Balances;
    type Slash = ();
    type Reward = ();
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    type SlashCancelOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type SessionInterface = Self;
    type UnixTime = pallet_timestamp::Pallet<Test>;
    type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
    type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
    type NextNewSession = Session;
    type WeightInfo = ();
    type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;
    type GenesisElectionProvider = Self::ElectionProvider;
}

impl pallet_offences::Config for Test {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
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
    // type AuthorityId = crypto::TestAuthId;
    type Call = Call;
    // type GracePeriod = GracePeriod;
    // type UnsignedInterval = UnsignedInterval;
    // type UnsignedPriority = UnsignedPriority;
    type AccountId32Converter = AccountId32Converter;
    type ToStandardizedGatewayBalance = CircuitToGateway;
    type WeightInfo = ();
}

impl pallet_im_online::Config for Test {
    type AuthorityId = UintAuthorityId;
    // type AuthorityId = crypto::TestAuthId;
    type Event = Event;
    type ValidatorSet = Historical;
    type NextSessionRotation = ();
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = UnsignedPriority;
    type WeightInfo = ();
}
// start of contracts VMs

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u128 = 16;
    pub const DepositPerContract: u128 = 8 * DepositPerStorageByte::get();
    pub const DepositPerStorageByte: u128 = 10_000;
    pub const DepositPerStorageItem: u128 = 10_000;
    pub RentFraction: Perbill = Perbill::from_rational(4u32, 10_000u32);
    pub const SurchargeReward: u128 = 500_000;
    pub const MaxValueSize: u32 = 16_384;
    pub const DeletionQueueDepth: u32 = 1024;
    pub const DeletionWeightLimit: Weight = 500_000_000_000;
    pub const MaxCodeSize: u32 = 2 * 1024;
    pub MySchedule: pallet_contracts::Schedule<Test> = <pallet_contracts::Schedule<Test>>::default();
}

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w.into()
    }
}

impl pallet_contracts::Config for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = Event;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type CallStack = [pallet_contracts::Frame<Self>; 31];
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MySchedule;
}

// EVM

parameter_types! {
    pub const ChainId: u64 = 33;
    pub const BlockGasLimit: U256 = U256::MAX;
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        1.into()
    }
}
pub struct HashedAddressMapping;

impl AddressMapping<AccountId> for HashedAddressMapping {
    fn into_account_id(address: H160) -> AccountId {
        let mut data = [0u8; 32];
        data[0..20].copy_from_slice(&address[..]);
        AccountId::from(Into::<[u8; 32]>::into(data))
    }
}

impl pallet_evm::Config for Test {
    type FeeCalculator = FixedGasPrice;
    type GasWeightMapping = ();
    type CallOrigin = pallet_evm::EnsureAddressTruncated;
    type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
    type AddressMapping = HashedAddressMapping;
    type Currency = Balances;
    type Event = Event;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type Precompiles = (
        pallet_evm_precompile_simple::ECRecover,
        pallet_evm_precompile_simple::Sha256,
        pallet_evm_precompile_simple::Ripemd160,
        pallet_evm_precompile_simple::Identity,
    );
    type ChainId = ChainId;
    type BlockGasLimit = BlockGasLimit;
    type OnChargeTransaction = ();
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping;
}

// start of bridge messages
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

type Blake2ValU64BridgeInstance = ();
type Blake2ValU32BridgeInstance = pallet_multi_finality_verifier::Instance1;
type Keccak256ValU64BridgeInstance = pallet_multi_finality_verifier::Instance2;
type Keccak256ValU32BridgeInstance = pallet_multi_finality_verifier::Instance3;

#[derive(Debug)]
pub struct Blake2ValU64Chain;
impl bp_runtime::Chain for Blake2ValU64Chain {
    type BlockNumber = <Test as frame_system::Config>::BlockNumber;
    type Hash = <Test as frame_system::Config>::Hash;
    type Hasher = <Test as frame_system::Config>::Hashing;
    type Header = <Test as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

#[derive(Debug)]
pub struct Keccak256ValU64Chain;
impl bp_runtime::Chain for Keccak256ValU64Chain {
    type BlockNumber = u64;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u64, Keccak256>;
}

#[derive(Debug)]
pub struct Keccak256ValU32Chain;
impl bp_runtime::Chain for Keccak256ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u32, Keccak256>;
}

parameter_types! {
    pub const MaxRequests: u32 = 2;
    pub const HeadersToKeep: u32 = 5;
    pub const SessionLength: u64 = 5;
    pub const NumValidators: u32 = 5;
}

impl pallet_multi_finality_verifier::Config<Blake2ValU64BridgeInstance> for Test {
    type BridgedChain = Blake2ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Blake2ValU32BridgeInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU64BridgeInstance> for Test {
    type BridgedChain = Keccak256ValU64Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU32BridgeInstance> for Test {
    type BridgedChain = Keccak256ValU32Chain;
    type MaxRequests = MaxRequests;
    type HeadersToKeep = HeadersToKeep;
    type WeightInfo = ();
}

parameter_types! {
    pub const EpochDuration: u64 = 3;
    pub const ExpectedBlockTime: u64 = 1;
    pub const ReportLongevity: u64 =
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl pallet_babe::Config for Test {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = ExternalTrigger;

    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, AuthorityId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        AuthorityId,
    )>>::IdentificationTuple;

    type HandleEquivocation =
        EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;

    type WeightInfo = ();
}

pub(crate) struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            known_xdns_records: vec![],
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
        );
        let gateway_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"gate",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
        );
        let polkadot_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"pdot",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
        );
        let kusama_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            *b"ksma",
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
        );
        self.known_xdns_records = vec![
            circuit_xdns_record,
            gateway_xdns_record,
            polkadot_xdns_record,
            kusama_xdns_record,
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
        }
        .assimilate_storage(&mut t)
        .expect("Pallet xdns can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
