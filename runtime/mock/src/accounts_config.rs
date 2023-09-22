use crate::*;
pub use frame_support::traits::{Imbalance, OnUnbalanced};
use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use frame_system::EnsureSigned;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::{ConstU32, ConvertInto};

parameter_types! {
    // TODO: update me to be better
    pub EscrowAccount: AccountId32 = AccountId32::new([51_u8; 32]);
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

impl pallet_assets::Config for Runtime {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type AssetDeposit = AssetDeposit;
    type AssetId = circuit_runtime_types::AssetId;
    type AssetIdParameter = circuit_runtime_types::AssetId;
    type Balance = Balance;
    type CallbackHandle = ();
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type Currency = Balances;
    type Extra = ();
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type RemoveItemsLimit = ConstU32<1000>;
    type RuntimeEvent = RuntimeEvent;
    type StringLimit = AssetsStringLimit;
    type WeightInfo = ();
}

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
	type RuntimeEvent = RuntimeEvent;
	type ReserveAssetModifierOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Assets = Assets;
	type WeightInfo = pallet_asset_registry::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRegistryBenchmarkHelper;
}
