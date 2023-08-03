use crate::*;
use codec::Encode;
use pallet_grandpa_finality_verifier::{
    bridges::runtime as bp_runtime,
    light_clients::{
        select_grandpa_light_client_instance, KusamaInstance, LightClient, PolkadotInstance,
        RococoInstance,
    },
};
use pallet_portal::Error as PortalError;
use sp_runtime::{DispatchResult, Percent};
use sp_std::{marker::PhantomData, prelude::*};

use sp_std::boxed::Box;

use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    parameter_types,
    traits::{fungibles::Destroy, ConstU32},
    weights::Weight,
    Blake2_128Concat, PalletId, StorageHasher,
};

use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, Convert, One, Saturating, Zero},
    Perbill,
};
use t3rn_primitives::GatewayVendor;

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

use crate::hooks::GlobalOnInitQueues;

parameter_types! {
    // TODO: update me to be better
    pub const EscrowAccount: AccountId = AccountId::new([51_u8; 32]);
    pub const RewardMultiplier: Balance = 1;
    pub const MinNominatorBond: Balance = 1;
    pub const MinAttesterBond: Balance = 1;
    pub const DefaultCommission: Percent = Percent::from_percent(10);
    pub const HourlyShufflingFrequency: BlockNumber = 60 * 60 / 12; // (60 * 60) / 12; assuming one distribution per two weeks
}

impl pallet_attesters::Config for Runtime {
    type ActiveSetSize = ConstU32<32>;
    type BatchingWindow = ConstU32<6>;
    type CommitmentRewardSource = EscrowAccount;
    type CommitteeSize = ConstU32<16>;
    type Currency = Balances;
    type DefaultCommission = DefaultCommission;
    type MaxBatchSize = ConstU32<128>;
    type MinAttesterBond = MinAttesterBond;
    type MinNominatorBond = MinNominatorBond;
    type Portal = Portal;
    type RandomnessSource = RandomnessCollectiveFlip;
    type ReadSFX = Circuit;
    type RepatriationPeriod = ConstU32<60>;
    type RewardMultiplier = RewardMultiplier;
    type Rewards = Rewards;
    type RuntimeEvent = RuntimeEvent;
    type ShufflingFrequency = HourlyShufflingFrequency;
    type SlashAccount = EscrowAccount;
    type Xdns = XDNS;
}

use t3rn_primitives::{monetary::TRN, xdns::PalletAssetsOverlay};

parameter_types! {
    pub const TotalInflation: Perbill = Perbill::from_parts(44_000_000); // 4.4%
    pub const AttesterInflation: Perbill = Perbill::from_parts(11_000_000); // 1.1%
    pub const ExecutorInflation: Perbill = Perbill::from_parts(8_000_000); // 0.8%
    pub const CollatorInflation: Perbill = Perbill::from_parts(5_000_000); // 0.5%
    pub const TreasuryInflation: Perbill = Perbill::from_parts(20_000_000); // 2%
    pub const AttesterBootstrapRewards: Percent = Percent::from_parts(40); // 40%
    pub const CollatorBootstrapRewards: Percent = Percent::from_parts(20); // 20%
    pub const ExecutorBootstrapRewards: Percent = Percent::from_parts(40); // 40%
    pub const StartingRepatriationPercentage: Percent = Percent::from_parts(10); // 10%
    pub const OneYear: BlockNumber = 2_628_000; // (365.25 * 24 * 60 * 60) / 12; assuming 12s block time
    pub const InflationDistributionPeriod: BlockNumber = 100_800; // (14 * 24 * 60 * 60) / 12; assuming one distribution per two weeks
    pub const HourlyInflationDistributionPeriod: BlockNumber = 60 * 60 / 12; // (60 * 60) / 12; assuming 12s block time
    pub const AvailableBootstrapSpenditure: Balance = 1_000_000 * (TRN as Balance); // 1 MLN UNIT
}

