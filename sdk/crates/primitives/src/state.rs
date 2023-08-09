use crate::{storage::BoundedVec, xc::Chain, BTreeMap, Debug, Vec, MAX_PARAMETERS_IN_FUNCTION};
use codec::{Decode, Encode, MaxEncodedLen};
use t3rn_types::fsx::FullSideEffect;

/// Some new side effects to submit
#[derive(Encode, Decode, MaxEncodedLen)]
pub struct SideEffects<AccountId, Balance, Hash>
where
    Hash: Encode + Decode,
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
{
    /// Indication of the current execution id
    pub execution_id: Hash,
    /// A fixed vector of side effects, bound by `MAX_PARAMETERS_IN_FUNCTION`
    pub side_effects: BoundedVec<Chain<AccountId, Balance, Hash>, MAX_PARAMETERS_IN_FUNCTION>,
}

// HELLO: This has to have field parity with Circuit::LocalStateExecutionView
/// The local state returned by the circuit for an execution
#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct ExecutionState<Hash, AccountId, BlockNumber, Balance>
where
    Hash: Encode + Decode + Debug + Clone,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
{
    /// An abstract state mapped by 32 bit hash and a vector of bytes
    pub local_state: BTreeMap<[u8; 32], Vec<u8>>,
    /// A multi-dimensional vector of side effects, indexed by `step`
    pub side_effects: Vec<Vec<FullSideEffect<AccountId, BlockNumber, Balance>>>,
    /// Current step and the length of steps
    pub steps_cnt: (u32, u32),
    /// The id for this execution
    pub xtx_id: Hash,
}

/// A handler trait that allows generics to provide some execution_id
pub trait GetExecutionId<Hash>
where
    Hash: Encode + Decode + Debug + Clone,
{
    fn get_execution_id(&self) -> &Hash;
}

impl<Hash, AccountId, BlockNumber, BalanceOf> GetExecutionId<Hash>
    for ExecutionState<Hash, AccountId, BlockNumber, BalanceOf>
where
    Hash: Encode + Decode + Debug + Clone,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    BalanceOf: Encode + Decode + Debug + Clone,
{
    fn get_execution_id(&self) -> &Hash {
        &self.xtx_id
    }
}

/// A handler trait that allows generics to provide some steps
pub trait GetSteps {
    fn get_index(&self) -> u32;

    fn get_len(&self) -> u32;

    fn reached_end(&self) -> bool;
}

impl<Hash, AccountId, BlockNumber, BalanceOf> GetSteps
    for ExecutionState<Hash, AccountId, BlockNumber, BalanceOf>
where
    Hash: Encode + Decode + Debug + Clone,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    BalanceOf: Encode + Decode + Debug + Clone,
{
    fn get_index(&self) -> u32 {
        self.steps_cnt.0
    }

    fn get_len(&self) -> u32 {
        self.steps_cnt.1
    }

    fn reached_end(&self) -> bool {
        self.steps_cnt.0 >= self.steps_cnt.1
    }
}

/// A marker trait that implies a generic type implements all available getters
pub trait Getters<Hash>: GetExecutionId<Hash> + GetSteps
where
    Hash: Encode + Decode + Debug + Clone,
{
}

impl<Hash, AccountId, BlockNumber, BalanceOf> Getters<Hash>
    for ExecutionState<Hash, AccountId, BlockNumber, BalanceOf>
where
    Hash: Encode + Decode + Debug + Clone,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    BalanceOf: Encode + Decode + Debug + Clone,
{
}
