//! Test utilities
use crate::{self as pallet_circuit, Config};

use codec::Encode;

use pallet_babe::EquivocationHandler;
use pallet_babe::ExternalTrigger;

use frame_support::pallet_prelude::GenesisBuild;
use frame_support::{
    parameter_types,
    traits::{ConstU32, Everything, KeyOwnerProofSystem, Nothing},
};
use sp_runtime::traits::Convert;
use sp_runtime::{
    curve::PiecewiseLinear,
    impl_opaque_keys,
    testing::{Header, TestXt},
    traits::{IdentityLookup, OpaqueKeys},
    Perbill,
};

use frame_election_provider_support::onchain;
use pallet_session::historical as pallet_session_historical;
use sp_consensus_babe::AuthorityId;
use sp_staking::EraIndex;
use sp_staking::SessionIndex;

use frame_support::{weights::Weight, PalletId};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::traits::{BlakeTwo256, Keccak256};

use pallet_xdns::{SideEffectInterface, XdnsRecord};
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
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},

        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        CircuitPortal: pallet_circuit_portal::{Pallet, Call, Storage, Event<T>},
        // BasicOutboundChannel: snowbridge_basic_channel::outbound::{Pallet, Config<T>, Storage, Event<T>},

        ORMLTokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},

        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>},
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
    type MaxConsumers = frame_support::traits::ConstU32<16>;
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

// ORML Tokens
use orml_traits::parameter_type_with_key;
use t3rn_primitives::abi::Type;

pub type CurrencyId = u32;
parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        Default::default()
    };
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = ();
    type DustRemovalWhitelist = Nothing;
}

impl pallet_xdns::Config for Test {
    type Event = Event;
    type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

pub type Balance = u64;
pub type Amount = i64;

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
    pub const ExistentialDeposit: Balance = 1;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = Balance;
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

parameter_types! {
    pub static MaxNominations: u32 = 16;
}

impl pallet_staking::Config for Test {
    type MaxNominations = MaxNominations;
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
    // type MaxUnlockingChunks = ConstU32<32>;
    type BenchmarkingConfig = pallet_staking::TestBenchmarkingConfig;
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

impl pallet_circuit_portal::Config for Test {
    type Event = Event;
    type Call = Call;
    type EthVerifier = EthereumMockVerifier;
    type AccountId32Converter = ();
    type ToStandardizedGatewayBalance = ();
    type WeightInfo = ();
    type PalletId = ExecPalletId;
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}

// start of contracts VMs

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w
    }
}

pub const INDEXING_PREFIX: &[u8] = b"commitment";
parameter_types! {
    pub const MaxMessagePayloadSize: u64 = 256;
    pub const MaxMessagesPerCommit: u64 = 20;
}

// impl snowbridge_basic_channel::outbound::Config for Test {
//     type Event = Event;
//     const INDEXING_PREFIX: &'static [u8] = INDEXING_PREFIX;
//     type Hashing = Keccak256;
//     type MaxMessagePayloadSize = MaxMessagePayloadSize;
//     type MaxMessagesPerCommit = MaxMessagesPerCommit;
//     type SetPrincipalOrigin = pallet_circuit_portal::EnsureCircuitPortal<Test>;
//     type WeightInfo = ();
// }

type Blake2ValU64BridgeInstance = ();
type Blake2ValU32BridgeInstance = pallet_multi_finality_verifier::Instance1;
type Keccak256ValU64BridgeInstance = pallet_multi_finality_verifier::Instance2;
type Keccak256ValU32BridgeInstance = pallet_multi_finality_verifier::Instance3;

#[derive(Debug)]
pub struct Blake2ValU64Chain;
impl t3rn_primitives::bridges::runtime::Chain for Blake2ValU64Chain {
    type BlockNumber = <Test as frame_system::Config>::BlockNumber;
    type Hash = <Test as frame_system::Config>::Hash;
    type Hasher = <Test as frame_system::Config>::Hashing;
    type Header = <Test as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl t3rn_primitives::bridges::runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

#[derive(Debug)]
pub struct Keccak256ValU64Chain;
impl t3rn_primitives::bridges::runtime::Chain for Keccak256ValU64Chain {
    type BlockNumber = u64;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u64, Keccak256>;
}

#[derive(Debug)]
pub struct Keccak256ValU32Chain;
impl t3rn_primitives::bridges::runtime::Chain for Keccak256ValU32Chain {
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

#[derive(Default)]
pub struct ExtBuilder {
    known_xdns_records: Vec<XdnsRecord<AccountId>>,
    standard_side_effects: Vec<SideEffectInterface>,
}

parameter_types! {
    pub const CircuitPalletId: PalletId = PalletId(*b"pal/circ");
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl Config for Test {
    type Event = Event;
    type Call = Call;
    type WeightInfo = ();
    type PalletId = CircuitPalletId;
    type SelfGatewayId = SelfGatewayId;
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
            vec![*b"tran"],
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
            vec![*b"tran"],
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
            vec![*b"tran", *b"swap"],
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
            vec![*b"tran"],
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
        //
        // map side_effects to id, keeping lib.rs clean
        self.standard_side_effects = vec![
            transfer_side_effect.clone(),
            swap_side_effect.clone(),
            add_liquidity_side_effect,
            call_evm_side_effect,
            get_data_side_effect,
        ];

        self
    }

    pub(crate) fn get_transfer_protocol_box() -> Box<SideEffectInterface> {
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
        Box::new(transfer_side_effect)
    }

    pub(crate) fn get_swap_protocol_box() -> Box<SideEffectInterface> {
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
        Box::new(swap_side_effect)
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
        .expect("Pallet xdns can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