impl pallet_rewards::Config for Runtime {
    type AccountManager = AccountManager;
    type AttesterBootstrapRewards = AttesterBootstrapRewards;
    type AttesterInflation = AttesterInflation;
    type Attesters = Attesters;
    type AvailableBootstrapSpenditure = AvailableBootstrapSpenditure;
    type Clock = Clock;
    type CollatorBootstrapRewards = CollatorBootstrapRewards;
    type CollatorInflation = CollatorInflation;
    type Currency = Balances;
    type ExecutorBootstrapRewards = ExecutorBootstrapRewards;
    type ExecutorInflation = ExecutorInflation;
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type InflationDistributionPeriod = HourlyInflationDistributionPeriod;
    type OneYear = OneYear;
    type RuntimeEvent = RuntimeEvent;
    type StartingRepatriationPercentage = StartingRepatriationPercentage;
    type TotalInflation = TotalInflation;
    type TreasuryAccounts = Runtime;
    type TreasuryInflation = TreasuryInflation;
}

impl pallet_vacuum::Config for Runtime {
    type CircuitSubmitAPI = Circuit;
    type Currency = Balances;
    type ReadSFX = Circuit;
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_xdns::Config for Runtime {
    type AssetsOverlay = Runtime;
    type AttestersRead = Attesters;
    type Balances = Balances;
    type CircuitDLQ = Circuit;
    type Currency = Balances;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SelfGatewayId = SelfGatewayId;
    type SelfTokenId = ConstU32<3333>;
    type Time = Timestamp;
    type TreasuryAccounts = Runtime;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl PalletAssetsOverlay<Runtime, Balance> for Runtime {
    fn contains_asset(asset_id: &AssetId) -> bool {
        const PALLET_NAME: &str = "Assets";
        const STORAGE_NAME: &str = "Asset";
        type Index = u32;
        type Data = u32;

        let pallet_hash = sp_io::hashing::twox_128(PALLET_NAME.as_bytes());
        let storage_hash = sp_io::hashing::twox_128(STORAGE_NAME.as_bytes());
        // Hashing the scale-encoded key
        let key_hashed = Blake2_128Concat::hash(&asset_id.encode());

        let mut final_key = Vec::new();
        final_key.extend_from_slice(&pallet_hash);
        final_key.extend_from_slice(&storage_hash);
        final_key.extend_from_slice(&key_hashed);

        frame_support::storage::unhashed::get::<Data>(&final_key).is_some()
    }

    fn force_create_asset(
        origin: RuntimeOrigin,
        asset_id: AssetId,
        admin: AccountId,
        is_sufficient: bool,
        min_balance: Balance,
    ) -> DispatchResult {
        log::debug!("t0rn::force_create_asset");
        log::debug!("t0rn::asset_id: {asset_id:?}");
        log::debug!("t0rn::asset_admin: {admin:?}");
        Assets::force_create(
            origin,
            asset_id,
            sp_runtime::MultiAddress::Id(admin),
            is_sufficient,
            min_balance,
        )
    }

    fn destroy(origin: RuntimeOrigin, asset_id: &AssetId) -> DispatchResultWithPostInfo {
        log::debug!("t0rn::freeze_asset ...");
        Assets::freeze_asset(origin.clone(), asset_id.clone())?;
        log::debug!("t0rn::start_destroy ...");
        Assets::start_destroy(origin.clone(), asset_id.clone())?;
        log::debug!("t0rn::destroy_accounts ...");
        Assets::destroy_accounts(origin.clone(), asset_id.clone())?;
        log::debug!("t0rn::destroy_approvals ...");
        Assets::destroy_approvals(origin.clone(), asset_id.clone())?;
        log::debug!("t0rn::finish_destroy ...");
        Assets::finish_destroy(origin.clone(), asset_id.clone())?;

        Ok(().into())
    }
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
}

pub struct SelectLightClientRegistry;

impl pallet_portal::SelectLightClient<Runtime> for SelectLightClientRegistry {
    fn select(
        vendor: GatewayVendor,
    ) -> Result<Box<dyn LightClient<Runtime>>, PortalError<Runtime>> {
        match vendor {
            GatewayVendor::Rococo =>
                select_grandpa_light_client_instance::<Runtime, RococoInstance>(vendor)
                    .ok_or(PortalError::<Runtime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Runtime>>),
            GatewayVendor::Kusama =>
                select_grandpa_light_client_instance::<Runtime, KusamaInstance>(vendor)
                    .ok_or(PortalError::<Runtime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Runtime>>),
            GatewayVendor::Polkadot =>
                select_grandpa_light_client_instance::<Runtime, PolkadotInstance>(vendor)
                    .ok_or(PortalError::<Runtime>::LightClientNotFoundByVendor)
                    .map(|lc| Box::new(lc) as Box<dyn LightClient<Runtime>>),
            GatewayVendor::Ethereum => Ok(Box::new(
                pallet_eth2_finality_verifier::Pallet::<Runtime>(PhantomData),
            )),
            GatewayVendor::Sepolia => Ok(Box::new(pallet_sepolia_finality_verifier::Pallet::<
                Runtime,
            >(PhantomData))),
            _ => Err(PortalError::<Runtime>::LightClientNotFoundByVendor),
        }
    }
}

impl pallet_portal::Config for Runtime {
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type SelectLightClient = SelectLightClientRegistry;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Runtime>;
    type Xdns = XDNS;
}

parameter_types! {
    pub const PortalPalletId: PalletId = PalletId(*b"pal/port");
}
pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

parameter_types! {
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
    pub const SelfGatewayIdOptimistic: [u8; 4] = [0, 3, 3, 3];
}

impl pallet_circuit::Config for Runtime {
    type AccountManager = AccountManager;
    type Attesters = Attesters;
    type Balances = Balances;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<100u32>;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type RuntimeEvent = RuntimeEvent;
    type SFXBiddingPeriod = ConstU32<3u32>;
    type SelfAccountId = crate::accounts_config::EscrowAccount;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    // type XBIPortal = XBIPortalRuntimeEntry;
    // type XBIPromise = XBIPortal;
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<10u32>;
    type XtxTimeoutDefault = ConstU32<400u32>;
}

parameter_types! {
    pub const HeadersToStore: u32 = 100;
    pub const RococoVendor: GatewayVendor = GatewayVendor::Rococo;
    pub const KusamaVendor: GatewayVendor = GatewayVendor::Kusama;
    pub const PolkadotVendor: GatewayVendor = GatewayVendor::Polkadot;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = generic::Header<u32, BlakeTwo256>;
}

impl pallet_grandpa_finality_verifier::Config<RococoInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = XDNS;
    type MyVendor = RococoVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<PolkadotInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = XDNS;
    type MyVendor = PolkadotVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<KusamaInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type EpochOffset = ConstU32<2_400u32>;
    type FastConfirmationOffset = ConstU32<0u32>;
    type FinalizedConfirmationOffset = ConstU32<0u32>;
    type HeadersToStore = HeadersToStore;
    type LightClientAsyncAPI = XDNS;
    type MyVendor = PolkadotVendor;
    type RationalConfirmationOffset = ConstU32<0u32>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const SyncCommitteeSize: u32 = 512;
    pub const GenesisValidatorsRoot: [u8; 32] = [216,234,23,31,60,148,174,162,30,188,66,161,237,97,5,42,207,63,146,9,192,14,78,251,170,221,172,9,237,155,128,120];
    pub const SlotsPerEpoch: u32 = 32;
    pub const EpochsPerSyncCommitteePeriod: u32 = 256;
    pub const HeadersToStoreEth: u32 = 50400 + 1; // 1 week + 1. We want a multiple of 32 + 1.
    pub const CommitteeMajorityThreshold: u32 = 80;
}

impl pallet_eth2_finality_verifier::Config for Runtime {
    type CommitteeMajorityThreshold = CommitteeMajorityThreshold;
    type EpochsPerSyncCommitteePeriod = EpochsPerSyncCommitteePeriod;
    type GenesisValidatorRoot = GenesisValidatorsRoot;
    type HeadersToStore = HeadersToStoreEth;
    type LightClientAsyncAPI = XDNS;
    type RuntimeEvent = RuntimeEvent;
    type SlotsPerEpoch = SlotsPerEpoch;
    type SyncCommitteeSize = SyncCommitteeSize;
    type WeightInfo = ();
}

impl pallet_sepolia_finality_verifier::Config for Runtime {
    type CommitteeMajorityThreshold = CommitteeMajorityThreshold;
    type EpochsPerSyncCommitteePeriod = EpochsPerSyncCommitteePeriod;
    type GenesisValidatorRoot = GenesisValidatorsRoot;
    type HeadersToStore = HeadersToStoreEth;
    type LightClientAsyncAPI = XDNS;
    type RuntimeEvent = RuntimeEvent;
    type SlotsPerEpoch = SlotsPerEpoch;
    type SyncCommitteeSize = SyncCommitteeSize;
    type WeightInfo = ();
}
