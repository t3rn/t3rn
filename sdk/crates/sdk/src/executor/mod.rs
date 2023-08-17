use codec::{Decode, Encode};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::{ExecutionState, SideEffects},
    Debug,
};

#[cfg(feature = "ink")]
pub mod ink;

#[derive(Debug)]
pub enum Error {
    /// The error originated in the ink environment
    #[cfg(feature = "ink")]
    InkEnv(ink_env::Error),
    /// The error originated in the ink executor
    #[cfg(feature = "ink")]
    Ink(ink::Error),
    /// No executor was specified, this is usually an import error, since we only support Ink at the moment
    NoExecutorSpecified,
}

#[cfg(feature = "ink")]
impl From<ink_env::Error> for Error {
    fn from(err: ink_env::Error) -> Self {
        Error::InkEnv(err)
    }
}

#[cfg(feature = "ink")]
impl From<ink::Error> for Error {
    fn from(err: ink::Error) -> Self {
        Error::Ink(err)
    }
}

/// This trait provides access for an executor to read local state from the circuit.
pub trait StateHandler<Hash, State>
where
    Hash: Encode + Decode,
    State: Encode + Decode,
{
    /// Gets the state for an execution
    fn get_state(execution_id: Option<Hash>) -> Result<State, Error>;
}

/// This trait provides access for an executor to submit something on-chain.
///
/// Usually Something is a new Step containing side_effects, however this trait is kept generic.
pub trait Submitter<Thing>
where
    Thing: Encode + Decode,
{
    /// Submits something on-chain
    fn submit(state: Thing) -> Result<(), Error>;
}

/// Macro entrypoint for logging based on feature flags
#[macro_export]
macro_rules! log_msg {
    ($($arg:tt)*) => {{
        #[cfg(feature = "ink")]
        ink_env::debug_print!("{}\n", ink_env::format!($($arg)*));
    }}
}

/// The SDK executor
pub struct Executor;

impl<Hash, AccountId, BlockNumber, Balance>
    StateHandler<Hash, ExecutionState<Hash, AccountId, BlockNumber, Balance>> for Executor
where
    Hash: Encode + Decode + Debug + Clone + Default,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
{
    fn get_state(
        execution_id: Option<Hash>,
    ) -> Result<ExecutionState<Hash, AccountId, BlockNumber, Balance>, Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "ink")] {
                <ink::InkProvider as StateHandler<Hash, ExecutionState<Hash, AccountId, BlockNumber, Balance>>>::get_state(execution_id)
            } else {
                Err(Error::NoExecutorSpecified)
            }
        }
    }
}
impl<AccountId, Balance, Hash> Submitter<SideEffects<AccountId, Balance, Hash>> for Executor
where
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
    Hash: Encode + Decode,
{
    fn submit(state: SideEffects<AccountId, Balance, Hash>) -> Result<(), Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "ink")] {
                <ink::InkProvider as Submitter<SideEffects<AccountId, Balance, Hash>>>::submit(state)
            } else {
                Err(Error::NoExecutorSpecified)
            }
        }
    }
}

impl<Hash> Signaller<Hash> for Executor
where
    Hash: Encode + Decode + Debug + Clone,
{
    type Result = Result<(), Error>;

    fn signal(signal: &ExecutionSignal<Hash>) -> Self::Result {
        cfg_if::cfg_if! {
            if #[cfg(feature = "ink")] {
                <ink::InkProvider as Signaller<Hash>>::signal(signal)
            } else {
                Err(Error::NoExecutorSpecified)
            }
        }
    }
}
