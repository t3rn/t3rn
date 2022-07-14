use crate::common::{Range, RoundIndex};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use frame_support::{pallet_prelude::*, traits::LockIdentifier};
use sp_runtime::{RuntimeDebug, Percent, traits::Zero};
use sp_std::{cmp::{Ordering, PartialOrd}, prelude::*};

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
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum StakerStatus {
    /// Active with no scheduled exit
    Active,
}

/// The activity status of the executor
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
#[derive(PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CapacityStatus {
    /// Reached capacity
    Full,
    /// Empty aka contains no stakes
    Empty,
    /// Partially full (nonempty and not full)
    Partial,
}

/// Request scheduled to change the executor candidate's self-bond.
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
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord, Default)]
pub struct Fixtures<Balance> {
    pub active_set_size: Range<u32>,
    pub   max_commission: Percent,
    pub max_risk: Percent,
    pub min_executor_bond: Balance,
    pub min_candidate_bond: Balance,
    pub min_atomic_stake: Balance,
    pub min_total_stake: Balance,
    pub max_top_stakes_per_candidate: u32,
    pub  max_bottom_stakes_per_candidate: u32,
    pub max_stakes_per_staker: u32,
    pub configure_executor_delay: u32,
    pub leave_candidates_delay: u32,
    pub leave_stakers_delay: u32,
    pub candidate_bond_less_delay: u32,
    pub revoke_stake_delay: u32,
}

impl<Balance: PartialEq + Zero + PartialOrd>  Fixtures<Balance> {
    /// Asserts that all included fixtures are greater than zero.
    pub fn are_valid(&self) -> bool {
        self.active_set_size.min > 0 &&
        self.active_set_size.ideal > 0 &&
        self.active_set_size.max > 0 &&
        self.max_commission > Percent::from_percent(0) &&
        self.max_risk > Percent::from_percent(0) &&
        self.min_executor_bond > Balance::zero() &&
        self.min_candidate_bond > Balance::zero() &&
        self.min_atomic_stake > Balance::zero() &&
        self.min_total_stake > Balance::zero() &&
        self.max_top_stakes_per_candidate > 0 &&
        self.max_bottom_stakes_per_candidate > 0 &&
        self.max_stakes_per_staker > 0 &&
        self.configure_executor_delay > 0 &&
        self.leave_candidates_delay > 0 &&
        self.leave_stakers_delay > 0 &&
        self.candidate_bond_less_delay > 0 &&
        self.revoke_stake_delay > 0
    }
}