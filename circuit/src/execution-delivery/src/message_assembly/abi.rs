#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::boxed::Box;
use sp_std::vec::Vec;

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub enum Type {
    Address(bool),
    Bool,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    DynamicBytes,
    String,
    Array(Box<Type>, Vec<u8>),
    Enum(usize),
    Struct(usize),
    Mapping(Box<Type>, Box<Type>),
    Contract(usize),
    Ref(Box<Type>),
    StorageRef(Box<Type>),
    /// There is no way to declare value in Solidity (should there be?)
    Value,
    Void,
    Unreachable,
    /// DynamicBytes and String are lowered to a vector.
    Slice,
}

#[cfg(test)]
mod tests {}
