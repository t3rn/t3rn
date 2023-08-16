use crate::{pallet::Error, *};
use frame_support::ensure;
use frame_system::pallet_prelude::BlockNumberFor;
pub fn no_mangle<T: Config>(
    _current_fsx: &mut Vec<
        FullSideEffect<<T as frame_system::Config>::AccountId, BlockNumberFor<T>, BalanceOf<T>>,
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
    _local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
) -> Result<(), Error<T>> {
    Ok(())
}

pub fn infallible_no_post_updates<T: Config>(
    _status_change: (CircuitStatus, CircuitStatus),
    _local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
) {
}

pub fn validate_fsx_against_xtx<T: Config>(
    local_ctx: &LocalXtxCtx<T, BalanceOf<T>>,
) -> Result<(), Error<T>> {
    for fsx_step in local_ctx.full_side_effects.iter() {
        for fsx in fsx_step.iter() {
            if local_ctx.xtx.status >= CircuitStatus::Ready {
                ensure!(
                    fsx.input.enforce_executor.is_some(),
                    Error::<T>::InvalidFTXStateUnassignedExecutorForReadySFX
                );
                match &fsx.best_bid {
                    Some(bid) => ensure!(
                        Some(bid.executor.clone()) == fsx.input.enforce_executor,
                        Error::<T>::InvalidFTXStateIncorrectExecutorForReadySFX
                    ),
                    None => return Err(Error::<T>::InvalidFTXStateEmptyBidForReadyXtx),
                }
            }
            if local_ctx.xtx.status >= CircuitStatus::Finished
                && local_ctx.xtx.status != CircuitStatus::Reverted(Cause::Timeout)
                && local_ctx.xtx.status != CircuitStatus::Reverted(Cause::IntentionalKill)
            {
                ensure!(
                    fsx.confirmed.is_some(),
                    Error::<T>::InvalidFTXStateEmptyConfirmationForFinishedXtx
                );
            }
        }
    }

    Ok(())
}
