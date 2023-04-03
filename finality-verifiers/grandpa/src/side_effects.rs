use crate::{Config, Error};
use codec::{Decode, Encode};
use sp_runtime::DispatchError;
use sp_std::{vec, vec::Vec};

#[derive(Encode, Decode)]
pub enum TransferEventStub<T: frame_system::Config, Balance> {
    Endowed(T::AccountId, Balance),
    DustLost(T::AccountId, Balance),
    Transfer {
        from: T::AccountId,
        to: T::AccountId,
        amount: Balance,
    },
}

type CurrencyId = u32;

#[derive(Encode, Decode)]
pub enum MultiTransferEventStub<T: frame_system::Config, Balance, CurrencyId> {
    Endowed(CurrencyId, T::AccountId, Balance),
    DustLost(CurrencyId, T::AccountId, Balance),
    Transfer {
        currency_id: CurrencyId,
        from: T::AccountId,
        to: T::AccountId,
        amount: Balance,
    },
}

pub(crate) fn decode_event<T: Config<I>, I: 'static>(
    id: &[u8; 4],
    mut encoded_event: Vec<u8>,
    value_abi_unsigned_type: &[u8],
) -> Result<(Vec<Vec<u8>>, Vec<u8>), DispatchError> {
    // the first byte is the pallet index, which we don't need
    let _ = encoded_event.remove(0);
    match &id {
        &b"tran" => {
            // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
            match value_abi_unsigned_type {
                b"uint32" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u32>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                b"uint64" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u64>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                b"uint128" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u128>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                &_ => Err(Error::<T, I>::EventDecodingFailed.into()),
            }
        },
        &b"swap" | &b"aliq" => match value_abi_unsigned_type {
            b"uint32" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u32, CurrencyId>::Transfer {
                    currency_id,
                    from,
                    to,
                    amount,
                }) => Ok((
                    vec![
                        from.encode(),
                        to.encode(),
                        currency_id.encode(),
                        amount.encode(),
                    ],
                    vec![],
                )),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            b"uint64" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u64, CurrencyId>::Transfer {
                    currency_id,
                    from,
                    to,
                    amount,
                }) => Ok((
                    vec![
                        from.encode(),
                        to.encode(),
                        currency_id.encode(),
                        amount.encode(),
                    ],
                    vec![],
                )),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            b"uint128" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u128, CurrencyId>::Transfer {
                    currency_id,
                    from,
                    to,
                    amount,
                }) => Ok((
                    vec![
                        from.encode(),
                        to.encode(),
                        currency_id.encode(),
                        amount.encode(),
                    ],
                    vec![],
                )),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            &_ => Err(Error::<T, I>::EventDecodingFailed.into()),
        },
        &_ => Err(Error::<T, I>::UnkownSideEffect.into()),
    }
}
