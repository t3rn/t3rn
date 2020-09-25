#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    storage::{child, child::ChildInfo},
    traits::{Currency, ExistenceRequirement, Time},
};

use balances;
use node_runtime::AccountId;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use sudo;
use system::{ensure_none, ensure_root, ensure_signed};

pub mod transfers;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
#[codec(compact)]
#[repr(u8)]
pub enum Phase {
    Execute = 0,
    Commit = 1,
    Revert = 2,
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        SomethingStored(u32, AccountId),

        MultistepExecutionResult(Vec<u8>),

        MultistepCommitResult(u32),

        MultistepRevertResult(u32),

        MultistepUnknownPhase(u8),

        RentProjectionCalled(AccountId, AccountId),

        GetStorageResult(Vec<u8>),
    }
);

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

pub trait Trait: system::Trait + sudo::Trait {
    type Currency: Currency<Self::AccountId>;
}
