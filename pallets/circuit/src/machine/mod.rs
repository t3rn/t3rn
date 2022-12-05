use crate::{pallet::Error, *};

use sp_std::marker::PhantomData;
use t3rn_primitives::transfers::EscrowedBalanceOf;

pub struct Machine<T: Config> {
    _phantom: PhantomData<T>,
}

pub fn no_mangle<T: Config>(
    _current_fsx: &mut Vec<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >,
    >,
    _local_state: LocalState,
    _steps_cnt: (u32, u32),
    _status: CircuitStatus,
    _requester: T::AccountId,
) -> Result<PrecompileResult<T>, Error<T>> {
    Ok(PrecompileResult::Continue)
}

pub fn no_post_updates<T: Config>(
    _status_change: (CircuitStatus, CircuitStatus),
    _local_ctx: &LocalXtxCtx<T>,
) -> Result<(), Error<T>> {
    Ok(())
}

// pub fn infallible_no_op<T: Config>(_local_ctx: &LocalXtxCtx<T>) {}

pub struct CircuitLog {}

pub enum PrecompileResult<T: Config> {
    UpdateFSX(
        Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        >,
    ),
    Continue,
    ForceUpdateStatus(CircuitStatus),
    TryKill(Cause),
    Revert(Cause),
}

// Further Refactors:
// - move all square_up actions to monetary module that always interacts with AccounManager and doesn't lock up balances directly
// - separate executors bidding to a separate pallet with individual storage entries for each bid?
//      - global clock then picks up all collected bids and calls either CircuitMachine::compile or CircuitMachine::kill
// Circuit as Mealy State Machine with finite number of State transitions between CircuitStatus
impl<T: Config> Machine<T> {
    // Only to be called after receiving new batch of SideEffects delivered by:
    // - on_extrinsic_trigger - gateway or user
    // - on_local_trigger - 3vm smart contract
    // Returns fresh LocalXtxContext
    pub fn setup(
        side_effects: &[SideEffect<T::AccountId, EscrowedBalanceOf<T, T::Escrowed>>],
        requester: &T::AccountId,
    ) -> Result<LocalXtxCtx<T>, Error<T>> {
        // ToDo: Introduce default delay
        let (timeouts_at, delay_steps_at): (T::BlockNumber, Option<Vec<T::BlockNumber>>) = (
            T::XtxTimeoutDefault::get() + frame_system::Pallet::<T>::block_number(),
            None,
        );

        let (xtx_id, xtx) = XExecSignal::<T::AccountId, T::BlockNumber>::setup_fresh::<T>(
            requester,
            timeouts_at,
            delay_steps_at,
        );

        if <pallet::Pallet<T> as Store>::XExecSignals::contains_key(xtx_id) {
            return Err(Error::<T>::SetupFailedDuplicatedXtx)
        }

        let mut local_xtx_ctx = LocalXtxCtx {
            local_state: LocalState::new(),
            use_protocol: UniversalSideEffectsProtocol::new(),
            xtx_id,
            xtx,
            full_side_effects: vec![],
        };

        pallet::Pallet::<T>::validate(side_effects, &mut local_xtx_ctx).map_err(|e| {
            log::error!("Self::validate hit an error -- {:?}", e);
            Error::<T>::SideEffectsValidationFailed
        })?;

        Ok(local_xtx_ctx)
    }

    // Infallible attempt to kill Xtx of given Id and cleanup all its state
    //  called by:
    //  - global clock on timeout after exceeding timeout
    //  - global clock after bids weren't collected to all FSX
    // Since infallible (must be bc of global clock is based on on_initialized block hooks returns bool if killed successfully or false if not found
    pub fn kill(
        xtx_id: XtxId<T>,
        cause: Cause,
        origin: &T::AccountId,
        infallible_post_update: impl FnOnce((CircuitStatus, CircuitStatus), &LocalXtxCtx<T>),
    ) -> bool {
        let _local_ctx = match Self::load_xtx(xtx_id) {
            Ok(ctx) => ctx,
            Err(err) => {
                log::info!("Kill attempt failed with an error: {:?}", err);
                return false
            },
        };

        Self::compile_infallible(
            xtx_id,
            |_, _, _, _, _| -> PrecompileResult<T> {
                if origin == &T::SelfAccountId::get() {
                    PrecompileResult::Revert(cause)
                } else {
                    PrecompileResult::TryKill(cause)
                }
            },
            infallible_post_update,
        );

        true
    }

