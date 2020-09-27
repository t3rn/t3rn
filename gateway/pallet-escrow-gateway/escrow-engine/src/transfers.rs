use escrow_gateway_primitives::Trait;
use codec::{Decode, Encode};
use frame_support::sp_runtime::traits::Saturating;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, ExistenceRequirement, Time},
};
use sp_std::{convert::TryInto, prelude::*, vec::Vec};
use system;

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
#[codec(compact)]
pub struct TransferEntry {
    pub to: Vec<u8>,
    pub value: u32,
    pub data: Vec<u8>,
}

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub fn just_transfer<'a, T: Trait>(
    transactor: &T::AccountId,
    dest: &T::AccountId,
    value: BalanceOf<T>,
) -> DispatchResult {
    <T as Trait>::Currency::transfer(transactor, dest, value, ExistenceRequirement::KeepAlive)
}

pub fn commit_deferred_transfers<T: Trait>(
    escrow_account: T::AccountId,
    transfers: &mut Vec<TransferEntry>,
) {
    // Give the money back to the requester from the transfers that succeeded.
    for mut transfer in transfers.iter() {
        just_transfer::<T>(
            &escrow_account,
            &T::AccountId::decode(&mut &transfer.to[..]).unwrap(),
            BalanceOf::<T>::from(transfer.value),
        );
    }
}

pub fn escrow_transfer<'a, T: Trait>(
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    target_to: &T::AccountId,
    value: BalanceOf<T>,
    mut transfers: &mut Vec<TransferEntry>,
) -> Result<(), DispatchError> {
    println!(
        "DEBUG escrow_exec -- escrow_transfer REQ {:?} ES_ACC {:?} VAL {:?}",
        requester, escrow_account, value
    );
    // Verify that requester has enough money to make the transfers from within the contract.
    if <T as Trait>::Currency::total_balance(&requester.clone()).saturating_sub(value)
        < <T as Trait>::Currency::minimum_balance()
    {
        println!(
            "DEBUG escrow_exec -- REQUESTER {:?} VAL {:?} ST {:?} ",
            <T as Trait>::Currency::free_balance(&requester.clone()),
            value,
            <T as Trait>::Currency::minimum_balance(),
        );
        return Err(DispatchError::Other(
            "Escrow Transfer failed as the requester doesn't have enough balance.",
        ));
    }
    // Just transfer here the value of internal for contract transfer to escrow account.
    return match just_transfer::<T>(requester, escrow_account, value) {
        Ok(_) => {
            transfers.push(TransferEntry {
                to: T::AccountId::encode(target_to),
                value: TryInto::<u32>::try_into(value).ok().unwrap(),
                data: Vec::new(),
            });
            Ok(())
        }
        Err(err) => Err(err),
    };
}
