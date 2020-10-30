use crate::*;
use frame_support::weights::Weight;
use sp_std::{convert::TryInto, prelude::*, vec::Vec};

pub type PlainAccountIdOf = H256;
pub type PlainSeedOf = H256;
pub type PlainTopicOf = H256;
pub type PlainBlockNumberOf = u64;
pub type PlainBalanceOf = u64;
pub type PlainMomentOf = u64;

pub trait ExtPlain {
    /// Returns the storage entry of the executing account by the given `key`.
    ///
    /// Returns `None` if the `key` wasn't previously set by `set_storage` or
    /// was deleted.
    fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>>;

    /// Sets the storage entry by the given key to the specified value. If `value` is `None` then
    /// the storage entry is deleted.
    fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>);

    /// Transfer some amount of funds into the specified account.
    fn transfer(
        &mut self,
        to: &PlainAccountIdOf,
        value: PlainBalanceOf,
        gas_meter: &mut GasMeterPlain,
    ) -> Result<(), DispatchError>;


    /// Call (possibly transferring some amount of funds) into the specified account.
    fn call(
        &mut self,
        to: &PlainAccountIdOf,
        value: PlainBalanceOf,
        gas_meter: &mut GasMeterPlain,
        input_data: Vec<u8>,
    ) -> ExecResult;

    /// Returns a reference to the account id of the caller.
    fn caller(&self) -> &PlainAccountIdOf;

    /// Returns a reference to the account id of the caller.
    fn requester(&self) -> &PlainAccountIdOf;

    /// Returns a reference to the account id of the current contract.
    fn address(&self) -> &PlainAccountIdOf;

    /// Returns the balance of the current contract.
    ///
    /// The `value_transferred` is already added.
    fn balance(&self) -> PlainBalanceOf;

    /// Returns the value transferred along with this call or as endowment.
    fn value_transferred(&self) -> PlainBalanceOf;

    /// Returns a reference to the timestamp of the current block
    fn now(&self) -> &PlainMomentOf;

    /// Returns the minimum balance that is required for creating an account.
    fn minimum_balance(&self) -> PlainBalanceOf;

    /// Returns the deposit required to create a tombstone upon contract eviction.
    fn tombstone_deposit(&self) -> PlainBalanceOf;

    /// Returns a random number for the current block with the given subject.
    fn random(&self, subject: &[u8]) -> PlainSeedOf;

    /// Deposit an event with the given topics.
    ///
    /// There should not be any duplicates in `topics`.
    fn deposit_event(&mut self, topics: Vec<PlainTopicOf>, data: Vec<u8>);

    /// Set rent allowance of the contract
    fn set_rent_allowance(&mut self, rent_allowance: PlainBalanceOf);

    /// Rent allowance of the contract
    fn rent_allowance(&self) -> PlainBalanceOf;

    /// Returns the current block number.
    fn block_number(&self) -> PlainBlockNumberOf;

    /// Returns the maximum allowed size of a storage item.
    fn max_value_size(&self) -> u32;

    /// Returns the price for the specified amount of weight.
    fn get_weight_price(&self, weight: Weight) -> PlainBalanceOf;
}
