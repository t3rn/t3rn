#![cfg_attr(not(feature = "std"), no_std)]
#![feature(more_qualified_paths)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod accounts_config;
pub mod circuit_config;
pub mod contracts_config;
pub mod hooks;
pub mod parachain_config;
pub mod signed_extrinsics_config;
pub mod system_config;
pub mod treasuries_config;
pub mod xcm_config;

pub use crate::{parachain_config::*, signed_extrinsics_config::*};
pub use circuit_runtime_types::*;
pub use frame_support::traits::EqualPrivilegeOnly;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};

use sp_runtime::traits::NumberFor;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use pallet_grandpa::{
    fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        fungibles::{Balanced, CreditOf},
        AsEnsureOriginWithArg, ConstU128, ConstU32, ConstU64, ConstU8, Imbalance,
        KeyOwnerProofSystem, OnUnbalanced, Randomness, StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
        },
        ConstantMultiplier, DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient,
        WeightToFeeCoefficients, WeightToFeePolynomial,
    },
    StorageValue,
};
use pallet_asset_tx_payment::HandleCredit;
use sp_runtime::traits::ConvertInto;

use polkadot_runtime_common::BlockHashCount;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic,
    traits::{AccountIdLookup, Block as BlockT},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use sp_std::{convert::TryInto, prelude::*};

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    // https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html
    spec_name: create_runtime_str!("t1rn"),
    impl_name: create_runtime_str!("t1rn Circuit Collator"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 2,
    // https://github.com/paritytech/cumulus/issues/998
    // https://github.com/paritytech/substrate/pull/9732
    // https://github.com/paritytech/substrate/pull/10073
    state_version: 1, // 0 = old, 1 = new; see above for details
};
use frame_system::EnsureRoot;
use t3rn_primitives::monetary::MILLIT3RN;

pub const TRN: Balance = UNIT;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // System support stuff.
        System: frame_system = 0,
        ParachainSystem: cumulus_pallet_parachain_system = 1,
        Timestamp: pallet_timestamp = 2,
        ParachainInfo: parachain_info = 3,
        Preimage: pallet_preimage = 4,
        Scheduler: pallet_scheduler = 5,
        Utility: pallet_utility = 6,

        // Monetary stuff.
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,

        // Treasuries
        Treasury: pallet_treasury = 12, // Keep old treasury index for backwards compatibility
        EscrowTreasury: pallet_treasury::<Instance1> = 16,
        FeeTreasury: pallet_treasury::<Instance2> = 17,
        ParachainTreasury: pallet_treasury::<Instance3> = 18,
        SlashTreasury: pallet_treasury::<Instance4> = 19,

        // Extend monetary to foreign assets #[cfg(feature = "foreign-assets")]
        Assets: pallet_assets = 13,
        AssetTxPayment: pallet_asset_tx_payment = 14,
        AccountManager: pallet_account_manager = 15,
        AssetRegistry: pallet_asset_registry = 35,

        // Global clock implementing most of t3rn hooks.
        Clock: pallet_clock= 110,

        // t3rn pallets
        XDNS: pallet_xdns = 100,
        Attesters: pallet_attesters = 101,
        Rewards: pallet_rewards = 102,
        ContractsRegistry: pallet_contracts_registry = 106,
        Circuit: pallet_circuit = 108,
        Vacuum: pallet_vacuum = 111,

        // // 3VM
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

        // Collator support. The order of these 4 are important and shall not change.
        Authorship: pallet_authorship = 20,
        CollatorSelection: pallet_collator_selection = 21,
        Session: pallet_session = 22,
        Aura: pallet_aura = 23,
        AuraExt: cumulus_pallet_aura_ext = 24,

        // // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue = 30,
        PolkadotXcm: pallet_xcm = 31,
        CumulusXcm: cumulus_pallet_xcm = 32,
        DmpQueue: cumulus_pallet_dmp_queue = 33,

        // Grandpa -- only for standalone
        // Grandpa: pallet_grandpa,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip = 200,

        Identity: pallet_identity = 122,

        Maintenance: pallet_maintenance_mode = 140,

        // Admin
        Sudo: pallet_sudo = 255,
    }
);

const MT3RN: Balance = MILLIT3RN as Balance;

pub struct CreditToBlockAuthor;
impl HandleCredit<AccountId, Assets> for CreditToBlockAuthor {
    fn handle_credit(credit: CreditOf<AccountId, Assets>) {
        if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
            let author_credit = credit
                .peek()
                .saturating_mul(80_u32.into())
                .saturating_div(<u32 as Into<Balance>>::into(100_u32));
            let (author_cut, treasury_cut) = credit.split(author_credit);
            // Drop the result which will trigger the `OnDrop` of the imbalance in case of error.
            let _ = Assets::resolve(&author, author_cut);
            let _ = Assets::resolve(&Treasury::account_id(), treasury_cut);
        }
    }
}

