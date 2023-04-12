use crate::{accounts_config::AccountManagerCurrencyAdapter, Hash as HashPrimitive, *};
use frame_support::{
    parameter_types,
    traits::{
        fungibles::{Balanced, CreditOf},
        ConstU32, ConstU8, Contains, OnFinalize, OnIdle, OnInitialize, OnRuntimeUpgrade,
    },
    weights::IdentityFee,
};
use pallet_asset_tx_payment::HandleCredit;
use polkadot_runtime_common::SlowAdjustingFeeUpdate;
use sp_runtime::traits::{BlakeTwo256, ConvertInto};

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = BaseCallFilter;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The maximum length of a block (in bytes).
    type BlockLength = circuit_runtime_types::BlockLength;
    /// The index type for blocks.
    type BlockNumber = circuit_runtime_types::BlockNumber;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = circuit_runtime_types::BlockWeights;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The ubiquitous event type.
    type Event = Event;
    /// The type for hashing blocks and tries.
    type Hash = HashPrimitive;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Version of the runtime.
    type Version = Version;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = MinimumPeriod;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1_u128;
}

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type Event = Event;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type OnChargeTransaction = AccountManagerCurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
}

/// A `HandleCredit` implementation that transfers 80% of the fees to the
/// block author and 20% to the treasury. Will drop and burn the assets
/// in case the transfer fails.
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
            match Assets::resolve(&author, author_cut) {
                Ok(_) => (),
                Err(_err) => {
                    log::error!("Failed to credit block author");
                },
            }
            match Assets::resolve(&Treasury::account_id(), treasury_cut) {
                Ok(_) => (),
                Err(_err) => {
                    log::error!("Failed to credit treasury");
                },
            }
        }
    }
}

impl pallet_asset_tx_payment::Config for Runtime {
    type Fungibles = Assets;
    type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
        pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
        CreditToBlockAuthor,
    >;
}

impl pallet_sudo::Config for Runtime {
    type Call = Call;
    type Event = Event;
}

