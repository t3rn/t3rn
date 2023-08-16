use crate::Debug;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

/// This trait provides access for a provider to send/recieve signals.
/// This is a bidirectional trait, both 3vm & sdk implement a variant of this with a differing error type.
///
/// This enables a contract to provide some feedback to the circuit and tell it if it should break or not.
/// This also allows a pallet to enable signalling.
pub trait Signaller<Hash>
where
    Hash: Encode + Decode + Debug + Clone,
{
    type Result;
    fn signal(signal: &ExecutionSignal<Hash>) -> Self::Result;
}

/// A representation of a signal
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
pub struct ExecutionSignal<Hash>
where
    Hash: Encode + Decode + Debug + Clone,
{
    /// The current step for the signal
    pub step: u32,
    /// The signal type
    pub kind: SignalKind,
    /// The id associated with the execution
    pub execution_id: Hash,
}

impl<Hash> ExecutionSignal<Hash>
where
    Hash: Encode + Decode + Debug + Clone,
{
    pub fn new(execution_id: &Hash, step: Option<u32>, kind: SignalKind) -> Self {
        ExecutionSignal {
            execution_id: execution_id.clone(),
            step: step.unwrap_or(0),
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub enum SignalKind {
    /// Allows the contract to finish execution in an optimistic manner
    Complete,
    /// Allows the contract to break execution as soon as possible
    Kill(KillReason),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub enum KillReason {
    /// The contract indicated that execution should be killed due to some undefined behavior
    Unhandled,
    /// A specific reason to kill which signals there may be some issue with encoding/decoding
    Codec,
    /// The contract indicated that a user defined timeout tripped
    Timeout,
}
