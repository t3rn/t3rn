use crate::{
    circuit::{XExecSignalId, XExecStepSideEffectId},
    xtx::LocalState,
    SpeedMode,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::DispatchError;
use frame_system::{pallet_prelude::BlockNumberFor, Config};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{crypto::AccountId32, hexdisplay::AsBytesRef, Hasher, H160, U256};
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::{convert::TryInto, default::Default, fmt::Debug, prelude::*};
use t3rn_types::sfx::TargetId;
pub use t3rn_types::sfx::{FullSideEffect, SecurityLvl, SideEffect};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
pub struct VacuumEVMOrder {
    pub destination: TargetId,

    pub asset: u32,

    pub amount: U256,

    pub reward_asset: H160,

    pub max_reward: U256,

    pub insurance: U256,

    pub target_account: AccountId32,
}

impl VacuumEVMOrder {
    pub fn new(
        destination: TargetId,
        asset: u32,
        amount: U256,
        reward_asset: H160,
        max_reward: U256,
        insurance: U256,
        target_account: AccountId32,
    ) -> Self {
        VacuumEVMOrder {
            destination,
            asset,
            amount,
            reward_asset,
            max_reward,
            insurance,
            target_account,
        }
    }

    pub fn from_rlp_encoded_packed(encoded_slice: &[u8]) -> Result<Self, DispatchError> {
        // Take the first 4 bytes and convert them to a TargetId
        let destination: TargetId = encoded_slice[0..4]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert destination"))?;

        // Take the next 4 bytes and convert them to a u32
        let asset: u32 = u32::from_be_bytes(
            encoded_slice[4..8]
                .try_into()
                .map_err(|_| DispatchError::Other("Failed to convert asset"))?,
        );

        // Take the next 32 bytes and convert them to a Vec<u8>
        let target_account: AccountId32 = AccountId32::decode(&mut &encoded_slice[8..40])
            .map_err(|_| DispatchError::Other("Failed to decode AccountId32"))?;

        // Take the next 32 bytes and convert them to a Balance
        let amount: U256 = encoded_slice[40..72]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert amount"))?;

        // Take the next 4 bytes and convert them to a u32
        let reward_asset: H160 = H160::decode(&mut &encoded_slice[72..92])
            .map_err(|_| DispatchError::Other("Failed to decode reward_asset"))?;

        // Take the next 32 bytes and convert them to a Balance
        let insurance: U256 = encoded_slice[92..124]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert insurance:"))?;

        // Take the next 32 bytes and convert them to a Balance
        let max_reward: U256 = encoded_slice[124..156]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert max_reward"))?;

        Ok(VacuumEVMOrder {
            destination,
            asset,
            amount,
            reward_asset,
            max_reward,
            insurance,
            target_account,
        })
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct VacuumEVMTeleportOrder {
    pub gateway_id: [u8; 4],

    pub order_proof: Vec<u8>,
}

impl VacuumEVMTeleportOrder {
    pub fn new(gateway_id: [u8; 4], order_proof: Vec<u8>) -> Self {
        VacuumEVMTeleportOrder {
            gateway_id,
            order_proof,
        }
    }

    pub fn from_rlp(encoded_slice: &[u8]) -> Result<Self, DispatchError> {
        let mut gateway_id: [u8; 4] = Default::default();
        let mut order_proof = vec![];

        // Assume at least 4 bytes for the gateway_id
        if encoded_slice.len() < 4 {
            return Err(DispatchError::Other("Invalid encoded slice"))
        }

        // Take the first 4 bytes and convert them to a TargetId
        gateway_id.copy_from_slice(&encoded_slice[0..4]);

        let encoded_slice = &encoded_slice[4..];

        // Skip the first 4 bytes

        let mut index = 0;
        let mut proof_vec = vec![&mut order_proof];

        for proof in proof_vec.iter_mut() {
            let proof_len = encoded_slice[index] as usize;
            index += 1;
            proof.extend_from_slice(&encoded_slice[index..index + proof_len]);
            index += proof_len;
        }

        Ok(VacuumEVMTeleportOrder {
            gateway_id,
            order_proof,
        })
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
pub struct VacuumEVM3DOrder {
    pub destination: TargetId,

    pub asset: u32,

    pub amount: U256,

    pub reward_asset: H160,

    pub max_reward: U256,

    pub insurance: U256,

    pub target_account: AccountId32,

    pub nonce: u32,
}

impl VacuumEVM3DOrder {
    pub fn new(
        destination: TargetId,
        asset: u32,
        amount: U256,
        reward_asset: H160,
        max_reward: U256,
        insurance: U256,
        target_account: AccountId32,
        nonce: u32,
    ) -> Self {
        VacuumEVM3DOrder {
            destination,
            asset,
            amount,
            reward_asset,
            max_reward,
            insurance,
            target_account,
            nonce,
        }
    }

    pub fn from_rlp_encoded_packed(encoded_slice: &[u8]) -> Result<Self, DispatchError> {
        // Take the first 4 bytes and convert them to a TargetId
        let destination: TargetId = encoded_slice[0..4]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert destination"))?;

        // Take the next 4 bytes and convert them to a u32
        let asset: u32 = u32::from_be_bytes(
            encoded_slice[4..8]
                .try_into()
                .map_err(|_| DispatchError::Other("Failed to convert asset"))?,
        );

        // Take the next 32 bytes and convert them to a Vec<u8>
        let target_account: AccountId32 = AccountId32::decode(&mut &encoded_slice[8..40])
            .map_err(|_| DispatchError::Other("Failed to decode AccountId32"))?;

        // Take the next 32 bytes and convert them to a Balance
        let amount: U256 = encoded_slice[40..72]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert amount"))?;

        // Take the next 4 bytes and convert them to a u32
        let reward_asset: H160 = H160::decode(&mut &encoded_slice[72..92])
            .map_err(|_| DispatchError::Other("Failed to decode reward_asset"))?;

        // Take the next 32 bytes and convert them to a Balance
        let insurance: U256 = encoded_slice[92..124]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert insurance:"))?;

        // Take the next 32 bytes and convert them to a Balance
        let max_reward: U256 = encoded_slice[124..156]
            .try_into()
            .map_err(|_| DispatchError::Other("Failed to convert max_reward"))?;

        let nonce: u32 = u32::from_be_bytes(
            encoded_slice[156..160]
                .try_into()
                .map_err(|_| DispatchError::Other("Failed to convert nonce"))?,
        );

        Ok(VacuumEVM3DOrder {
            destination,
            asset,
            amount,
            reward_asset,
            max_reward,
            insurance,
            target_account,
            nonce,
        })
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct VacuumEVMProof {
    pub gateway_id: [u8; 4],

    pub order_proof: Vec<u8>,

    pub bid_proof: Vec<u8>,

    pub execution_proof: Vec<u8>,

    pub attestation_proof: Vec<u8>,
}

impl VacuumEVMProof {
    pub fn new(
        gateway_id: [u8; 4],
        order_proof: Vec<u8>,
        bid_proof: Vec<u8>,
        execution_proof: Vec<u8>,
        attestation_proof: Vec<u8>,
    ) -> Self {
        VacuumEVMProof {
            gateway_id,
            order_proof,
            bid_proof,
            execution_proof,
            attestation_proof,
        }
    }

    pub fn from_rlp(encoded_slice: &[u8]) -> Result<Self, DispatchError> {
        let mut gateway_id: [u8; 4] = Default::default();
        let mut order_proof = vec![];
        let mut bid_proof = vec![];
        let mut execution_proof = vec![];
        let mut attestation_proof = vec![];

        // Assume at least 4 bytes for the gateway_id
        if encoded_slice.len() < 4 {
            return Err(DispatchError::Other("Invalid encoded slice"))
        }

        // Take the first 4 bytes and convert them to a TargetId
        gateway_id.copy_from_slice(&encoded_slice[0..4]);

        let encoded_slice = &encoded_slice[4..];

        // Skip the first 4 bytes

        let mut index = 0;
        let mut proof_vec = vec![
            &mut order_proof,
            &mut bid_proof,
            &mut execution_proof,
            &mut attestation_proof,
        ];

        for proof in proof_vec.iter_mut() {
            let proof_len = encoded_slice[index] as usize;
            index += 1;
            proof.extend_from_slice(&encoded_slice[index..index + proof_len]);
            index += proof_len;
        }

        Ok(VacuumEVMProof {
            gateway_id,
            order_proof,
            bid_proof,
            execution_proof,
            attestation_proof,
        })
    }
}

type SystemHashing<T> = <T as Config>::Hashing;

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
    pub fn check_transition<T: Config>(
        previous: CircuitStatus,
        new: CircuitStatus,
        maybe_forced: Option<CircuitStatus>,
    ) -> Result<CircuitStatus, DispatchError> {
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
                        Err(DispatchError::Other("UpdateStateTransitionDisallowed"))
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
                                Err(DispatchError::Other(
                                    "UpdateForcedStateTransitionDisallowed",
                                ))
                            },
                        Cause::Timeout =>
                            if new <= CircuitStatus::InBidding {
                                Ok(forced)
                            } else {
                                Err(DispatchError::Other(
                                    "UpdateForcedStateTransitionDisallowed",
                                ))
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
                            Err(DispatchError::Other(
                                "UpdateForcedStateTransitionDisallowed",
                            ))
                        },
                    CircuitStatus::InBidding =>
                        if new == CircuitStatus::PendingBidding || new == CircuitStatus::InBidding {
                            Ok(forced)
                        } else {
                            Err(DispatchError::Other(
                                "UpdateForcedStateTransitionDisallowed",
                            ))
                        },
                    CircuitStatus::PendingExecution =>
                        if new == CircuitStatus::Ready {
                            Ok(forced)
                        } else {
                            Err(DispatchError::Other(
                                "UpdateForcedStateTransitionDisallowed",
                            ))
                        },
                    CircuitStatus::Reserved =>
                        if new <= CircuitStatus::Reserved {
                            Ok(forced)
                        } else {
                            Err(DispatchError::Other(
                                "UpdateForcedStateTransitionDisallowed",
                            ))
                        },
                    CircuitStatus::Committed =>
                        if new == CircuitStatus::FinishedAllSteps {
                            Ok(CircuitStatus::Committed)
                        } else {
                            Err(DispatchError::Other(
                                "UpdateForcedStateTransitionDisallowed",
                            ))
                        },
                    _ => Err(DispatchError::Other(
                        "UpdateForcedStateTransitionDisallowed",
                    )),
                }
            },
        }
    }

    fn determine_fsx_bidding_status<T: Config, Balance: Clone>(
        fsx: FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>,
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
    pub fn determine_bidding_status<T: Config, Balance: Clone>(
        fsx_step: &[FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>],
    ) -> CircuitStatus {
        let mut lowest_bidding_status = CircuitStatus::InBidding;
        let mut highest_bidding_status = CircuitStatus::PendingBidding;

        for fsx in fsx_step.iter() {
            let current_bidding_status =
                Self::determine_fsx_bidding_status::<T, Balance>(fsx.clone());
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

    pub fn determine_execution_status<T: Config, Balance: Clone>(
        fsx_step: &[FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>],
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
    pub fn determine_step_status<T: Config, Balance: Clone>(
        step: &[FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>],
    ) -> CircuitStatus {
        if step.is_empty() {
            return CircuitStatus::Finished
        }
        let determined_bidding_status = Self::determine_bidding_status::<T, Balance>(step);
        if determined_bidding_status < CircuitStatus::Ready {
            return determined_bidding_status
        }
        Self::determine_execution_status::<T, Balance>(step)
    }

    pub fn determine_xtx_status<T: Config, Balance: Clone>(
        steps: &[Vec<FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>>],
    ) -> CircuitStatus {
        let mut lowest_determined_status = CircuitStatus::Requested;

        // If all of the steps are empty assume CircuitStatus::Reserved status
        if steps.iter().all(|step| step.is_empty()) {
            return CircuitStatus::Reserved
        }
        for step in steps.iter() {
            let current_step_status = Self::determine_step_status::<T, Balance>(step);
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

pub struct LocalXtxCtx<T: Config, Balance: Clone> {
    pub local_state: LocalState,
    pub xtx_id: XExecSignalId<T>,
    pub xtx: XExecSignal<T::AccountId, BlockNumberFor<T>>,
    pub full_side_effects: Vec<Vec<FullSideEffect<T::AccountId, BlockNumberFor<T>, Balance>>>,
}

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct XExecSignal<AccountId, BlockNumber> {
    /// The owner of the bid
    pub requester: AccountId,

    /// The owner of the bid
    pub requester_nonce: u32,

    /// Expiry timeout
    pub timeouts_at: AdaptiveTimeout<BlockNumber, TargetId>,

    /// Speed of confirmation
    pub speed_mode: SpeedMode,

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
        timeouts_at: AdaptiveTimeout<BlockNumber, TargetId>,
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
    pub fn generate_id<T: Config, Hasher: sp_core::Hasher>(&self) -> XExecSignalId<T> {
        let mut requester_on_32b_as_vec = self.requester.encode();

        let nonce_as_4b_word: [u8; 4] = self.requester_nonce.to_be_bytes();
        let mut nonce_as_32b_word: [u8; 32];
        nonce_as_32b_word = [0; 32];
        nonce_as_32b_word[28..32].copy_from_slice(&nonce_as_4b_word);
        requester_on_32b_as_vec.extend_from_slice(&nonce_as_32b_word);

        let hash = sp_runtime::traits::Keccak256::hash(requester_on_32b_as_vec.as_slice());

        let mut system_hash: T::Hash = T::Hash::default();

        system_hash.as_mut().copy_from_slice(&hash.as_ref()[..32]);

        system_hash
    }

    pub fn generate_step_id<T: Config>(
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
        timeouts_at: AdaptiveTimeout<BlockNumberFor<T>, TargetId>,
        // Speed of confirmation
        speed_mode: SpeedMode,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<BlockNumberFor<T>>>,
    ) -> (
        XExecSignalId<T>,
        XExecSignal<T::AccountId, BlockNumberFor<T>>,
    ) {
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
        let id = signal.generate_id::<T, SystemHashing<T>>();
        (id, signal)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AdaptiveTimeout<BlockNumber, TargetId> {
    pub estimated_height_here: BlockNumber,
    pub estimated_height_there: BlockNumber,
    pub submit_by_height_here: BlockNumber,
    pub submit_by_height_there: BlockNumber,
    pub emergency_timeout_here: BlockNumber,
    pub there: TargetId,
    pub dlq: Option<BlockNumber>,
}

impl<BlockNumber: Zero + From<u32>, TargetId: Default> AdaptiveTimeout<BlockNumber, TargetId> {
    pub fn default_401() -> Self {
        AdaptiveTimeout {
            estimated_height_here: Zero::zero(),
            estimated_height_there: Zero::zero(),
            submit_by_height_here: Zero::zero(),
            submit_by_height_there: Zero::zero(),
            emergency_timeout_here: BlockNumber::from(401u32),
            there: TargetId::default(),
            dlq: None,
        }
    }

    pub fn new_emergency(emergency_timeout_here: BlockNumber) -> Self {
        AdaptiveTimeout {
            estimated_height_here: Zero::zero(),
            estimated_height_there: Zero::zero(),
            submit_by_height_here: Zero::zero(),
            submit_by_height_there: Zero::zero(),
            emergency_timeout_here,
            there: TargetId::default(),
            dlq: None,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum SFXAction<Account, Asset, Balance, Destination, Input, MaxCost> {
    // All sorts of calls: composable, wasm, evm, etc. are vacuumed into a single Call SFX in the protocol level.
    Call(Destination, Account, Balance, MaxCost, Input),
    // All of the DEX-related SFXs are vacuumed into a Transfer SFX in the protocol level: swap, add_liquidity, remove_liquidity, transfer asset, transfer native
    Transfer(Destination, Asset, Account, Balance),
    DynamicDestinationDeal(Destination, Asset, Balance),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct OrderSFX<AccountId, Asset, Balance, Destination, Input, MaxCost> {
    pub sfx_action: SFXAction<AccountId, Asset, Balance, Destination, Input, MaxCost>,
    pub max_reward: Balance,
    pub reward_asset: Asset,
    pub insurance: Balance,
    pub remote_origin_nonce: Option<u32>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum OrderOrigin<AccountId> {
    Local(AccountId),
    Remote(u32),
}

impl<AccountId: Encode + Decode + Clone> OrderOrigin<AccountId> {
    pub fn is_local(&self) -> bool {
        match self {
            OrderOrigin::Local(_) => true,
            _ => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        match self {
            OrderOrigin::Remote(_) => true,
            _ => false,
        }
    }

    pub fn try_remote_source(source: &AccountId) -> Result<OrderOrigin<AccountId>, DispatchError> {
        let maybe_remote = Self::new(source);
        match maybe_remote {
            OrderOrigin::Local(_) => Err("InvalidRemoteSource".into()),
            OrderOrigin::Remote(_) => Ok(maybe_remote),
        }
    }

    pub fn new(source: &AccountId) -> OrderOrigin<AccountId> {
        let source_bytes = source.encode();
        // check if first 28 bytes are 0
        if !source_bytes[..28].iter().all(|x| *x == 0) {
            return OrderOrigin::Local(source.clone())
        }
        let mut remote_origin_nonce = [0u8; 4];
        remote_origin_nonce.copy_from_slice(&source_bytes[28..32]);
        OrderOrigin::Remote(u32::from_be_bytes(remote_origin_nonce))
    }

    pub fn from_remote_nonce(remote_nonce: u32) -> OrderOrigin<AccountId> {
        OrderOrigin::Remote(remote_nonce)
    }

    pub fn to_account_id(&self) -> AccountId {
        match self {
            OrderOrigin::Local(a) => a.clone(),
            OrderOrigin::Remote(nonce) => {
                let mut account_bytes = [0u8; 32];
                let nonce_bytes = nonce.to_be_bytes();
                account_bytes[28..32].copy_from_slice(&nonce_bytes);
                AccountId::decode(&mut &account_bytes[..])
                    .expect("AccountId should always decode from [u8; 32]")
            },
        }
    }
}

impl<AccountId, Asset: Clone, Balance, Destination, Input, MaxCost>
    TryInto<SideEffect<AccountId, Balance>>
    for OrderSFX<AccountId, Asset, Balance, Destination, Input, MaxCost>
where
    u32: From<Asset>,
    Balance: Encode,
    MaxCost: Encode,
    AccountId: Encode,
    Input: AsBytesRef,
    Destination: From<[u8; 4]>,
    [u8; 4]: From<Destination>,
{
    type Error = DispatchError;

    fn try_into(self) -> Result<SideEffect<AccountId, Balance>, Self::Error> {
        let (action, target, encoded_args) = match self.sfx_action {
            SFXAction::Call(target, destination, value, max_cost, input) => {
                let mut encoded_args = vec![];
                // todo: lookup destination target and derive the ActionType (call evm / wasm / composable)
                encoded_args.push(destination.encode()); // target
                encoded_args.push(value.encode()); // value
                encoded_args.push(input.as_bytes_ref().to_vec()); // value
                encoded_args.push(max_cost.encode()); // value
                (*b"cevm", target.into(), encoded_args)
            },
            SFXAction::Transfer(target, asset, destination, amount) => {
                let mut encoded_args: Vec<Vec<u8>> = vec![];
                let asset_id: u32 = <Asset as Into<u32>>::into(asset);
                let action_id = if asset_id != 0 {
                    encoded_args.push(asset_id.to_le_bytes().to_vec());
                    *b"tass"
                } else {
                    *b"tran"
                };
                encoded_args.push(destination.encode());
                encoded_args.push(amount.encode());
                (action_id, target.into(), encoded_args)
            },
            SFXAction::DynamicDestinationDeal(target, asset, amount) => {
                let mut encoded_args: Vec<Vec<u8>> = vec![];
                let asset_id: u32 = <Asset as Into<u32>>::into(asset);
                let action_id = *b"tddd";
                encoded_args.push(asset_id.to_le_bytes().to_vec());
                encoded_args.push(amount.encode());
                (action_id, target.into(), encoded_args)
            },
        };

        let reward_asset_id = if <Asset as Into<u32>>::into(self.reward_asset.clone()) == 0 {
            None
        } else {
            Some(<Asset as Into<u32>>::into(self.reward_asset))
        };

        let side_effect = SideEffect {
            target,
            max_reward: self.max_reward,
            insurance: self.insurance,
            action,
            encoded_args,
            signature: vec![],
            enforce_executor: None,
            reward_asset_id,
        };

        Ok(side_effect)
    }
}

#[cfg(test)]
mod tests {
    use super::{OrderSFX, SFXAction};
    use crate::{
        circuit::{AdaptiveTimeout, XExecSignal},
        SpeedMode,
    };
    use frame_support::assert_ok;
    use hex_literal::hex;
    use mini_mock::MiniRuntime;
    use sp_core::crypto::AccountId32;
    use sp_runtime::traits::Keccak256;
    use sp_std::convert::TryInto;
    use t3rn_types::sfx::SideEffect;

    #[test]
    fn sfx_id_is_compatible_with_keccak_and_20b_accounts() {
        let _account_20b = [2u8; 20];
        let account_32b = AccountId32::new(hex!(
            "0000000000000000000000000202020202020202020202020202020202020202"
        ));
        let dest_account_32b = AccountId32::new(hex!(
            "0000000000000000000000000303030303030303030303030303030303030303"
        ));

        assert_eq!(
            account_32b,
            hex!("0000000000000000000000000202020202020202020202020202020202020202").into()
        );

        let xtx = XExecSignal::<AccountId32, u32>::new(
            &account_32b,
            5u32,
            AdaptiveTimeout::default_401(),
            None,
            SpeedMode::Finalized,
            (0, 0),
        );

        let xtx_id = xtx.generate_id::<MiniRuntime, Keccak256>();

        assert_eq!(
            xtx_id,
            hex!("94a85672500de62cf1c799e77e211ce7af26e924a11efc3274ee5de7c855dd74").into()
        );

        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Transfer([3u8; 4], 1u32, dest_account_32b, 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            remote_origin_nonce: Some(5u32),
        };

        let sfx: SideEffect<AccountId32, u128> = order_sfx.try_into().unwrap();
        let sfx_id = sfx.generate_id::<Keccak256>(xtx_id.0.as_slice(), 0);

        assert_eq!(
            sfx_id,
            hex!("e290576c0364fb6aab8d36e1cf94e32a1c1c23e25e30e455a62f597186981984").into()
        );
    }

    #[test]
    fn sfx_id_calculates_expected_values_for_nonce_0_1_and_2_for_hardcoded_20b_accounts() {
        let account_32b = AccountId32::new(hex!(
            "000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        ));

        let xtx_nonce_0 = XExecSignal::<AccountId32, u32>::new(
            &account_32b,
            0u32,
            AdaptiveTimeout::default_401(),
            None,
            SpeedMode::Finalized,
            (0, 0),
        );

        let xtx_nonce_1 = XExecSignal::<AccountId32, u32>::new(
            &account_32b,
            1u32,
            AdaptiveTimeout::default_401(),
            None,
            SpeedMode::Finalized,
            (0, 0),
        );

        let xtx_nonce_2 = XExecSignal::<AccountId32, u32>::new(
            &account_32b,
            2u32,
            AdaptiveTimeout::default_401(),
            None,
            SpeedMode::Finalized,
            (0, 0),
        );

        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Transfer([3u8; 4], 1u32, account_32b.clone(), 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            remote_origin_nonce: Some(5u32),
        };

        let sfx: SideEffect<AccountId32, u128> = order_sfx.try_into().unwrap();

        let xtx_id_nonce_0 = xtx_nonce_0.generate_id::<MiniRuntime, Keccak256>();
        let xtx_id_nonce_1 = xtx_nonce_1.generate_id::<MiniRuntime, Keccak256>();
        let xtx_id_nonce_2 = xtx_nonce_2.generate_id::<MiniRuntime, Keccak256>();

        let sfx_id_nonce_0 = sfx.generate_id::<Keccak256>(xtx_id_nonce_0.0.as_slice(), 0);
        let sfx_id_nonce_1 = sfx.generate_id::<Keccak256>(xtx_id_nonce_1.0.as_slice(), 0);
        let sfx_id_nonce_2 = sfx.generate_id::<Keccak256>(xtx_id_nonce_2.0.as_slice(), 0);

        let expected_hash_nonce_0 =
            hex!("4fd5cefb43ccd33cfe1f4f9a0405af51205e021449892cb08809d97610cfe722");
        let expected_hash_nonce_1 =
            hex!("733fc9182521d4ac5f3465c0b0382a5b4bad7af476f8c7517e2739536a42bb94");
        let expected_hash_nonce_2 =
            hex!("8066494f08af53d6edb5c6df49048a3b4ae59e20df6d59c86bf8c8650304747e");

        assert_eq!(sfx_id_nonce_0, expected_hash_nonce_0.into());
        assert_eq!(sfx_id_nonce_1, expected_hash_nonce_1.into());
        assert_eq!(sfx_id_nonce_2, expected_hash_nonce_2.into());
    }

    #[test]
    fn test_try_into_transfer() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Transfer([3u8; 4], 1u32, AccountId32::new([2u8; 32]), 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            remote_origin_nonce: None,
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.target, [3u8; 4]);
        assert_eq!(side_effect.action, *b"tass");
        assert_eq!(side_effect.reward_asset_id, Some(1u32));
        assert_eq!(side_effect.encoded_args.len(), 3);
        assert_eq!(side_effect.encoded_args[0], vec![1u8, 0, 0, 0]);
        assert_eq!(side_effect.encoded_args[1], [2u8; 32]);
        assert_eq!(
            side_effect.encoded_args[2],
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_try_into_dynamic_destination_deal() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::DynamicDestinationDeal([3u8; 4], 1u32, 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            remote_origin_nonce: None,
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.target, [3u8; 4]);
        assert_eq!(side_effect.action, *b"tddd");
        assert_eq!(side_effect.reward_asset_id, Some(1u32));
        assert_eq!(side_effect.encoded_args.len(), 2);
        assert_eq!(side_effect.encoded_args[0], vec![1u8, 0, 0, 0]);
        assert_eq!(
            side_effect.encoded_args[1],
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        // Transformation of SFXAction::DynamicDestinationDeal into SFXAction::Transfer is also supported
        // let order_converted_back = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128>::from_sfx(side_effect);
    }

    #[test]
    fn test_try_into_transfer_native_for_asset_zero() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Transfer([3u8; 4], 0u32, AccountId32::new([2u8; 32]), 100u128),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 0u32,
            remote_origin_nonce: None,
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.target, [3u8; 4]);
        assert_eq!(side_effect.action, *b"tran");
        assert_eq!(side_effect.reward_asset_id, None);
        assert_eq!(side_effect.encoded_args.len(), 2);
        assert_eq!(side_effect.encoded_args[0], [2u8; 32]);
        assert_eq!(
            side_effect.encoded_args[1],
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_try_into_call() {
        let order_sfx = OrderSFX::<AccountId32, u32, u128, [u8; 4], Vec<u8>, u128> {
            sfx_action: SFXAction::Call(
                [1u8; 4],
                AccountId32::new([2u8; 32]),
                100u128,
                200u128,
                vec![3u8; 4],
            ),
            max_reward: 200u128,
            insurance: 50u128,
            reward_asset: 1u32,
            remote_origin_nonce: None,
        };

        let result: Result<SideEffect<AccountId32, u128>, _> = order_sfx.try_into();
        assert_ok!(&result);

        let side_effect = result.unwrap();
        assert_eq!(side_effect.max_reward, 200);
        assert_eq!(side_effect.insurance, 50);
        assert_eq!(side_effect.action, *b"cevm");
        assert_eq!(side_effect.reward_asset_id, Some(1u32));
    }
}
