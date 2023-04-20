use crate::{BalanceOf, Config, Error, Pallet, PrecompileIndex};
use codec::{Decode, Encode};
use frame_support::{dispatch::RawOrigin, sp_runtime::DispatchError};
use sp_std::{vec, vec::Vec};
use t3rn_abi::{Codec as T3rnCodec, FilledAbi};
use t3rn_primitives::{
    circuit::{LocalTrigger, OnLocalTrigger},
    portal::{get_portal_interface_abi, Portal, PortalExecution, PortalPrecompileInterfaceEnum},
    threevm::{
        GetState, LocalStateAccess, PrecompileArgs, PrecompileInvocation,
        EVM_RECODING_BYTE_SELECTOR, GET_STATE, PORTAL, POST_SIGNAL, SUBMIT,
    },
};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::SideEffects,
};

const LOG_TARGET: &str = "3vm::precompile";

type CodecResult<T> = Result<T, codec::Error>;

pub(crate) fn lookup<T: Config>(dest: &T::Hash) -> Option<u8> {
    PrecompileIndex::<T>::get(dest)
}

// FIXME: figure out charging, costing
pub(crate) fn invoke_raw<T: Config>(precompile: &u8, args: &mut &[u8], output: &mut Vec<u8>) {
    if args.len() < 2 {
        return Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output)
    }

    // First byte determines if it came from EVM or WASM
    let codec_selector = args[0];

    // Strip the selector
    let args = &mut &args[1..];

    let codec = if codec_selector == EVM_RECODING_BYTE_SELECTOR {
        T3rnCodec::Rlp
    } else {
        T3rnCodec::default()
    };

    match extract_origin::<T>(&codec, args) {
        Some(origin) => match *precompile {
            GET_STATE => {
                let args: CodecResult<GetState<T>> = match codec {
                    T3rnCodec::Scale => Decode::decode(args),
                    T3rnCodec::Rlp =>
                        Err(codec::Error::from("Cannot decode GetState with RLP yet")),
                };

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
                    match codec {
                        T3rnCodec::Scale => Decode::decode(args),
                        T3rnCodec::Rlp =>
                            Err(codec::Error::from("Cannot decode SideEffects with RLP yet")),
                    };
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
                let args: CodecResult<ExecutionSignal<T::Hash>> = match codec {
                    T3rnCodec::Scale => Decode::decode(args),
                    T3rnCodec::Rlp => Err(codec::Error::from("Cannot decode Signals with RLP yet")),
                };

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
            PORTAL => {
                if args.len() < 2 {
                    return Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output)
                }

                // First byte is portal selector
                let portal_selector = &args[0];
                // The rest is the input for portal
                let input_without_portal_selector = &args[1..];

                let mut result = match codec {
                    T3rnCodec::Rlp => {
                        log::debug!(target: LOG_TARGET, "Rlp encoding bytes for portal selector {}", portal_selector);
                        log::debug!(target: LOG_TARGET, "Bytes {:?}", input_without_portal_selector);
                        FilledAbi::try_fill_abi(
                            get_portal_interface_abi(), // This panics :<
                            input_without_portal_selector.to_vec(),
                            codec.clone(),
                        )
                        .and_then(|abi| {
                            log::debug!(target: LOG_TARGET, "ABI was filled, recoding to scale {}", portal_selector);
                            abi.recode_as(&codec.clone(), &T3rnCodec::Scale)
                        })
                    }
                    T3rnCodec::Scale => Ok(input_without_portal_selector.to_vec())
                }
                .map(|mut recoded| {
                    recoded.insert(0, *portal_selector);
                    recoded
                })
                .and_then(|recoded| {
                    PortalPrecompileInterfaceEnum::decode(&mut &recoded[..])
                        .map_err(|_e| DispatchError::Other("Failed to decode portal interface enum"))
                })
                .and_then(|recoded_call_as_enum| {
                    log::debug!(target: LOG_TARGET, "Built recoded call {:?}", recoded_call_as_enum);
                    invoke::<T>(PrecompileArgs::Portal(recoded_call_as_enum)).map(|x| if let PrecompileInvocation::Portal(i) = x {
                        let bytes: Vec<u8> = i.into();
                        bytes
                    } else {
                        log::warn!(target: LOG_TARGET, "Exceptional issue, portal precompile invocation returned something other than a portal result");
                        Default::default()
                    })
                })
                .map_err(|e| match e {
                    DispatchError::Other(msg) => msg,
                    _ => "Failed to invoke portal",
                });

                log::debug!(target: LOG_TARGET, "Result {:?}", result);

                match result {
                    Ok(ref mut bytes) => {
                        output.push(0); // It's an ok
                        output.append(bytes);
                    },
                    Err(msg) => {
                        log::error!(target: LOG_TARGET, "Failed to invoke portal: {}", msg);
                        output.push(1); // It's an error
                                        // output.append(msg.as_bytes().to_vec().as_mut()) No need to write result if it gets thrown away
                    },
                }
            },
            _ => Err::<(), _>(Error::<T>::InvalidPrecompilePointer).encode_to(output),
        },
        None => Err::<(), _>(Error::<T>::InvalidOrigin).encode_to(output),
    }
}

