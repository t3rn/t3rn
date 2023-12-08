#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// To learn more about runtime versioning and what each of the following value means:
//   https://docs.substrate.io/v3/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("t2rn"),
    impl_name: create_runtime_str!("t2rn"),
    authoring_version: 17,
    spec_version: 17,
    impl_version: 17,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 17,
    state_version: 1,
};

use t3rn_primitives::monetary::MILLIT3RN;

use frame_system::EnsureRoot;

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
pub use frame_system::Call as SystemCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::{Perbill, Permill};

pub mod accounts_config;
pub mod circuit_config;
pub mod consensus_aura_config;
pub mod contracts_config;
pub mod hooks;
pub mod signed_extrinsics_config;
pub mod system_config;
pub mod treasuries_config;

pub use crate::{consensus_aura_config::*, signed_extrinsics_config::*};
pub use circuit_runtime_types::*;

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
        AccountMapping: pallet_3vm_account_mapping = 126,
        //Ethereum: pallet_ethereum = 227,

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
pub use pallet_balances::Call as BalancesCall;
use pallet_circuit::ChainId;
use pallet_grandpa::AuthorityId as GrandpaId;
pub use pallet_timestamp::Call as TimestampCall;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use t3rn_primitives::{
    circuit::ReadSFX,
    portal::HeightResult,
    xdns::{FullGatewayRecord, GatewayRecord},
    TreasuryAccountProvider,
};

pub use crate::consensus_aura_config::*;
use pallet_xdns_rpc_runtime_api::GatewayABIConfig;

use sp_runtime::create_runtime_str;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

use t3rn_types::sfx::SideEffect;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

     impl pallet_xdns_rpc_runtime_api::XdnsRuntimeApi<Block, AccountId> for Runtime {
        fn fetch_records() -> Vec<GatewayRecord<AccountId>> {
             <XDNS as t3rn_primitives::xdns::Xdns<Runtime, Balance>>::fetch_gateways()
        }

        fn fetch_full_gateway_records() -> Vec<FullGatewayRecord<AccountId>> {
             <XDNS as t3rn_primitives::xdns::Xdns<Runtime, Balance>>::fetch_full_gateway_records()
        }

        fn fetch_abi(_chain_id: ChainId) -> Option<GatewayABIConfig> {
            // deprecated
            None
        }

        fn retreive_treasury_address(treasury_account: t3rn_primitives::TreasuryAccount) -> AccountId {
            Runtime::get_treasury_account(treasury_account)
        }
    }

     impl pallet_portal_rpc_runtime_api::PortalRuntimeApi<Block, AccountId, Balance, Hash> for Runtime {
        fn fetch_head_height(chain_id: ChainId) -> Option<u128> {
            let res = <Portal as t3rn_primitives::portal::Portal<Runtime>>::get_fast_height(chain_id);

            match res {
                Ok(HeightResult::Height(height)) => Some(height.into()),
                _ => None,
            }
        }

       fn fetch_all_active_xtx(for_executor: AccountId) -> Vec<(
            Hash,                              // xtx_id
            Vec<SideEffect<AccountId, Balance>>, // side_effects
            Vec<Hash>,                         // sfx_ids
        )> {
            Circuit::get_pending_xtx_for(for_executor)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here. If any of the pre/post migration checks fail, we shall stop
            // right here and right now.
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, BlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
        }
    }
}