impl pallet_asset_tx_payment::Config for Runtime {
    type Fungibles = Assets;
    type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
        pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
        CreditToBlockAuthor,
    >;
    type RuntimeEvent = RuntimeEvent;
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

impl pallet_assets::Config for Runtime {
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

// Pallet Maintenance config starts here
use frame_support::traits::{
    Contains, OffchainWorker, OnFinalize, OnIdle, OnInitialize, OnRuntimeUpgrade,
};

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
    fn contains(c: &RuntimeCall) -> bool {
        match c {
            // System support
            RuntimeCall::System(_) => true,
            RuntimeCall::ParachainSystem(_) => true,
            RuntimeCall::Timestamp(_) => true,
            RuntimeCall::Preimage(_) => true,
            RuntimeCall::Scheduler(_) => true,
            RuntimeCall::Utility(_) => true,
            RuntimeCall::Identity(_) => true,
            // Monetary
            RuntimeCall::Balances(_) => true,
            RuntimeCall::Assets(_) => true,
            RuntimeCall::Treasury(_) => true,
            RuntimeCall::AccountManager(method) => matches!(
                method,
                pallet_account_manager::Call::deposit { .. }
                    | pallet_account_manager::Call::finalize { .. }
            ),
            // Collator support
            RuntimeCall::CollatorSelection(_) => true,
            RuntimeCall::Session(_) => true,
            // XCM helpers
            RuntimeCall::XcmpQueue(_) => true,
            RuntimeCall::PolkadotXcm(_) => false,
            RuntimeCall::DmpQueue(_) => true,
            // RuntimeCall::XBIPortal(_) => true,
            RuntimeCall::AssetRegistry(_) => true,
            // t3rn pallets
            RuntimeCall::XDNS(_) => true,
            RuntimeCall::ContractsRegistry(method) => matches!(
                method,
                pallet_contracts_registry::Call::add_new_contract { .. }
                    | pallet_contracts_registry::Call::purge { .. }
            ),
            RuntimeCall::Circuit(method) => matches!(
                method,
                pallet_circuit::Call::on_local_trigger { .. }
                    | pallet_circuit::Call::on_xcm_trigger { .. }
                    | pallet_circuit::Call::on_remote_gateway_trigger { .. }
                    | pallet_circuit::Call::cancel_xtx { .. }
                    | pallet_circuit::Call::revert { .. }
                    | pallet_circuit::Call::on_extrinsic_trigger { .. }
                    | pallet_circuit::Call::bid_sfx { .. }
                    | pallet_circuit::Call::confirm_side_effect { .. }
            ),
            RuntimeCall::Attesters(_) => true,
            // 3VM
            RuntimeCall::ThreeVm(_) => false,
            RuntimeCall::Contracts(method) => matches!(
                method,
                pallet_3vm_contracts::Call::call { .. }
                    | pallet_3vm_contracts::Call::instantiate_with_code { .. }
                    | pallet_3vm_contracts::Call::instantiate { .. }
                    | pallet_3vm_contracts::Call::upload_code { .. }
                    | pallet_3vm_contracts::Call::remove_code { .. }
            ),
            RuntimeCall::Evm(method) => matches!(
                method,
                pallet_3vm_evm::Call::withdraw { .. }
                    | pallet_3vm_evm::Call::call { .. }
                    | pallet_3vm_evm::Call::create { .. }
                    | pallet_3vm_evm::Call::create2 { .. } // | pallet_3vm_evm::Call::claim { .. } TODO: wheres this gone
            ),
            // Portal
            RuntimeCall::Portal(_) => true,
            _ => true,
        }
    }
}

/// Maintenance mode Call filter
///
/// For maintenance mode, we disallow everything
pub struct MaintenanceFilter;
impl Contains<RuntimeCall> for MaintenanceFilter {
    fn contains(c: &RuntimeCall) -> bool {
        match c {
            // We want to make calls to the system and scheduler pallets
            RuntimeCall::System(_) => true,
            RuntimeCall::Scheduler(_) => true,
            // Sometimes scheduler/system calls require utility calls, particularly batch
            RuntimeCall::Utility(_) => true,
            // We dont manually control these so likely we dont want to block them during maintenance mode
            RuntimeCall::Balances(_) => true,
            RuntimeCall::Assets(_) => true,
            // We wanna be able to make sudo calls in maintenance mode just incase
            RuntimeCall::Sudo(_) => true,
            RuntimeCall::ParachainSystem(_) => true,
            RuntimeCall::Timestamp(_) => true,
            RuntimeCall::Session(_) => true,
            RuntimeCall::RococoBridge(_) => true,
            RuntimeCall::KusamaBridge(_) => true,
            RuntimeCall::PolkadotBridge(_) => true,
            RuntimeCall::EthereumBridge(_) => true,
            RuntimeCall::SepoliaBridge(_) => true,
            #[allow(unreachable_patterns)] // We need this as an accidental catchall
            _ => false,
        }
    }
}

/// Hooks to run when in Maintenance Mode
pub struct MaintenanceHooks;

impl OnInitialize<BlockNumber> for MaintenanceHooks {
    fn on_initialize(n: BlockNumber) -> frame_support::weights::Weight {
        AllPalletsWithSystem::on_initialize(n)
    }
}

/// Only two pallets use `on_idle`: xcmp and dmp queues.
/// Empty on_idle, in case we want the pallets to execute it, should be provided here.
impl OnIdle<BlockNumber> for MaintenanceHooks {
    fn on_idle(_n: BlockNumber, _max_weight: Weight) -> Weight {
        Weight::zero()
    }
}

impl OnRuntimeUpgrade for MaintenanceHooks {
    fn on_runtime_upgrade() -> Weight {
        AllPalletsWithSystem::on_runtime_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
        AllPalletsWithSystem::pre_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
        AllPalletsWithSystem::post_upgrade()
    }
}

impl OnFinalize<BlockNumber> for MaintenanceHooks {
    fn on_finalize(n: BlockNumber) {
        AllPalletsWithSystem::on_finalize(n)
    }
}

impl OffchainWorker<BlockNumber> for MaintenanceHooks {
    fn offchain_worker(n: BlockNumber) {
        AllPalletsWithSystem::offchain_worker(n)
    }
}

impl pallet_maintenance_mode::Config for Runtime {
    type MaintenanceCallFilter = MaintenanceFilter;
    type MaintenanceExecutiveHooks = AllPalletsWithSystem;
    type MaintenanceOrigin = EnsureRoot<AccountId>;
    type NormalCallFilter = BaseCallFilter;
    type NormalExecutiveHooks = AllPalletsWithSystem;
    type RuntimeEvent = RuntimeEvent;
}
// Pallet Maintenance config ends here

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_collator_selection, CollatorSelection]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_timestamp, Timestamp]
        [pallet_treaury, Treasury]
        [pallet_utility, Utility]
        [pallet_scheduler, Scheduler]
        [pallet_preimage, Preimage]
        [pallet_xcm, PolkadotXcm]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
        [cumulus_pallet_parachain_system, ParachainSystem]
    );
}

