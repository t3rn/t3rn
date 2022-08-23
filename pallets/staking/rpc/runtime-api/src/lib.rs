//! Runtime API definition required by Contracts Registry RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding Contracts Registry access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sp_std::vec::Vec;
use t3rn_primitives::{
    common::{OrderedSet, RoundIndex},
    staking::{
        Bond, CandidateMetadataFormat, ExecutorInfo, ExecutorSnapshot, Fixtures,
        StakerMetadataFormat,
    },
};

#[derive(Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
/// a wrapper around a balance, used in RPC to workaround a bug where using u128
/// in runtime-apis fails. See <https://github.com/paritytech/substrate/issues/4641>
pub struct RpcBalance<T> {
    #[cfg_attr(feature = "std", serde(bound(serialize = "T: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    #[cfg_attr(feature = "std", serde(bound(deserialize = "T: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    pub balance: T,
}

#[cfg(feature = "std")]
fn serialize_as_string<S: Serializer, T: std::fmt::Display>(
    t: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&t.to_string())
}

#[cfg(feature = "std")]
fn deserialize_from_string<'de, D: Deserializer<'de>, T: std::str::FromStr>(
    deserializer: D,
) -> Result<T, D::Error> {
    let s = String::deserialize(deserializer)?;
    s.parse::<T>()
        .map_err(|_| serde::de::Error::custom("Parse from string failed"))
}

sp_api::decl_runtime_apis! {
    pub trait StakingRuntimeApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        // fn get_fixtures() -> Fixtures<RpcBalance<Balance>>;
        fn get_total_value_locked() -> RpcBalance<Balance>;
        fn get_active_stake(round: RoundIndex) -> RpcBalance<Balance>;
        // fn get_executor_config(who: AccountId) -> Option<ExecutorInfo>;
        // fn get_executor_snapshot(round: RoundIndex, who: AccountId) -> Option<ExecutorSnapshot<AccountId, RpcBalance<Balance>>>;
        // fn get_candidate_info(who: AccountId) -> Option<CandidateMetadataFormat<RpcBalance<Balance>>>;
        // fn get_staker_info(who: AccountId) -> Option<StakerMetadataFormat<AccountId, RpcBalance<Balance>>>;
        // fn list_candidates() -> OrderedSet<Bond<AccountId, RpcBalance<Balance>>>;
        // fn list_active_set() -> Vec<AccountId>;
    }
}
