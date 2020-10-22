#![cfg_attr(not(feature = "std"), no_std)]
use crate::transfers::BalanceOf;
use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, Time},
};

pub mod proofs;
pub mod transfers;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
#[codec(compact)]
#[repr(u8)]
pub enum Phase {
    Execute = 0,
    Commit = 1,
    Revert = 2,
}

// ToDo: Encode errors properly before storing making the below enum obsolete.
#[derive(Clone)]
#[repr(u8)]
pub enum ErrCodes {
    RequesterNotEnoughBalance = 0,

    BalanceTransferFailed = 1,

    PutCodeFailure = 2,

    InitializationFailure = 3,

    ExecutionFailure = 4,

    CallFailure = 5,

    TerminateFailure = 6,
}

pub trait EscrowTrait: system::Trait + sudo::Trait {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}

/// Dispatch calls to runtime requested during execution of WASM Binaries.
pub trait DispatchRuntimeCall<T: EscrowTrait> {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        input: &[u8],
        escrow_account: <T as system::Trait>::AccountId,
        requested: <T as system::Trait>::AccountId,
        callee: <T as system::Trait>::AccountId,
        value: BalanceOf<T>,
        gas: u64,
    ) -> DispatchResult;
}

pub trait ExtendedWasm: EscrowTrait {
    type DispatchRuntimeCall: DispatchRuntimeCall<Self>;
}
