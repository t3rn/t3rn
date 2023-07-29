use crate::*;
use frame_support::PalletId;
use frame_system::EnsureRoot;
use smallvec::smallvec;
use sp_runtime::impl_opaque_keys;
use sp_std::prelude::*;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// XCM Imports
use xcm::latest::prelude::BodyId;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;

    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
        // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
        let p = MILLIUNIT / 10;
        let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_runtime::{generic, traits::BlakeTwo256};

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

/// Calculate the storage deposit based on the number of storage items and the
/// combined byte size of those items.
pub const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance) * 56 * MILLIUNIT + (bytes as Balance) * 50 * MICROUNIT
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u16 = 42;
}

parameter_types! {
    pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
    type EventHandler = (CollatorSelection,);
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
    pub const Period: u32 = 6 * HOURS;
    pub const Offset: u32 = 0;
    pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
    type Keys = SessionKeys;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type RuntimeEvent = RuntimeEvent;
    // Essentially just Aura, but lets be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type SessionManager = CollatorSelection;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    // pub const MinCandidates: u32 = 2;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
    pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
    type Currency = Balances;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type MaxCandidates = MaxCandidates;
    type MaxInvulnerables = MaxInvulnerables;
    type MinCandidates = MinCandidates;
    type PotId = PotId;
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = CollatorSelectionUpdateOrigin;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = ();
}

parameter_types! {
    pub const PreimageMaxSize: u32 = 4096 * 1024;
    pub const PreImageBaseDeposit: Balance = deposit(2, 64);
    pub const PreImageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
    type BaseDeposit = PreImageBaseDeposit;
    type ByteDeposit = PreImageByteDeposit;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_preimage::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        RuntimeBlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
    pub const NoPreimagePostponement: Option<BlockNumber> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type MaximumWeight = MaximumSchedulerWeight;
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type PalletsOrigin = OriginCaller;
    type Preimages = ();
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TreasuryId: PalletId = PalletId(*b"pottrsry");
    pub const MaxApprovals: u32 = 100;
    pub const ProposalBond: Permill = Permill::from_percent(3);
    pub const SpendPeriod: u32 = (60 * 60 * 24) / 12;
    pub const ProposalBondMinimum: u128 = 10_u128 * 1_000_000_000_000_u128;
}

impl pallet_treasury::Config for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = Treasury;
    type PalletId = TreasuryId;
    type ProposalBond = ProposalBond;
    type ProposalBondMaximum = ();
    type ProposalBondMinimum = ProposalBondMinimum;
    type RejectOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type SpendFunds = ();
    type SpendOrigin = NeverEnsureOrigin<Balance>;
    type SpendPeriod = SpendPeriod;
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
}
