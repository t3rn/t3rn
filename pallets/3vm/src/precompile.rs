use crate::{BalanceOf, Config, Error, Pallet, PrecompileIndex};
use codec::{Decode, Encode};
use frame_support::{dispatch::RawOrigin, sp_runtime::DispatchError};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
use t3rn_primitives::{
    circuit::{LocalTrigger, OnLocalTrigger},
    threevm::{GetState, LocalStateAccess, PrecompileArgs, PrecompileInvocation},
    SpeedMode,
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
                let args: CodecResult<(
                    SideEffects<T::AccountId, BalanceOf<T>, T::Hash>,
                    SpeedMode,
                )> = Decode::decode(args);

                if let Ok((sfx_arg, speed_mode_arg)) = args {
                    match invoke::<T>(PrecompileArgs::SubmitSideEffects(
                        origin,
                        sfx_arg,
                        speed_mode_arg,
                    )) {
                        Ok(precompile_invocation) => {
                            let out_execution_state_view = precompile_invocation.get_submit();

                            // Insert encoded execution state view into output buffer
                            out_execution_state_view.encode_to(output);

                            // Insert Res::Success 0 byte
                            output.insert(0, 0u8);

                            Ok::<_, Error<T>>(());
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
        PrecompileArgs::SubmitSideEffects(origin, side_effects, speed_mode) => {
            let account = ensure_signed(origin.clone()).map_err(|_e| Error::<T>::InvalidOrigin)?;

            // todo: change parameter of t3rn_sdk::state::SideEffects to have optional execution_id
            let maybe_xtx_id = if side_effects.execution_id.encode() == [0; 32] {
                None
            } else {
                Some(side_effects.execution_id)
            };
            if !side_effects.side_effects.is_empty() {
                let trigger = LocalTrigger::<T>::new(
                    account, // FIXME: this is not right, investigate the contract address param
                    side_effects
                        .side_effects
                        .iter()
                        .map(|i| i.encode())
                        .collect(),
                    speed_mode,
                    maybe_xtx_id,
                );

                T::OnLocalTrigger::on_local_trigger(&origin, trigger).map(
                    |local_state_execution_view| {
                        PrecompileInvocation::Submit(local_state_execution_view)
                    },
                )
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
    use crate::mock::{new_test_ext, AccountId, Test};
    use sp_core::H256;
    use sp_runtime::traits::Hash;
    use t3rn_primitives::circuit::LocalStateExecutionView;
    use t3rn_sdk_primitives::{
        storage::BoundedVec,
        xc::{Chain, Operation},
    };

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
                    steps_cnt: (0, 1),
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

    #[test]
    fn invoke_submit_sfx_with_speed_mode() {
        new_test_ext().execute_with(|| {
            let account = 4_u64;
            let caller = 5_u64;
            let dest = 6_u64;
            let mut side_effects_bounded_vec: BoundedVec<Chain<AccountId, u128, H256>, 16> =
                BoundedVec::default();

            side_effects_bounded_vec
                .try_push(Chain::Kusama(Operation::Transfer {
                    caller,
                    to: dest,
                    amount: 1_u128,
                    insurance: None,
                }))
                .unwrap();

            let speed_mode = SpeedMode::Finalized;
            let submit_sfx_args = SideEffects::<AccountId, u128, H256> {
                execution_id: H256::zero(),
                side_effects: side_effects_bounded_vec,
            };

            let args = &mut &[
                account.encode(),
                submit_sfx_args.encode(),
                speed_mode.encode(),
            ]
            .concat()[..];
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
                    steps_cnt: (0, 1),
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
