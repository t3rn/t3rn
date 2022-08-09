use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, prelude::*};
use crate::bridges::polkadot_core::BlockNumber;
use crate::circuit_clock::{CircuitRole, BenefitSource};

pub type ExecutionId = u64;

use crate::circuit_clock::ClaimableArtifacts;
use crate::common::RoundInfo;

/// General round information consisting ofindex (one-based), head
/// (beginning block number), and term (round length in number of blocks).
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Settlement<Account, Balance> {
    pub requester: Account,
    pub recipient: Account,
    pub settlement_amount: Balance,
    pub outcome: Outcome,
    pub source: BenefitSource,
    pub role: CircuitRole,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct ExecutionRegistryItem<Account, Balance> {
    pub payee: Account,
    pub recipient: Account,
    pub balance: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct RequestCharge<Account, Balance> {
    pub payee: Account,
    pub offered_reward: Balance,
    pub charge_fee: Balance,
    pub recipient: Account,
    pub source: BenefitSource,
    pub role: CircuitRole,
}

impl<Account, Balance> ExecutionRegistryItem<Account, Balance> {
    pub fn new(payee: Account, recipient: Account, balance: Balance) -> Self {
        Self {
            payee,
            recipient,
            balance,
        }
    }

    pub fn payee(&self) -> &Account {
        &self.payee
    }

    pub fn recipient(&self) -> &Account {
        &self.recipient
    }

    pub fn balance(&self) -> &Balance {
        &self.balance
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum Outcome {
    UnexpectedFailure,
    Revert,
    Commit,
}

pub trait AccountManager<Account, Balance, Hash, BlockNumber> {
    /// Lookup charge by Id and fail if not found
    fn get_charge_or_fail(charge_id: Hash) -> Result<RequestCharge<Account, Balance>, &'static str>;
    /// Lookup charge by Id and fail if not found
    fn no_charge_or_fail(charge_id: Hash) -> Result<(), &'static str>;
    /// Bump contracts registry nonce in Account Manager nonce state and return charge request Id
    fn bump_contracts_registry_nonce() -> Result<Hash, &'static str>;
    /// Send funds to a recipient via the escrow
    fn deposit(charge_id: Hash, payee: &Account, charge_fee: Balance, offered_reward: Balance, source: BenefitSource, role: CircuitRole, recipient: Option<Account>) -> DispatchResult;
    /// Finalize a transaction, with an optional reason for failures
    fn finalize(charge_id: Hash, outcome: Outcome, maybe_recipient: Option<Account>, maybe_actual_fees: Option<Balance>) -> DispatchResult;

    fn on_collect_claimable(n: BlockNumber, r: RoundInfo<BlockNumber>) -> Result<Vec<ClaimableArtifacts<Account, Balance>>, &'static str>;
}
