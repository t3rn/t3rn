use crate::ExecutionId;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, prelude::*, vec::Vec};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct ExecutionRegistryItem<Account, Balance> {
    payee: Account,
    recipient: Account,
    balance: Balance,
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

pub enum Reason {
    UnexpectedFailure,
    ContractReverted,
}

pub trait AccountManager<Account, Balance> {
    /// Send funds to a recipient via the escrow
    fn deposit(
        &self,
        execution_id: ExecutionId,
        payee: Account,
        recipient: Account,
        amount: Balance,
    );
    /// Finalize a transaction, with an optional reason for failures
    fn finalize(&self, execution_id: ExecutionId, reason: Option<Reason>);
    /// Issue the funds in the escrow to the recipient
    fn issue(&self, recipient: &Account, amount: Balance);
    /// Split the funds, providing an optional reason for the split
    fn split(&self, item: ExecutionRegistryItem<Account, Balance>, reason: Option<Reason>);
}
