#![cfg_attr(not(feature = "std"), no_std)]
#![feature(more_qualified_paths)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
use codec::Decode;
use t3rn_primitives::monetary::MILLIT3RN;

use pallet_grandpa::AuthorityId as GrandpaId;

use frame_system::EnsureRoot;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT},
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
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub mod accounts_config;
pub mod circuit_config;
pub mod consensus_aura_config;
pub mod contracts_config;
pub mod impl_versioned_runtime_with_api;
pub mod signed_extrinsics_config;
pub mod system_config;
pub mod xbi_config;

pub use crate::{consensus_aura_config::*, signed_extrinsics_config::*};
pub use circuit_runtime_types::*;
pub use impl_versioned_runtime_with_api::*;

pub type CurrencyAdapter = accounts_config::AccountManagerCurrencyAdapter<Balances, ()>;

const MT3RN: Balance = MILLIT3RN as Balance;

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
    type Event = Event;
    type FieldDeposit = FieldDeposit;
    type ForceOrigin = EnsureRoot<AccountId>;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type MaxSubAccounts = MaxSubAccounts;
    type RegistrarOrigin = EnsureRoot<AccountId>;
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
        System: frame_system,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Sudo: pallet_sudo,
        Utility: pallet_utility,
        Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 122,

        // Monetary
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,
        Treasury: pallet_treasury::{Pallet, Storage, Config, Event<T>, Call} = 12,
        Assets: pallet_assets = 13,

        // Circuit
        // t3rn pallets
        XDNS: pallet_xdns::{Pallet, Call, Config<T>, Storage, Event<T>} = 100,
        ContractsRegistry: pallet_contracts_registry::{Pallet, Call, Config<T>, Storage, Event<T>} = 106,
        Circuit: pallet_circuit::{Pallet, Call, Storage, Event<T>} = 108,
        Clock: pallet_clock::{Pallet, Config<T>, Storage, Event<T>} = 110,

        XBIPortal: pallet_xbi_portal::{Pallet, Call, Storage, Event<T>} = 111,

        // 3VM
        ThreeVm: pallet_3vm = 119,
        Contracts: pallet_3vm_contracts = 120,
        Evm: pallet_3vm_evm = 121,
        AccountManager: pallet_account_manager = 125,

        // Portal
        Portal: pallet_portal::{Pallet, Call, Storage, Event<T>} = 128,
        RococoBridge: pallet_grandpa_finality_verifier::{
            Pallet, Storage
        } = 129,

    }
);
