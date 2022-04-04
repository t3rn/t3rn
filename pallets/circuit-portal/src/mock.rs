//! Test utilities
use crate::{self as pallet_circuit_portal, bp_runtime, Config};
use codec::Encode;

use sp_runtime::{
    testing::{Header, TestXt},
    traits::{Convert, IdentityLookup, OpaqueKeys},
    Perbill,
};

use frame_support::{pallet_prelude::GenesisBuild, parameter_types, traits::Everything};

use frame_support::{weights::Weight, PalletId};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::traits::{BlakeTwo256, Keccak256};

use t3rn_primitives::{
    abi::Type, side_effect::interface::SideEffectInterface, transfers::BalanceOf, xdns::XdnsRecord,
    EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor,
};
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
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        MultiFinalityVerifier: pallet_multi_finality_verifier::{Pallet},
        Sudo: pallet_sudo::{Pallet, Call, Event<T>},
        XDNS: pallet_xdns::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Portal: pallet_circuit_portal::{Pallet, Call, Storage, Event<T>},
        // BasicOutboundChannel: snowbridge_basic_channel::outbound::{Pallet, Config<T>, Storage, Event<T>},
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId;
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

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type Extrinsic = TestXt<Call, ()>;
    type OverarchingCall = Call;
}

impl pallet_sudo::Config for Test {
    type Call = Call;
    type Event = Event;
}

parameter_types! {
    pub const TransactionByteFee: u64 = 1;
    pub const OperationalFeeMultiplier: u8 = 5;

}

impl EscrowTrait<Test> for Test {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_xdns::Config for Test {
    type Balances = Balances;
    type Escrowed = Self;
    type Event = Event;
    type WeightInfo = ();
}

pub type Balance = u128;

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = u128;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

parameter_types! {
    pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(75);
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
        val
    }
}

parameter_types! {
    pub const ExecPalletId: PalletId = PalletId(*b"pal/exec");
}

impl Config for Test {
    type AccountId32Converter = AccountId32Converter;
    type Balances = Balances;
    type Call = Call;
    type Escrowed = Self;
    type EthVerifier = EthereumMockVerifier;
    type Event = Event;
    type PalletId = ExecPalletId;
    type ToStandardizedGatewayBalance = CircuitToGateway;
    type WeightInfo = ();
    type Xdns = XDNS;
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}

impl pallet_timestamp::Config for Test {
    type MinimumPeriod = MinimumPeriod;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w.into()
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
//     type SetPrincipalOrigin = crate::EnsureCircuitPortal<Test>;
//     type WeightInfo = ();
// }

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
    type Escrowed = Self;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_multi_finality_verifier::Config<Blake2ValU32BridgeInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type Escrowed = Self;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU64BridgeInstance> for Test {
    type BridgedChain = Keccak256ValU64Chain;
    type Escrowed = Self;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_multi_finality_verifier::Config<Keccak256ValU32BridgeInstance> for Test {
    type BridgedChain = Keccak256ValU32Chain;
    type Escrowed = Self;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

pub struct ExtBuilder {
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
        .expect("Pallet xdns can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
