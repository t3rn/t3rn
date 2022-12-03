use crate::{
    pallet::{Config, Error},
    *,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::Hasher;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::{default::Default, fmt::Debug};
use t3rn_primitives::{
    circuit::{XExecSignalId, XExecStepSideEffectId},
    transfers::EscrowedBalanceOf,
};

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
#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitStatus {
    Requested,
    PendingBidding,
    Ready,
    PendingExecution,
    Finished,
    FinishedAllSteps,
    Committed,
    DroppedAtBidding,
    Reverted,
    RevertTimedOut,
    RevertKill,
    RevertMisbehaviour,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitKillCause {
    DroppedAtBidding,
    RevertTimedOut,
    // Attempt to kill on user's request
    RevertKill,
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
    fn determine_fsx_bidding_status<T: Config>(
        fsx: FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
    ) -> Result<CircuitStatus, Error<T>> {
        if let Some(_bid) = fsx.best_bid {
            Ok(CircuitStatus::Ready)
        } else {
            Ok(CircuitStatus::PendingBidding)
        }
    }

    /// Check if all FSX have the bidding companion.
    /// Additionally,
    /// if SFX::Optimistic check if the optional insurance and bonded_deposit fields are present
    /// if SFX::Escrow check if the optional insurance and bonded_deposit are set to None
    pub fn determine_bidding_status<T: Config>(
        fsx_step: &[FullSideEffect<
            T::AccountId,
            T::BlockNumber,
            EscrowedBalanceOf<T, T::Escrowed>,
        >],
    ) -> Result<CircuitStatus, Error<T>> {
        for fsx in fsx_step.iter() {
            if Self::determine_fsx_bidding_status::<T>(fsx.clone())?
                == CircuitStatus::PendingBidding
            {
                return Ok(CircuitStatus::PendingBidding)
            }
        }
        Ok(CircuitStatus::Ready)
    }

    /// Based solely on full steps + insurance deposits determine the execution status.
    /// Start with checking the criteria from the earliest status to latest
    pub fn determine_step_status<T: Config>(
        step: &[FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>],
    ) -> Result<CircuitStatus, Error<T>> {
        // Those are determined post - ready
        let mut highest_post_ready_determined_status = CircuitStatus::Ready;
        let mut lowest_post_ready_determined_status = CircuitStatus::Finished;

        let current_determined_status = Self::determine_bidding_status::<T>(step)?;

        for (_step_cnt, full_side_effect) in step.iter().enumerate() {
            if current_determined_status == CircuitStatus::PendingBidding
                && highest_post_ready_determined_status > CircuitStatus::Ready
            {
                // If we are here it means that the side effect has requested for insurance that is still pending
                //  but at the same time some of the previous side effects already has been confirmed.
                // This should never happen and the refund for users should be handled
                //  with the same time punishing relayers responsible for too early execution
                return Err(Error::<T>::DeterminedForbiddenXtxStatus)
            }

            if current_determined_status != CircuitStatus::Ready {
                return Ok(current_determined_status)
            }
            // Checking further only if CircuitStatus::Ready after this point
            if full_side_effect.confirmed.is_some() {
                highest_post_ready_determined_status = CircuitStatus::Finished
            } else {
                lowest_post_ready_determined_status = CircuitStatus::PendingExecution
            }
        }

        // Find CircuitStatus::min(lowest_determined, highest_determined)
        let lowest_determined =
            if highest_post_ready_determined_status >= lowest_post_ready_determined_status {
                // Either CircuitStatus::Finished if never found a side effect with CircuitStatus::PendingExecution
                //  Or CircuitStatus::PendingExecution otherwise
                lowest_post_ready_determined_status
            } else {
                // Either CircuitStatus::Finished if never found a side effect with CircuitStatus::PendingExecution
                //  Or CircuitStatus::Ready otherwise if None of the side effects are confirmed yet
                highest_post_ready_determined_status
            };

        Ok(lowest_determined)
    }

    pub fn determine_xtx_status<T: Config>(
        steps: &[Vec<
            FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>,
        >],
    ) -> Result<CircuitStatus, Error<T>> {
        let mut lowest_determined_status = CircuitStatus::Requested;

        for step in steps.iter() {
            let current_step_status = Self::determine_step_status::<T>(step)?;
            log::debug!(
                "Determine determine_xtx_status in loop Before -- {:?}",
                current_step_status.clone()
            );
            if current_step_status > lowest_determined_status {
                lowest_determined_status = current_step_status;
            }
            // Xtx status is reflected with the lowest status of unresolved Step -
            //  break the loop on the first unresolved step
            if lowest_determined_status < CircuitStatus::Finished {
                break
            }
        }
        Ok(lowest_determined_status)
    }
}

pub struct LocalXtxCtx<T: Config> {
    pub local_state: LocalState,
    pub use_protocol: UniversalSideEffectsProtocol,
    pub xtx_id: XExecSignalId<T>,
    pub xtx: XExecSignal<T::AccountId, T::BlockNumber>,
    pub full_side_effects:
        Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, EscrowedBalanceOf<T, T::Escrowed>>>>,
}

impl Default for CircuitStatus {
    fn default() -> Self {
        CircuitStatus::Requested
    }
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
        // Current steps count
        steps_cnt: (u32, u32),
    ) -> Self {
        XExecSignal {
            requester: requester.clone(),
            requester_nonce,
            timeouts_at,
            delay_steps_at,
            status: Default::default(),
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

    pub fn setup_fresh<T: frame_system::Config>(
        // Requester of xtx
        requester: &T::AccountId,
        // Expiry timeout
        timeouts_at: T::BlockNumber,
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
            (0, 0),
        );
        let id = signal.generate_id::<T>();
        (id, signal)
    }
}