    pub fn compile_infallible(
        xtx_id: XtxId<T>,
        infallible_precompile: impl FnOnce(
            &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            >,
            LocalState,
            // steps count
            (u32, u32),
            CircuitStatus,
            // requester
            T::AccountId,
        ) -> PrecompileResult<T>,
        infallible_post_update: impl FnOnce((CircuitStatus, CircuitStatus), &LocalXtxCtx<T>),
    ) {
        Self::compile(
            xtx_id,
            |
                fsx: &mut Vec<
                    FullSideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        EscrowedBalanceOf<T, T::Escrowed>,
                    >,
                >,
                local_state: LocalState,
                steps_count: (u32, u32),
                status: CircuitStatus,
                requester: T::AccountId,
            | -> Result<PrecompileResult<T>, Error<T>> {
                Ok(infallible_precompile(fsx, local_state, steps_count, status, requester))
            },
            |status_change, local_ctx: &LocalXtxCtx<T>| -> Result<(), Error<T>> {
                infallible_post_update(status_change, local_ctx);
                Ok(())
            }
        ).expect("Expect compile to be infallible when called with infallible precompile and post_update");
    }

    // External interface exposed to all of the that can transition state, multiple FSX at the time i.e:
    // - submit_bid
    // - confirm_side_effect
    // - confirm side effect via XBI
    pub fn compile(
        xtx_id: XtxId<T>,
        precompile: impl FnOnce(
            &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    EscrowedBalanceOf<T, T::Escrowed>,
                >,
            >,
            LocalState,
            // steps count
            (u32, u32),
            CircuitStatus,
            T::AccountId,
        ) -> Result<PrecompileResult<T>, Error<T>>,
        post_update: impl FnOnce(
            (CircuitStatus, CircuitStatus),
            &LocalXtxCtx<T>,
        ) -> Result<(), Error<T>>,
    ) -> Result<(), Error<T>> {
        let mut local_ctx = Self::load_xtx(xtx_id)?;
        let current_fsx = Self::read_current_step_fsx(&local_ctx).clone();
        let local_state = local_ctx.local_state.clone();
        let steps_cnt = local_ctx.xtx.steps_cnt.clone();
        let status = local_ctx.xtx.status.clone();
        let requester = local_ctx.xtx.requester.clone();

        let enforced_new_status: Option<CircuitStatus> = match precompile(
            &mut current_fsx.clone(),
            local_state,
            steps_cnt,
            status,
            requester,
        )? {
            PrecompileResult::UpdateFSX(updated_fsx) => {
                Self::update_current_step_fsx(&mut local_ctx, &updated_fsx);
                None
            },
            PrecompileResult::Continue => None,
            // Assume kill attempt with fallible post_update to be intended as infallible cleanup to kill op
            //  in case fallible post_update passes, proceed with kill op
            // ToDo: check between allowed status enforcements - kill status / allowed enforced status
            PrecompileResult::TryKill(cause) => Some(CircuitStatus::Killed(cause)),
            PrecompileResult::ForceUpdateStatus(status) => Some(status),
            PrecompileResult::Revert(cause) => Some(CircuitStatus::Reverted(cause)),
        };

        let status_change = Self::update_status(&mut local_ctx, enforced_new_status)?;
        post_update(status_change.clone(), &local_ctx)?;
        Self::apply(&mut local_ctx, status_change);
        Ok(())
    }

    pub fn load_xtx(xtx_id: XtxId<T>) -> Result<LocalXtxCtx<T>, Error<T>> {
        let xtx = <pallet::Pallet<T> as Store>::XExecSignals::get(xtx_id)
            .ok_or(Error::<T>::SetupFailedUnknownXtx)?;
        let full_side_effects = <pallet::Pallet<T> as Store>::FullSideEffects::get(xtx_id)
            .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;
        let local_state = <pallet::Pallet<T> as Store>::LocalXtxStates::get(xtx_id)
            .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;

        Ok(LocalXtxCtx {
            local_state,
            use_protocol: UniversalSideEffectsProtocol::new(),
            xtx_id,
            xtx,
            full_side_effects,
        })
    }

    fn read_all_fsx(
        _xtx_id: XtxId<T>,
    ) -> Vec<
        Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        >,
    > {
        vec![vec![]]
    }

    pub fn read_single_fsx(
        _xtx_id: XtxId<T>,
        _sfx_id: SideEffectId<T>,
    ) -> Option<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >,
    > {
        None
    }

