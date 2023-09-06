use crate as pallet_3vm;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32},
};
use frame_system as system;

use circuit_runtime_types::AssetId;
use frame_support::{
    pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
    traits::AsEnsureOriginWithArg,
};
use frame_system::EnsureRoot;
use pallet_grandpa_finality_verifier::light_clients::{
    select_grandpa_light_client_instance, KusamaInstance, LightClient, PolkadotInstance,
    RococoInstance,
};
use pallet_portal::Error as PortalError;
use sp_std::boxed::Box;
use t3rn_primitives::{GatewayVendor, TreasuryAccount, TreasuryAccountProvider};

use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, ConvertInto, IdentityLookup, Keccak256},
    AccountId32, DispatchError,
};
use t3rn_primitives::xdns::PalletAssetsOverlay;

type Header = generic::Header<u32, BlakeTwo256>;
pub type AccountId = u64;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DJANGO: AccountId = 4;
pub const FRED: AccountId = 5;
pub const ESCROW: AccountId = 15;

pub type BlockNumber = u32;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = sp_runtime::generic::Block<
    sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>,
    frame_system::mocking::MockUncheckedExtrinsic<Test>,
>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        ThreeVm: pallet_3vm,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Assets: pallet_assets,
        Utility: pallet_utility,
        ContractsRegistry: pallet_contracts_registry,
        Sudo: pallet_sudo,
        Circuit: pallet_circuit,
        Portal: pallet_portal,
        Xdns: pallet_xdns,
        AccountManager: pallet_account_manager,
        RococoBridge: pallet_grandpa_finality_verifier,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1>,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2>,
    }
);

impl system::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
    type AccountId = u64;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = Block;
    type BlockHashCount = ConstU32<250>;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = Keccak256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = ConstU32<16>;
    type Nonce = u64;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type SS58Prefix = ConstU16<42>;
    type SystemWeightInfo = ();
    type Version = ();
}

impl pallet_sudo::Config for Test {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxHolds = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = ();
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
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
    pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];
    pub const CircuitTargetIdOptimistic: t3rn_primitives::ChainId = [0, 3, 3, 3];
    pub EscrowAccount: AccountId = ESCROW;
}

parameter_types! {
    pub const AssetDeposit: Balance = 0; // 1 UNIT deposit to create asset
    pub const ApprovalDeposit: Balance = 0;
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
    type AssetId = circuit_runtime_types::AssetId;
    type AssetIdParameter = circuit_runtime_types::AssetId;
    type Balance = Balance;
    type CallbackHandle = ();
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
    type Currency = Balances;
    type Extra = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type RemoveItemsLimit = ConstU32<1>;
    type RuntimeEvent = RuntimeEvent;
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
    type OnLocalTrigger = Circuit;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SignalBounceThreshold = ConstU32<2>;
}

impl pallet_contracts_registry::Config for Test {
    type Balances = Balances;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_account_manager::Config for Test {
    type AssetBalanceOf = ConvertInto;
    type AssetId = u32;
    type Assets = Assets;
    type Clock = t3rn_primitives::clock::ClockMock<Self>;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type RuntimeEvent = RuntimeEvent;
    type Time = Timestamp;
    type WeightInfo = ();
}

parameter_types! {
    pub const CircuitAccountId: AccountId = 33;
}

impl pallet_circuit::Config for Test {
    type AccountManager = AccountManager;
    type Attesters =
        t3rn_primitives::attesters::AttestersReadApiEmptyMock<AccountId, Balance, DispatchError>;
    type Balances = Balances;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<1024>;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SFXBiddingPeriod = ConstU32<3>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = CircuitTargetId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<4>;
    type TreasuryAccounts = Test;
    type WeightInfo = ();
    type Xdns = Xdns;
    type XtxTimeoutCheckInterval = ConstU32<1024>;
    type XtxTimeoutDefault = ConstU32<1024>;
}

impl TreasuryAccountProvider<AccountId> for Test {
    fn get_treasury_account(_treasury_account: TreasuryAccount) -> AccountId {
        CircuitAccountId::get()
    }
}

// There are no tests in 3VM testing the XDNS Assets Overlay, so safe to mock with false values
impl PalletAssetsOverlay<Test, Balance> for Test {
    fn contains_asset(_asset_id: &AssetId) -> bool {
        false
    }

    fn force_create_asset(
        _origin: RuntimeOrigin,
        _asset_id: AssetId,
        _admin: AccountId,
        _is_sufficient: bool,
        _min_balance: Balance,
    ) -> DispatchResult {
        Err("Mock PalletAssetsOverlay::force_create_asset - not implemented".into())
    }

    fn destroy(_origin: RuntimeOrigin, _asset_id: &AssetId) -> DispatchResultWithPostInfo {
        Err("Mock PalletAssetsOverlay::destroy - not implemented".into())
    }
}

impl pallet_xdns::Config for Test {
    type AssetsOverlay = Test;
    type AttestersRead =
        t3rn_primitives::attesters::AttestersReadApiEmptyMock<AccountId, Balance, DispatchError>;
    type Balances = Balances;
    type CircuitDLQ = Circuit;
    type Currency = Balances;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SelfGatewayId = CircuitTargetId;
    type SelfTokenId = ConstU32<3333>;
    type Time = Timestamp;
    type TreasuryAccounts = Test;
    type WeightInfo = ();
}

pub type CurrencyId = u32;
pub type Balance = u64;
pub type Amount = i64;

parameter_types! {
    pub const HeadersToStore: u32 = 100;
    pub const RococoVendor: GatewayVendor = GatewayVendor::Rococo;
    pub const KusamaVendor: GatewayVendor = GatewayVendor::Kusama;
    pub const PolkadotVendor: GatewayVendor = GatewayVendor::Polkadot;
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
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = Xdns;
    type MyVendor = RococoVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<PolkadotInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = Xdns;
    type MyVendor = PolkadotVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<KusamaInstance> for Test {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = Xdns;
    type MyVendor = KusamaVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
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
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type SelectLightClient = SelectLightClientRegistry;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Test>;
    type Xdns = Xdns;
}
use sp_runtime::BuildStorage;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
