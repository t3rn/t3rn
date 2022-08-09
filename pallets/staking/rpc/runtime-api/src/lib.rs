//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::codec::Codec;
use sp_std::vec::Vec;
use t3rn_primitives::{
    common::{OrderedSet, RoundIndex},
    staking::{
        Bond, CandidateMetadataFormat, ExecutorInfo, ExecutorSnapshot, Fixtures,
        StakerMetadataFormat,
    },
};

sp_api::decl_runtime_apis! {
    pub trait StakingRuntimeApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        fn get_fixtures() -> Fixtures<Balance>;
        fn get_total_value_locked() -> Balance;
        fn get_active_stake(round: RoundIndex) -> Balance;
        fn get_executor_config(who: AccountId) -> Option<ExecutorInfo>;
        fn get_executor_snapshot(round: RoundIndex, who: AccountId) -> Option<ExecutorSnapshot<AccountId, Balance>>;
        fn get_candidate_info(who: AccountId) -> Option<CandidateMetadataFormat<Balance>>;
        fn get_staker_info(who: AccountId) -> Option<StakerMetadataFormat<AccountId, Balance>>;
        fn list_candidates() -> OrderedSet<Bond<AccountId, Balance>>;
        fn list_active_set() -> Vec<AccountId>;
    }
}
