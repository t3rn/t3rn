use crate::EscrowTrait;
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, ExistenceRequirement},
};
use sp_core::H256;

use frame_system as system;
use sp_std::{convert::TryInto, prelude::*, vec::Vec};

#[derive(Debug, Default, PartialEq, Eq, Encode, Decode, Clone)]
// #[codec(compact)] ToDo: Check if events can still be encoded/decoded from ext clients
pub struct TransferEntry {
    pub to: H256,
    pub value: u32,
    pub data: Vec<u8>,
}

pub type EscrowedBalanceOf<T, E> =
    <<E as EscrowTrait<T>>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

pub type EscrowedCurrencyOf<T, E> = <E as EscrowTrait<T>>::Currency;

pub type BalanceOf<T> =
    <<T as EscrowTrait<T>>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

pub fn just_transfer<T: frame_system::Config, E: EscrowTrait<T>>(
    transactor: &T::AccountId,
    dest: &T::AccountId,
    value: EscrowedBalanceOf<T, E>,
) -> DispatchResult {
    EscrowedCurrencyOf::<T, E>::transfer(transactor, dest, value, ExistenceRequirement::KeepAlive)
}

pub fn commit_deferred_transfers<T: frame_system::Config, E: EscrowTrait<T>>(
    escrow_account: T::AccountId,
    transfers: &mut [TransferEntry],
) -> DispatchResult {
    // Give the money back to the requester from the transfers that succeeded.s
    for transfer in transfers.iter() {
        just_transfer::<T, E>(
            &escrow_account,
            &h256_to_account(transfer.to),
            EscrowedBalanceOf::<T, E>::from(transfer.value),
        )
        .map_err(|e| e)?;
    }
    Ok(())
}

pub fn escrow_transfer<T: frame_system::Config, E: EscrowTrait<T>>(
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    target_to: &T::AccountId,
    value: EscrowedBalanceOf<T, E>,
    transfers: &mut Vec<TransferEntry>,
) -> Result<(), DispatchError> {
    // Verify that requester has enough money to make the transfers from within the contract.
    let balance = <E as EscrowTrait<T>>::Currency::total_balance(&requester.clone());
    let min = <E as EscrowTrait<T>>::Currency::minimum_balance();

    if balance < min + value {
        return Err(DispatchError::Other(
            "Escrow Transfer failed as the requester doesn't have enough balance.",
        ))
    }
    // Just transfer here the value of internal for contract transfer to escrow account.
    return match just_transfer::<T, E>(requester, escrow_account, value) {
        Ok(_) => {
            transfers.push(TransferEntry {
                to: account_encode_to_h256(target_to.encode().as_slice()),
                value: TryInto::<u32>::try_into(value).ok().unwrap(),
                data: Vec::new(),
            });
            Ok(())
        },
        Err(err) => Err(err),
    }
}

pub fn account_encode_to_h256(account_bytes: &[u8]) -> H256 {
    match account_bytes.len() {
        // Normal case, expect 32-bytes long account id (public key) for regular runtime.
        32 => H256::from_slice(account_bytes),
        // Shorter (8-bytes) account id (represented as u64) for tests.
        8 => {
            // H256::from_low_u64_be doesn't work for runtime as it has no std.
            H256::from_slice(
                &[
                    [0_u8; 24].to_vec(),
                    u64::from_le_bytes(account_bytes.try_into().unwrap())
                        .to_be_bytes()
                        .to_vec(),
                ]
                .concat()[..],
            )
        },
        _ => {
            panic!(
                "Surprised by AccountId bytes length different than 32 or 8 bytes while serializing. Not supported."
            );
            // H256::default()
        },
    }
}

pub fn h256_to_account<D: Decode + Encode>(account_h256: H256) -> D {
    let decoded_account = D::decode(&mut &account_h256[..]).unwrap();

    match decoded_account.encode().len() {
        32 => decoded_account,
        8 => {
            let mut last_8b = account_h256.as_bytes()[24..].to_vec();
            last_8b.reverse();
            D::decode(&mut &last_8b[..]).unwrap()
        },
        _ => {
            panic!(
                "Surprised by AccountId bytes length different than 32 or 8 bytes while deserializing. Not supported."
            );
            // D::decode(&mut &H256::default()[..]).unwrap()
        },
    }
}