fn extract_origin<T: Config>(codec: &T3rnCodec, args: &mut &[u8]) -> Option<T::Origin> {
    match codec {
        T3rnCodec::Scale => match <T::AccountId as Decode>::decode(args) {
            Ok(account) => Some(T::Origin::from(RawOrigin::Signed(account))),
            Err(err) => {
                log::debug!(target: LOG_TARGET, "Failed to decode origin: {:?}", err);
                None
            },
        },
        T3rnCodec::Rlp => {
            // TODO: inject addressmapping here, dont always assume padded 12
            let address_bytes = vec![args.take(..20)?, &[0_u8; 12][..]].concat();

            match <T::AccountId as Decode>::decode(&mut &address_bytes[..]) {
                Ok(account) => Some(T::Origin::from(RawOrigin::Signed(account))),
                Err(err) => {
                    log::debug!(target: LOG_TARGET, "Failed to decode origin: {:?}", err);
                    None
                },
            }
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
        PrecompileArgs::Portal(args) => match args {
            PortalPrecompileInterfaceEnum::GetLatestFinalizedHeader(chain_id) =>
                T::Portal::get_latest_finalized_header(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::GetLatestFinalizedHeight(chain_id) =>
                T::Portal::get_latest_finalized_height(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::GetLatestUpdatedHeight(chain_id) =>
                T::Portal::get_latest_updated_height(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::GetCurrentEpoch(chain_id) =>
                T::Portal::get_current_epoch(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::ReadEpochOffset(chain_id) =>
                T::Portal::read_epoch_offset(chain_id)
                    .map(|x| PrecompileInvocation::Portal(PortalExecution::BlockNumber(x))),
            PortalPrecompileInterfaceEnum::ReadFastConfirmationOffset(chain_id) =>
                T::Portal::read_fast_confirmation_offset(chain_id)
                    .map(|x| PrecompileInvocation::Portal(PortalExecution::BlockNumber(x))),
            PortalPrecompileInterfaceEnum::ReadRationalConfirmationOffset(chain_id) =>
                T::Portal::read_rational_confirmation_offset(chain_id)
                    .map(|x| PrecompileInvocation::Portal(PortalExecution::BlockNumber(x))),
            PortalPrecompileInterfaceEnum::VerifyEventInclusion(chain_id, event) =>
                T::Portal::verify_event_inclusion(chain_id, event, None)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::VerifyStateInclusion(chain_id, event) =>
                T::Portal::verify_state_inclusion(chain_id, event, None)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileInterfaceEnum::VerifyTxInclusion(chain_id, event) =>
                T::Portal::verify_tx_inclusion(chain_id, event, None)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
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
            let _result = extract_origin::<Test>(&T3rnCodec::Scale, buffer).unwrap();
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

    // #[test]
    // fn test_get_latest_finalized_header_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = GetLatestFinalizedHeader(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, GetLatestFinalizedHeader(chain_id));
    // }

    // #[test]
    // fn test_get_latest_finalized_height_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = GetLatestFinalizedHeight(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, GetLatestFinalizedHeight(chain_id));
    // }

    // #[test]
    // fn test_get_latest_updated_height_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = GetLatestUpdatedHeight(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, GetLatestUpdatedHeight(chain_id));
    // }

    // #[test]
    // fn test_get_current_epoch_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = GetCurrentEpoch(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, GetCurrentEpoch(chain_id));
    // }

    // #[test]
    // fn test_read_epoch_offset_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = ReadEpochOffset(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, ReadEpochOffset(chain_id));
    // }

    // #[test]
    // fn test_read_fast_confirmation_offset_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = ReadFastConfirmationOffset(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, ReadFastConfirmationOffset(chain_id));
    // }

    // #[test]
    // fn test_read_rational_confirmation_offset_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let portal_call = ReadRationalConfirmationOffset(chain_id);
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(
    //         recoded_portal_call,
    //         ReadRationalConfirmationOffset(chain_id)
    //     );
    // }

    // #[test]
    // fn test_verify_event_inclusion_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let event = vec![1, 2, 3, 4];
    //     let portal_call = VerifyEventInclusion(chain_id, event.clone());
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, VerifyEventInclusion(chain_id, event));
    // }

    // #[test]
    // fn test_verify_state_inclusion_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let event = vec![1, 2, 3, 4];
    //     let portal_call = VerifyStateInclusion(chain_id, event.clone());
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, VerifyStateInclusion(chain_id, event));
    // }

    // #[test]
    // fn test_verify_tx_inclusion_recodes_correctly_to_scale() {
    //     let chain_id: [u8; 4] = [9, 9, 9, 9];
    //     let event = vec![1, 2, 3, 4];
    //     let portal_call = VerifyTxInclusion(chain_id, event.clone());
    //     let encoded_portal_call = portal_call.encode();
    //     let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

    //     assert_eq!(recoded_portal_call, VerifyTxInclusion(chain_id, event));
    // }
}
