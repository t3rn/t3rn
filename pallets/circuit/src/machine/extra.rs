#![cfg_attr(not(feature = "std"), no_std)]

use crate::{pallet::Error, *};

use t3rn_primitives::transfers::EscrowedBalanceOf;

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

pub fn infallible_no_post_updates<T: Config>(
    _status_change: (CircuitStatus, CircuitStatus),
    _local_ctx: &LocalXtxCtx<T>,
) {
}
