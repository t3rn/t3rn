use crate::*;
use codec::Decode;

use sp_std::marker::PhantomData;

pub struct Escrow<T: Config> {
    _phantom: PhantomData<T>,
}

trait EscrowExec<T: Config> {
    fn exec(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
    fn revert(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
    fn commit(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
}

impl<T: Config> Escrow<T> {
    pub fn exec(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::exec(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::exec(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::exec(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::exec(encoded_args, escrow_account, executioner),
            // b"call" => Call::exec(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::exec(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::exec(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::exec(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }

    pub fn commit(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::commit(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::commit(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::commit(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::commit(encoded_args, escrow_account, executioner),
            // b"call" => Call::commit(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::commit(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::commit(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::commit(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }

    pub fn revert(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::revert(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::revert(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::revert(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::revert(encoded_args, escrow_account, executioner),
            // b"call" => Call::revert(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::revert(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::revert(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::revert(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }
}

pub struct Transfer<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> EscrowExec<T> for Transfer<T> {
    fn exec(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let _dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(&executioner, &escrow_account, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            executioner,
            escrow_account,
            value,
        ));

        Ok(())
    }
    fn revert(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(&escrow_account, &executioner, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            escrow_account,
            executioner,
            value,
        ));

        Ok(())
    }
    fn commit(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        _executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;
        let dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(&escrow_account, &dest, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail
        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(escrow_account, dest, value));

        Ok(())
    }
}
