use codec::{Decode, Encode};

use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{fmt::Debug, prelude::*};

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitRole {
    Ambassador,
    Executor,
    Attester,
    Staker,
    Collator,
    ContractAuthor, // Builders
    Relayer,
    Requester,
    Local,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum BenefitSource {
    BootstrapPool,
    Inflation,
    TrafficFees,
    TrafficRewards,
    EscrowUnlock,
    Unsettled,
    SlashTreasury,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct ClaimableArtifacts<Account, Balance> {
    pub beneficiary: Account,
    pub role: CircuitRole,
    pub total_round_claim: Balance,
    pub non_native_asset_id: Option<u32>,
    pub benefit_source: BenefitSource,
}
