#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use t3rn_primitives::monetary::MILLIUNIT;

use frame_system::EnsureRoot;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

use sp_runtime::{
    generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256},
};
use sp_std::{
    convert::{TryFrom, TryInto},
    prelude::*,
};

// A few exports that help ease life for downstream crates.
use frame_support::weights::ConstantMultiplier;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstU128, ConstU32, ConstU8, Imbalance, KeyOwnerProofSystem, OnUnbalanced, Randomness,
        StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
        },
        IdentityFee, Weight,
    },
    StorageValue,
};

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub mod accounts_config;
pub mod circuit_config;
pub mod consensus_aura_config;
pub mod contracts_config;
pub mod hooks;
pub mod impl_versioned_runtime_with_api;
pub mod signed_extrinsics_config;
pub mod system_config;
pub mod treasuries_config;

pub use crate::{consensus_aura_config::*, signed_extrinsics_config::*};
pub use circuit_runtime_types::*;
pub use impl_versioned_runtime_with_api::*;

pub type CurrencyAdapter = accounts_config::AccountManagerCurrencyAdapter<Balances, ()>;

const MT3RN: Balance = MILLIUNIT as Balance;

parameter_types! {
    pub const BasicDeposit: Balance = 5 * MT3RN;
    pub const FieldDeposit: Balance = MT3RN;
    pub const SubAccountDeposit: Balance = 2 * MT3RN;
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
    type BasicDeposit = BasicDeposit;
    type Currency = Balances;
    type FieldDeposit = FieldDeposit;
    type ForceOrigin = EnsureRoot<AccountId>;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type MaxSubAccounts = MaxSubAccounts;
    type RegistrarOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type Slashed = ();
    type SubAccountDeposit = SubAccountDeposit;
    type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system = 0,
        Timestamp: pallet_timestamp = 1,
        Aura: pallet_aura = 2,
        Grandpa: pallet_grandpa = 3,

        Utility: pallet_utility = 6,

        // Monetary stuff.
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,

        Assets: pallet_assets = 13,
        AssetTxPayment: pallet_asset_tx_payment = 14,
        AccountManager: pallet_account_manager = 15,

        // Treasuries
        Treasury: pallet_treasury = 12, // Keep old treasury index for backwards compatibility
        EscrowTreasury: pallet_treasury::<Instance1> = 16,
        FeeTreasury: pallet_treasury::<Instance2> = 17,
        ParachainTreasury: pallet_treasury::<Instance3> = 18,
        SlashTreasury: pallet_treasury::<Instance4> = 19,

        // Global clock implementing most of t3rn hooks.
        Clock: pallet_clock= 110,

        // t3rn pallets
        XDNS: pallet_xdns = 100,
        Attesters: pallet_attesters = 101,
        Rewards: pallet_rewards = 102,
        ContractsRegistry: pallet_contracts_registry = 106,
        Circuit: pallet_circuit = 108,
        Vacuum: pallet_vacuum = 111,

        // 3VM
        ThreeVm: pallet_3vm = 119,
        Contracts: pallet_3vm_contracts = 120,
        Evm: pallet_3vm_evm = 121,

         // Portal
        Portal: pallet_portal = 128,
        RococoBridge: pallet_grandpa_finality_verifier = 129,
        PolkadotBridge: pallet_grandpa_finality_verifier::<Instance1> = 130,
        KusamaBridge: pallet_grandpa_finality_verifier::<Instance2> = 131,
        EthereumBridge: pallet_eth2_finality_verifier = 132,
        SepoliaBridge: pallet_sepolia_finality_verifier = 133,

        Identity: pallet_identity = 122,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip = 200,

        // Handy utilities
        MaintenanceMode: pallet_maintenance_mode = 140,

        Sudo: pallet_sudo = 255,
    }
);
