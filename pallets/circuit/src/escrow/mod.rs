use crate::*;
use codec::{Decode, Encode};
use sp_core::Hasher;
use sp_runtime::RuntimeDebug;
use sp_std::marker::PhantomData;

pub struct Escrow<T: Config> {
    _phantom: PhantomData<T>,
}

trait EscrowExec<T: Config> {
    fn get_id() -> [u8; 4];
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
    fn exec(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) {
        match encoded_type {
            Transfer::get_id() => Transfer::exec(encoded_args, escrow_account, executioner),
            // TransferMulti::get_id() => TransferMulti::exec(encoded_args, escrow_account, executioner),
            // Swap::get_id() => Swap::exec(encoded_args, escrow_account, executioner),
            // AddLiquidity::get_id() => AddLiquidity::exec(encoded_args, escrow_account, executioner),
            // Call::get_id() => Call::exec(encoded_args, escrow_account, executioner),
            // CallWasm::get_id() => CallWasm::exec(encoded_args, escrow_account, executioner),
            // CallEvm::get_id() => CallEvm::exec(encoded_args, escrow_account, executioner),
            // CallComposable::get_id() => CallComposable::exec(encoded_args, escrow_account, executioner),
        }
    }

    fn commit(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) {
        match encoded_type {
            Transfer::get_id() => Transfer::commit(encoded_args, escrow_account, executioner),
            // TransferMulti::get_id() => TransferMulti::commit(encoded_args, escrow_account, executioner),
            // Swap::get_id() => Swap::commit(encoded_args, escrow_account, executioner),
            // AddLiquidity::get_id() => AddLiquidity::commit(encoded_args, escrow_account, executioner),
            // Call::get_id() => Call::commit(encoded_args, escrow_account, executioner),
            // CallWasm::get_id() => CallWasm::commit(encoded_args, escrow_account, executioner),
            // CallEvm::get_id() => CallEvm::commit(encoded_args, escrow_account, executioner),
            // CallComposable::get_id() => CallComposable::commit(encoded_args, escrow_account, executioner),
        }
    }

    fn revert(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) {
        match encoded_type {
            Transfer::get_id() => Transfer::revert(encoded_args, escrow_account, executioner),
            // TransferMulti::get_id() => TransferMulti::revert(encoded_args, escrow_account, executioner),
            // Swap::get_id() => Swap::revert(encoded_args, escrow_account, executioner),
            // AddLiquidity::get_id() => AddLiquidity::revert(encoded_args, escrow_account, executioner),
            // Call::get_id() => Call::revert(encoded_args, escrow_account, executioner),
            // CallWasm::get_id() => CallWasm::revert(encoded_args, escrow_account, executioner),
            // CallEvm::get_id() => CallEvm::revert(encoded_args, escrow_account, executioner),
            // CallComposable::get_id() => CallComposable::revert(encoded_args, escrow_account, executioner),
        }
    }
}

struct Transfer {}

impl<T: Config> EscrowExec<T> for Transfer {
    fn get_id() -> [u8; 4] {
        b"tran"
    }
    fn exec(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let _dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(&executioner, escrow_account, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        // Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id))
        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            executioner,
            escrow_account,
            value,
        ))
    }
    fn revert(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(escrow_account, &executioner, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        // Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id))
        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            escrow_account,
            executioner,
            value,
        ))
    }
    fn commit(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;
        let dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;

        <T as EscrowTrait>::Currency::transfer(escrow_account, &dest, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        // Self::deposit_event(Event::XTransactionReceivedForExec(xtx_id))
        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(escrow_account, dest, value))
    }
}

//
//
//
// fn escrow_multi_transfer() {
//
// }
//
// fn escrow_swap() {
//
// }
//
// fn escrow_add_liquidity() {
//
// }
//
// /// Call any pallet on Circuit Runtime
// fn escrow_call() {
//
// }
//
// /// Call Pallet EVM
// fn escrow_call_evm() {
//
// }
//
// /// Call Pallet EVM
// fn escrow_call_composable() {
//
// }
