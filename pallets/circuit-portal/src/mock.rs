//! Test utilities
use crate::{self as pallet_circuit_portal, Config};

use codec::Encode;

use pallet_babe::EquivocationHandler;
use pallet_babe::ExternalTrigger;

use sp_runtime::{
    curve::PiecewiseLinear,
    impl_opaque_keys,
    testing::{Header, TestXt},
    traits::{IdentityLookup, OpaqueKeys},
    Perbill,
};
use sp_runtime::{testing::UintAuthorityId, traits::Convert};

use frame_support::pallet_prelude::GenesisBuild;
use frame_support::{
    parameter_types,
    traits::{ConstU32, Everything, KeyOwnerProofSystem},
};

use frame_election_provider_support::onchain;
use pallet_session::historical as pallet_session_historical;
use pallet_staking::EraIndex;
use sp_consensus_babe::AuthorityId;
use sp_staking::SessionIndex;

use frame_support::{weights::Weight, PalletId};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::traits::{BlakeTwo256, Keccak256};

use pallet_evm::{AddressMapping, FeeCalculator};
use pallet_xdns::XdnsRecord;
use t3rn_primitives::transfers::BalanceOf;
use t3rn_primitives::EscrowTrait;
use t3rn_primitives::{GatewaySysProps, GatewayType, GatewayVendor};
use t3rn_protocol::side_effects::confirm::ethereum::EthereumMockVerifier;

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
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Historical: pallet_session_historical::{Pallet},
        Offences: pallet_offences::{Pallet, Storage, Event},
        MultiFinalityVerifier: pallet_multi_finality_verifier::{Pallet},

        Babe: pallet_babe::{Pallet, Call, Storage, Config},
        TransactionPayment: pallet_transaction_payment::{Pallet},
        Staking: pallet_staking::{Pallet, Call, Storage, Config<T>, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        ImOnline: pallet_im_online::{Pallet, Call, Storage, Config<T>, Event<T>},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},

        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Storage, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        Portal: pallet_circuit_portal::{Pallet, Call, Storage, Event<T>},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
        BasicOutboundChannel: snowbridge_basic_channel::outbound::{Pallet, Config<T>, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

/// The hashing algorithm used.
pub type Hashing = BlakeTwo256;

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
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
    pub const TransactionByteFee: u64 = 1;
    pub const OperationalFeeMultiplier: u8 = 5;

}

use frame_support::weights::IdentityFee;
use t3rn_protocol::side_effects::confirm::ethereum::EthereumMockVerifier;

impl pallet_transaction_payment::Config for Test {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

impl EscrowTrait for Test {
    type Currency = Balances;
    type Time = Timestamp;
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

impl pallet_session::Config for Test {
    type Event = Event;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
    type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = MockSessionKeys;
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

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
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
    type Accuracy = Perbill;
    type DataProvider = Staking;
}

parameter_types! {
    pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(75);
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
    type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
    type SortedListProvider = pallet_staking::UseNominatorsMap<Self>;
    type WeightInfo = ();
    type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;
    type GenesisElectionProvider = Self::ElectionProvider;
}

impl pallet_offences::Config for Test {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
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

parameter_types! {
    pub const ExecPalletId: PalletId = PalletId(*b"pal/exec");
}

impl Config for Test {
    type Event = Event;
    type Call = Call;
    type EthVerifier = EthereumMockVerifier;
    type AccountId32Converter = AccountId32Converter;
    type ToStandardizedGatewayBalance = CircuitToGateway;
    type WeightInfo = ();
    type PalletId = ExecPalletId;
    type EthVerifier = EthereumMockVerifier;
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}

impl pallet_im_online::Config for Test {
    type AuthorityId = UintAuthorityId;
    type Event = Event;
    type ValidatorSet = Historical;
    type NextSessionRotation = ();
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = UnsignedPriority;
    type WeightInfo = ();
    type MaxKeys = ConstU32<10_000>;
    type MaxPeerInHeartbeats = ConstU32<10_000>;
    type MaxPeerDataEncodingSize = ConstU32<1_000>;
}

// start of contracts VMs

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w.into()
    }
}

// Pallet Contracts

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u128 = 16;
    pub const ContractDeposit: u64 = 16;
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

impl pallet_contracts::Config for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = Event;
    type Call = Call;
    type CallFilter = frame_support::traits::Nothing;
    type ContractDeposit = ContractDeposit;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = MySchedule;
    type CallStack = [pallet_contracts::Frame<Self>; 31];
}

pub const INDEXING_PREFIX: &'static [u8] = b"commitment";
parameter_types! {
    pub const MaxMessagePayloadSize: u64 = 256;
    pub const MaxMessagesPerCommit: u64 = 20;
}

impl snowbridge_basic_channel::outbound::Config for Test {
    type Event = Event;
    const INDEXING_PREFIX: &'static [u8] = INDEXING_PREFIX;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type SetPrincipalOrigin = crate::EnsureCircuitPortal<Test>;
    type WeightInfo = ();
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
    type DisabledValidators = Session;

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
    type MaxAuthorities = ConstU32<10>;
}

pub struct ExtBuilder {
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
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("T3RN"),
                token_decimals: 12,
            },
            vec![],
        );
        let zero_xdns_record = <XdnsRecord<AccountId>>::new(
            vec![],
            [0u8, 0u8, 0u8, 0u8],
            Default::default(),
            GatewayVendor::Substrate,
            GatewayType::ProgrammableExternal(0),
            Default::default(),
            GatewaySysProps {
                ss58_format: 1333,
                token_symbol: Encode::encode("ZERO"),
                token_decimals: 0,
            },
            vec![],
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
            zero_xdns_record,
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
