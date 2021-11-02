use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

pub type SideEffectId<T> = <T as frame_system::Config>::Hash;
pub type TargetId = bp_runtime::ChainId;
type Bytes = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct InboundSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub target: TargetId,
    pub prize: BalanceOf,
    pub ordered_at: BlockNumber,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executioner: AccountId,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct OutboundSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<Bytes>,
    pub output: Option<Bytes>,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct SideEffect<AccountId, BlockNumber, BalanceOf> {
    pub inbound: InboundSideEffect<AccountId, BlockNumber, BalanceOf>,
    pub outbound: Option<InboundSideEffect<AccountId, BlockNumber, BalanceOf>>,
}
