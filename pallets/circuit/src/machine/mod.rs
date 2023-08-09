use crate::{pallet::Error, *};
use frame_support::ensure;

use crate::square_up::SquareUp;

pub mod extra;
pub use extra::*;

#[cfg(test)]
pub mod test;
#[cfg(test)]
pub mod test_extra;
#[cfg(test)]
pub mod test_extra_stress;

use sp_std::marker::PhantomData;
use t3rn_primitives::SpeedMode;

pub struct Machine<T: Config> {
    _phantom: PhantomData<T>,
}

pub enum PrecompileResult<T: Config> {
    TryUpdateFSX(
        Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
        >,
    ),
    TryBid(
        (
            SideEffectId<T>,
            BalanceOf<T>,
            <T as frame_system::Config>::AccountId,
        ),
    ),
    TryConfirm(
        SideEffectId<T>,
        ConfirmedSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
    ),
    TryRequest,
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
        side_effects: &[SideEffect<T::AccountId, BalanceOf<T>>],
        requester: &T::AccountId,
        maybe_adaptive_timeout: Option<
            AdaptiveTimeout<frame_system::pallet_prelude::BlockNumberFor<T>, TargetId>,
        >,
    ) -> Result<LocalXtxCtx<T, BalanceOf<T>>, Error<T>> {
        // ToDo: Introduce default delay
        let (timeouts_at, delay_steps_at): (
            frame_system::pallet_prelude::BlockNumberFor<T>,
            Option<Vec<BlockNumberFor<T>>>,
        ) = (
            T::XtxTimeoutDefault::get() + frame_system::Pallet::<T>::block_number(),
            None,
        );

        let adaptive_timeout = match maybe_adaptive_timeout {
            None => AdaptiveTimeout::<
                frame_system::pallet_prelude::BlockNumberFor<T>,
                TargetId,
            >::new_emergency(timeouts_at),
            Some(adaptive_timeout) => adaptive_timeout,
        };

        let (xtx_id, xtx) = XExecSignal::<T::AccountId, BlockNumberFor<T>>::setup_fresh::<T>(
            requester,
            adaptive_timeout,
            SpeedMode::Finalized,
            delay_steps_at,
        );

        if <pallet::Pallet<T> as Store>::XExecSignals::contains_key(xtx_id) {
            return Err(Error::<T>::SetupFailedDuplicatedXtx)
        }

        let mut local_xtx_ctx = LocalXtxCtx {
            local_state: LocalState::new(),
            xtx_id,
            xtx,
            full_side_effects: vec![],
        };

        pallet::Pallet::<T>::validate(side_effects, &mut local_xtx_ctx).map_err(|e| {
            log::error!("Self::validate hit an error -- {e:?}");
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
        infallible_post_update: impl FnOnce(
            (CircuitStatus, CircuitStatus),
            &LocalXtxCtx<T, BalanceOf<T>>,
        ),
    ) -> bool {
        let mut local_ctx = match Self::load_xtx(xtx_id) {
            Ok(ctx) => ctx,
            Err(_err) => return false,
        };
        Self::compile_infallible(
            &mut local_ctx,
            |_, _, _, _, _| -> PrecompileResult<T> { PrecompileResult::TryKill(cause) },
            infallible_post_update,
        )
    }

    pub fn revert(
        xtx_id: XtxId<T>,
        cause: Cause,
        infallible_post_update: impl FnOnce(
            (CircuitStatus, CircuitStatus),
            &LocalXtxCtx<T, BalanceOf<T>>,
        ),
    ) -> bool {
        let mut local_ctx = match Self::load_xtx(xtx_id) {
            Ok(ctx) => ctx,
            Err(_err) => return false,
        };
        Self::compile_infallible(
            &mut local_ctx,
            |_, _, _, _, _| -> PrecompileResult<T> { PrecompileResult::Revert(cause) },
            infallible_post_update,
        )
    }

    pub fn compile_infallible(
        local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>,
        infallible_precompile: impl FnOnce(
            &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
                >,
            >,
            LocalState,
            // steps count
            (u32, u32),
            CircuitStatus,
            // requester
            T::AccountId,
        ) -> PrecompileResult<T>,
        infallible_post_update: impl FnOnce(
            (CircuitStatus, CircuitStatus),
            &LocalXtxCtx<T, BalanceOf<T>>,
        ),
    ) -> bool {
        Self::compile(
            local_ctx,
            |
                fsx: &mut Vec<
                    FullSideEffect<
                        <T as frame_system::Config>::AccountId,
                        <T as frame_system::Config>::BlockNumber,
                        BalanceOf<T>,
                    >,
                >,
                local_state: LocalState,
                steps_count: (u32, u32),
                status: CircuitStatus,
                requester: T::AccountId,
            | -> Result<PrecompileResult<T>, Error<T>> {
                Ok(infallible_precompile(fsx, local_state, steps_count, status, requester))
            },
            |status_change, local_ctx: &LocalXtxCtx<T, BalanceOf<T>>| -> Result<(), Error<T>> {
                infallible_post_update(status_change, local_ctx);
                Ok(())
            }
        ).expect("Expect compile to be infallible when called with infallible precompile and post_update")
    }

    // External interface exposed to all of the that can transition state, multiple FSX at the time i.e:
    // - submit_bid
    // - confirm_side_effect
    // - confirm side effect via XBI
    pub fn compile(
        local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>,
        precompile: impl FnOnce(
            &mut Vec<
                FullSideEffect<
                    <T as frame_system::Config>::AccountId,
                    <T as frame_system::Config>::BlockNumber,
                    BalanceOf<T>,
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
            &LocalXtxCtx<T, BalanceOf<T>>,
        ) -> Result<(), Error<T>>,
    ) -> Result<bool, Error<T>> {
        let mut current_fsx = Self::read_current_step_fsx(local_ctx).clone();
        let local_state = local_ctx.local_state.clone();
        let steps_cnt = local_ctx.xtx.steps_cnt;
        let status = local_ctx.xtx.status.clone();
        let requester = local_ctx.xtx.requester.clone();

        let enforced_new_status: Option<CircuitStatus> = match precompile(
            &mut current_fsx,
            local_state,
            steps_cnt,
            status.clone(),
            requester.clone(),
        )? {
            PrecompileResult::TryRequest => {
                // Try deposit from requester
                SquareUp::<T>::try_request(local_ctx)
                    .map_err(|_e| Error::<T>::RequesterNotEnoughBalance)?;
                None
            },
            PrecompileResult::TryUpdateFSX(updated_fsx) => {
                Self::update_current_step_fsx(local_ctx, &updated_fsx);
                None
            },
            PrecompileResult::TryConfirm(sfx_id, confirmed_sfx) => {
                let mut found = false;
                current_fsx.iter_mut().for_each(|fsx| {
                    if fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id) == sfx_id {
                        found = true;
                        fsx.confirmed = Some(confirmed_sfx.clone());
                    }
                });
                ensure!(found, Error::<T>::FSXNotFoundById);
                Self::update_current_step_fsx(local_ctx, &current_fsx);
                None
            },
            PrecompileResult::TryBid((sfx_id, bid_amount, bidder)) => {
                match status {
                    CircuitStatus::PendingBidding | CircuitStatus::InBidding => {
                        // Try to replace the current bid with the new one if the amount is lower.
                        // This will also replace a deposit in AccountManager from with required insurance from the new bidder.
                        let updated_fsx = Bids::<T>::try_bid(
                            &mut current_fsx,
                            bid_amount,
                            &bidder,
                            &requester,
                            sfx_id,
                            local_ctx.xtx_id,
                        )?;

                        Self::update_current_step_fsx(local_ctx, &updated_fsx);

                        Some(CircuitStatus::InBidding)
                    },
                    _ => return Err(Error::<T>::BiddingInactive),
                }
            },
            PrecompileResult::Continue => None,
            // Assume kill attempt with fallible post_update to be intended as infallible cleanup to kill op
            //  in case fallible post_update passes, proceed with kill op
            // ToDo: check between allowed status enforcements - kill status / allowed enforced status
            PrecompileResult::TryKill(cause) => Some(CircuitStatus::Killed(cause)),
            PrecompileResult::ForceUpdateStatus(force_status) => {
                if CircuitStatus::InBidding == status && force_status == CircuitStatus::Ready {
                    SquareUp::<T>::bind_bidders(local_ctx);
                }
                Some(force_status)
            },
            PrecompileResult::Revert(cause) => Some(CircuitStatus::Reverted(cause)),
        };
        let status_change = Self::update_status(local_ctx, enforced_new_status)?;
        post_update(status_change.clone(), local_ctx)?;
        Ok(Self::apply(local_ctx, status_change))
    }

    pub fn load_xtx(xtx_id: XtxId<T>) -> Result<LocalXtxCtx<T, BalanceOf<T>>, Error<T>> {
        let xtx = <pallet::Pallet<T> as Store>::XExecSignals::get(xtx_id)
            .ok_or(Error::<T>::XtxDoesNotExist)?;
        let full_side_effects = <pallet::Pallet<T> as Store>::FullSideEffects::get(xtx_id)
            .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;
        let local_state = <pallet::Pallet<T> as Store>::LocalXtxStates::get(xtx_id)
            .ok_or(Error::<T>::SetupFailedXtxStorageArtifactsNotFound)?;

        Ok(LocalXtxCtx {
            local_state,
            xtx_id,
            xtx,
            full_side_effects,
        })
    }

    fn update_current_step_fsx(
        local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>,
        updated_fsx: &Vec<
            FullSideEffect<
                <T as frame_system::Config>::AccountId,
                <T as frame_system::Config>::BlockNumber,
                BalanceOf<T>,
            >,
        >,
    ) {
        let (current_step, _) = local_ctx.xtx.steps_cnt;

        match local_ctx.full_side_effects.get_mut(current_step as usize) {
            Some(current_step) => {
                *current_step = updated_fsx.to_vec();
            },
            None => {
                *local_ctx
                    .full_side_effects
                    .last_mut()
                    .expect("read_current_step_fsx to have at least one step in FSX steps") =
                    updated_fsx.to_vec();
            },
        };
    }

    pub fn read_current_step_fsx(
        local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
    ) -> &Vec<
        FullSideEffect<
            <T as frame_system::Config>::AccountId,
            <T as frame_system::Config>::BlockNumber,
            BalanceOf<T>,
        >,
    > {
        let (current_step, _) = local_ctx.xtx.steps_cnt;
        local_ctx
            .full_side_effects
            .get(current_step as usize)
            .unwrap_or_else(|| {
                local_ctx
                    .full_side_effects
                    .last()
                    .expect("read_current_step_fsx to have at least one step in FSX steps")
            })
    }

    // Following methods aren't exposed to Pallet - internal use by compile only
    fn check_bump_steps(
        local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
        status_change: (CircuitStatus, CircuitStatus),
    ) -> (u32, u32) {
        let (prev_status, new_status) = status_change;
        match (prev_status, new_status) {
            (
                CircuitStatus::Requested,
                CircuitStatus::Requested
                | CircuitStatus::Reserved
                | CircuitStatus::PendingBidding
                | CircuitStatus::InBidding,
            ) => (0, local_ctx.full_side_effects.len() as u32),
            (CircuitStatus::Ready | CircuitStatus::PendingExecution, CircuitStatus::Finished) => {
                let (current_step, steps_cnt) = local_ctx.xtx.steps_cnt;
                (current_step + 1, steps_cnt)
            },
            (
                CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished,
                CircuitStatus::FinishedAllSteps,
            ) => {
                let (_, steps_cnt) = local_ctx.xtx.steps_cnt;
                (steps_cnt, steps_cnt)
            },
            (_, _) => local_ctx.xtx.steps_cnt,
        }
    }

    // Update should have all of the info accessible in LocalXtxCtx to transition between next states.
    fn update_status(
        local_ctx: &mut LocalXtxCtx<T, BalanceOf<T>>,
        enforce_new_status: Option<CircuitStatus>,
    ) -> Result<(CircuitStatus, CircuitStatus), Error<T>> {
        let current_status = local_ctx.xtx.status.clone();
        // Apply will try to move the status of Xtx from the current to the closest valid one.
        match current_status {
            CircuitStatus::Reverted(_cause) => return Err(Error::<T>::UpdateAttemptDoubleRevert),
            CircuitStatus::Killed(_cause) => return Err(Error::<T>::UpdateAttemptDoubleKill),
            _ => {},
        }

        let mut new_status =
            CircuitStatus::determine_xtx_status::<T, BalanceOf<T>>(&local_ctx.full_side_effects);

        new_status = CircuitStatus::check_transition::<T>(
            current_status.clone(),
            new_status,
            enforce_new_status,
        )
        .map_err(|_e| Error::<T>::UpdateStateTransitionDisallowed)?;

        local_ctx.xtx.steps_cnt =
            Self::check_bump_steps(local_ctx, (current_status.clone(), new_status.clone()));
        local_ctx.xtx.status = new_status.clone();

        Ok((current_status, new_status))
    }

    fn apply(
        local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
        status_change: (CircuitStatus, CircuitStatus),
    ) -> bool {
        let (old_status, new_status) = (status_change.0, status_change.1);

        // Assume no op. for equal statuses - although this should be caught before apply by disallowed state transitions.
        //  only use case might be for delays of timeout cleaning the storage PendingXtxTimeouts.
        // Also, disallow any downgrade status from Committed.
        if old_status == new_status
            && new_status != CircuitStatus::PendingExecution
            && new_status != CircuitStatus::InBidding
            || old_status == CircuitStatus::Committed
        {
            return false
        }

        match (old_status, new_status) {
            (CircuitStatus::Requested, CircuitStatus::Reserved | CircuitStatus::PendingBidding) => {
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
                                    .calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id)
                            })
                            .map(|side_effect_hash| {
                                (
                                    cnt,
                                    side_effect_hash,
                                    XExecSignal::<T::AccountId, BlockNumberFor<T>>::generate_step_id::<
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
                                frame_system::pallet_prelude::BlockNumberFor<T>,
                                BalanceOf<T>,
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
                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::insert(
                    local_ctx.xtx_id,
                    &local_ctx.xtx.timeouts_at,
                );
                <pallet::Pallet<T> as Store>::PendingXtxBidsTimeoutsMap::insert::<
                    XExecSignalId<T>,
                    frame_system::pallet_prelude::BlockNumberFor<T>,
                >(
                    local_ctx.xtx_id,
                    T::SFXBiddingPeriod::get() + frame_system::Pallet::<T>::block_number(),
                );
                <pallet::Pallet<T> as Store>::XExecSignals::insert::<
                    XExecSignalId<T>,
                    XExecSignal<T::AccountId, BlockNumberFor<T>>,
                >(local_ctx.xtx_id, local_ctx.xtx.clone());

                true
            },
            (CircuitStatus::PendingBidding, CircuitStatus::InBidding) => {
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <pallet::Pallet<T> as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                true
            },
            (CircuitStatus::InBidding, CircuitStatus::InBidding) => {
                <pallet::Pallet<T> as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                true
            },
            (CircuitStatus::InBidding, CircuitStatus::Ready) => {
                <pallet::Pallet<T> as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });
                // Always clean temporary PendingSFXBids and TimeoutsMap after bidding
                <pallet::Pallet<T> as Store>::PendingXtxBidsTimeoutsMap::remove(local_ctx.xtx_id);

                true
            },
            (
                CircuitStatus::Reserved | CircuitStatus::PendingBidding | CircuitStatus::InBidding,
                CircuitStatus::Killed(_cause),
            ) => {
                // Clean all associated Xtx entries
                <pallet::Pallet<T> as Store>::XExecSignals::remove(local_ctx.xtx_id);
                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                <pallet::Pallet<T> as Store>::LocalXtxStates::remove(local_ctx.xtx_id);
                <pallet::Pallet<T> as Store>::FullSideEffects::remove(local_ctx.xtx_id);

                let mut fsx_mut_arr = local_ctx.full_side_effects.clone();
                for fsx_step in fsx_mut_arr.iter_mut() {
                    for fsx in fsx_step {
                        let sfx_id = fsx.calc_sfx_id::<SystemHashing<T>, T>(local_ctx.xtx_id);
                        <pallet::Pallet<T> as Store>::SFX2XTXLinksMap::remove(sfx_id);
                    }
                }
                // Always clean temporary PendingXtxBidsTimeoutsMap after bidding
                <pallet::Pallet<T> as Store>::PendingXtxBidsTimeoutsMap::remove(local_ctx.xtx_id);

                SquareUp::<T>::kill(local_ctx);

                true
            },
            (
                CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished,
                CircuitStatus::Reverted(_cause),
            ) => {
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                <pallet::Pallet<T> as Store>::DLQ::remove(local_ctx.xtx_id);

                SquareUp::<T>::finalize(local_ctx);

                true
            },
            (CircuitStatus::FinishedAllSteps, CircuitStatus::Committed) => {
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                SquareUp::<T>::commit(local_ctx);

                true
            },
            (
                CircuitStatus::Finished | CircuitStatus::Ready | CircuitStatus::PendingExecution,
                CircuitStatus::FinishedAllSteps,
            ) => {
                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                <pallet::Pallet<T> as Store>::PendingXtxTimeoutsMap::remove(local_ctx.xtx_id);
                <pallet::Pallet<T> as Store>::DLQ::remove(local_ctx.xtx_id);

                // Update set of full side effects - only makes sense for Xtx with single SFX.
                //  for the rest FSX are updated in sequence
                //  when reaching CircuitStatus::PendingExecution | CircuitStatus::Finished status.
                <pallet::Pallet<T> as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                SquareUp::<T>::finalize(local_ctx);

                true
                // <pallet::Pallet<T> as Store>::LocalXtxStates::remove::<XExecSignalId<T>>(
                //     local_ctx.xtx_id,
                // );
            },
            // ongoing execution - update FSX and Xtx status
            (
                CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished,
                CircuitStatus::Ready | CircuitStatus::PendingExecution | CircuitStatus::Finished,
            ) => {
                // Update set of full side effects assuming the new confirmed has appeared
                <pallet::Pallet<T> as Store>::FullSideEffects::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.full_side_effects.clone())
                });

                <pallet::Pallet<T> as Store>::XExecSignals::mutate(local_ctx.xtx_id, |x| {
                    *x = Some(local_ctx.xtx.clone())
                });

                true
            },
            (_, _) => false,
        }
    }
}
