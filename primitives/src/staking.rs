use crate::common::{OrderedSet, Range, RoundIndex};
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::*, traits::LockIdentifier};
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sp_runtime::{traits::Zero, Percent, RuntimeDebug};
use sp_std::{
    cmp::{Ordering, PartialOrd},
    prelude::*,
};

pub const EXECUTOR_LOCK_ID: LockIdentifier = *b"execstkl";
pub const STAKER_LOCK_ID: LockIdentifier = *b"stkrstkl";

/// Staker's bond adjustment - used with locks.
#[derive(Clone, Copy, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum StakeAdjust<Balance> {
    Increase(Balance),
    Decrease,
}

/// Convey relevant information describing if a delegator was added to the top or bottom
/// Stakes added to the top yield a new total
#[derive(Clone, Copy, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum StakerAdded<Balance> {
    ToTop { new_total: Balance },
    ToBottom,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
/// Snapshot of collator state at the start of the round for which they are selected
pub struct ExecutorSnapshot<AccountId, Balance> {
    /// The total value locked by the collator.
    pub bond: Balance,

    /// The rewardable stakes. This list is a subset of total delegators, where certain
    /// delegators are adjusted based on their scheduled
    /// [DelegationChange::Revoke] or [DelegationChange::Decrease] action.
    pub stakes: Vec<Bond<AccountId, Balance>>,

    /// The total counted value locked for the collator, including the self bond + total staked by
    /// top delegators.
    pub total: Balance,
}

impl<A: PartialEq, B: PartialEq> PartialEq for ExecutorSnapshot<A, B> {
    fn eq(&self, other: &Self) -> bool {
        let must_be_true = self.bond == other.bond && self.total == other.total;
        if !must_be_true {
            return false
        }
        for (
            Bond {
                owner: o1,
                amount: a1,
            },
            Bond {
                owner: o2,
                amount: a2,
            },
        ) in self.stakes.iter().zip(other.stakes.iter())
        {
            if o1 != o2 || a1 != a2 {
                return false
            }
        }
        true
    }
}

impl<A, B: Default> Default for ExecutorSnapshot<A, B> {
    fn default() -> ExecutorSnapshot<A, B> {
        ExecutorSnapshot {
            bond: B::default(),
            stakes: Vec::new(),
            total: B::default(),
        }
    }
}

/// Generic type describing either an executor's self-bond or a staker's bond.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct Bond<AccountId, Balance> {
    pub owner: AccountId,
    pub amount: Balance,
}

impl<A: Decode, B: Default> Default for Bond<A, B> {
    fn default() -> Bond<A, B> {
        Bond {
            owner: A::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed"),
            amount: B::default(),
        }
    }
}

impl<A, B: Default> Bond<A, B> {
    pub fn from_owner(owner: A) -> Self {
        Bond {
            owner,
            amount: B::default(),
        }
    }
}

impl<AccountId: Ord, Balance: PartialEq> Eq for Bond<AccountId, Balance> {}

impl<AccountId: Ord, Balance: PartialEq> Ord for Bond<AccountId, Balance> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.owner.cmp(&other.owner)
    }
}

impl<AccountId: Ord, Balance: PartialEq> PartialOrd for Bond<AccountId, Balance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// impl<AccountId: Ord, Balance> PartialEq for Bond<AccountId, Balance> {
//     fn eq(&self, other: &Self) -> bool {
//         self.owner == other.owner
//     }
// }

/// The activity status of the staker.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum StakerStatus {
    /// Active with no scheduled exit
    Active,
}

/// The activity status of the executor
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum ExecutorStatus {
    /// Committed to be online and producing valid blocks (not equivocating)
    Active,
    /// Temporarily inactive and excused for inactivity
    Idle,
    /// Bonded until the inner round
    Leaving(RoundIndex),
}

impl Default for ExecutorStatus {
    fn default() -> ExecutorStatus {
        ExecutorStatus::Idle
    }
}

/// Capacity status for top or bottom stakes.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CapacityStatus {
    /// Reached capacity
    Full,
    /// Empty aka contains no stakes
    Empty,
    /// Partially full (nonempty and not full)
    Partial,
}

/// Request scheduled to change the executor candidate's self-bond.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CandidateBondLessRequest<Balance> {
    pub amount: Balance,
    pub when_executable: RoundIndex,
}

/// An action that can be performed upon a stake
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub enum StakingAction<Balance> {
    Revoke(Balance),
    Decrease(Balance),
}

