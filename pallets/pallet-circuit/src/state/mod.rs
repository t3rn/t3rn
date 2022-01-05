use crate::*;
use codec::{Decode, Encode};
use sp_core::Hasher;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

type SystemHashing<T> = <T as frame_system::Config>::Hashing;
pub type XExecSignalId<T> = <T as frame_system::Config>::Hash;

// use t3rn_primitives::side_effect::*;
// use t3rn_primitives::volatile::{LocalState, Volatile};
use scale_info::TypeInfo;

/// Status of Circuit storage items:
/// Requested - default
/// Requested -> Validated - successfully passed the validation
/// Option<Validated -> PendingInsurance>: If there are some side effects that request insurance,
///         the status will stay in PendingInsurance until all insurance deposits are committed
/// Validated/PendingInsurance -> Ready - ready for relayers to pick up and start executing on targets
/// Ready -> PendingExecution - at least one side effect has already been confirmed, but not all of them
/// Circuit::Apply -> called internally - based on the side effects confirmations decides:
///     Ready -> Committed: All of the side effects have been successfully confirmed
///     Ready -> Reverted: Some of the side effects failed and the Xtx was reverted
#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitStatus {
    Requested,
    PendingInsurance,
    Bonded,
    Ready,
    PendingExecution,
    Finished,
    Committed,
    Reverted,
    RevertedTimedOut,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitRole {
    Relayer,
    Requester,
    Local,
}

impl CircuitStatus {
    fn determine_insurance_status<T: Config>(
        side_effect_id: SideEffectId<T>,
        insurance_deposits: &Vec<(
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        )>,
    ) -> CircuitStatus {
        return if let Some((_id, insurance_request)) = insurance_deposits
            .iter()
            .find(|(id, _)| *id == side_effect_id)
        {
            if let Some(_) = insurance_request.bonded_relayer {
                CircuitStatus::Bonded
            } else {
                CircuitStatus::PendingInsurance
            }
        } else {
            CircuitStatus::Ready
        };
    }

    /// Based solely on full steps + insurance deposits determine the execution status.
    /// Start with checking the criteria from the earliest status to latest
    pub fn determine_step_status<T: Config>(
        step: &Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>,
        insurance_deposits: &Vec<(
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        )>,
    ) -> Result<CircuitStatus, Error<T>> {
        // Those are determined post - ready
        let mut highest_post_ready_determined_status = CircuitStatus::Ready;
        let mut lowest_post_ready_determined_status = CircuitStatus::Finished;

        for (_i, full_side_effect) in step.iter().enumerate() {
            let current_id = full_side_effect.input.generate_id::<SystemHashing<T>>();
            let current_determined_status =
                Self::determine_insurance_status::<T>(current_id, insurance_deposits);
            if current_determined_status == CircuitStatus::PendingInsurance
                && highest_post_ready_determined_status > CircuitStatus::Ready
            {
                // If we are here it means that the side effect has requested for insurance that is still pending
                //  but at the same time some of the previous side effects already has been confirmed.
                // This should never happen and the refund for users should be handled
                //  with the same time punishing relayers responsible for too early execution
                return Err(Error::<T>::DeterminedForbiddenXtxStatus);
            }

            if current_determined_status != CircuitStatus::Ready {
                return Ok(current_determined_status);
            }
            // Checking further only if CircuitStatus::Ready after this point
            if let Some(_) = full_side_effect.confirmed {
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
        steps: &Vec<Vec<FullSideEffect<T::AccountId, T::BlockNumber, BalanceOf<T>>>>,
        insurance_deposits: &Vec<(
            SideEffectId<T>,
            InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        )>,
    ) -> Result<CircuitStatus, Error<T>> {
        let mut lowest_determined_status = CircuitStatus::Requested;

        for step in steps.iter() {
            let current_step_status = Self::determine_step_status::<T>(&step, insurance_deposits)?;
            if current_step_status > lowest_determined_status {
                lowest_determined_status = current_step_status;
            }
        }
        Ok(lowest_determined_status)
    }
}

pub struct LocalXtxCtx<T: Config> {
    pub local_state: LocalState,
    pub use_protocol: UniversalSideEffectsProtocol,
    pub xtx_id: XExecSignalId<T>,
    pub xtx: XExecSignal<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    pub insurance_deposits: Vec<(
        SideEffectId<T>,
        InsuranceDeposit<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    )>,
}

impl Default for CircuitStatus {
    fn default() -> Self {
        CircuitStatus::Requested
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct InsuranceDeposit<AccountId, BlockNumber, BalanceOf> {
    pub insurance: BalanceOf,
    pub reward: BalanceOf,
    pub requester: AccountId,
    pub bonded_relayer: Option<AccountId>,
    pub status: CircuitStatus,
    pub requested_at: BlockNumber,
}

impl<
        AccountId: Encode + Clone + Debug,
        BlockNumber: Ord + Copy + Zero + Encode + Clone + Debug,
        BalanceOf: Copy + Zero + Encode + Decode + Clone + Debug,
    > InsuranceDeposit<AccountId, BlockNumber, BalanceOf>
{
    pub fn new(
        insurance: BalanceOf,
        reward: BalanceOf,
        requester: AccountId,
        requested_at: BlockNumber,
    ) -> Self {
        InsuranceDeposit {
            insurance,
            reward,
            requester,
            bonded_relayer: None,
            status: CircuitStatus::Requested,
            requested_at,
        }
    }
}

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct XExecSignal<AccountId, BlockNumber, BalanceOf> {
    // todo: Add missing DFDs
    // pub contracts_dfd: InterExecSchedule -> ContractsDFD
    // pub side_effects_dfd: SideEffectsDFD
    // pub gateways_dfd: GatewaysDFD
    /// The owner of the bid
    pub requester: AccountId,

    /// Expiry timeout
    pub timeouts_at: Option<BlockNumber>,

    /// Schedule execution of steps in the future intervals
    pub delay_steps_at: Option<Vec<BlockNumber>>,

    /// Has returned status already and what
    pub status: CircuitStatus,

    /// Total reward
    pub total_reward: Option<BalanceOf>,
}

impl<
        AccountId: Encode + Clone + Debug,
        BlockNumber: Ord + Copy + Zero + Encode + Clone + Debug,
        BalanceOf: Copy + Zero + Encode + Decode + Clone + Debug,
    > XExecSignal<AccountId, BlockNumber, BalanceOf>
{
    pub fn new(
        // Requester of xtx
        requester: &AccountId,
        // Expiry timeout
        timeouts_at: Option<BlockNumber>,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<BlockNumber>>,
        // Total reward
        total_reward: Option<BalanceOf>,
    ) -> Self {
        XExecSignal {
            requester: requester.clone(),
            timeouts_at,
            delay_steps_at,
            status: Default::default(),
            total_reward,
        }
    }

    pub fn generate_id<T: frame_system::Config>(&self) -> XExecSignalId<T> {
        SystemHashing::<T>::hash(Encode::encode(self).as_ref())
    }

    pub fn setup_fresh<T: frame_system::Config>(
        // Requester of xtx
        requester: &T::AccountId,
        // Expiry timeout
        timeouts_at: Option<T::BlockNumber>,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<T::BlockNumber>>,
        // Total reward
        total_reward: Option<BalanceOf>,
    ) -> (
        XExecSignalId<T>,
        XExecSignal<T::AccountId, T::BlockNumber, BalanceOf>,
    ) {
        let signal = XExecSignal::new(requester, timeouts_at, delay_steps_at, total_reward);
        let id = signal.generate_id::<T>();
        (id, signal)
    }
}