impl pallet_utility::Config for Runtime {
    type Call = Call;
    type Event = Event;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

// Moonbeam and Akala runtimes have references for BaseCallFilter
// MaintenanceFilter, NormalFilter and for Proxy type
// `impl pallet_evm_precompile_proxy::EvmProxyCallFilter for ProxyType`
pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
    fn contains(c: &Call) -> bool {
        match c {
            // System support
            Call::System(_) => true,
            Call::ParachainSystem(_) => true,
            Call::Timestamp(_) => true,
            Call::Preimage(_) => true,
            Call::Scheduler(_) => true,
            Call::Utility(_) => true,
            Call::Identity(_) => true,
            // Monetary
            Call::Balances(_) => true,
            Call::Assets(_) => true,
            Call::Treasury(_) => true,
            Call::AccountManager(method) => matches!(
                method,
                pallet_account_manager::Call::deposit { .. }
                    | pallet_account_manager::Call::finalize { .. }
            ),
            // Collator support
            Call::Authorship(_) => true,
            Call::CollatorSelection(_) => true,
            Call::Session(_) => true,
            // XCM helpers
            Call::XcmpQueue(_) => true,
            Call::PolkadotXcm(_) => false,
            Call::DmpQueue(_) => true,
            Call::XBIPortal(_) => true,
            Call::AssetRegistry(_) => true,
            // t3rn pallets
            Call::XDNS(method) => matches!(
                method,
                pallet_xdns::Call::purge_gateway { .. }
                    | pallet_xdns::Call::purge_gateway_record { .. }
            ),
            Call::ContractsRegistry(method) => matches!(
                method,
                pallet_contracts_registry::Call::add_new_contract { .. }
                    | pallet_contracts_registry::Call::purge { .. }
            ),
            Call::Circuit(method) => matches!(
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
            // 3VM
            Call::ThreeVm(_) => false,
            Call::Contracts(method) => matches!(
                method,
                pallet_3vm_contracts::Call::call { .. }
                    | pallet_3vm_contracts::Call::instantiate_with_code { .. }
                    | pallet_3vm_contracts::Call::instantiate { .. }
                    | pallet_3vm_contracts::Call::upload_code { .. }
                    | pallet_3vm_contracts::Call::remove_code { .. }
            ),
            Call::Evm(method) => matches!(
                method,
                pallet_3vm_evm::Call::withdraw { .. }
                    | pallet_3vm_evm::Call::call { .. }
                    | pallet_3vm_evm::Call::create { .. }
                    | pallet_3vm_evm::Call::create2 { .. }
                    | pallet_3vm_evm::Call::claim { .. }
            ),
            // Portal
            Call::Portal(method) => matches!(method, pallet_portal::Call::register_gateway { .. }),
            Call::RococoBridge(method) => matches!(
                method,
                pallet_grandpa_finality_verifier::Call::submit_headers { .. }
            ),
            // TODO: check this one
            Call::PolkadotBridge(method) => matches!(
                method,
                pallet_grandpa_finality_verifier::Call::submit_headers { .. }
            ),
            // TODO: check this one
            Call::KusamaBridge(method) => matches!(
                method,
                pallet_grandpa_finality_verifier::Call::submit_headers { .. }
            ),
            // Admin
            Call::Sudo(_) => true,
            _ => false,
        }
    }
}

/// Maintenance mode Call filter
///
/// For maintenance mode, we disallow everything
pub struct MaintenanceFilter;
impl Contains<Call> for MaintenanceFilter {
    fn contains(c: &Call) -> bool {
        match c {
            Call::System(_) => false,
            Call::ParachainSystem(_) => false,
            Call::Timestamp(_) => false,
            Call::Preimage(_) => false,
            Call::Scheduler(_) => false,
            Call::Utility(_) => false,
            Call::Identity(_) => false,
            Call::Balances(_) => false,
            Call::Assets(_) => false,
            Call::Treasury(_) => false,
            Call::AccountManager(_) => false,
            Call::Authorship(_) => false,
            Call::CollatorSelection(_) => false,
            Call::Session(_) => false,
            Call::XcmpQueue(_) => false,
            Call::PolkadotXcm(_) => false,
            Call::DmpQueue(_) => false,
            Call::XBIPortal(_) => false,
            Call::AssetRegistry(_) => false,
            Call::XDNS(_) => false,
            Call::ContractsRegistry(_) => false,
            Call::Circuit(_) => false,
            Call::ThreeVm(_) => false,
            Call::Contracts(_) => false,
            Call::Evm(_) => false,
            Call::Portal(_) => false,
            Call::RococoBridge(_) => false,
            Call::PolkadotBridge(_) => false,
            Call::KusamaBridge(_) => false,
            Call::Sudo(_) => false,
            _ => true,
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
/// Empty on_idle in case we want the pallets to execute it, we need to provide it here.
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

// TODO: check if we need to implement this, and what should go here
impl pallet_maintenance_mode::Config for Runtime {
    type Event = Event;
    type MaintenanceFilter = MaintenanceFilter;
    type MaintenanceHooks = MaintenanceHooks;
    type MaintenanceOrigin = EnsureRootOrHalfCouncil;
    type NormalCallFilter = NormalFilter;
    type WeightInfo = ();
}

/// Normal Call filter
/// We don't allow for a certain number of calls, but we allow everything else.
/// Now, only System calls are allowed as a default before implementing.
pub struct NormalFilter;
impl Contains<Call> for NormalFilter {
    fn contains(c: &Call) -> bool {
        match c {
            Call::System(_) => false,
            Call::ParachainSystem(_) => false,
            Call::Timestamp(_) => false,
            Call::Preimage(_) => false,
            Call::Scheduler(_) => false,
            Call::Utility(_) => false,
            Call::Identity(_) => false,
            Call::Balances(_) => false,
            Call::Assets(_) => false,
            Call::Treasury(_) => false,
            Call::AccountManager(_) => false,
            Call::Authorship(_) => false,
            Call::CollatorSelection(_) => false,
            Call::Session(_) => false,
            Call::XcmpQueue(_) => false,
            Call::PolkadotXcm(_) => false,
            Call::DmpQueue(_) => false,
            Call::XBIPortal(_) => false,
            Call::AssetRegistry(_) => false,
            Call::XDNS(_) => false,
            Call::ContractsRegistry(_) => false,
            Call::Circuit(_) => false,
            Call::ThreeVm(_) => false,
            Call::Contracts(_) => false,
            Call::Evm(_) => false,
            Call::Portal(_) => false,
            Call::RococoBridge(_) => false,
            Call::PolkadotBridge(_) => false,
            Call::KusamaBridge(_) => false,
            Call::Sudo(_) => false,
            _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_call_filter_returns_true_with_allowed_calls() {
        // System support
        let call = frame_system::Call::remark { remark: vec![] }.into();
        assert!(BaseCallFilter::contains(&call));

        // let call = cumulus_pallet_parachain_system::Call::set_validation_data {
        //     data: Default::default(),
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        let call = pallet_timestamp::Call::set { now: 0 }.into();
        assert!(BaseCallFilter::contains(&call));

        let call = pallet_preimage::Call::note_preimage { bytes: vec![0] }.into();
        assert!(BaseCallFilter::contains(&call));

        // let call = pallet_scheduler::Call::schedule {
        //     when: 0,
        //     maybe_periodic: None,
        //     priority: 0,
        //     call: Box::new(frame_system::Call::remark { remark: vec![] }.into()),
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // let call = pallet_utility::Call::dispatch_as {
        //     call: Box::new(frame_system::Call::remark { remark: vec![] }.into()),
        //     as_origin: Default::default(),
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        let call = pallet_identity::Call::add_registrar {
            account: sp_runtime::AccountId32::new([0; 32]),
        }
        .into();
        assert!(BaseCallFilter::contains(&call));

        // Monetary
        let call = pallet_balances::Call::transfer {
            dest: MultiAddress::Address32([0; 32]),
            value: 0,
        }
        .into();
        assert!(BaseCallFilter::contains(&call));

        let call = pallet_assets::Call::create {
            id: Default::default(),
            admin: MultiAddress::Address32([0; 32]),
            min_balance: 0,
        }
        .into();
        assert!(BaseCallFilter::contains(&call));

        let call = pallet_treasury::Call::propose_spend {
            value: 0,
            beneficiary: MultiAddress::Address32([0; 32]),
        }
        .into();
        assert!(BaseCallFilter::contains(&call));

        // let call = pallet_account_manager::Call::deposit {
        //     charge_id: todo!(),
        //     payee: sp_runtime::AccountId32::new([0; 32]),
        //     charge_fee: todo!(),
        //     offered_reward: todo!(),
        //     source: todo!(),
        //     role: todo!(),
        //     recipient: todo!(),
        //     maybe_asset_id: todo!(),
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // Collator support
        let call = pallet_authorship::Call::set_uncles { new_uncles: vec![] }.into();
        assert!(BaseCallFilter::contains(&call));

        let call = pallet_collator_selection::Call::set_invulnerables { new: vec![] }.into();
        assert!(BaseCallFilter::contains(&call));

        // let call = pallet_session::Call::set_keys {
        //     keys: parachain_config::SessionKeys {
        //         aura: sp_consensus_aura::sr25519::AuthorityId::default(),
        //     },
        //     proof: vec![],
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // XCM helpers
        // let call = pallet_xcm::Call::force_default_xcm_version {
        //     maybe_xcm_version: None,
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // 3VM
        let call = pallet_3vm_evm::Call::withdraw {
            address: Default::default(),
            value: 0,
        }
        .into();
        assert!(BaseCallFilter::contains(&call));

        // let call = pallet_3vm_contracts::Call::call {
        //     dest: MultiAddress::Address32([0; 32]),
        //     value: 0,
        //     gas_limit: 0,
        //     storage_deposit_limit: Some(pallet_3vm_contracts::BalanceOf::<Runtime>::max_value()),
        //     data: vec![],
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // let call = pallet_grandpa_finality_verifier::Call::<T,I>::submit_headers {
        //     range: vec![0],
        //     signed_header: vec![0],
        //     justification: pallet_grandpa_finality_verifier::bridges::header_chain::justification::GrandpaJustification{
        //         round: 0,
        //         commit: finality_grandpa::Commit {
        //             target_hash: Default::default(),
        //             target_number: 0,
        //             precommits: vec![],
        //         },
        //         votes_ancestries: vec![],
        //     },
        // }
        // .into();
        // assert!(BaseCallFilter::contains(&call));

        // Admin
        let call = pallet_sudo::Call::sudo {
            call: Box::new(frame_system::Call::remark { remark: vec![] }.into()),
        }
        .into();
        assert!(BaseCallFilter::contains(&call));
    }

    #[test]
    fn base_call_filter_returns_false_with_disallowed_call() {
        // Create a call for PolkadotXCM
        let call = pallet_xcm::Call::force_default_xcm_version {
            maybe_xcm_version: Some(1),
        }
        .into();
        assert!(!BaseCallFilter::contains(&call));
    }
}