    fn update_current_step_fsx(
        local_ctx: &mut LocalXtxCtx<T>,
        updated_fsx: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                EscrowedBalanceOf<T, T::Escrowed>,
            >,
        >,
    ) {
        let current_step = local_ctx.xtx.steps_cnt.0;
        // todo: iterate and change one by one
        local_ctx.full_side_effects[current_step as usize] = updated_fsx.clone();
    }

    pub fn read_current_step_fsx(
        local_ctx: &LocalXtxCtx<T>,
    ) -> &Vec<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >,
    > {
        &local_ctx.full_side_effects[local_ctx.xtx.steps_cnt.0 as usize]
    }

    // Following methods aren't exposed to Pallet - internal use by compile only

    // Update should have all of the info accessible in LocalXtxCtx to transition between next states.
    fn update_status(
        local_ctx: &mut LocalXtxCtx<T>,
        enforce_new_status: Option<CircuitStatus>,
    ) -> Result<(CircuitStatus, CircuitStatus), Error<T>> {
        let current_status = local_ctx.xtx.status.clone();
        // Apply will try to move the status of Xtx from the current to the closest valid one.
        match current_status {
            CircuitStatus::Requested => {
                local_ctx.xtx.steps_cnt = (0, local_ctx.full_side_effects.len() as u32);
            },
            CircuitStatus::Reverted(_cause) => return Err(Error::<T>::UpdateAttemptDoubleRevert),
            CircuitStatus::Killed(_cause) => return Err(Error::<T>::UpdateAttemptDoubleKill),
            CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished => {
                // Check whether all of the side effects in this steps are confirmed - the status now changes to CircuitStatus::Finished
                if !Self::read_current_step_fsx(local_ctx)
                    .iter()
                    .any(|fsx| fsx.confirmed.is_none())
                {
                    local_ctx.xtx.steps_cnt =
                        (local_ctx.xtx.steps_cnt.0 + 1, local_ctx.xtx.steps_cnt.1);

                    local_ctx.xtx.status = CircuitStatus::Finished;

                    // All of the steps are completed - the xtx has been finalized
                    if local_ctx.xtx.steps_cnt.0 == local_ctx.xtx.steps_cnt.1 {
                        local_ctx.xtx.status = CircuitStatus::FinishedAllSteps;
                        return Ok((current_status, CircuitStatus::FinishedAllSteps))
                    }
                }
            },
            _ => {},
        }

        let mut new_status = CircuitStatus::determine_xtx_status(&local_ctx.full_side_effects)?;
        local_ctx.xtx.status = new_status.clone();

        new_status = CircuitStatus::check_transition(
            current_status.clone(),
            new_status,
            enforce_new_status,
        )?;

        Ok((current_status, new_status))
    }

