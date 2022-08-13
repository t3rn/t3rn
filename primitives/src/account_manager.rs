use crate::claimable::{BenefitSource, CircuitRole};
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::{fmt::Debug, prelude::*};

pub type ExecutionId = u64;

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
pub struct RequestCharge<Account, Balance> {
    pub payee: Account,
    pub offered_reward: Balance,
    pub charge_fee: Balance,
    pub recipient: Account,
    pub source: BenefitSource,
    pub role: CircuitRole,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum Outcome {
    UnexpectedFailure,
    Revert,
    Commit,
}

pub trait AccountManager<Account, Balance, Hash, BlockNumber> {
    /// Lookup charge by Id and fail if not found
    fn get_charge_or_fail(
        charge_id: Hash,
    ) -> Result<RequestCharge<Account, Balance>, DispatchError>;
    /// Lookup charge by Id and fail if not found
    fn no_charge_or_fail(charge_id: Hash) -> Result<(), DispatchError>;
    /// Bump contracts registry nonce in Account Manager nonce state and return charge request Id
    fn bump_contracts_registry_nonce() -> Result<Hash, DispatchError>;
    /// Send funds to a recipient via the escrow
    fn deposit(
        charge_id: Hash,
        payee: &Account,
        charge_fee: Balance,
        offered_reward: Balance,
        source: BenefitSource,
        role: CircuitRole,
        recipient: Option<Account>,
    ) -> DispatchResult;
    /// Finalize a transaction, with an optional reason for failures
    fn finalize(
        charge_id: Hash,
        outcome: Outcome,
        maybe_recipient: Option<Account>,
        maybe_actual_fees: Option<Balance>,
    ) -> DispatchResult;
}
