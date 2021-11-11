use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

type Bytes = Vec<u8>;
pub type SideEffectId<T> = <T as frame_system::Config>::Hash;
pub type TargetId = [u8; 4];

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct SideEffect<AccountId, BlockNumber, BalanceOf> {
    pub target: TargetId,
    pub prize: BalanceOf,
    pub ordered_at: BlockNumber,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executioner: AccountId,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub err: Option<Bytes>,
    pub output: Option<Bytes>,
    pub inclusion_proof: Option<Bytes>,
    pub executioner: AccountId,
    pub received_at: BlockNumber,
    pub cost: Option<BalanceOf>,
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BlockNumber, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
}
