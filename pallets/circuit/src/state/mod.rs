use crate::{
    pallet::{Config, Error},
    *,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::Hasher;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::{default::Default, fmt::Debug};
use t3rn_primitives::circuit::{SpeedMode, XExecSignalId, XExecStepSideEffectId};

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

type SystemHashing<T> = <T as frame_system::Config>::Hashing;

/// Status of Circuit storage items:
/// Requested - default
/// Requested -> Validated - successfully passed the validation
/// Option<Validated -> PendingInsurance>: If there are some side effects that request insurance,
///         the status will stay in PendingInsurance until all insurance deposits are committed
/// Validated/PendingInsurance -> Ready - ready for relayers to pick up and start executing on targets
/// Ready -> PendingExecution - at least one side effect has already been confirmed, but not all of them
/// PendingExecution -> Finished - all of the side effects are confirmed, now awaiting for the decision about Revert/Commit
/// Circuit::Apply -> called internally - based on the side effects confirmations decides:
///     Ready -> Committed: All of the side effects have been successfully confirmed
///     Ready -> Reverted: Some of the side effects failed and the Xtx was reverted
#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
pub enum CircuitStatus {
    /// unvalidated xtx requested
    #[default]
    Requested,
    /// validated xtx with empty side effects - reserved for 3vm following execution
    Reserved,
    /// validated xtx pending for bidding; no bids has been posted so far
    PendingBidding,
    /// at least one bid has already been posted, still awaiting for bidding resolution
    InBidding,
    /// xtx killed on user's request or Dropped at Bidding
    Killed(Cause),
    /// all bids has been posted and xtx is awaiting confirmations of execution
    Ready,
    /// at least one valid confirmation of execution has already been accepted; xtx still in progress
    PendingExecution,
    /// xtx step successfully finished
    Finished,
    /// all of the steps has successfully finished - can still await for attestations to move to foreign consensus targets
    FinishedAllSteps,
    /// xtx reverts due timeout when confirmations haven't arrived on time,
    Reverted(Cause),
    Committed,
}

/// Kill or Revert cause
#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Cause {
    /// timeout expired with incomplete expectations: either bids or SFX confirmations
    Timeout,
    /// Attempt to kill on user's request
    IntentionalKill,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitRole {
    Relayer,
    Executor,
    Requester,
    ContractAuthor,
    Local,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum InsuranceEnact {
    Reward,
    RefundAndPunish,
    RefundBoth,
}

impl CircuitStatus {
    pub(crate) fn check_transition<T: Config>(
        previous: CircuitStatus,
        new: CircuitStatus,
        maybe_forced: Option<CircuitStatus>,
    ) -> Result<CircuitStatus, Error<T>> {
        match maybe_forced {
            None => {
                match (previous.clone(), new.clone()) {
                    (CircuitStatus::Requested, CircuitStatus::Requested) => Ok(new),
                    // todo: shouldn't be allowed to schedule empty Xtx with no SFX, but load_local_state uses a lot for 3VM setup
                    (CircuitStatus::Reserved, CircuitStatus::Reserved) => Ok(new),
                    (CircuitStatus::Reserved, CircuitStatus::PendingBidding) => Ok(new),
                    (CircuitStatus::Requested, CircuitStatus::Reserved) => Ok(new),

                    // success flow
                    (CircuitStatus::Requested, CircuitStatus::PendingBidding) => Ok(new),
                    (CircuitStatus::Requested, CircuitStatus::InBidding) => Ok(new),
                    (CircuitStatus::PendingBidding, CircuitStatus::InBidding) => Ok(new),
                    (CircuitStatus::InBidding, CircuitStatus::InBidding) => Ok(new),
                    (CircuitStatus::InBidding, CircuitStatus::Ready) => Ok(new),
                    (CircuitStatus::PendingBidding, CircuitStatus::Ready) => Ok(new),
                    (CircuitStatus::Ready, CircuitStatus::PendingExecution) => Ok(new),
                    (CircuitStatus::Ready, CircuitStatus::FinishedAllSteps) => Ok(new),
                    (CircuitStatus::PendingExecution, CircuitStatus::PendingExecution) => Ok(new),
                    (CircuitStatus::PendingExecution, CircuitStatus::Finished) => Ok(new),
                    (CircuitStatus::PendingExecution, CircuitStatus::FinishedAllSteps) => Ok(new),
                    (CircuitStatus::PendingExecution, CircuitStatus::Committed) => Ok(new),
                    // next steps transitions
                    (CircuitStatus::Finished, CircuitStatus::PendingExecution) => Ok(new),
                    (CircuitStatus::Finished, CircuitStatus::Ready) => Ok(new),
                    (CircuitStatus::Finished, CircuitStatus::FinishedAllSteps) => Ok(new),

                    (CircuitStatus::FinishedAllSteps, CircuitStatus::Committed) => Ok(new),
                    (_, _) => {
                        log::error!(
                            "check_transition::UpdateStateTransitionDisallowedInvalid {:?} -> {:?}",
                            previous,
                            new
                        );
                        Err(Error::<T>::UpdateStateTransitionDisallowed)
                    },
                }
            },
            Some(forced) => {
                if forced == new {
                    return Ok(new)
                }
                // Only cases we allow forced transitions are:
                // revert by protocol,
                // kill either by protocol or user's attempt,
                // from ready to pending execution for XBI's execution
                // from pending bidding to in bidding for posted bid
                // from reserved to pending execution
                match forced.clone() {
                    CircuitStatus::Killed(cause) => match cause {
                        Cause::IntentionalKill =>
                            if new <= CircuitStatus::PendingBidding {
                                Ok(forced)
                            } else {
                                Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                            },
                        Cause::Timeout =>
                            if new <= CircuitStatus::InBidding {
                                Ok(forced)
                            } else {
                                Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                            },
                    },
                    CircuitStatus::Reverted(cause) =>
                        if new < CircuitStatus::Ready {
                            Ok(CircuitStatus::Killed(cause))
                        } else if new < CircuitStatus::FinishedAllSteps {
                            Ok(forced)
                        } else {
                            // Revert assumed infallible - fallback to new state which results in no op. in apply.
                            Ok(new)
                        },
                    CircuitStatus::Ready =>
                        if previous == CircuitStatus::InBidding {
                            Ok(forced)
                        } else {
                            Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                        },
                    CircuitStatus::InBidding =>
                        if new == CircuitStatus::PendingBidding || new == CircuitStatus::InBidding {
                            Ok(forced)
                        } else {
                            Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                        },
                    CircuitStatus::PendingExecution =>
                        if new == CircuitStatus::Ready {
                            Ok(forced)
                        } else {
                            Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                        },
                    CircuitStatus::Reserved =>
                        if new <= CircuitStatus::Reserved {
                            Ok(forced)
                        } else {
                            Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                        },
                    CircuitStatus::Committed =>
                        if new == CircuitStatus::FinishedAllSteps {
                            Ok(CircuitStatus::Committed)
                        } else {
                            Err(Error::<T>::UpdateForcedStateTransitionDisallowed)
                        },
                    _ => Err(Error::<T>::UpdateForcedStateTransitionDisallowed),
                }
            },
        }
    }

    fn determine_fsx_bidding_status<T: Config>(
        fsx: FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    ) -> CircuitStatus {
        if let Some(_bid) = fsx.best_bid {
            CircuitStatus::InBidding
        } else {
            CircuitStatus::PendingBidding
        }
    }

    /// Check if all FSX have the bidding companion.
    /// Additionally,
    /// if SFX::Optimistic check if the optional insurance and bonded_deposit fields are present
    /// if SFX::Escrow check if the optional insurance and bonded_deposit are set to None
    pub fn determine_bidding_status<T: Config>(
        fsx_step: &[FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>],
    ) -> CircuitStatus {
        let mut lowest_bidding_status = CircuitStatus::InBidding;
        let mut highest_bidding_status = CircuitStatus::PendingBidding;

        for fsx in fsx_step.iter() {
            let current_bidding_status = Self::determine_fsx_bidding_status::<T>(fsx.clone());
            if current_bidding_status == CircuitStatus::PendingBidding {
                lowest_bidding_status = CircuitStatus::PendingBidding;
            } else {
                highest_bidding_status = CircuitStatus::InBidding;
            }
        }
        if lowest_bidding_status == CircuitStatus::InBidding {
            // Check if all FSX have already executors assigned to them as a precondition for the CircuitStatus to be ready.
            if fsx_step
                .iter()
                .all(|fsx| fsx.input.enforce_executor.is_some())
            {
                CircuitStatus::Ready
            } else {
                CircuitStatus::InBidding
            }
        } else if highest_bidding_status == CircuitStatus::PendingBidding {
            CircuitStatus::PendingBidding
        } else {
            CircuitStatus::InBidding
        }
    }

    pub fn determine_execution_status<T: Config>(
        fsx_step: &[FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>],
    ) -> CircuitStatus {
        let mut lowest_execution_status = CircuitStatus::Finished;
        let mut highest_execution_status = CircuitStatus::Ready;

        for fsx in fsx_step.iter() {
            if fsx.confirmed.is_some() {
                highest_execution_status = CircuitStatus::Finished;
            } else {
                lowest_execution_status = CircuitStatus::Ready;
            }
        }
        if lowest_execution_status == CircuitStatus::Finished {
            CircuitStatus::Finished
        } else if highest_execution_status == CircuitStatus::Ready {
            CircuitStatus::Ready
        } else {
            CircuitStatus::PendingExecution
        }
    }

    /// Based solely on full steps + insurance deposits determine the execution status.
    /// Start with checking the criteria from the earliest status to latest
    pub fn determine_step_status<T: Config>(
        step: &[FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>],
    ) -> CircuitStatus {
        if step.is_empty() {
            return CircuitStatus::Finished
        }
        let determined_bidding_status = Self::determine_bidding_status::<T>(step);
        if determined_bidding_status < CircuitStatus::Ready {
            return determined_bidding_status
        }
        Self::determine_execution_status::<T>(step)
    }

    pub fn determine_xtx_status<T: Config>(
        steps: &[Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>],
    ) -> CircuitStatus {
        let mut lowest_determined_status = CircuitStatus::Requested;

        // If all of the steps are empty assume CircuitStatus::Reserved status
        if steps.iter().all(|step| step.is_empty()) {
            return CircuitStatus::Reserved
        }
        for step in steps.iter() {
            let current_step_status = Self::determine_step_status::<T>(step);
            if current_step_status > lowest_determined_status {
                lowest_determined_status = current_step_status.clone();
            }
            // Xtx status is reflected with the lowest status of unresolved Step -
            //  break the loop on the first unresolved step
            if lowest_determined_status < CircuitStatus::Finished {
                return current_step_status
            }
        }
        CircuitStatus::FinishedAllSteps
    }
}

pub struct LocalXtxCtx<T: Config> {
    pub local_state: LocalState,
    pub xtx_id: XExecSignalId<T>,
    pub xtx: XExecSignal<T::AccountId, T::BlockNumber>,
    pub full_side_effects: Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
}

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct XExecSignal<AccountId, BlockNumber> {
    /// The owner of the bid
    pub requester: AccountId,

    /// The owner of the bid
    pub requester_nonce: u32,

    /// Expiry timeout
    pub timeouts_at: BlockNumber,

    /// Schedule execution of steps in the future intervals
    pub delay_steps_at: Option<Vec<BlockNumber>>,

    /// Has returned status already and what
    pub status: CircuitStatus,

    /// Speed of confirmation
    pub speed_mode: SpeedMode,

    /// Has returned status already and what
    pub steps_cnt: (u32, u32),
}

impl<
        AccountId: Encode + Clone + Debug,
        BlockNumber: Ord + Copy + Zero + Encode + Clone + Debug,
    > XExecSignal<AccountId, BlockNumber>
{
    pub fn new(
        // Requester of xtx
        requester: &AccountId,
        // Requester' nonce of xtx
        requester_nonce: u32,
        // Expiry timeout
        timeouts_at: BlockNumber,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<BlockNumber>>,
        // Speed of confirmation
        speed_mode: SpeedMode,
        // Current steps count
        steps_cnt: (u32, u32),
    ) -> Self {
        XExecSignal {
            requester: requester.clone(),
            requester_nonce,
            timeouts_at,
            delay_steps_at,
            status: Default::default(),
            speed_mode,
            steps_cnt,
        }
    }

    // xtx_id is generated by hashing requester + requester_nonce. This ensures it will always be unique
    pub fn generate_id<T: frame_system::Config>(&self) -> XExecSignalId<T> {
        let mut requester_and_nonce = self.requester.encode();
        requester_and_nonce.extend_from_slice(&self.requester_nonce.to_be_bytes());
        SystemHashing::<T>::hash(requester_and_nonce.as_ref())
    }

    pub fn generate_step_id<T: frame_system::Config>(
        xtx_id: XExecSignalId<T>,
        n_step: usize,
    ) -> XExecStepSideEffectId<T> {
        let mut xtx_id_buf = xtx_id.encode();
        xtx_id_buf.append(&mut (n_step as u32).encode());
        SystemHashing::<T>::hash(xtx_id_buf.as_ref())
    }

    pub fn set_speed_mode(&mut self, speed_mode: SpeedMode) {
        self.speed_mode = speed_mode;
    }

    pub fn setup_fresh<T: frame_system::Config>(
        // Requester of xtx
        requester: &T::AccountId,
        // Expiry timeout
        timeouts_at: T::BlockNumber,
        // Speed of confirmation
        speed_mode: SpeedMode,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<T::BlockNumber>>,
    ) -> (XExecSignalId<T>, XExecSignal<T::AccountId, T::BlockNumber>) {
        let requester_nonce = Decode::decode(
            &mut &frame_system::Pallet::<T>::account_nonce(requester).encode()[..],
        )
        .expect(
            "System::Index decoding try to u32 should always succeed since set in Runtime Config",
        );
        let signal = XExecSignal::new(
            requester,
            requester_nonce,
            timeouts_at,
            delay_steps_at,
            speed_mode,
            (0, 0),
        );
        let id = signal.generate_id::<T>();
        (id, signal)
    }
}
