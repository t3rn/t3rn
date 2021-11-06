#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_grandpa::fg_primitives;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::traits::{
    AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Verify,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

use codec::{Decode, Encode};
use sp_core::H256;
use sp_runtime::FixedU128;
use sp_std::collections::btree_map::BTreeMap;

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
use bp_runtime::Size;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node-template"),
    impl_name: create_runtime_str!("node-template"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
        ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = ();

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation = ();

    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 500;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

use frame_support::pallet_prelude::DispatchResult;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::EscrowTrait;

impl EscrowTrait for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

parameter_types! {
    pub MyVVMSchedule: versatile_wasm::Schedule = <versatile_wasm::simple_schedule_v2::Schedule>::default();
}

use sp_runtime::traits::Convert;
impl Convert<Weight, BalanceOf<Self>> for Runtime {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w
    }
}

pub struct ExampleDispatchRuntimeCall;

impl versatile_wasm::DispatchRuntimeCall<Runtime> for ExampleDispatchRuntimeCall {
    fn dispatch_runtime_call(
        _module_name: &str,
        _fn_name: &str,
        _input: &[u8],
        _escrow_account: &<Runtime as frame_system::Config>::AccountId,
        _requested: &<Runtime as frame_system::Config>::AccountId,
        _callee: &<Runtime as frame_system::Config>::AccountId,
        _value: BalanceOf<Runtime>,
        _gas_meter: &mut versatile_wasm::gas::GasMeter<Runtime>,
    ) -> DispatchResult {
        Ok(())
    }
}

impl versatile_wasm::VersatileWasm for Runtime {
    type DispatchRuntimeCall = ExampleDispatchRuntimeCall;
    type Event = Event;
    type Call = Call;
    type Randomness = RandomnessCollectiveFlip;
    type CallStack = [versatile_wasm::call_stack::Frame<Self>; 31];
    type WeightPrice = Self;
    type Schedule = MyVVMSchedule;
}

impl pallet_runtime_gateway::Config for Runtime {
    type Event = Event;
}

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

impl pallet_bridge_messages::Config for Runtime {
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

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Config<T>},
        Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},

        RuntimeGateway: pallet_runtime_gateway::{Pallet, Call, Storage, Event<T>},
        VersatileWasmVM: versatile_wasm::{Pallet, Call, Event<T>},
        Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},
        Utility: pallet_utility::{Pallet, Call, Event},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            hash: <Block as BlockT>::Hash
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
