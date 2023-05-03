use crate as pallet_3vm;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64},
};
use frame_system as system;

use pallet_grandpa_finality_verifier::light_clients::{
    select_grandpa_light_client_instance, KusamaInstance, LightClient, PolkadotInstance,
    RococoInstance,
};
use pallet_portal::Error as PortalError;
use sp_std::boxed::Box;
use t3rn_primitives::GatewayVendor;

use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, ConvertInto, IdentityLookup},
    AccountId32,
};
type Header = generic::Header<u32, BlakeTwo256>;
pub type AccountId = u64;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DJANGO: AccountId = 4;
pub const FRED: AccountId = 5;
pub const ESCROW: AccountId = 15;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ThreeVm: pallet_3vm::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Assets: pallet_assets,
        Utility: pallet_utility::{Pallet, Call, Storage, Event},
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Storage, Config<T>, Event<T>},
        Sudo: pallet_sudo,
        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>},
        CircuitPortal: pallet_portal,
        Xdns: pallet_xdns,
        AccountManager: pallet_account_manager,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,
    }
);

impl system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = u64;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = ConstU32<250>;
    type BlockLength = ();
    type BlockNumber = u32;
    type BlockWeights = ();
    type Call = Call;
    type DbWeight = ();
    type Event = Event;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type Origin = Origin;
    type PalletInfo = PalletInfo;
    type SS58Prefix = ConstU16<42>;
    type SystemWeightInfo = ();
    type Version = ();
}

impl pallet_sudo::Config for Test {
    type Call = Call;
    type Event = Event;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ConstU64<1>;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

parameter_types! {
    pub MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
    // type MinimumPeriod = ConstU64<1>;
    type MinimumPeriod = MinimumPeriod;
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl pallet_utility::Config for Test {
    type Call = Call;
    type Event = Event;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = ();
}

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
    pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];
    pub EscrowAccount: AccountId = ESCROW;
}

parameter_types! {
    pub const AssetDeposit: Balance = 1; // 1 UNIT deposit to create asset
    pub const ApprovalDeposit: Balance = 1;
    pub const AssetsStringLimit: u32 = 50;
    /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
    // https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
    pub const AssetAccountDeposit: Balance = 0;
}

impl pallet_assets::Config for Test {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type AssetDeposit = AssetDeposit;
    type AssetId = u32;
    type Balance = Balance;
    type Currency = Balances;
    type Event = Event;
    type Extra = ();
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type StringLimit = AssetsStringLimit;
    type WeightInfo = ();
}

impl pallet_3vm::Config for Test {
    type AccountManager = AccountManager;
    type AssetId = u32;
    type CircuitTargetId = CircuitTargetId;
    type ContractsRegistry = ContractsRegistry;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type OnLocalTrigger = Circuit;
    type SignalBounceThreshold = ConstU32<2>;
}

impl pallet_contracts_registry::Config for Test {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type WeightInfo = ();
}

impl pallet_account_manager::Config for Test {
    type AssetBalanceOf = ConvertInto;
    type AssetId = u32;
    type Assets = Assets;
    type Clock = t3rn_primitives::clock::ClockMock<Self>;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Time = Timestamp;
    type WeightInfo = ();
}

parameter_types! {
    pub const CircuitAccountId: AccountId = 33;
}

impl pallet_circuit::Config for Test {
    type AccountManager = AccountManager;
    type Balances = Balances;
    type Call = Call;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<1024>;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = CircuitPortal;
    type SFXBiddingPeriod = ConstU32<3>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = CircuitTargetId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<4>;
    type WeightInfo = ();
    type Xdns = Xdns;
    type XtxTimeoutCheckInterval = ConstU32<1024>;
    type XtxTimeoutDefault = ConstU32<1024>;
}

impl pallet_xdns::Config for Test {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type Time = Timestamp;
    type WeightInfo = ();
}

pub type CurrencyId = u32;
pub type Balance = u64;
pub type Amount = i64;

parameter_types! {
    pub const HeadersToStore: u32 = 100;
}

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl pallet_grandpa_finality_verifier::bridges::runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

impl pallet_grandpa_finality_verifier::Config<RococoInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<PolkadotInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<KusamaInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type Event = Event;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExecPalletId: frame_support::PalletId = frame_support::PalletId(*b"pal/exec");
}

pub struct SelectLightClientRegistry;

impl pallet_portal::SelectLightClient<Test> for SelectLightClientRegistry {
    fn select(vendor: GatewayVendor) -> Result<Box<dyn LightClient<Test>>, PortalError<Test>> {
        match vendor {
            GatewayVendor::Rococo =>
                select_grandpa_light_client_instance::<Test, RococoInstance>(vendor)
                    .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
            GatewayVendor::Kusama =>
                select_grandpa_light_client_instance::<Test, KusamaInstance>(vendor)
                    .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
            GatewayVendor::Polkadot =>
                select_grandpa_light_client_instance::<Test, PolkadotInstance>(vendor)
                    .ok_or(PortalError::<Test>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Test>>),
            _ => Err(PortalError::<Test>::UnimplementedGatewayVendor),
        }
    }
}

impl pallet_portal::Config for Test {
    type Event = Event;
    type SelectLightClient = SelectLightClientRegistry;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Test>;
    type Xdns = Xdns;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
