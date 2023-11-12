use crate::{
    hooks::GlobalOnInitQueues, treasuries_config::EscrowTreasuryId, AssetId, Assets, Balance,
    Balances, Clock, Imbalance, OnUnbalanced, Runtime, RuntimeCall, RuntimeEvent, ThreeVm,
    Timestamp,
};
use frame_support::parameter_types;
use sp_core::{crypto::AccountId32, ConstU32};
use sp_runtime::traits::{AccountIdConversion, ConvertInto};

parameter_types! {
    pub EscrowAccount: AccountId32 = EscrowTreasuryId::get().into_account_truncating();
}

impl pallet_clock::Config for Runtime {
    type OnFinalizeQueues = t3rn_primitives::clock::EmptyOnHookQueues<Self>;
    type OnInitializeQueues = GlobalOnInitQueues;
    type RoundDuration = ConstU32<300u32>;
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_account_manager::Config for Runtime {
    type AssetBalanceOf = ConvertInto;
    type AssetId = AssetId;
    type Assets = Assets;
    type Clock = Clock;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type RuntimeEvent = RuntimeEvent;
    type Time = Timestamp;
    type WeightInfo = ();
}

pallet_account_manager::setup_currency_adapter!();

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

#[cfg(feature = "runtime-benchmarks")]
use parachains_common::AssetIdForTrustBackedAssets;
#[cfg(feature = "runtime-benchmarks")]
pub struct AssetRegistryBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_asset_registry::BenchmarkHelper<AssetIdForTrustBackedAssets>
    for AssetRegistryBenchmarkHelper
{
    fn get_registered_asset() -> AssetIdForTrustBackedAssets {
        use sp_runtime::traits::StaticLookup;

        let root = frame_system::RawOrigin::Root.into();
        let asset_id = 1;
        let caller = frame_benchmarking::whitelisted_caller();
        let caller_lookup = <Runtime as frame_system::Config>::Lookup::unlookup(caller);
        Assets::force_create(root, asset_id.into(), caller_lookup, true, 1)
            .expect("Should have been able to force create asset");
        asset_id
    }
}

impl pallet_asset_registry::Config for Runtime {
    type Assets = Assets;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = AssetRegistryBenchmarkHelper;
    type ReserveAssetModifierOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_asset_registry::weights::SubstrateWeight<Runtime>;
}