impl<Balance: Copy> StakingAction<Balance> {
    /// Returns the wrapped amount value.
    pub fn amount(&self) -> Balance {
        match self {
            StakingAction::Revoke(amount) => *amount,
            StakingAction::Decrease(amount) => *amount,
        }
    }
}

/// Represents a scheduled request that define a [StakingAction]. The request is executable
/// iff the provided [RoundIndex] is achieved.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub struct ScheduledStakingRequest<AccountId, Balance> {
    pub staker: AccountId,
    pub when_executable: RoundIndex,
    pub action: StakingAction<Balance>,
}

/// Represents a cancelled scheduled request for emitting an event.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CancelledScheduledStakingRequest<Balance> {
    pub when_executable: RoundIndex,
    pub action: StakingAction<Balance>,
}

impl<A, B> From<ScheduledStakingRequest<A, B>> for CancelledScheduledStakingRequest<B> {
    fn from(request: ScheduledStakingRequest<A, B>) -> Self {
        CancelledScheduledStakingRequest {
            when_executable: request.when_executable,
            action: request.action,
        }
    }
}

/// Executor configuration information.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ExecutorInfo {
    pub commission: Percent,
    pub risk: Percent,
}

/// Represents a scheduled request for an executor configuration change.
/// The request is executable if the provided [RoundIndex] is achieved.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub struct ScheduledConfigurationRequest {
    pub when_executable: RoundIndex,
    pub commission: Percent,
    pub risk: Percent,
}

/// Protocol enforced thresholds and delays for staking.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord, Default,
)]
pub struct Fixtures<Balance> {
    pub active_set_size: Range<u32>,
    pub max_commission: Percent,
    pub max_risk: Percent,
    pub min_executor_bond: Balance,
    pub min_candidate_bond: Balance,
    pub min_atomic_stake: Balance,
    pub min_total_stake: Balance,
    pub max_top_stakes_per_candidate: u32,
    pub max_bottom_stakes_per_candidate: u32,
    pub max_stakes_per_staker: u32,
    pub configure_executor_delay: u32,
    pub leave_candidates_delay: u32,
    pub leave_stakers_delay: u32,
    pub candidate_bond_less_delay: u32,
    pub revoke_stake_delay: u32,
}

impl<Balance: PartialEq + Zero + PartialOrd> Fixtures<Balance> {
    /// Asserts that all included fixtures are greater than zero.
    pub fn are_valid(&self) -> bool {
        self.active_set_size.min > 0
            && self.active_set_size.ideal > 0
            && self.active_set_size.max > 0
            && self.max_commission > Percent::from_percent(0)
            && self.max_risk > Percent::from_percent(0)
            && self.min_executor_bond > Balance::zero()
            && self.min_candidate_bond > Balance::zero()
            && self.min_atomic_stake > Balance::zero()
            && self.min_total_stake > Balance::zero()
            && self.max_top_stakes_per_candidate > 0
            && self.max_bottom_stakes_per_candidate > 0
            && self.max_stakes_per_staker > 0
            && self.configure_executor_delay > 0
            && self.leave_candidates_delay > 0
            && self.leave_stakers_delay > 0
            && self.candidate_bond_less_delay > 0
            && self.revoke_stake_delay > 0
    }
}

#[derive(Eq, PartialEq, Encode, Decode, Default, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
/// a wrapper around a balance, used in RPC to workaround a bug where using u128
/// in runtime-apis fails. See <https://github.com/paritytech/substrate/issues/4641>
pub struct RpcBalance<T> {
    #[cfg_attr(feature = "std", serde(bound(serialize = "T: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    #[cfg_attr(feature = "std", serde(bound(deserialize = "T: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    pub amount: T,
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

// StakingFixtures<RpcBalance<Balance>>
/// Protocol enforced thresholds and delays for staking.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord, Default,
)]
pub struct RpcFixtures<RpcBalance> {
    pub active_set_size: Range<u32>,
    pub max_commission: Percent,
    pub max_risk: Percent,
    pub min_executor_bond: RpcBalance,
    pub min_candidate_bond: RpcBalance,
    pub min_atomic_stake: RpcBalance,
    pub min_total_stake: RpcBalance,
    pub max_top_stakes_per_candidate: u32,
    pub max_bottom_stakes_per_candidate: u32,
    pub max_stakes_per_staker: u32,
    pub configure_executor_delay: u32,
    pub leave_candidates_delay: u32,
    pub leave_stakers_delay: u32,
    pub candidate_bond_less_delay: u32,
    pub revoke_stake_delay: u32,
}