use pallet_xdns_rpc_runtime_api::{ChainId, GatewayABIConfig};
use t3rn_primitives::{
    light_client::HeightResult,
    xdns::{FullGatewayRecord, GatewayRecord},
    TreasuryAccountProvider,
};

impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
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

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
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

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
             let collation_info = ParachainSystem::collect_collation_info(header);
             if let Some(ref new_validation_code) = collation_info.new_validation_code {
                 log::info!("RuntimeUpgrade::submitting new validation code via HRMP to relay chain {:?}", new_validation_code.hash());
             }
             collation_info
        }
    }

    impl pallet_3vm_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
        for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<Balance>,
            input_data: Vec<u8>,
        ) -> pallet_3vm_contracts_primitives::ContractExecResult<Balance> {
            Contracts::bare_call(origin, dest, value, gas_limit.unwrap_or_default(), storage_deposit_limit, input_data, CONTRACTS_DEBUG_OUTPUT, pallet_3vm_contracts::Determinism::AllowIndeterminism)
        }

        fn instantiate(
            origin: AccountId,
            value: Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<Balance>,
            code: pallet_3vm_contracts_primitives::Code<Hash>,
            data: Vec<u8>,
            salt: Vec<u8>,
        ) -> pallet_3vm_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
        {
            Contracts::bare_instantiate(origin, value, gas_limit.unwrap_or_default(), storage_deposit_limit, code, data, salt, CONTRACTS_DEBUG_OUTPUT)
        }

        fn upload_code(
            origin: AccountId,
            code: Vec<u8>,
            storage_deposit_limit: Option<Balance>,
            determinism: pallet_3vm_contracts::Determinism,
        ) -> pallet_3vm_contracts_primitives::CodeUploadResult<Hash, Balance>
        {
            Contracts::bare_upload_code(origin, code, storage_deposit_limit, determinism)
        }

        fn get_storage(
            address: AccountId,
            key: Vec<u8>,
        ) -> pallet_3vm_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
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

     impl pallet_portal_rpc_runtime_api::PortalRuntimeApi<Block, AccountId> for Runtime {
        fn fetch_head_height(chain_id: ChainId) -> Option<u128> {
            let res = <Portal as t3rn_primitives::portal::Portal<Runtime>>::get_fast_height(chain_id);

            match res {
                Ok(HeightResult::Height(height)) => Some(height.into()),
                _ => None,
            }
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade() -> (Weight, Weight) {
            log::info!("try-runtime::on_runtime_upgrade parachain-template.");
            let weight = Executive::try_runtime_upgrade().unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block_no_check(block: Block) -> Weight {
            Executive::execute_block_no_check(block)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
            impl cumulus_pallet_session_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // RuntimeEvent Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