    fn apply(local_ctx: &LocalXtxCtx<T>, status_change: (CircuitStatus, CircuitStatus)) {
        let (old_status, new_status) = (status_change.0, status_change.1);

        match old_status {
            CircuitStatus::Requested => {
                // Iterate over full side effects to detect ones to execute locally.
                fn is_local<T: Config>(gateway_id: &[u8; 4]) -> bool {
                    if *gateway_id == T::SelfGatewayId::get() {
                        return true
                    }
                    let gateway_type = <T as Config>::Xdns::get_gateway_type_unsafe(gateway_id);
                    gateway_type == GatewayType::ProgrammableInternal(0)
                }

                let steps_side_effects_ids: Vec<(
                    usize,
                    SideEffectId<T>,
                    XExecStepSideEffectId<T>,
                )> = local_ctx
                    .full_side_effects
                    .clone()
                    .iter()
                    .enumerate()
                    .flat_map(|(cnt, fsx_step)| {
                        fsx_step
                            .iter()
                            .map(|full_side_effect| {
                                full_side_effect
                                    .generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id)
                            })
                            .map(|side_effect_hash| {
                                (
                                    cnt,
                                    side_effect_hash,
                                    XExecSignal::<T::AccountId, T::BlockNumber>::generate_step_id::<
                                        T,
                                    >(local_ctx.xtx_id, cnt),
                                )
                            })
                            .collect::<Vec<(usize, SideEffectId<T>, XExecStepSideEffectId<T>)>>()
                    })
                    .collect();

                <pallet::Pallet<T> as Store>::FullSideEffects::insert::<
                    XExecSignalId<T>,
                    Vec<
                        Vec<
                            FullSideEffect<
                                T::AccountId,
                                T::BlockNumber,
                                EscrowedBalanceOf<T, T::Escrowed>,
                            >,
                        >,
                    >,
                >(local_ctx.xtx_id, local_ctx.full_side_effects.clone());

                for (_step_cnt, side_effect_id, _step_side_effect_id) in steps_side_effects_ids {
                    <pallet::Pallet<T> as Store>::SFX2XTXLinksMap::insert::<
                        SideEffectId<T>,
                        XExecSignalId<T>,
                    >(side_effect_id, local_ctx.xtx_id);
                }

                <pallet::Pallet<T> as Store>::LocalXtxStates::insert::<XExecSignalId<T>, LocalState>(
                    local_ctx.xtx_id,
                    local_ctx.local_state.clone(),
                );
                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::insert::<
                    XExecSignalId<T>,
                    T::BlockNumber,
                >(local_ctx.xtx_id, local_ctx.xtx.timeouts_at);
                <pallet::Pallet<T> as Store>::PendingXtxBidsTimeoutsMap::insert::<
                    XExecSignalId<T>,
                    T::BlockNumber,
                >(
                    local_ctx.xtx_id,
                    T::SFXBiddingPeriod::get() + frame_system::Pallet::<T>::block_number(),
                );
                <pallet::Pallet<T> as Store>::XExecSignals::insert::<
                    XExecSignalId<T>,
                    XExecSignal<T::AccountId, T::BlockNumber>,
                >(local_ctx.xtx_id, local_ctx.xtx.clone());
            },
            CircuitStatus::PendingBidding => {
                // Always clean temporary PendingSFXBids and TimeoutsMap after bidding
                <pallet::Pallet<T> as Store>::PendingSFXBids::remove_prefix(local_ctx.xtx_id, None);
                <pallet::Pallet<T> as Store>::PendingXtxBidsTimeoutsMap::remove(local_ctx.xtx_id);
                match new_status {
                    CircuitStatus::Ready => {
                        <pallet::Pallet<T> as Store>::FullSideEffects::mutate(
                            local_ctx.xtx_id,
                            |x| *x = Some(local_ctx.full_side_effects.clone()),
                        );
                        <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });
                    },
                    CircuitStatus::Killed(_cause) => {
                        // Clean all associated Xtx entries
                        <pallet::Pallet<T> as Store>::XExecSignals::remove(local_ctx.xtx_id);
                        <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(
                            local_ctx.xtx_id,
                        );
                        <pallet::Pallet<T> as Store>::LocalXtxStates::remove(local_ctx.xtx_id);
                        <pallet::Pallet<T> as Store>::FullSideEffects::remove(local_ctx.xtx_id);
                        for fsx_step in &local_ctx.full_side_effects {
                            for fsx in fsx_step {
                                <pallet::Pallet<T> as Store>::SFX2XTXLinksMap::remove(
                                    fsx.generate_id::<SystemHashing<T>, T>(local_ctx.xtx_id),
                                );
                            }
                        }
                    },
                    _ => {},
                }
            },
            CircuitStatus::Reverted(_cause) => {
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
            },
            // fixme: Separate for Bonded
            CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished => {
                match new_status {
                    CircuitStatus::FinishedAllSteps => {
                        // todo: cleanup all of the local storage
                        // TODO cleanup sfx2xtx map
                        <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });

                        <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(
                            local_ctx.xtx_id,
                        );

                        <pallet::Pallet<T> as Store>::LocalXtxStates::remove::<XExecSignalId<T>>(
                            local_ctx.xtx_id,
                        );
                    },
                    _ => {
                        // Update set of full side effects assuming the new confirmed has appeared
                        <pallet::Pallet<T> as Store>::FullSideEffects::mutate(
                            local_ctx.xtx_id,
                            |x| *x = Some(local_ctx.full_side_effects.clone()),
                        );

                        <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                            *x = Some(local_ctx.xtx.clone())
                        });
                    },
                }
            },
            CircuitStatus::FinishedAllSteps => {
                // TODO cleanup sfx2xtx map
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);

                <pallet::Pallet<T> as Store>::LocalXtxStates::remove::<XExecSignalId<T>>(
                    local_ctx.xtx_id,
                );
            },
            CircuitStatus::Committed => {},
            CircuitStatus::Killed(_) => {},
            // todo: consider moving bids to local xtx state
            CircuitStatus::InBidding => {},
        }
    }
}
