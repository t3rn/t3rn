use crate::{Config, Determinism, Schedule};
use codec::{Decode, Encode, MaxEncodedLen};

use frame_support::{
    dispatch::RawOrigin,
    ensure,
    pallet_prelude::{Get, Weight},
    traits::Currency,
};
use frame_system::pallet_prelude::OriginFor;
use pallet_contracts_primitives::{
    ContractExecResult, ExecReturnValue, ReturnFlags, StorageDeposit,
};
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{
    traits::{Saturating, UniqueSaturatedInto, Zero},
    DispatchError, RuntimeDebug,
};

use sp_std::{marker::PhantomData, vec::Vec};
use t3rn_primitives::{
    threevm::{
        GetState, ModuleOperations, Precompile, PrecompileArgs, PrecompileInvocation, ThreeVm,
    },
    SpeedMode,
};
use t3rn_sdk_primitives::{
    signal::ExecutionSignal, state::SideEffects, GET_STATE_FUNCTION_CODE,
    POST_SIGNAL_FUNCTION_CODE, SUBMIT_FUNCTION_CODE,
};

const CONTRACTS_LOG_TARGET: &str = "runtime::contracts::chain_extension";
const GET_STATE_LOG_TARGET: &str = "runtime::contracts::get_state";
const SIGNAL_LOG_TARGET: &str = "runtime::contracts::signal";

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ComposableExecReturnValue {
    /// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
    pub flags: ReturnFlags,
    /// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
    pub data: Vec<u8>,
    /// Side effects returned from the call
    pub side_effects: Vec<Vec<u8>>,
}

impl ComposableExecReturnValue {
    /// The contract did revert all storage changes.
    pub fn did_revert(&self) -> bool {
        self.flags.contains(ReturnFlags::REVERT)
    }
}

pub trait Contracts<AccountId, Balance> {
    type Outcome;
    fn call(
        origin: AccountId,
        dest: AccountId,
        value: Balance,
        gas_limit: Weight,
        storage_deposit_limit: Option<Balance>,
        data: Vec<u8>,
        debug: bool,
    ) -> Self::Outcome;
}

impl<AccountId, Balance: Default> Contracts<AccountId, Balance> for () {
    type Outcome = ContractExecResult<Balance>;

    fn call(
        _origin: AccountId,
        _dest: AccountId,
        _value: Balance,
        _gas_limit: Weight,
        _storage_deposit_limit: Option<Balance>,
        _data: Vec<u8>,
        _debug: bool,
    ) -> Self::Outcome {
        ContractExecResult {
            gas_consumed: Weight::zero(),
            gas_required: Weight::zero(),
            debug_message: Vec::new(),
            storage_deposit: StorageDeposit::Refund(Default::default()),
            result: Ok(ExecReturnValue {
                flags: ReturnFlags::empty(),
                data: Vec::default(),
            }),
        }
    }
}

// Chain extensions
use crate::BalanceOf;

pub struct ThreeVmExtension;
use crate::{
    chain_extension::{
        BufInBufOutState, ChainExtension, Environment, Ext, InitState, RegisteredChainExtension,
        RetVal, SysConfig,
    },
    exec::Executable,
    wasm::{PrefabWasmModule, TryInstantiate},
};

impl<C: Config> ChainExtension<C> for ThreeVmExtension {
    fn call<E>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        E: Ext<T = C>,
    {
        let func_id = env.func_id() as u32;
        log::trace!(
            target: CONTRACTS_LOG_TARGET,
            "[ChainExtension]|call|func_id:{:}",
            func_id
        );
        match func_id {
            GET_STATE_FUNCTION_CODE => {
                let mut env = env.buf_in_buf_out();

                // For some reason the parameter is passed through as a default, not an option
                let execution_id: C::Hash = env.read_as()?;
                log::debug!(
                    target: GET_STATE_LOG_TARGET,
                    "reading state for execution_id: {:?}",
                    execution_id
                );
                let default: C::Hash = Default::default();
                let execution_id = if execution_id == default {
                    None
                } else {
                    // TODO: allow a modifiable multiplier constant in the config
                    env.charge_weight(size_to_weight(&execution_id))?;
                    Some(execution_id)
                };

                let origin = origin_from_environment(&mut env);

                let invocation = <C as Config>::ThreeVm::invoke(PrecompileArgs::GetState(
                    origin,
                    GetState {
                        xtx_id: execution_id,
                    },
                ))?;
                let state = invocation.get_state().ok_or("NoStateReturned")?;

                let xtx_id = state.xtx_id;
                let bytes = state.encode();
                log::debug!(
                    target: GET_STATE_LOG_TARGET,
                    "loaded local state id: {:?}, state: {:?}",
                    xtx_id,
                    bytes,
                );

                env.write(&bytes[..], false, None)?;

                Ok(RetVal::Converging(0))
            },
            SUBMIT_FUNCTION_CODE => {
                let mut env = env.buf_in_buf_out();

                let arg: (SideEffects<C::AccountId, BalanceOf<C>, C::Hash>, SpeedMode) =
                    read_from_environment(&mut env)?;

                let origin = RawOrigin::Signed(env.ext().caller().clone());
                <C as Config>::ThreeVm::invoke(PrecompileArgs::SubmitSideEffects(
                    C::RuntimeOrigin::from(origin),
                    arg.0,
                    arg.1,
                ))?;
                Ok(RetVal::Converging(0))
            },
            POST_SIGNAL_FUNCTION_CODE => {
                let mut env = env.buf_in_buf_out();

                let signal: ExecutionSignal<C::Hash> = read_from_environment(&mut env)?;
                log::debug!(target: SIGNAL_LOG_TARGET, "submitting signal {:?}", signal);

                let origin = RawOrigin::Signed(env.ext().caller().clone());
                C::ThreeVm::invoke(PrecompileArgs::Signal(
                    C::RuntimeOrigin::from(origin),
                    signal,
                ))?;
                Ok(RetVal::Converging(0))
            },
            n => {
                log::error!(
                    target: CONTRACTS_LOG_TARGET,
                    "Called an unregistered `func_id`: {:}",
                    func_id
                );
                Ok(RetVal::Converging(n))
            },
        }
    }
}

