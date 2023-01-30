use crate::{BalanceOf, Config, Error, Pallet, PrecompileIndex};
use codec::{Decode, Encode};
use frame_support::{dispatch::RawOrigin, sp_runtime::DispatchError};
use sp_std::vec::Vec;
use t3rn_primitives::{
    circuit::{LocalTrigger, OnLocalTrigger},
    threevm::{GetState, LocalStateAccess, PrecompileArgs, PrecompileInvocation},
};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::SideEffects,
};

const LOG_TARGET: &str = "3vm::precompile";

// Precompile pointers baked into the binary.
// Genesis exists only to map hashes to pointers.
pub const GET_STATE: u8 = 55;
pub const SUBMIT: u8 = 56;
pub const POST_SIGNAL: u8 = 57;

type CodecResult<T> = Result<T, codec::Error>;

pub(crate) fn lookup<T: Config>(dest: &T::Hash) -> Option<u8> {
    PrecompileIndex::<T>::get(dest)
}

// fixme: figure out charging, costing
pub(crate) fn invoke_raw<T: Config>(precompile: &u8, args: &mut &[u8], output: &mut Vec<u8>) {
    match extract_origin::<T>(args) {
        Some(origin) => match *precompile {
            GET_STATE => {
                let args: CodecResult<GetState<T>> = Decode::decode(args);
                if let Ok(args) = args {
                    if let Ok(PrecompileInvocation::GetState(state)) =
                        invoke::<T>(PrecompileArgs::GetState(origin, args))
                    {
                        Ok::<_, Error<T>>(state).encode_to(output)
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output)
                }
            },
            SUBMIT => {
                let args: CodecResult<SideEffects<T::AccountId, BalanceOf<T>, T::Hash>> =
                    Decode::decode(args);
                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::SubmitSideEffects(origin, args)) {
                        Ok(_) => {
                            // No need to write output other than a success byte
                            Ok::<_, Error<T>>(()).encode_to(output)
                        },
                        Err(e) => Err::<(), _>(e).encode_to(output),
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output)
                }
            },
            POST_SIGNAL => {
                let args: CodecResult<ExecutionSignal<T::Hash>> = Decode::decode(args);

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::Signal(origin, args)) {
                        Ok(_) => {
                            // No need to write output other than a success byte
                            Ok::<_, Error<T>>(()).encode_to(output)
                        },
                        Err(e) => Err::<(), _>(e).encode_to(output),
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output)
                }
            },
            _ => Err::<(), _>(Error::<T>::InvalidPrecompilePointer).encode_to(output),
        },
        None => Err::<(), _>(Error::<T>::InvalidOrigin).encode_to(output),
    }
}

fn extract_origin<T: frame_system::Config>(args: &mut &[u8]) -> Option<T::Origin> {
    match <T::AccountId as Decode>::decode(args) {
        Ok(account) => Some(T::Origin::from(RawOrigin::Signed(account))),
        Err(err) => {
            log::debug!(target: LOG_TARGET, "Failed to decode origin: {:?}", err);
            None
        },
    }
}

