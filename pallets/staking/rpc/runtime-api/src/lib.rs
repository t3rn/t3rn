//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use sp_std::vec::Vec;
use t3rn_primitives::{
    common::{OrderedSet, RoundIndex},
    staking::{
        Bond, ExecutorInfo, ExecutorSnapshot, Fixtures, RpcBalance, RpcBond, RpcCandidateMetadata,
        RpcExecutorSnapshot, RpcFixtures, RpcStakerMetadata,
    },
};

sp_api::decl_runtime_apis! {
    pub trait StakingRuntimeApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        fn get_fixtures() -> RpcFixtures<RpcBalance<Balance>>;
        fn get_total_value_locked() -> RpcBalance<Balance>;
        fn get_active_stake(round: RoundIndex) -> RpcBalance<Balance>;
        fn get_executor_config(who: AccountId) -> Option<ExecutorInfo>;
        fn get_executor_snapshot(round: RoundIndex, who: AccountId) -> Option<RpcExecutorSnapshot<AccountId, RpcBalance<Balance>>>;
        fn get_candidate_info(who: AccountId) -> Option<RpcCandidateMetadata<RpcBalance<Balance>>>;
        fn get_staker_info(who: AccountId) -> Option<RpcStakerMetadata<AccountId, RpcBalance<Balance>>>;
        fn list_candidates() -> OrderedSet<RpcBond<AccountId, RpcBalance<Balance>>>;
        fn list_active_set() -> Vec<AccountId>;
    }
}
