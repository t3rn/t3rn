#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_std::boxed::Box;
use sp_std::vec::Vec;

use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeString;

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

#[cfg(feature = "std")]
use std::fmt::Debug;

pub type Bytes = sp_core::Bytes;

#[derive(Serialize, Deserialize, PartialEq, Clone, Encode, Decode, Eq, Hash, Debug)]
pub enum Type {
    Address(u16),
    DynamicAddress,
    Bool,
    Int(u16),
    Uint(u16),
    /// where u8 is bytes length
    Bytes(u8),
    DynamicBytes,
    String,
    Enum(u8),
    Struct(u8),
    Mapping(Box<Type>, Box<Type>),
    Contract(u8),
    Ref(Box<Type>),
    StorageRef(Box<Type>),
    /// There is no way to declare value in Solidity (should there be?)
    Value,
    /// DynamicBytes and String are lowered to a vector.
    Slice,
}

/// When resolving a Solidity file, this holds all the resolved items
#[derive(Serialize, Deserialize, PartialEq, Clone, Encode, Decode, Eq, Debug)]
pub struct GatewayGenesis {
    /// address length in bytes
    pub address_length: u32,
    /// value length in bytes
    pub value_length: u32,
    /// value length in bytes
    pub decimals: u32,
    /// value length in bytes
    pub structs: Vec<StructDecl>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Encode, Decode, Eq, Debug)]
pub struct Parameter {
    // The name can empty (e.g. in an event field or unnamed parameter/return)
    pub name: Option<RuntimeString>,
    pub ty: Type,
    pub no: u32,
    pub indexed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Encode, Decode, Eq, Debug)]
pub struct StructDecl {
    pub name: RuntimeString,
    pub fields: Vec<Parameter>,
    // List of offsets of the fields, last entry is the offset for the struct overall size
    pub offsets: Vec<u16>,
}

impl Type {
    /// Calculate how much memory we expect this type to use when allocated on the
    /// stack or on the heap. Depending on the llvm implementation there might be
    /// padding between elements which is not accounted for.
    pub fn size_of(&self, gen: &GatewayGenesis) -> Result<usize, &'static str> {
        match self {
            Type::Enum(_) => Ok(1),
            Type::Bool => Ok(1),
            Type::Contract(_) | Type::Address(_) => Ok(gen.address_length as usize),
            Type::Bytes(n) => Ok(*n as usize),
            Type::Uint(n) | Type::Int(n) => Ok((n / 8).into()),
            Type::Struct(n) => {
                let struct_size = gen
                    .structs
                    .get(*n as usize)
                    .ok_or("Can't access requested struct from gateway genesis")?
                    .offsets
                    .last()
                    .cloned()
                    .unwrap_or_else(|| 0);
                Ok(struct_size.into())
            }
            Type::String | Type::DynamicBytes => Ok(4),
            _ => unimplemented!(),
        }
    }

    /// eval assumes encoded_val is bytes Vector encoded with SCALE
    pub fn eval(
        &self,
        encoded_val: Vec<u8>,
        _gen: &GatewayGenesis,
    ) -> Result<Box<dyn sp_std::any::Any>, &'static str> {
        match self {
            Type::Address(size) => match size {
                20 => {
                    let res: [u8; 20] = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                32 => {
                    let res: [u8; 32] = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                _ => Err("Unknown Address size"),
            },
            Type::DynamicAddress => {
                let res: Vec<u8> = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            }
            Type::Bool => {
                let res: bool = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            }
            Type::Int(size) => match size {
                32 => {
                    let res: i32 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                64 => {
                    let res: i64 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                128 => {
                    let res: i128 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                _ => Err("Unknown Uint size"),
            },
            Type::Uint(size) => match size {
                32 => {
                    let res: u32 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                64 => {
                    let res: u64 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                128 => {
                    let res: u128 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                }
                _ => Err("Unknown Uint size"),
            },
            Type::Bytes(_) => {
                let res: Bytes = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            }
            Type::DynamicBytes => {
                let res: Vec<u8> = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            }
            Type::String => {
                let res: RuntimeString = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            }
            _ => unimplemented!(),
        }
    }
}

pub fn create_signature() {
    unimplemented!()
}

pub fn from_signature_to_abi() {
    unimplemented!()
}

pub fn decode_buf2val<D: Decode>(buf: Vec<u8>) -> Result<D, &'static str> {
    D::decode(&mut &buf[..]).map_err(|_| "Decoding error: decode_buf2val")
}

#[cfg(test)]
mod tests {}
