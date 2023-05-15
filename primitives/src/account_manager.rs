use crate::{
    claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
    common::RoundInfo,
};
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
pub struct RequestCharge<Account, Balance, AssetId> {
    pub payee: Account,
    pub offered_reward: Balance,
    pub maybe_asset_id: Option<AssetId>,
    pub charge_fee: Balance,
    pub recipient: Option<Account>,
    pub source: BenefitSource,
    pub role: CircuitRole,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum Outcome {
    UnexpectedFailure,
    Revert,
    Commit,
    Slash,
}

pub trait AccountManager<Account, Balance, Hash, BlockNumber, AssetId> {
    /// Lookup charge by Id and fail if not found
    fn get_charge_or_fail(
        charge_id: Hash,
    ) -> Result<RequestCharge<Account, Balance, AssetId>, DispatchError>;
    /// Lookup charge by Id and fail if not found
    fn no_charge_or_fail(charge_id: Hash) -> Result<(), DispatchError>;
    /// Lookup charge by Id and fail if not found
    fn get_settlement(charge_id: Hash) -> Option<Settlement<Account, Balance>>;
    /// Get all settlements by role in the current round
    fn get_settlements_by_role(role: CircuitRole) -> Vec<(Account, Settlement<Account, Balance>)>;
    /// Bump contracts registry nonce in Account Manager nonce state and return charge request Id
    fn bump_contracts_registry_nonce() -> Result<Hash, DispatchError>;
    /// Validate deposit goes through
    fn validate_deposit(
        charge_id: Hash,
        request_charge: RequestCharge<Account, Balance, AssetId>,
    ) -> Result<Balance, DispatchError>;
    /// Send batch deposits to a recipient via the escrow
    fn deposit_batch(batch: &[(Hash, RequestCharge<Account, Balance, AssetId>)]) -> DispatchResult;
    /// Send funds to a recipient via the escrow
    fn deposit(
        charge_id: Hash,
        request_charge: RequestCharge<Account, Balance, AssetId>,
    ) -> DispatchResult;
    /// Finalize a transaction, with an optional reason for failures
    fn finalize(
        charge_id: Hash,
        outcome: Outcome,
        maybe_recipient: Option<Account>,
        maybe_actual_fees: Option<Balance>,
    ) -> DispatchResult;
    /// Assert infallible finalize of a transaction if exists
    fn finalize_infallible(charge_id: Hash, outcome: Outcome) -> bool;

    fn cancel_deposit(charge_id: Hash) -> bool;

    fn assign_deposit(charge_id: Hash, recipient: &Account) -> bool;

    fn transfer_deposit(
        charge_id: Hash,
        new_charge_id: Hash,
        new_reward: Option<Balance>,
        new_payee: Option<&Account>,
        new_recipient: Option<&Account>,
    ) -> DispatchResult;

    fn on_collect_claimable(
        n: BlockNumber,
        r: RoundInfo<BlockNumber>,
    ) -> Result<Vec<ClaimableArtifacts<Account, Balance>>, DispatchError>;

    fn can_withdraw(beneficiary: &Account, amount: Balance, asset_id: Option<AssetId>) -> bool;

    fn deposit_immediately(beneficiary: &Account, amount: Balance, asset_id: Option<AssetId>);

    fn withdraw_immediately(
        beneficiary: &Account,
        amount: Balance,
        asset_id: Option<AssetId>,
    ) -> DispatchResult;
}