use crate::AccountId;
type Balance = u128;

impl From<Fixtures<Balance>> for RpcFixtures<RpcBalance<Balance>> {
    fn from(fixtures: Fixtures<Balance>) -> Self {
        RpcFixtures {
            active_set_size: fixtures.active_set_size,
            max_commission: fixtures.max_commission,
            max_risk: fixtures.max_risk,
            min_executor_bond: RpcBalance {
                amount: fixtures.min_executor_bond,
            },
            min_candidate_bond: RpcBalance {
                amount: fixtures.min_candidate_bond,
            },
            min_atomic_stake: RpcBalance {
                amount: fixtures.min_atomic_stake,
            },
            min_total_stake: RpcBalance {
                amount: fixtures.min_total_stake,
            },
            max_top_stakes_per_candidate: fixtures.max_top_stakes_per_candidate,
            max_bottom_stakes_per_candidate: fixtures.max_bottom_stakes_per_candidate,
            max_stakes_per_staker: fixtures.max_stakes_per_staker,
            configure_executor_delay: fixtures.configure_executor_delay,
            leave_candidates_delay: fixtures.leave_candidates_delay,
            leave_stakers_delay: fixtures.leave_stakers_delay,
            candidate_bond_less_delay: fixtures.candidate_bond_less_delay,
            revoke_stake_delay: fixtures.revoke_stake_delay,
        }
    }
}

/// Generic type describing either an executor's self-bond or a staker's bond.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
pub struct RpcBond<AccountId, RpcBalance> {
    pub owner: AccountId,
    pub amount: RpcBalance,
}

