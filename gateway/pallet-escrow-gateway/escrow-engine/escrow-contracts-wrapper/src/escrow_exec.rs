// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

use crate::exec::*;

use crate::{
    gas::GasMeter, BalanceOf, CodeHash, Config, ContractInfo, ContractInfoOf, Error, Trait,
};

use codec::{Decode, Encode};

use frame_support::{
    dispatch::DispatchError,
    storage::child,
    traits::{Currency, Randomness, Time},
    weights::Weight,
    StorageMap,
};
use gateway_escrow_engine::{
    transfers::{escrow_transfer, just_transfer, BalanceOf as EscrowBalanceOf, TransferEntry},
    EscrowTrait,
};
use sp_runtime::traits::Zero;
use sp_std::{convert::TryInto, prelude::*};

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
#[codec(compact)]
pub struct DeferredStorageWrite {
    pub dest: Vec<u8>,
    pub trie_id: Vec<u8>,
    pub key: [u8; 32],
    pub value: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Default, Clone)]
#[codec(compact)]
pub struct CallStamp {
    pub pre_storage: Vec<u8>,
    pub post_storage: Vec<u8>,
    pub dest: Vec<u8>,
}

pub struct EscrowCallContext<'a, 'b: 'a, T: Trait + 'b, V: Vm<T> + 'b, L: Loader<T>> {
    pub config: &'a Config<T>,
    pub transfers: &'a mut Vec<TransferEntry>,
    pub deferred_storage_writes: &'a mut Vec<DeferredStorageWrite>,
    pub call_stamps: &'a mut Vec<CallStamp>,
    pub caller: T::AccountId,
    pub requester: T::AccountId,
    pub value_transferred: BalanceOf<T>,
    pub timestamp: MomentOf<T>,
    pub block_number: T::BlockNumber,
    pub call_context: CallContext<'a, 'b, T, V, L>,
}

impl<'a, 'b: 'a, T, E, V, L> Ext for EscrowCallContext<'a, 'b, T, V, L>
where
    T: Trait + EscrowTrait + 'b,
    V: Vm<T, Executable = E>,
    L: Loader<T, Executable = E>,
{
    type T = T;

    fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>> {
        self.call_context.get_storage(key)
    }

    fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) {
        let trie_id = self.call_context.ctx.self_trie_id.as_ref().expect(
            "`ctx.self_trie_id` points to an alive contract within the `CallContext`;\
				it cannot be `None`;\
				expect can't fail;\
				qed",
        );

        self.deferred_storage_writes.push(DeferredStorageWrite {
            dest: T::AccountId::encode(&self.call_context.ctx.self_account),
            trie_id: trie_id.to_vec(),
            key,
            value: value.clone(),
        });

        self.call_context.set_storage(key, value)
    }

    fn instantiate(
        &mut self,
        code_hash: &CodeHash<T>,
        endowment: BalanceOf<T>,
        gas_meter: &mut GasMeter<T>,
        input_data: Vec<u8>,
    ) -> Result<(AccountIdOf<T>, ExecReturnValue), ExecError> {
        self.call_context
            .instantiate(code_hash, endowment, gas_meter, input_data)
    }

    fn transfer(
        &mut self,
        to: &T::AccountId,
        value: BalanceOf<T>,
        _gas_meter: &mut GasMeter<T>,
    ) -> Result<(), DispatchError> {
        escrow_transfer::<T>(
            &self.caller.clone(),
            &self.requester,
            to,
            EscrowBalanceOf::<T>::from(TryInto::<u32>::try_into(value).ok().unwrap()),
            self.transfers,
        )
    }

    fn terminate(
        &mut self,
        beneficiary: &AccountIdOf<Self::T>,
        gas_meter: &mut GasMeter<Self::T>,
    ) -> Result<(), DispatchError> {
        self.call_context.terminate(beneficiary, gas_meter)
    }

    fn call(
        &mut self,
        to: &T::AccountId,
        value: BalanceOf<T>,
        gas_meter: &mut GasMeter<T>,
        input_data: Vec<u8>,
    ) -> ExecResult {
        let executable = if let Some(ContractInfo::Alive(info)) = <ContractInfoOf<T>>::get(to) {
            self.call_context
                .ctx
                .loader
                .load_main(&info.code_hash)
                .map_err(|_| Error::<T>::CodeNotFound)?
        } else {
            Err(Error::<T>::NotCallable)?
        };

        self.call_context.ctx.escrow_call(
            &self.caller.clone(),
            &self.requester.clone(),
            &to,
            &to,
            value,
            gas_meter,
            input_data,
            self.transfers,
            self.deferred_storage_writes,
            self.call_stamps,
            &executable,
        )
    }

    fn restore_to(
        &mut self,
        dest: AccountIdOf<Self::T>,
        code_hash: CodeHash<Self::T>,
        rent_allowance: BalanceOf<Self::T>,
        delta: Vec<StorageKey>,
    ) -> Result<(), &'static str> {
        self.call_context
            .restore_to(dest, code_hash, rent_allowance, delta)
    }

    fn caller(&self) -> &T::AccountId {
        &self.caller
    }

    fn address(&self) -> &T::AccountId {
        &self.call_context.ctx.self_account
    }

    fn balance(&self) -> BalanceOf<T> {
        <T as Trait>::Currency::free_balance(&self.call_context.ctx.self_account)
    }

    fn value_transferred(&self) -> BalanceOf<T> {
        self.value_transferred
    }

    fn now(&self) -> &MomentOf<T> {
        &self.timestamp
    }

    fn minimum_balance(&self) -> BalanceOf<T> {
        self.config.existential_deposit
    }

    fn tombstone_deposit(&self) -> BalanceOf<T> {
        self.config.tombstone_deposit
    }

    fn random(&self, subject: &[u8]) -> SeedOf<T> {
        <T as Trait>::Randomness::random(subject)
    }

    fn deposit_event(&mut self, topics: Vec<T::Hash>, data: Vec<u8>) {
        self.call_context.deposit_event(topics, data);
    }

    fn set_rent_allowance(&mut self, rent_allowance: BalanceOf<T>) {
        self.call_context.set_rent_allowance(rent_allowance);
    }

    fn rent_allowance(&self) -> BalanceOf<T> {
        self.call_context.rent_allowance()
    }

    fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    fn max_value_size(&self) -> u32 {
        self.config.max_value_size
    }

    fn get_weight_price(&self, weight: Weight) -> BalanceOf<Self::T> {
        self.call_context.get_weight_price(weight)
    }
}

