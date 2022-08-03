use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, prelude::*};

pub type ExecutionId = u64;


/// General round information consisting ofindex (one-based), head
/// (beginning block number), and term (round length in number of blocks).
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct SfxSettlement<Balance, Account> {
    pub reward: Balance,

    pub fee: Balance,

    pub payer: Account,

    pub executor: Account,
    // consider adding source::contract?
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct ExecutionRegistryItem<Account, Balance> {
    payee: Account,
    recipient: Account,
    balance: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct SfxRequestCharge<Account, Balance> {
    requester: Account,
    offered_reward: Balance,
    charge_fee: Balance,
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
pub enum Reason {
    UnexpectedFailure,
    ContractReverted,
}

pub trait AccountManager<Account, Balance, Hash> {
    /// Send funds to a recipient via the escrow
    fn deposit(payee: &Account, recipient: &Account, amount: Balance) -> DispatchResult;
    /// Finalize a transaction, with an optional reason for failures
    fn finalize(execution_id: ExecutionId, reason: Option<Reason>) -> DispatchResult;
    /// Issue the funds in the escrow to the recipient
    fn issue(recipient: &Account, amount: Balance) -> DispatchResult;
    /// Split the funds, providing an optional reason for the split
    fn split(
        item: ExecutionRegistryItem<Account, Balance>,
        reason: Option<Reason>,
    ) -> DispatchResult;
    /// Reward executor for successful sfx execution - accounted after successful xtx resolution
    fn commit_charge(
        executor: Account,
        sfx_id: Hash,
    ) -> DispatchResult;

    /// Refund the reward back to the requester. Keep the fees.
    fn refund_charge(
        sfx_id: Hash,
    ) -> DispatchResult;

    /// Charge requester for SFX submission: reward + fees.
    fn charge_requester(
        charge: SfxRequestCharge<Account, Balance>,
    ) -> DispatchResult;
}
