use crate::tests::*;
use frame_support::{
    pallet_prelude::{DispatchError, DispatchResult},
    parameter_types,
    traits::ConstU32,
};
use pallet_grandpa_finality_verifier::light_clients::{
    select_grandpa_light_client_instance, KusamaInstance, LightClient, PolkadotInstance,
    RococoInstance,
};
use sp_std::boxed::Box;
use t3rn_primitives::GatewayVendor;

pub type AssetId = u32;
use pallet_portal::Error as PortalError;
use sp_runtime::traits::{BlakeTwo256, ConvertInto};
use t3rn_primitives::xdns::PalletAssetsOverlay;

parameter_types! {
    pub const CreateSideEffectsPrecompileDest: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
    pub const CircuitTargetId: t3rn_primitives::ChainId = [3, 3, 3, 3];
    pub const CircuitTargetIdOptimistic: t3rn_primitives::ChainId = [0, 3, 3, 3];
    pub EscrowAccount: AccountId32 = AccountId32::new([15_u8; 32]);
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
    type Portal = CircuitPortal;
    type SignalBounceThreshold = ConstU32<2>;
}

impl pallet_sudo::Config for Test {
    type Call = Call;
    type Event = Event;
}

impl pallet_contracts_registry::Config for Test {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type WeightInfo = ();
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
    pub const CircuitAccountId: AccountId32 = AccountId32::new([51u8; 32]); // 0x333....3
}

impl pallet_circuit::Config for Test {
    type AccountManager = AccountManager;
    type Attesters =
        t3rn_primitives::attesters::AttestersReadApiEmptyMock<AccountId32, Balance, DispatchError>;
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

// There are no tests in 3VM testing the XDNS Assets Overlay, so safe to mock with false values
impl PalletAssetsOverlay<Test, Balance> for Test {
    fn contains_asset(asset_id: &AssetId) -> bool {
        false
    }

    fn force_create_asset(
        origin: Origin,
        asset_id: AssetId,
        admin: AccountId32,
        is_sufficient: bool,
        min_balance: Balance,
    ) -> DispatchResult {
        Err("Mock PalletAssetsOverlay::force_create_asset - not implemented".into())
    }

    fn destroy(origin: Origin, asset_id: &AssetId) -> DispatchResultWithPostInfo {
        Err("Mock PalletAssetsOverlay::destroy - not implemented".into())
    }
}

impl pallet_xdns::Config for Test {
    type AssetsOverlay = Test;
    type AttestersRead =
        t3rn_primitives::attesters::AttestersReadApiEmptyMock<AccountId32, Balance, DispatchError>;
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type Portal = CircuitPortal;
    type SelfGatewayIdEscrow = CircuitTargetId;
    type SelfGatewayIdOptimistic = CircuitTargetIdOptimistic;
    type SelfTokenId = ConstU32<3333>;
    type Time = Timestamp;
    type WeightInfo = ();
}

pub type CurrencyId = u32;
pub type Balance = u64;
pub type Amount = i64;

parameter_types! {
    pub const HeadersToStore: u32 = 100;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

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
    type Currency = Balances;
    type Event = Event;
    type SelectLightClient = SelectLightClientRegistry;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Test>;
    type Xdns = Xdns;
}
