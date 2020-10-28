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

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchError,
    storage::child,
    storage::child::ChildInfo,
    traits::{Currency, Time},
};
use gateway_escrow_engine::{
    transfers::{escrow_transfer, BalanceOf as EscrowBalanceOf, TransferEntry},
    EscrowTrait, ExtendedWasm,
};
use sp_runtime::traits::{Hash, Zero};
use sp_sandbox;
use sp_std::{convert::TryInto, prelude::*};
use system::Trait as SystemTrait;

use crate::env_def::FunctionImplProvider;
use crate::ext::{DefaultRuntimeEnv, ExtStandards};
use crate::gas::{Gas, GasMeter};
use crate::*;

pub struct Runtime<'a, E: ExtStandards + 'a> {
    pub ext: &'a mut E,
    pub input_data: Option<Vec<u8>>,
    pub value: EscrowBalanceOf<E::T>,
    pub gas_used: Gas,
    pub gas_limit: Gas,
    pub requester_available_balance: u64,
    pub requester_encoded: Vec<u8>,
    pub escrow_account_encoded: Vec<u8>,
    pub escrow_account_trie_id: ChildInfo,
    pub memory: sp_sandbox::Memory,
    pub max_value_size: u32,
    pub max_event_topics: u32,
    pub trap_reason: Option<TrapReason>,
    // pub transfers: &'a mut Vec<TransferEntry>,
}

impl<'a, E: ExtStandards + 'a> Runtime<'a, E> {
    pub fn new(
        ext: &'a mut E,
        gas_limit: Gas,
        memory: sp_sandbox::Memory,
        requester: &AccountIdOf<E::T>,
        escrow_account: &AccountIdOf<E::T>,
        escrow_account_trie_id: ChildInfo,
        input_data: Option<Vec<u8>>,
        value: EscrowBalanceOf<E::T>,
    ) -> Self {
        Runtime {
            ext,
            input_data,
            value,
            gas_used: 0,
            gas_limit,
            requester_available_balance: TryInto::<u64>::try_into(
                <E::T as EscrowTrait>::Currency::free_balance(&requester),
            )
            .ok()
            .unwrap(),
            requester_encoded: requester.encode(),
            escrow_account_encoded: escrow_account.encode(),
            escrow_account_trie_id: escrow_account_trie_id.clone(),
            memory,
            max_value_size: u32::MAX,
            max_event_topics: 16,
            trap_reason: None,
        }
    }
}

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

pub fn get_child_storage_for_current_execution<T: EscrowTrait>(
    escrow_account: &T::AccountId,
    code: T::Hash,
) -> ChildInfo {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"gateway_escrow");
    buf.extend_from_slice(&escrow_account.encode()[..]);
    buf.extend_from_slice(&code.encode()[..]);
    child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
}