pub(crate) fn invoke<T: Config>(
    precompile: PrecompileArgs<T, BalanceOf<T>>,
) -> Result<PrecompileInvocation<T, BalanceOf<T>>, DispatchError> {
    match precompile {
        PrecompileArgs::GetState(ref origin, args) => {
            log::debug!(target: LOG_TARGET, "Reading state {:?}", args.xtx_id,);
            let state = <Pallet<T> as LocalStateAccess<T, BalanceOf<T>>>::load_local_state(
                origin,
                args.xtx_id.as_ref(),
            )?;
            Ok(PrecompileInvocation::GetState(state))
        },
        PrecompileArgs::SubmitSideEffects(origin, side_effects) => {
            log::debug!(
                target: LOG_TARGET,
                "Submitting {:?} side effects: {:?}",
                side_effects.execution_id,
                side_effects.side_effects,
            );

            let origin_account = if let RawOrigin::Signed(account) = origin
                .clone()
                .into()
                .map_err(|_| Error::<T>::InvalidOrigin)?
            {
                account
            } else {
                return Err(Error::<T>::InvalidOrigin.into())
            };

            if !side_effects.side_effects.is_empty() {
                let trigger = LocalTrigger::<T>::new(
                    origin_account, // FIXME: this is not right, investigate the contract address param
                    side_effects
                        .side_effects
                        .iter()
                        .map(|i| i.encode())
                        .collect(),
                    Some(side_effects.execution_id),
                );

                T::OnLocalTrigger::on_local_trigger(&origin, trigger)
                    .map(|_| PrecompileInvocation::Submit)
            } else {
                Err(Error::<T>::CannotTriggerWithoutSideEffects.into())
            }
        },
        PrecompileArgs::Signal(origin, signal) => {
            match <Pallet<T> as Signaller<T::Hash>>::signal(&signal) {
                Ok(_) => {
                    <T::OnLocalTrigger>::on_signal(&origin, signal).map_err(|e| {
                        log::error!(target: LOG_TARGET, "handling signal {:?}", e);
                        e
                    })?;

                    log::debug!(target: LOG_TARGET, "Successfully posted signal");
                    Ok(PrecompileInvocation::Signal)
                },
                Err(e) => {
                    log::error!(target: LOG_TARGET, "error posting signal",);
                    Err(e)
                },
            }
        },
    }
}
// TODO: tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use sp_runtime::traits::Hash;
    use t3rn_primitives::circuit::LocalStateExecutionView;

    #[test]
    fn test_extract_origin_consumes_buffer() {
        new_test_ext().execute_with(|| {
            let account = 4_u64;
            let buffer = &mut &account.encode()[..];
            let _result = extract_origin::<Test>(buffer).unwrap();
            assert_eq!(buffer.len(), 0)
        });
    }

    #[test]
    fn invoke_raw_bad_pointer() {
        new_test_ext().execute_with(|| {
            let account = 4_u64;
            let args = &mut &account.encode()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, args, &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompilePointer));
        });
    }

    // TODO: errors from pallet-circuit should not cause a panic in the buffer.
    #[ignore]
    #[test]
    fn invoke_get_state_circuit_error() {
        new_test_ext().execute_with(|| {
            let account = 4_u64;
            let get_state = GetState::<Test> {
                xtx_id: Some(<Test as frame_system::Config>::Hashing::hash_of(
                    b"hello world sir",
                )),
            };
            let args = &mut &[account.encode(), get_state.encode()].concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&GET_STATE, args, &mut out);
            let res = <core::result::Result<
                LocalStateExecutionView<Test, BalanceOf<Test>>,
                DispatchError,
            > as Decode>::decode(&mut &out[..])
            .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompilePointer.into()));
        });
    }

    #[test]
    fn invoke_get_state() {
        new_test_ext().execute_with(|| {
            let account = 4_u64;
            let get_state = GetState::<Test> { xtx_id: None };
            let args = &mut &[account.encode(), get_state.encode()].concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&GET_STATE, args, &mut out);
            let res = <core::result::Result<
                LocalStateExecutionView<Test, BalanceOf<Test>>,
                crate::Error<Test>,
            > as Decode>::decode(&mut &out[..])
            .unwrap();
            assert_eq!(
                res,
                Ok(LocalStateExecutionView::<Test, BalanceOf<Test>> {
                    local_state: Default::default(),
                    hardened_side_effects: vec![vec![]],
                    steps_cnt: (0, 0),
                    xtx_id: <Test as frame_system::Config>::Hash::decode(
                        &mut &hex::decode(
                            "e0f81c92f7ec3253b2bc5356d5bd928792d40f3022c38ce088553dc8f5bb32c0"
                        )
                        .unwrap()[..]
                    )
                    .unwrap()
                })
            );
        });
    }
}