impl<'a, T, E, V, L> ExecutionContext<'a, T, V, L>
where
    T: Trait + EscrowTrait,
    L: Loader<T, Executable = E>,
    V: Vm<T, Executable = E>,
{
    /// Make a call to the specified address, optionally transferring some funds.
    pub fn escrow_call(
        &mut self,
        escrow_account: &T::AccountId,
        requester: &T::AccountId,
        dest: &T::AccountId,
        transfer_dest: &T::AccountId,
        value: BalanceOf<T>,
        gas_meter: &mut GasMeter<T>,
        input_data: Vec<u8>,
        transfers: &mut Vec<TransferEntry>,
        deferred_storage_writes: &mut Vec<DeferredStorageWrite>,
        call_stamps: &mut Vec<CallStamp>,
        executable: &E,
    ) -> ExecResult {
        if self.depth == self.config.max_depth as usize {
            Err(Error::<T>::MaxCallDepthReached)?
        }

        if gas_meter
            .charge(self.config, ExecFeeToken::Call)
            .is_out_of_gas()
        {
            Err(Error::<T>::OutOfGas)?
        }

        // Assumption: `collect_rent` doesn't collide with overlay because
        // `collect_rent` will be done on first call and destination contract and balance
        // cannot be changed before the first call
        // We do not allow 'calling' plain accounts. For transfering value
        // `seal_transfer` must be used.
        let contract = if let Some(ContractInfo::Alive(info)) = <ContractInfoOf<T>>::get(dest) {
            info
        } else {
            Err(Error::<T>::NotCallable)?
        };

        let pre_storage = child::root(&contract.child_trie_info());
        let mut post_storage = vec![];

        // Set both possible output variables in outer scope.
        let successful_execution_err = DispatchError::Other(
            "Rollback after successful execution as it's an escrow execution.",
        );
        let mut output_data = vec![];

        let escrow_exec_result =
            self.with_nested_context(dest.clone(), contract.trie_id.clone(), |nested| {
                if value > BalanceOf::<T>::zero() {
                    escrow_transfer::<T>(
                        &escrow_account.clone(),
                        &requester.clone(),
                        &transfer_dest.clone(),
                        EscrowBalanceOf::<T>::from(TryInto::<u32>::try_into(value).ok().unwrap()),
                        transfers,
                    )
                    .map_err(|e| e)?
                }

                let ext = EscrowCallContext {
                    config: &nested.config.clone(),
                    block_number: <frame_system::Module<T>>::block_number(),
                    caller: escrow_account.clone(),
                    requester: requester.clone(),
                    timestamp: <T as Trait>::Time::now(),
                    value_transferred: value.clone(),
                    transfers,
                    deferred_storage_writes,
                    call_stamps,
                    call_context: nested.new_call_context(escrow_account.clone(), value),
                };

                let output = ext
                    .call_context
                    .ctx
                    .vm
                    .execute(executable, ext, input_data, gas_meter)
                    .map_err(|e| ExecError {
                        error: e.error,
                        origin: ErrorOrigin::Callee,
                    })?;

                post_storage = child::root(
                    &<ContractInfoOf<T>>::get(dest.clone())
                        .unwrap()
                        .get_alive()
                        .unwrap()
                        .child_trie_info(),
                );
                output_data = output.data.clone();

                // Assume that top level + 1 gets called as the very last one in recursion chain of calls from with the contract (ext_call).
                if nested.depth == 1 {
                    // Signalize error despite successful execution to revert the changes made to potentially external contract.
                    // The changes at this point can be twofold: storage writes and transfers from requester to escrow account.
                    Err(ExecError {
                        error: successful_execution_err,
                        origin: ErrorOrigin::Caller,
                    })
                } else {
                    Ok(output)
                }
            });

        call_stamps.push(CallStamp {
            pre_storage,
            post_storage,
            dest: T::AccountId::encode(&dest.clone()),
        });
        match escrow_exec_result {
            Ok(output) => Ok(output),
            Err(err) => {
                if err.error == successful_execution_err {
                    // Write should be reverted, but the transfer should stay.
                    // Transfer funds from requester to escrow account again.
                    for transfer in transfers.iter() {
                        just_transfer::<T>(
                            &requester.clone(),
                            &escrow_account.clone(),
                            EscrowBalanceOf::<T>::from(transfer.value),
                        )
                        .map_err(|e| e)?
                    }
                    Ok(ExecReturnValue {
                        flags: ReturnFlags::REVERT,
                        data: output_data,
                    })
                } else {
                    Err(err)
                }
            }
        }
    }
}