define_env!(Env, <E: ExtStandards>,
    gas (_ctx, amount: u32) => {
        let amount = Gas::from(amount);
        if !amount.is_zero() {
            Ok(())
        } else {
            Err(sp_sandbox::HostError)
        }
    },
    seal_deposit_event (ctx, topics_ptr: u32, topics_len: u32, data_ptr: u32, data_len: u32) => {
        let mut topics: Vec::<TopicOf<<E as ExtStandards>::T>> = match topics_len {
            0 => Vec::new(),
            _ => read_sandbox_memory_as(ctx, topics_ptr, topics_len)?,
        };

        // If there are more than `max_event_topics`, then trap.
        if topics.len() > ctx.max_event_topics as usize {
            return Err(sp_sandbox::HostError);
        }

        // Check for duplicate topics. If there are any, then trap.
        if has_duplicates(&mut topics) {
            return Err(sp_sandbox::HostError);
        }

        let event_data = read_sandbox_memory(ctx, data_ptr, data_len)?;

        // charge_gas(
        //     ctx.gas_meter,
        //     ctx.schedule,
        //     &mut ctx.trap_reason,
        //     RuntimeToken::DepositEvent(topics.len() as u32, data_len)
        // )?;

        ctx.ext.deposit_event(topics, event_data);

        Ok(())
    },
    seal_input (ctx, buf_ptr: u32, buf_len_ptr: u32) => {
        if let Some(input) = ctx.input_data.take() {
            write_sandbox_output(ctx, buf_ptr, buf_len_ptr, &input, false)
        } else {
            Err(sp_sandbox::HostError)
        }
    },
    seal_return (ctx, flags: u32, data_ptr: u32, data_len: u32) => {
        ctx.trap_reason = Some(TrapReason::Return(ReturnData {
            flags,
            data: read_sandbox_memory(ctx, data_ptr, data_len)?,
        }));

        // The trap mechanism is used to immediately terminate the execution.
        // This trap should be handled appropriately before returning the result
        // to the user of this crate.
        Err(sp_sandbox::HostError)
    },
    seal_call (
        ctx,
        callee_ptr: u32,
        callee_len: u32,
        gas: u64,
        value_ptr: u32,
        value_len: u32,
        input_data_ptr: u32,
        input_data_len: u32,
        output_ptr: u32,
        output_len_ptr: u32
    ) -> ReturnCode => {
        // [32, 64> bytes of input reserved for a module name.
        let module_name = String::from_utf8(
            read_sandbox_memory(ctx, 32, 32)?
            .into_iter()
            .filter(|i| *i > 0 as u8)
            .collect()
        ).unwrap();
        // [64, 96> bytes of input reserved for a module name. 64 bytes reserved in total.
        let fn_name = String::from_utf8(
            read_sandbox_memory(ctx, 64, 32)?
            .into_iter()
            .filter(|i| *i > 0 as u8)
            .collect()
        ).unwrap();

        // Everything >64 reserved bytes constitutes actual input.
        let input = read_sandbox_memory(ctx, input_data_ptr + 64, input_data_len)?;
        let callee: <E::T as SystemTrait>::AccountId = read_sandbox_memory_as(ctx, callee_ptr, callee_len)?;
        let value: EscrowBalanceOf::<E::T> = read_sandbox_memory_as(ctx, value_ptr, value_len)?;
        let mut adhoc_gas_meter = GasMeter::<E::T>::new(gas);
        match ctx.ext.call(
            &module_name,
            &fn_name,
            &callee,
            value,
            &mut adhoc_gas_meter,
            input,
        ) {
            Ok(_) => {
                write_sandbox_output(ctx, output_ptr, output_len_ptr, &[], true)?;
                Ok(ReturnCode::Success)
            },
            Err(_err) => {
                // todo: Store error.
                Err(sp_sandbox::HostError)
            }
        }
    },
    seal_transfer (ctx, account_ptr: u32, account_len: u32, value_ptr: u32, value_len: u32) -> ReturnCode => {
        let callee: <E::T as SystemTrait>::AccountId = read_sandbox_memory_as(ctx, account_ptr, account_len)?;
        let value: EscrowBalanceOf::<E::T> = read_sandbox_memory_as(ctx, value_ptr, value_len)?;
        let mut adhoc_gas_meter = GasMeter::<E::T>::new(ctx.gas_limit);

        match ctx.ext.transfer(
            &callee,
            value,
            &mut adhoc_gas_meter,
        ) {
            Ok(_) => {
                Ok(ReturnCode::Success)
            },
            Err(_err) => {
                // todo: Store error.
                Err(sp_sandbox::HostError)
            }
        }
    },
    seal_get_storage (ctx, key_ptr: u32, out_ptr: u32, out_len_ptr: u32) -> ReturnCode => {
        let mut key: StorageKey = [0; 32];
        read_sandbox_memory_into_buf(ctx, key_ptr, &mut key)?;
        if let Some(value) = ctx.ext.get_storage(&key) {
            write_sandbox_output(ctx, out_ptr, out_len_ptr, &value, false)?;
            Ok(ReturnCode::Success)
        } else {
            Ok(ReturnCode::KeyNotFound)
        }
    },
    seal_set_storage (ctx, key_ptr: u32, value_ptr: u32, value_len: u32) => {
        if value_len > ctx.max_value_size {
            // Bail out if value length exceeds the set maximum value size.
            return Err(sp_sandbox::HostError);
        }
        let mut key: StorageKey = [0; 32];
        read_sandbox_memory_into_buf(ctx, key_ptr, &mut key)?;
        let value = Some(read_sandbox_memory(ctx, value_ptr, value_len)?);

        ctx.ext.set_storage(key, value);

        Ok(())
    },
    seal_minimum_balance (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(ctx, out_ptr, out_len_ptr, &ctx.ext.minimum_balance().encode(), false)
    },
    seal_tombstone_deposit (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(ctx, out_ptr, out_len_ptr, &ctx.ext.tombstone_deposit().encode(), false)
    },
    seal_max_value_size (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(ctx, out_ptr, out_len_ptr, &ctx.ext.max_value_size().encode(), false)
    },
    seal_escrow_address (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(ctx, out_ptr, out_len_ptr, &ctx.escrow_account_encoded.clone(), false)
    },
    seal_requester (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(ctx, out_ptr, out_len_ptr, &ctx.requester_encoded.clone(), false)
    },
    seal_value_transferred (ctx, out_ptr: u32, out_len_ptr: u32) => {
        write_sandbox_output(
            ctx, out_ptr, out_len_ptr, &ctx.value.encode(), false
        )
    },
);

