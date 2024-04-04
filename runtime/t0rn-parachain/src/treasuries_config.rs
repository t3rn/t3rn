use super::{AccountId, Balance, Balances, Runtime, RuntimeEvent, SlashTreasury};
use frame_support::{parameter_types, traits::NeverEnsureOrigin, PalletId};
use frame_system::EnsureRoot;
use sp_runtime::{traits::AccountIdConversion, Permill};

use t3rn_primitives::{monetary::UNIT as TRN, TreasuryAccount, TreasuryAccountProvider};

pub type DefaultTreasuryInstance = ();
pub type EscrowTreasuryInstance = pallet_treasury::pallet::Instance1;
pub type FeeTreasuryInstance = pallet_treasury::pallet::Instance2;
pub type ParachainTreasuryInstance = pallet_treasury::pallet::Instance3;
pub type SlashTreasuryInstance = pallet_treasury::pallet::Instance4;

// Treasury#1 - default Treasury
parameter_types! {
    pub const TreasuryId: PalletId = PalletId(*b"pottrsry");
    pub const MaxApprovals: u32 = 10;
    pub const ProposalBond: Permill = Permill::from_percent(5);
    // Set to 1 week
    pub const SpendPeriod: u32 = 7 * 24 * 60 * 60;
    pub const ProposalBondMinimum: Balance = 100 * (TRN as Balance);
}

impl pallet_treasury::Config<DefaultTreasuryInstance> for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = SlashTreasury;
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

// Treasury#2 - EscrowTreasury
parameter_types! {
    pub const EscrowTreasuryId: PalletId = PalletId(*b"escrowry");
}

impl pallet_treasury::Config<EscrowTreasuryInstance> for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = SlashTreasury;
    type PalletId = EscrowTreasuryId;
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

// Treasury#3 - FeeTreasury
parameter_types! {
    pub const FeeTreasuryId: PalletId = PalletId(*b"feetrsry");
}

impl pallet_treasury::Config<FeeTreasuryInstance> for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = SlashTreasury;
    type PalletId = FeeTreasuryId;
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

// Treasury#4 - ParachainTreasury
parameter_types! {
    pub const ParachainTreasuryId: PalletId = PalletId(*b"partrsry");
}

impl pallet_treasury::Config<ParachainTreasuryInstance> for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = SlashTreasury;
    type PalletId = ParachainTreasuryId;
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

// Treasury#5 - SlashTreasury
parameter_types! {
    pub const SlashTreasuryId: PalletId = PalletId(*b"slhtrsry");
}

impl pallet_treasury::Config<SlashTreasuryInstance> for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type OnSlash = SlashTreasury;
    type PalletId = SlashTreasuryId;
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

impl TreasuryAccountProvider<AccountId> for Runtime {
    fn get_treasury_account(treasury_account: TreasuryAccount) -> AccountId {
        match treasury_account {
            TreasuryAccount::Treasury => TreasuryId::get().into_account_truncating(),
            TreasuryAccount::Escrow => EscrowTreasuryId::get().into_account_truncating(),
            TreasuryAccount::Fee => FeeTreasuryId::get().into_account_truncating(),
            TreasuryAccount::Parachain => ParachainTreasuryId::get().into_account_truncating(),
            TreasuryAccount::Slash => SlashTreasuryId::get().into_account_truncating(),
        }
    }
}
