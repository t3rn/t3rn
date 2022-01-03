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

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum CircuitExecStatus {
    Requested,
    Validated,
    Bonded,
    Committed,
    Reverted,
    RevertedTimedOut,
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

impl Default for CircuitExecStatus {
    fn default() -> Self {
        CircuitExecStatus::Requested
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct InsuranceDeposit<AccountId, BlockNumber, BalanceOf> {
    pub insurance: BalanceOf,
    pub reward: BalanceOf,
    pub requester: AccountId,
    pub bonded_relayer: Option<AccountId>,
    pub status: CircuitExecStatus,
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
            status: CircuitExecStatus::Requested,
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
    pub status: CircuitExecStatus,

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
