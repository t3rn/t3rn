//! This module provides a means for executing contracts
//! represented in wasm.

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchError,
    traits::{Currency, Time},
};
use gateway_escrow_engine::EscrowTrait;
use parity_wasm::elements::ValueType;
use sp_sandbox;
use sp_std::prelude::*;
#[macro_use]
pub mod env_def;
pub mod ext;
pub mod gas;
pub mod prepare;
pub mod runtime;

use self::env_def::ConvertibleToWasm;
use system::Trait as SystemTrait;

pub type MomentOf<T> = <<T as EscrowTrait>::Time as Time>::Moment;
pub type AccountIdOf<T> = <T as SystemTrait>::AccountId;
pub type SeedOf<T> = <T as SystemTrait>::Hash;
pub type TopicOf<T> = <T as SystemTrait>::Hash;
pub type BlockNumberOf<T> = <T as SystemTrait>::BlockNumber;

/// A prepared wasm module ready for execution.
#[derive(Clone, Encode, Decode)]
pub struct PrefabWasmModule {
    /// Version of the schedule with which the code was instrumented.
    #[codec(compact)]
    schedule_version: u32,
    #[codec(compact)]
    initial: u32,
    #[codec(compact)]
    maximum: u32,
    /// This field is reserved for future evolution of format.
    ///
    /// Basically, for now this field will be serialized as `None`. In the future
    /// we would be able to extend this structure with.
    _reserved: Option<()>,
    /// Code instrumented with the latest schedule.
    code: Vec<u8>,
}

/// Wasm executable loaded by `WasmLoader` and executed by `WasmVm`.
pub struct WasmExecutable {
    pub entrypoint_name: &'static str,
    pub prefab_module: PrefabWasmModule,
}

use bitflags::bitflags;
pub type StorageKey = [u8; 32];

/// Error returned by contract exection.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct ExecError {
    /// The reason why the execution failed.
    pub error: DispatchError,
    /// Origin of the error.
    pub origin: ErrorOrigin,
}

impl<T: Into<DispatchError>> From<T> for ExecError {
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            origin: ErrorOrigin::Caller,
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub enum ErrorOrigin {
    /// The error happened in the current exeuction context rather than in the one
    /// of the contract that is called into.
    Caller,
    /// The error happened during execution of the called contract.
    Callee,
}

bitflags! {
    /// Flags used by a contract to customize exit behaviour.
    pub struct ReturnFlags: u32 {
        /// If this bit is set all changes made by the contract exection are rolled back.
        const REVERT = 0x0000_0001;
    }
}
/// Output of a contract call or instantiation which ran to completion.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct ExecReturnValue {
    /// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
    pub flags: ReturnFlags,
    /// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
    pub data: Vec<u8>,
}

impl ExecReturnValue {
    /// We understand the absense of a revert flag as success.
    pub fn is_success(&self) -> bool {
        !self.flags.contains(ReturnFlags::REVERT)
    }
}

pub type ExecResult = Result<ExecReturnValue, ExecError>;

pub enum TrapReason {
    /// The supervisor trapped the contract because of an error condition occurred during
    /// execution in privileged code.
    SupervisorError(DispatchError),
    /// Signals that trap was generated in response to call `seal_return` host function.
    Return(ReturnData),
    /// Signals that a trap was generated in response to a successful call to the
    /// `seal_terminate` host function.
    Termination,
    /// Signals that a trap was generated because of a successful restoration.
    Restoration,
}

/// Every error that can be returned to a contract when it calls any of the host functions.
#[repr(u32)]
pub enum ReturnCode {
    /// API call successful.
    Success = 0,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped = 1,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted = 2,
    /// The passed key does not exist in storage.
    KeyNotFound = 3,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold = 4,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed = 5,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded = 6,
    /// No code could be found at the supplied code hash.
    CodeNotFound = 7,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable = 8,
}

impl ConvertibleToWasm for ReturnCode {
    type NativeType = Self;
    const VALUE_TYPE: ValueType = ValueType::I32;
    fn to_typed_value(self) -> sp_sandbox::Value {
        sp_sandbox::Value::I32(self as i32)
    }
    fn from_typed_value(_: sp_sandbox::Value) -> Option<Self> {
        debug_assert!(
            false,
            "We will never receive a ReturnCode but only send it to wasm."
        );
        None
    }
}

/// The data passed through when a contract uses `seal_return`.
pub struct ReturnData {
    /// The flags as passed through by the contract. They are still unchecked and
    /// will later be parsed into a `ReturnFlags` bitflags struct.
    pub flags: u32,
    /// The output buffer passed by the contract as return data.
    pub data: Vec<u8>,
}
