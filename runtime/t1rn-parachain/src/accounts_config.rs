use crate::{
    hooks::GlobalOnInitQueues, treasuries_config::EscrowTreasuryId, AccountId, AccountManager,
    AssetId, Assets, Balance, Balances, Clock, EnsureRoot, Imbalance, OnUnbalanced, Runtime,
    RuntimeCall, RuntimeEvent, Timestamp,
};
use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use sp_core::{crypto::AccountId32, ConstU32};
use sp_runtime::traits::{AccountIdConversion, ConvertInto};

parameter_types! {
    pub EscrowAccount: AccountId32 = EscrowTreasuryId::get().into_account_truncating();
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
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

// pallet_account_manager::setup_currency_adapter!();

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

parameter_types! {
    pub const RegCost: u128 = 100_000_000_000;
}

impl pallet_asset_registry::Config for Runtime {
    type Assets = Assets;
    type Currency = Balances;
    type RegistrationCost = RegCost;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
}