pub fn to_execution_result<E: ExtStandards>(
    exec_state: Runtime<E>,
    sandbox_result: Result<sp_sandbox::ReturnValue, sp_sandbox::Error>,
) -> ExecResult {
    // If a trap reason is set we base our decision solely on that.
    if let Some(trap_reason) = exec_state.trap_reason {
        return match trap_reason {
            // The trap was the result of the execution `return` host function.
            TrapReason::Return(ReturnData { flags, data }) => {
                let flags = ReturnFlags::from_bits(flags)
                    .ok_or_else(|| "used reserved bit in return flags")?;
                Ok(ExecReturnValue { flags, data })
            }
            TrapReason::Termination => Ok(ExecReturnValue {
                flags: ReturnFlags::empty(),
                data: Vec::new(),
            }),
            TrapReason::Restoration => Ok(ExecReturnValue {
                flags: ReturnFlags::empty(),
                data: Vec::new(),
            }),
            TrapReason::SupervisorError(error) => Err(error)?,
        };
    }

    // Check the exact type of the error.
    match sandbox_result {
        // No traps were generated. Proceed normally.
        Ok(_) => Ok(ExecReturnValue {
            flags: ReturnFlags::empty(),
            data: Vec::new(),
        }),
        // `Error::Module` is returned only if instantiation or linking failed (i.e.
        // wasm binary tried to import a function that is not provided by the host).
        // This shouldn't happen because validation process ought to reject such binaries.
        //
        // Because panics are really undesirable in the runtime code, we treat this as
        // a trap for now. Eventually, we might want to revisit this.
        Err(sp_sandbox::Error::Module) => Err("validation error")?,
        // Any other kind of a trap should result in a failure.
        Err(sp_sandbox::Error::Execution) | Err(sp_sandbox::Error::OutOfBounds) => {
            Err(ExecError {
                /// The reason why the execution failed.
                error: DispatchError::Other("Contract Trapped"),
                // Origin of the error.
                origin: ErrorOrigin::Callee,
            })?
        }
    }
}