// impl<C: Config> ChainExtension<C> for () {
//     fn call<E: Ext<T = C>>(
//         &mut self,
//         env: Environment<E, InitState>,
//     ) -> crate::chain_extension::Result<RetVal> {
//         todo!()
//     }
// }

impl<C: Config> RegisteredChainExtension<C> for () {
    const ID: u16 = 3330;
}

fn read_from_environment<C, T, E>(
    env: &mut Environment<E, BufInBufOutState>,
) -> Result<T, DispatchError>
where
    C: Config,
    T: Decode + MaxEncodedLen,
    E: Ext<T = C>,
{
    let bytes = env.read(<T as MaxEncodedLen>::max_encoded_len() as u32)?;

    Decode::decode(&mut &bytes[..])
        .map_err(|e| {
            log::error!(target: CONTRACTS_LOG_TARGET, "decoding type failed {:?}", e);
            "read_from_environment::DecodingFailed".into()
        })
        .and_then(|t: T| env.charge_weight(size_to_weight(&t)).map(|_| t))
}

fn size_to_weight<T: Encode>(encodable: &T) -> Weight {
    Weight::from_ref_time(encodable.encoded_size() as u64)
}

fn origin_from_environment<C, E>(
    env: &mut Environment<E, BufInBufOutState>,
) -> <C as frame_system::Config>::RuntimeOrigin
where
    C: Config,
    E: Ext<T = C>,
{
    OriginFor::<C>::from(RawOrigin::Signed(env.ext().caller().clone()))
}

// Used in src/lib.rs
pub fn try_instantiate_from_contracts_registry<T: Config>(
    origin: &T::AccountId,
    hash: &T::Hash,
    schedule: &Schedule<T>,
) -> Result<(BalanceOf<T>, PrefabWasmModule<T>), DispatchError> {
    // Use ThreeVm to try to retrieve a module from the registry.
    // If found, attempt to construct a PrefabWasmModule from it.
    let module = T::ThreeVm::from_registry::<PrefabWasmModule<T>, _>(&hash, |bytes| {
        PrefabWasmModule::from_code(
            bytes,
            &schedule,
            origin.clone(),
            Determinism::Deterministic,
            TryInstantiate::Instantiate,
        )
        .unwrap_or(PrefabWasmModule::<T>::new_empty())
    })?;

    if module.is_empty() {
        return Err("Could not instantiate from contracts registry".into())
    }

    T::ThreeVm::instantiate_check(module.get_type())?;

    // Retrieve the fee for using the module, or use a default if not specified
    let fee = module
        .get_author()
        .as_ref()
        .and_then(|author| author.fees_per_single_use)
        .unwrap_or_default();

    // Return the fee and the module itself
    Ok((fee, module))
}

pub fn try_submit_side_effects<T: Config>(
    caller: &T::AccountId,
    input_data: &[u8],
) -> Result<PrecompileInvocation<T, BalanceOf<T>>, DispatchError> {
    // Try to decode the input data into the expected arguments
    let decoded_args = Decode::decode(&mut input_data.as_ref());

    // Use match to deal with the Result, which is more idiomatic in Rust than if let
    match decoded_args {
        Ok((side_effects, speed_mode)) => {
            // If decoding succeeded, invoke the ThreeVm function with the decoded arguments
            T::ThreeVm::invoke(PrecompileArgs::SubmitSideEffects(
                RawOrigin::Signed(caller.clone()).into(),
                side_effects,
                speed_mode,
            ))
        },
        Err(_) => {
            // If decoding failed, return an error
            Err("Failed to decode side effects".into())
        },
    }
}
