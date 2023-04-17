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
        EVM_RECODING_BYTE_SELECTOR, WASM_RECODING_BYTE_SELECTOR,
    },
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
pub const PORTAL: u8 = 70;

type CodecResult<T> = Result<T, codec::Error>;

pub(crate) fn lookup<T: Config>(dest: &T::Hash) -> Option<u8> {
    PrecompileIndex::<T>::get(dest)
}

// FIXME: figure out charging, costing
pub(crate) fn invoke_raw<T: Config>(precompile: &u8, args: &mut &[u8], output: &mut Vec<u8>) {
    // TODO: assert the length here

    // First byte determines if it came from EVM or WASM
    let codec_selector = args[1];

    // Strip the selector
    let args = &mut &args[1..];

    let codec = if codec_selector == EVM_RECODING_BYTE_SELECTOR {
        T3rnCodec::Rlp
    } else {
        T3rnCodec::default()
    };

    // TODO: Is origin provided first or at all? if so that also needs recoding
    match extract_origin::<T>(args) {
        Some(origin) => match *precompile {
            GET_STATE => {
                let args: CodecResult<GetState<T>> = match codec {
                    T3rnCodec::Scale => Decode::decode(args),
                    T3rnCodec::Rlp =>
                        Err(codec::Error::from("Cannot decode GetState with RLP yet")),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::GetState(origin, args)) {
                        Ok(PrecompileInvocation::GetState(state)) =>
                            Ok::<_, Error<T>>(state).encode_to(output),
                        Err(e) => {
                            Err::<(), _>(Error::<T>::DownstreamCircuit).encode_to(output);
                            Err::<(), _>(e).encode_to(output)
                        },
                        _ => {},
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
                // First byte is portal selector
                let portal_selector = &args[0];
                // The rest is the input for portal
                let input_without_vm_selectors = &args[1..];

                let mut result = match codec {
                    T3rnCodec::Rlp => {
                        FilledAbi::try_fill_abi(
                            get_portal_interface_abi(), // This panics :<
                            input_without_vm_selectors.to_vec(),
                            codec.clone(),
                        )
                        .and_then(|abi| {
                            abi.recode_as(&codec.clone(), &T3rnCodec::Scale)
                        })
                    }
                    T3rnCodec::Scale => Ok(input_without_vm_selectors.to_vec())
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
                    invoke::<T>(PrecompileArgs::Portal(recoded_call_as_enum)).map(|x| if let PrecompileInvocation::Portal(i) = x {
                        let bytes: Vec<u8> = i.into();
                        bytes
                    } else {
                        log::warn!("Exceptional issue, portal precompile invocation returned something other than a portal result");
                        Default::default()
                    })
                })
                .map_err(|e| match e {
                    DispatchError::Other(msg) => msg,
                    _ => "Failed to invoke portal",
                });

                match result {
                    Ok(ref mut bytes) => output.append(bytes),
                    Err(msg) => output.append(msg.as_bytes().to_vec().as_mut()),
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
            let address_bytes = vec![args.take(..=20)?, &[0_u8; 12][..]].concat();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, AccountId, Test, ALICE};
    use sp_core::{H160, H256};
    use sp_runtime::traits::Hash;
    use t3rn_primitives::circuit::LocalStateExecutionView;
    use t3rn_sdk_primitives::{
        storage::BoundedVec,
        xc::{Chain, Operation},
    };

    #[test]
    fn test_extract_origin_consumes_buffer() {
        new_test_ext().execute_with(|| {
            let buffer = &mut &ALICE.encode()[..];
            let result = extract_origin::<Test>(&T3rnCodec::Scale, buffer).unwrap();
            println!("{result:?}");

            assert_eq!(buffer.len(), 0)
        });
    }

    #[test]
    fn test_extract_origin_consumes_buffer_rlp() {
        new_test_ext().execute_with(|| {
            let account = H160::from_low_u64_be(4);
            let buffer = &mut &rlp::encode(&account)[..];
            let result = extract_origin::<Test>(&T3rnCodec::Rlp, buffer).unwrap();
            println!("{result:?}");

            assert_eq!(buffer.len(), 0)
        });
    }

    #[test]
    fn invoke_raw_bad_pointer_rlp() {
        new_test_ext().execute_with(|| {
            let account = H160::from_low_u64_be(4);
            let args = &mut &vec![vec![T3rnCodec::Rlp.into()], rlp::encode(&account).to_vec()]
                .concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, args, &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompilePointer));
        });
    }

    #[test]
    fn invoke_raw_bad_pointer_scale() {
        new_test_ext().execute_with(|| {
            let args = &mut &vec![vec![T3rnCodec::Scale.into()], ALICE.encode()].concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, args, &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompilePointer));
        });
    }

    #[test]
    fn invoke_bad_origin() {
        new_test_ext().execute_with(|| {
            // RLP codec, scale encoded origin
            let args = vec![vec![1], ALICE.encode()].concat();
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, &mut &args[..], &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidOrigin));
        });
    }

    #[test]
    fn invoke_get_state_circuit_error() {
        new_test_ext().execute_with(|| {
            let get_state = GetState::<Test> {
                xtx_id: Some(<Test as frame_system::Config>::Hashing::hash_of(
                    b"hello world",
                )),
            };

            let mut args: Vec<u8> = vec![T3rnCodec::Scale.into()];
            args.extend(ALICE.encode());
            args.extend(get_state.encode());

            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&GET_STATE, &mut &args[..], &mut out);

            let res = <core::result::Result<
                LocalStateExecutionView<Test, BalanceOf<Test>>,
                crate::Error<Test>,
            > as Decode>::decode(&mut &out[..])
            .unwrap();
            assert_eq!(res, Err(Error::<Test>::DownstreamCircuit));
        });
    }

    #[test]
    fn invoke_get_state() {
        new_test_ext().execute_with(|| {
            let get_state = GetState::<Test> { xtx_id: None };

            let mut args: Vec<u8> = vec![T3rnCodec::Scale.into()];
            args.extend(ALICE.encode());
            args.extend(get_state.encode());

            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&GET_STATE, &mut &args[..], &mut out);

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
                            "e8b9e71dc3f878ecf9dae04303767b76882fd0034dd73c98589ff45dab57b05b"
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
