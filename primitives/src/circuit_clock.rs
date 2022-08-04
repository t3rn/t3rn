use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{fmt::Debug, prelude::*};
use crate::common::RoundInfo;

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

#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum BenefitSource {
    TrafficRewards,
    BootstrapPool,
    Inflation,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct ClaimableArtifacts<Account, Balance> {
    pub beneficiary: Account,
    pub role: CircuitRole,
    pub total_round_claim: Balance,
    pub benefit_source: BenefitSource,
}

pub trait CircuitClock<T: frame_system::Config, Balance> {
    fn current_round() -> RoundInfo<T::BlockNumber>;
}
