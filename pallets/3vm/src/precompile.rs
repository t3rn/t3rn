use crate::{BalanceOf, Config, Error, Pallet, PrecompileIndex};
use codec::{Decode, Encode};
use frame_support::{dispatch::RawOrigin, sp_runtime::DispatchError};
use frame_system::ensure_signed;
use sp_core::H160;
use sp_std::prelude::*;
use t3rn_primitives::{
    circuit::{
        LocalTrigger, OnLocalTrigger, VacuumEVM3DOrder, VacuumEVMOrder, VacuumEVMProof,
        VacuumEVMTeleportOrder,
    },
    execution_source_to_option,
    portal::{Portal, PrecompileArgs as PortalPrecompileArgs},
    threevm::{
        AddressMapping, GetState, LocalStateAccess, PrecompileArgs, PrecompileInvocation,
        VacuumAccess, GET_STATE, PORTAL, POST_SIGNAL, SUBMIT,
    },
    SpeedMode, T3rnCodec,
};

use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::SideEffects,
};

// pub enum VacuumAction {
//     VacuumOrder = 90u8,
//     Vacuum3DOrder = 91u8,
//     VacuumConfirm = 92u8,
//     VacuumSubmitCorrectnessProof = 93u8,
//     VacuumSubmitFaultProof = 94u8,
// }

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
    let codec = T3rnCodec::from(args[0]);

    // Strip the selector
    let args = &mut &args[1..];

    match extract_origin::<T>(&codec, args) {
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
                let args: CodecResult<(
                    SideEffects<T::AccountId, BalanceOf<T>, T::Hash>,
                    SpeedMode,
                )> = match codec {
                    T3rnCodec::Scale => Decode::decode(args),
                    T3rnCodec::Rlp =>
                        Err(codec::Error::from("Cannot decode SideEffects with RLP yet")),
                };

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
            VACUUM_ORDER => {
                let args: CodecResult<VacuumEVMOrder> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVMOrder::from_rlp_encoded_packed(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::VacuumOrder(origin, args)) {
                        Ok(PrecompileInvocation::VacuumOrder(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
                }
            },
            VACUUM_3D_ORDER => {
                let args: CodecResult<VacuumEVM3DOrder> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVM3DOrder::from_rlp_encoded_packed(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::Vacuum3DOrder(origin, args)) {
                        Ok(PrecompileInvocation::VacuumOrder(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
                }
            },
            VACUUM_CONFIRM => {
                let args: CodecResult<VacuumEVMOrder> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVMOrder::from_rlp_encoded_packed(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::VacuumConfirm(origin, args)) {
                        Ok(PrecompileInvocation::VacuumConfirm(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
                }
            },
            VACUUM_SUBMIT_CORRECTNESS_PROOF => {
                let args: CodecResult<VacuumEVMProof> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVMProof::from_rlp(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::VacuumSubmitCorrectnessProof(origin, args)) {
                        Ok(PrecompileInvocation::VacuumSubmitCorrectnessProof(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
                }
            },
            VACUUM_SUBMIT_FAULT_PROOF => {
                let args: CodecResult<VacuumEVMProof> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVMProof::from_rlp(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::VacuumSubmitFaultProof(origin, args)) {
                        Ok(PrecompileInvocation::VacuumSubmitFaultProof(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
                }
            },
            VACUUM_TELEPORT_ORDER => {
                let args: CodecResult<VacuumEVMTeleportOrder> = match codec {
                    T3rnCodec::Scale => Decode::decode(&mut &args[..]),
                    T3rnCodec::Rlp => VacuumEVMTeleportOrder::from_rlp(&args[..]).map_err(|e| {
                        log::debug!(target: LOG_TARGET, "Failed to decode vacuum order: {:?}", e);
                        codec::Error::from("Failed to decode vacuum order")
                    }),
                };

                if let Ok(args) = args {
                    match invoke::<T>(PrecompileArgs::VacuumTeleportOrder(origin, args)) {
                        Ok(PrecompileInvocation::VacuumTeleportOrder(success)) => {
                            match success {
                                true => {
                                    output.push(0); // It's an ok
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                                false => {
                                    output.push(1); // It's an error
                                    Ok::<_, Error<T>>(()).encode_to(output)
                                },
                            }
                        },
                        Err(e) => {
                            log::error!(target: LOG_TARGET, "Failed to invoke vacuum: {:?}", e);
                            output.push(1); // It's an error
                            output.append(&mut e.encode());
                        },
                        _ => {},
                    }
                } else {
                    Err::<(), _>(Error::<T>::InvalidPrecompileArgs).encode_to(output);
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
                let mut result = PortalPrecompileArgs::recode_to_scale_and_decode(&codec, args)
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

fn extract_origin<T: Config>(codec: &T3rnCodec, args: &mut &[u8]) -> Option<T::RuntimeOrigin> {
    match codec {
        T3rnCodec::Scale => match <T::AccountId as Decode>::decode(args) {
            Ok(account) => Some(T::RuntimeOrigin::from(RawOrigin::Signed(account))),
            Err(err) => {
                log::debug!(target: LOG_TARGET, "Failed to decode origin: {:?}", err);
                None
            },
        },
        T3rnCodec::Rlp => {
            // Check if longer than 20b and assume address_bytes
            let address_bytes = if args.len() >= 20 {
                let address_bytes_out = &args[..20];
                *args = &args[20..];
                address_bytes_out
            } else {
                return None
            };

            let evm_address = H160::from_slice(&address_bytes);

            // Tap into evm_address
            let mapped_account = T::AddressMapping::into_account_id(&evm_address);

            Some(T::RuntimeOrigin::from(RawOrigin::Signed(mapped_account)))
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
        PrecompileArgs::VacuumOrder(origin, vacuum_order) => {
            log::debug!(target: LOG_TARGET, "Vacuum Order");
            let success = <Pallet<T> as VacuumAccess<T>>::evm_order(&origin, vacuum_order)?;
            // let success = true;
            Ok(PrecompileInvocation::VacuumOrder(success))
        },
        PrecompileArgs::VacuumConfirm(origin, vacuum_order) => {
            let success = true;
            Ok(PrecompileInvocation::VacuumConfirm(success))
        },
        PrecompileArgs::Vacuum3DOrder(origin, vacuum_order) => {
            let success = true;
            Ok(PrecompileInvocation::VacuumOrder(success))
        },
        PrecompileArgs::VacuumSubmitCorrectnessProof(origin, vacuum_proof) => {
            let success = true;
            Ok(PrecompileInvocation::VacuumSubmitCorrectnessProof(success))
        },
        PrecompileArgs::VacuumSubmitFaultProof(origin, vacuum_proof) => {
            let success = true;
            Ok(PrecompileInvocation::VacuumSubmitFaultProof(success))
        },
        PrecompileArgs::VacuumTeleportOrder(origin, vacuum_order) => {
            let success = true;
            Ok(PrecompileInvocation::VacuumTeleportOrder(success))
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
            PortalPrecompileArgs::GetLatestFinalizedHeader(chain_id) =>
                T::Portal::get_latest_finalized_header(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::GetFinalizedHeight(chain_id) =>
                T::Portal::get_finalized_height(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::GetRationalHeight(chain_id) =>
                T::Portal::get_rational_height(chain_id)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::GetFastHeight(chain_id) =>
                T::Portal::get_fast_height(chain_id).map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::VerifyEventInclusion(chain_id, speed_mode, source, event) =>
                T::Portal::verify_event_inclusion(
                    chain_id,
                    speed_mode,
                    execution_source_to_option(source),
                    event,
                )
                .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::VerifyStateInclusion(chain_id, speed_mode, event) =>
                T::Portal::verify_state_inclusion(chain_id, speed_mode, event)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
            PortalPrecompileArgs::VerifyTxInclusion(chain_id, speed_mode, event) =>
                T::Portal::verify_tx_inclusion(chain_id, speed_mode, event)
                    .map(|x| PrecompileInvocation::Portal(x.into())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, AccountId, Test, ALICE};
    use sp_core::{H160, H256, U256};
    use sp_runtime::traits::Hash;
    use t3rn_primitives::{circuit::LocalStateExecutionView, threevm::VACUUM_ORDER};
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
            let mut buffer = &mut account.0.as_slice();
            let result = extract_origin::<Test>(&T3rnCodec::Rlp, &mut buffer);
            println!("{result:?}");

            assert_eq!(buffer.len(), 0)
        });
    }

    #[test]
    #[ignore]
    fn invoke_raw_bad_pointer_rlp() {
        new_test_ext().execute_with(|| {
            let account = H160::from_low_u64_be(4);
            let args = &mut &[T3rnCodec::Rlp.encode(), account.0.to_vec()].concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, args, &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidOrigin));
        });
    }

    #[test]
    fn invoke_raw_bad_pointer_scale() {
        new_test_ext().execute_with(|| {
            let args = &mut &[vec![T3rnCodec::Scale.into()], ALICE.encode()].concat()[..];
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, args, &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompileArgs));
        });
    }

    #[test]
    fn invoke_bad_origin() {
        new_test_ext().execute_with(|| {
            // RLP codec, scale encoded origin
            let args = [T3rnCodec::Scale.encode(), ALICE.encode()].concat();
            let mut out = Vec::<u8>::new();

            invoke_raw::<Test>(&244_u8, &mut &args[..], &mut out);
            let res =
                <core::result::Result<(), crate::Error<Test>> as Decode>::decode(&mut &out[..])
                    .unwrap();
            assert_eq!(res, Err(Error::<Test>::InvalidPrecompileArgs));
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
                            "c9c2b9c48fb9c3ca9e71817fb01e907be3e0eda4d950bbdcb6dcc4c1a73a6537"
                        )
                        .unwrap()[..]
                    )
                    .unwrap()
                })
            );
        });
    }

    #[test]
    #[ignore]
    fn invoke_vacuum_remote_order_to_single_order_as_rlp_contract_arguments() {
        new_test_ext().execute_with(|| {
            // Mocks RLP pack-encoded arguments for vacuum order
            // function order(bytes4 destination, uint32 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward) public payable
            let mut args: Vec<u8> = vec![T3rnCodec::Rlp.into()];
            let msg_sender_20b = H160([1u8; 20]).encode();
            let target_4bytes = 4_u32.to_be_bytes().to_vec();
            let asset_uint32 = 1_u32.to_be_bytes().to_vec();
            let target_account_32bytes = H256([2u8; 32]).encode();
            let amount_uint256 = U256::from(1).encode();
            let reward_asset_address_20b = H160([3u8; 20]).encode();
            let insurance_uint256 = U256::from(0).encode();
            let max_reward_uint256 = U256::from(11).encode();

            args.extend(target_4bytes);
            args.extend(asset_uint32);
            args.extend(target_account_32bytes);
            args.extend(amount_uint256);
            args.extend(reward_asset_address_20b);
            args.extend(insurance_uint256);
            args.extend(max_reward_uint256);

            let mut out = Vec::<u8>::new();

            dbg!("args len test: ", args.len());

            // TODO: Not sure which value should be used her in place of VACUUM so ignoring the test for now
            invoke_raw::<Test>(&VACUUM_ORDER, &mut &args[..], &mut out);

            assert_eq!(out, vec![0]);
        });
    }

    #[test]
    fn invoke_submit_sfx_with_speed_mode() {
        new_test_ext().execute_with(|| {
            let account = AccountId::from([4u8; 32]);
            let caller = AccountId::from([5u8; 32]);
            let dest = AccountId::from([6u8; 32]);
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
                            "21efe62a0d0952014033c62b30772e0cd26e7009dfa24a81d47ad264e8e90ad2"
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