pub fn raw_escrow_call<T: EscrowTrait + ExtendedWasm + SystemTrait, E: ExtStandards<T = T>>(
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    transfer_dest: &T::AccountId,
    value: EscrowBalanceOf<T>,
    gas_limit: Gas,
    input_data: Vec<u8>,
    mut transfers: &mut Vec<TransferEntry>,
    _deferred_storage_writes: &mut Vec<DeferredStorageWrite>,
    call_stamps: &mut Vec<CallStamp>,
    exec: &WasmExecutable,
    code_hash: T::Hash,
) -> ExecResult {
    if value > EscrowBalanceOf::<T>::zero() {
        escrow_transfer::<T>(
            &escrow_account.clone(),
            &requester.clone(),
            &transfer_dest.clone(),
            EscrowBalanceOf::<T>::from(TryInto::<u32>::try_into(value).ok().unwrap()),
            transfers,
        )
        .map_err(|e| e)?
    }
    let escrow_account_trie_id =
        get_child_storage_for_current_execution::<T>(escrow_account, code_hash);

    let pre_storage = child::root(&escrow_account_trie_id.clone());

    let memory =
        sp_sandbox::Memory::new(exec.prefab_module.initial, Some(exec.prefab_module.maximum))
            .unwrap_or_else(|_| {
                // unlike `.expect`, explicit panic preserves the source location.
                // Needed as we can't use `RUST_BACKTRACE` in here.
                panic!(
                    "exec.prefab_module.initial can't be greater than exec.prefab_module.maximum;
						thus Memory::new must not fail;
						qed"
                )
            });

    let mut env_builder = sp_sandbox::EnvironmentDefinitionBuilder::new();
    env_builder.add_memory(
        crate::prepare::IMPORT_MODULE_MEMORY,
        "memory",
        memory.clone(),
    );

    Env::impls(&mut |name, func_ptr| {
        env_builder.add_host_func(self::prepare::IMPORT_MODULE_FN, name, func_ptr);
    });

    let mut ext = DefaultRuntimeEnv::<T> {
        input_data: Some(input_data.clone()),
        inner_exec_transfers: &mut transfers,
        requester,
        block_number: <system::Module<T>>::block_number(),
        escrow_account,
        escrow_account_trie_id: escrow_account_trie_id.clone(),
        timestamp: T::Time::now(),
    };

    let mut state = Runtime::new(
        &mut ext,
        gas_limit,
        memory,
        requester,
        escrow_account,
        escrow_account_trie_id.clone(),
        Some(input_data.clone()),
        value,
    );

    let sandbox_result =
        sp_sandbox::Instance::new(&exec.prefab_module.code, &env_builder, &mut state)
            .and_then(|mut instance| instance.invoke(exec.entrypoint_name, &[], &mut state));

    let result = to_execution_result(state, sandbox_result);

    match result {
        Ok(result) => {
            call_stamps.push(CallStamp {
                pre_storage,
                post_storage: child::root(&escrow_account_trie_id.clone()),
                dest: T::AccountId::encode(&escrow_account.clone()),
            });
            Ok(result)
        }
        Err(_err) => Err(ExecError {
            error: DispatchError::Other(
                "Failed instantiating code with sandbox instance for provided WASM code.",
            ),
            origin: ErrorOrigin::Caller,
        })?,
    }
}

fn read_sandbox_memory<E: ExtStandards>(
    ctx: &mut Runtime<E>,
    ptr: u32,
    len: u32,
) -> Result<Vec<u8>, sp_sandbox::HostError> {
    let mut buf = vec![0u8; len as usize];
    ctx.memory
        .get(ptr, buf.as_mut_slice())
        .map_err(|_| sp_sandbox::HostError)?;
    Ok(buf)
}

pub fn read_sandbox_memory_as<D: Decode, E: ExtStandards>(
    ctx: &mut Runtime<E>,
    ptr: u32,
    len: u32,
) -> Result<D, sp_sandbox::HostError> {
    let buf = read_sandbox_memory(ctx, ptr, len)?;
    D::decode(&mut &buf[..]).map_err(|_| sp_sandbox::HostError)
}

fn read_sandbox_memory_into_buf<E: ExtStandards>(
    ctx: &mut Runtime<E>,
    ptr: u32,
    buf: &mut [u8],
) -> Result<(), sp_sandbox::HostError> {
    ctx.memory.get(ptr, buf).map_err(|_| sp_sandbox::HostError)
}

fn write_sandbox_output<E: ExtStandards>(
    ctx: &mut Runtime<E>,
    out_ptr: u32,
    out_len_ptr: u32,
    buf: &[u8],
    allow_skip: bool,
) -> Result<(), sp_sandbox::HostError> {
    if allow_skip && out_ptr == u32::max_value() {
        return Ok(());
    }

    let buf_len = buf.len() as u32;
    let len: u32 = read_sandbox_memory_as(ctx, out_len_ptr, 4)?;

    if len < buf_len {
        Err(sp_sandbox::HostError)?
    }

    ctx.memory.set(out_ptr, buf)?;
    ctx.memory.set(out_len_ptr, &buf_len.encode())?;

    Ok(())
}

fn has_duplicates<T: PartialEq + AsRef<[u8]>>(items: &mut Vec<T>) -> bool {
    // Sort the vector
    items.sort_by(|a, b| Ord::cmp(a.as_ref(), b.as_ref()));
    // And then find any two consecutive equal elements.
    items.windows(2).any(|w| match w {
        &[ref a, ref b] => a == b,
        _ => false,
    })
}
