//! This module provides a means for executing contracts
//! represented in wasm.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, Randomness, Time, UnfilteredDispatchable},
    weights::{GetDispatchInfo, Weight},
    RuntimeDebug,
};
use sp_runtime::traits::Convert;

use parity_wasm::elements::ValueType;
use sp_sandbox;
use sp_std::prelude::*;
use t3rn_primitives::{transfers::BalanceOf, EscrowTrait};

pub use crate::pallet::*;

#[macro_use]
pub mod env_def;
pub mod ext;
pub mod fake_storage;
pub mod fees;
pub mod gas;
pub mod prepare;
pub mod runtime;
pub mod simple_schedule_v2;

pub use crate::simple_schedule_v2::Schedule;

use self::env_def::ConvertibleToWasm;
use system::Config as SystemTrait;

pub type MomentOf<T> = <<T as EscrowTrait>::Time as Time>::Moment;
pub type AccountIdOf<T> = <T as SystemTrait>::AccountId;
pub type SeedOf<T> = <T as SystemTrait>::Hash;
pub type TopicOf<T> = <T as SystemTrait>::Hash;
pub type BlockNumberOf<T> = <T as SystemTrait>::BlockNumber;

pub type CodeHash<T> = <T as SystemTrait>::Hash;
pub type TrieId = Vec<u8>;

pub struct DisabledDispatchRuntimeCall {}

impl<T: VersatileWasm> DispatchRuntimeCall<T> for DisabledDispatchRuntimeCall {
    fn dispatch_runtime_call(
        _module_name: &str,
        _fn_name: &str,
        _input: &[u8],
        _escrow_account: &<T as system::Config>::AccountId,
        _requested: &<T as system::Config>::AccountId,
        _callee: &<T as system::Config>::AccountId,
        _value: BalanceOf<T>,
        _gas: &mut crate::gas::GasMeter<T>,
    ) -> DispatchResult {
        unimplemented!()
    }
}

/// Dispatch calls to runtime requested during execution of WASM Binaries.
pub trait DispatchRuntimeCall<T: VersatileWasm> {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        input: &[u8],
        escrow_account: &<T as system::Config>::AccountId,
        requested: &<T as system::Config>::AccountId,
        callee: &<T as system::Config>::AccountId,
        value: BalanceOf<T>,
        gas: &mut crate::gas::GasMeter<T>,
    ) -> DispatchResult;
}

pub use crate::pallet::Config as VersatileWasm;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: system::Config + EscrowTrait + transaction_payment::Config {
        type Event: From<Event<Self>>
            + IsType<<Self as system::Config>::Event>
            + Into<<Self as system::Config>::Event>;

        type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type DispatchRuntimeCall: DispatchRuntimeCall<Self>;
        /// Cost schedule and limits.
        #[pallet::constant]
        type Schedule: Get<Schedule>;
        /// The type of the call stack determines the maximum nesting depth of contract calls.
        ///
        /// The allowed depth is `CallStack::size() + 1`.
        /// Therefore a size of `0` means that a contract cannot use call or instantiate.
        /// In other words only the origin called "root contract" is allowed to execute then.
        type CallStack: smallvec::Array<Item = Frame<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::error]
    pub enum Error<T> {
        StorageExhausted,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> where T::AccountId: AsRef<[u8]> {}

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        /// An event deposited upon execution of a contract from the account.
        /// \[escrow_account, requester_account, data\]
        VersatileVMExecution(T::AccountId, T::AccountId, Vec<u8>),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

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

pub type StackTrace = Vec<StackTraceEntry>;

#[derive(RuntimeDebug, PartialEq, Eq, Clone)]
pub struct StackTraceEntry {
    pub host_fn_name: &'static str,
    pub arguments_list: &'static str,
}

pub type ExecResultTrace = Result<(ExecReturnValue, StackTrace), ExecError>;

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