impl From<Bond<AccountId, Balance>> for RpcBond<AccountId, RpcBalance<Balance>> {
    fn from(bond: Bond<AccountId, Balance>) -> Self {
        RpcBond {
            owner: bond.owner,
            amount: RpcBalance {
                amount: bond.amount,
            },
        }
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// Snapshot of collator state at the start of the round for which they are selected
pub struct RpcExecutorSnapshot<AccountId, RpcBalance> {
    /// The total value locked by the collator.
    pub bond: RpcBalance,

    /// The rewardable stakes. This list is a subset of total delegators, where certain
    /// delegators are adjusted based on their scheduled
    /// [DelegationChange::Revoke] or [DelegationChange::Decrease] action.
    pub stakes: Vec<RpcBond<AccountId, RpcBalance>>,

    /// The total counted value locked for the collator, including the self bond + total staked by
    /// top delegators.
    pub total: RpcBalance,
}

impl From<ExecutorSnapshot<AccountId, Balance>>
    for RpcExecutorSnapshot<AccountId, RpcBalance<Balance>>
{
    fn from(executor_snapshot: ExecutorSnapshot<AccountId, Balance>) -> Self {
        RpcExecutorSnapshot {
            bond: RpcBalance {
                amount: executor_snapshot.bond,
            },
            stakes: executor_snapshot
                .stakes
                .into_iter()
                .map(|bond| RpcBond::from(bond))
                .collect::<Vec<RpcBond<AccountId, RpcBalance<Balance>>>>(),
            total: RpcBalance {
                amount: executor_snapshot.total,
            },
        }
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// Staker state
pub struct StakerMetadataFormat<AccountId, Balance> {
    /// Staker account
    pub id: AccountId,
    /// All current stakes
    pub stakes: OrderedSet<Bond<AccountId, Balance>>,
    /// Total balance locked for this staker
    pub total: Balance,
    /// Sum of pending revocation amounts + bond less amounts
    pub less_total: Balance,
    /// Status for this staker
    pub status: StakerStatus,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// Staker state
pub struct RpcStakerMetadata<AccountId, RpcBalance> {
    /// Staker account
    pub id: AccountId,
    /// All current stakes
    pub stakes: OrderedSet<RpcBond<AccountId, RpcBalance>>,
    /// Total balance locked for this staker
    pub total: RpcBalance,
    /// Sum of pending revocation amounts + bond less amounts
    pub less_total: RpcBalance,
    /// Status for this staker
    pub status: StakerStatus,
}

impl From<StakerMetadataFormat<AccountId, Balance>>
    for RpcStakerMetadata<AccountId, RpcBalance<Balance>>
{
    fn from(staker_metadata: StakerMetadataFormat<AccountId, Balance>) -> Self {
        RpcStakerMetadata {
            id: staker_metadata.id,
            stakes: OrderedSet::from(
                staker_metadata
                    .stakes
                    .0
                    .into_iter()
                    .map(|bond| RpcBond::from(bond))
                    .collect::<Vec<RpcBond<AccountId, RpcBalance<Balance>>>>(),
            ),
            total: RpcBalance {
                amount: staker_metadata.total,
            },
            less_total: RpcBalance {
                amount: staker_metadata.less_total,
            },
            status: staker_metadata.status,
        }
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// All candidate info except the top and bottom stakes
pub struct CandidateMetadataFormat<Balance> {
    /// This candidate's self bond amount
    pub bond: Balance,
    /// Total number of stakes to this candidate
    pub stake_count: u32,
    /// Self bond + sum of top stakes
    pub total_counted: Balance,
    /// The smallest top stake amount
    pub lowest_top_stake_amount: Balance,
    /// The highest bottom stake amount
    pub highest_bottom_stake_amount: Balance,
    /// The smallest bottom stake amount
    pub lowest_bottom_stake_amount: Balance,
    /// Capacity status for top stakes
    pub top_capacity: CapacityStatus,
    /// Capacity status for bottom stakes
    pub bottom_capacity: CapacityStatus,
    /// Maximum 1 pending request to decrease candidate self bond at any given time
    pub request: Option<CandidateBondLessRequest<Balance>>,
    /// Current status of the executor
    pub status: ExecutorStatus,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
/// All candidate info except the top and bottom stakes
pub struct RpcCandidateMetadata<RpcBalance> {
    /// This candidate's self bond amount
    pub bond: RpcBalance,
    /// Total number of stakes to this candidate
    pub stake_count: u32,
    /// Self bond + sum of top stakes
    pub total_counted: RpcBalance,
    /// The smallest top stake amount
    pub lowest_top_stake_amount: RpcBalance,
    /// The highest bottom stake amount
    pub highest_bottom_stake_amount: RpcBalance,
    /// The smallest bottom stake amount
    pub lowest_bottom_stake_amount: RpcBalance,
    /// Capacity status for top stakes
    pub top_capacity: CapacityStatus,
    /// Capacity status for bottom stakes
    pub bottom_capacity: CapacityStatus,
    /// Maximum 1 pending request to decrease candidate self bond at any given time
    pub request: Option<RpcCandidateBondLessRequest<RpcBalance>>,
    /// Current status of the executor
    pub status: ExecutorStatus,
}

impl From<CandidateMetadataFormat<Balance>> for RpcCandidateMetadata<RpcBalance<Balance>> {
    fn from(candidate_metadata: CandidateMetadataFormat<Balance>) -> Self {
        RpcCandidateMetadata {
            bond: RpcBalance {
                amount: candidate_metadata.bond,
            },
            stake_count: candidate_metadata.stake_count,
            total_counted: RpcBalance {
                amount: candidate_metadata.total_counted,
            },
            lowest_top_stake_amount: RpcBalance {
                amount: candidate_metadata.lowest_top_stake_amount,
            },
            highest_bottom_stake_amount: RpcBalance {
                amount: candidate_metadata.highest_bottom_stake_amount,
            },
            lowest_bottom_stake_amount: RpcBalance {
                amount: candidate_metadata.lowest_bottom_stake_amount,
            },
            top_capacity: candidate_metadata.top_capacity,
            bottom_capacity: candidate_metadata.bottom_capacity,
            request: candidate_metadata
                .request
                .map(|candidate_bond_less_request| RpcCandidateBondLessRequest {
                    amount: RpcBalance {
                        amount: candidate_bond_less_request.amount,
                    },
                    when_executable: candidate_bond_less_request.when_executable,
                }),
            status: candidate_metadata.status,
        }
    }
}

/// Request scheduled to change the executor candidate's self-bond.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RpcCandidateBondLessRequest<RpcBalance> {
    pub amount: RpcBalance,
    pub when_executable: RoundIndex,
}

pub trait Staking<AccountId, Balance> {
    fn fixtures() -> Fixtures<Balance>;
    fn total_value_locked() -> Balance;
    fn staked(round: RoundIndex) -> Balance;
    fn executor_config(who: AccountId) -> Option<ExecutorInfo>;
    fn at_stake(round: RoundIndex, who: AccountId) -> Option<ExecutorSnapshot<AccountId, Balance>>;
    fn candidate_info(who: AccountId) -> Option<CandidateMetadataFormat<Balance>>;
    fn staker_info(who: AccountId) -> Option<StakerMetadataFormat<AccountId, Balance>>;
    fn candidate_pool() -> OrderedSet<Bond<AccountId, Balance>>;
    fn active_set() -> Vec<AccountId>;
}
