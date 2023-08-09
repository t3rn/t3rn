use crate::executor::Error as ExecutorError;
use t3rn_sdk_primitives::{error::Error as PrimitiveError, signal::KillReason};

#[derive(Debug)]
pub enum Error {
    /// The error originated in sdk primitives
    Primitive(PrimitiveError),
    /// The error originated in an executor
    Executor(ExecutorError),
    /// More than the max capacity of parameters were provided
    TooManyFunctionParams,
    /// An easy hook to a signal
    ShouldKill(KillReason),
}

impl From<ExecutorError> for Error {
    fn from(e: ExecutorError) -> Self {
        Self::Executor(e)
    }
}

impl From<PrimitiveError> for Error {
    fn from(e: PrimitiveError) -> Self {
        Self::Primitive(e)
    }
}
