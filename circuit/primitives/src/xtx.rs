use codec::{Decode, Encode};
use sp_runtime::{
    traits::{Hash, Zero},
    RuntimeDebug,
};
use sp_std::vec::Vec;

type SystemHashing<T> = <T as frame_system::Config>::Hashing;
pub type XtxId<T> = <T as frame_system::Config>::Hash;

/// A composable cross-chain (X) transaction that has already been verified to be valid and submittable
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct Xtx<AccountId, BlockNumber, BalanceOf> {
    // todo: Add missing DFDs
    // pub contracts_dfd: InterExecSchedule -> ContractsDFD
    // pub side_effects_dfd: SideEffectsDFD
    // pub gateways_dfd: GatewaysDFD
    /// The owner of the bid
    pub requester: AccountId,

    /// Encoded content of composable tx
    pub initial_input: Vec<u8>,

    /// Expiry timeout
    pub timeouts_at: Option<BlockNumber>,

    /// Schedule execution of steps in the future intervals
    pub delay_steps_at: Option<Vec<BlockNumber>>,

    /// Has returned status already and what
    pub result_status: Option<Vec<u8>>,

    /// Total reward
    pub total_reward: Option<BalanceOf>,
}

impl<
        AccountId: Encode,
        BlockNumber: Ord + Copy + Zero + Encode,
        BalanceOf: Copy + Zero + Encode + Decode,
    > Xtx<AccountId, BlockNumber, BalanceOf>
{
    pub fn new(
        // Requester of xtx
        requester: AccountId,
        // Encoded initial input set by a requester/SDK - base for the xtx state
        initial_input: Vec<u8>,
        // Expiry timeout
        timeouts_at: Option<BlockNumber>,
        // Schedule execution of steps in the future intervals
        delay_steps_at: Option<Vec<BlockNumber>>,
        // Total reward
        total_reward: Option<BalanceOf>,
    ) -> Self {
        Xtx {
            requester,
            initial_input,
            timeouts_at,
            delay_steps_at,
            result_status: None,
            total_reward,
        }
    }

    pub fn generate_xtx_id<T: frame_system::Config>(&self) -> XtxId<T> {
        SystemHashing::<T>::hash(Encode::encode(self).as_ref())
    }
}
