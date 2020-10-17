#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    storage::{child, child::ChildInfo},
    traits::{Currency, ExistenceRequirement, Time},
};

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

pub trait Trait: system::Trait + sudo::Trait {
    type Currency: Currency<Self::AccountId>;
    type Time: Time;
}
