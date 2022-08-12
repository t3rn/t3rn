use crate::common::RoundInfo;
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{fmt::Debug, prelude::*};

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitRole {
    Ambassador,
    Executor,
    Staker,
    Collator,
    ContractAuthor, // Builders
    Relayer,
    Requester,
    Local,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum BenefitSource {
    TrafficRewards,
    BootstrapPool,
    Inflation,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct ClaimableArtifacts<Account, Balance> {
    pub beneficiary: Account,
    pub role: CircuitRole,
    pub total_round_claim: Balance,
    pub benefit_source: BenefitSource,
}
