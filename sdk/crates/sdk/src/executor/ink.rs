use crate::{
    executor::{Error as ExecutorError, StateHandler, Submitter},
    log_msg,
};
use codec::{Decode, Encode};
use ink_env::{self, chain_extension::ChainExtensionMethod};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::{ExecutionState, SideEffects},
    Debug, GET_STATE_FUNCTION_CODE, POST_SIGNAL_FUNCTION_CODE, SUBMIT_FUNCTION_CODE,
};

/// This provider utilizes chain extensions in the case of ink to call through to 3vm.
///
/// It heavily utilises ink's low-level API for generating chain extensions.
///
/// There is room for potential collisions with function pointers.
// TODO: pick an arbitrary number, not iterated from 0.
#[derive(Clone, Copy, Debug)]
pub struct InkProvider {}

#[derive(Debug)]
pub enum Error {
    StateHandler(StateHandlerError),
    EmptyAppendState,
}

impl From<StateHandlerError> for Error {
    fn from(e: StateHandlerError) -> Self {
        Error::StateHandler(e)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StateHandlerError {
    /// Requested state could not be found
    StateNotFound,
    /// There was an error appending a step
    AppendFailure,
    /// There was a failure in encoding/decoding state
    Codec,
    /// An internal failure in the wasm call stack
    CallFailure,
    /// There was a failure posting a signal
    PostSignal,
    /// Accounts for all unhandled status codes
    UnhandledCode,
}

impl ink_env::chain_extension::FromStatusCode for StateHandlerError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::StateNotFound),
            2 => Err(Self::AppendFailure),
            3 => Err(Self::Codec),
            4 => Err(Self::CallFailure),
            5 => Err(Self::PostSignal),
            _ => Err(Self::UnhandledCode),
        }
    }
}

impl From<codec::Error> for StateHandlerError {
    fn from(error: codec::Error) -> Self {
        log_msg!("[INKSDK][StateHandler] error decoding state {:?}", error);
        StateHandlerError::Codec
    }
}

impl<Hash, AccountId, BlockNumber, Balance>
    StateHandler<Hash, ExecutionState<Hash, AccountId, BlockNumber, Balance>> for InkProvider
where
    Hash: Encode + Decode + Debug + Clone + Default,
    AccountId: Encode + Decode + Debug + Clone,
    BlockNumber: Encode + Decode + Debug + Clone,
    Balance: Encode + Decode + Debug + Clone,
{
    fn get_state(
        execution_id: Option<Hash>,
    ) -> Result<ExecutionState<Hash, AccountId, BlockNumber, Balance>, ExecutorError> {
        log_msg!("[INKSDK] getting state for execution: {:?}", execution_id);

        // Quirk: can't use option across chain extensions, it just mangles the hash
        let execution_id = execution_id.unwrap_or_else(|| Hash::default());

        // Low level api for generating chain extensions dynamically
        ChainExtensionMethod::build(GET_STATE_FUNCTION_CODE)
            .input::<Hash>()
            .output_result::<ExecutionState<Hash, AccountId, BlockNumber, Balance>, StateHandlerError>()
            .handle_error_code::<StateHandlerError>()
            .call(&execution_id)
            .map_err(Error::from)
            .map_err(ExecutorError::from)
    }
}

impl<AccountId, Balance, Hash> Submitter<SideEffects<AccountId, Balance, Hash>> for InkProvider
where
    AccountId: Encode + Decode,
    Balance: Encode + Decode,
    Hash: Encode + Decode,
{
    fn submit(state: SideEffects<AccountId, Balance, Hash>) -> Result<(), ExecutorError> {
        log_msg!("[INKSDK] handling append state");

        // Low level api for generating chain extensions dynamically
        ChainExtensionMethod::build(SUBMIT_FUNCTION_CODE)
            .input::<SideEffects<AccountId, Balance, Hash>>()
            .output_result::<(), StateHandlerError>()
            .handle_error_code::<StateHandlerError>()
            .call(&state)
            .map_err(Error::from)
            .map_err(ExecutorError::from)
    }
}

impl<Hash> Signaller<Hash> for InkProvider
where
    Hash: Encode + Decode + Debug + Clone,
{
    type Result = Result<(), ExecutorError>;

    fn signal(signal: &ExecutionSignal<Hash>) -> Self::Result {
        log_msg!(
            "[INKSDK] handling signal {:?} bytes {:?}",
            signal,
            signal.encode()
        );

        // Low level api for generating chain extensions dynamically
        ChainExtensionMethod::build(POST_SIGNAL_FUNCTION_CODE)
            .input::<ExecutionSignal<Hash>>()
            .output_result::<(), StateHandlerError>()
            .handle_error_code::<StateHandlerError>()
            .call(signal)
            .map_err(Error::from)
            .map_err(ExecutorError::from)
    }
}
