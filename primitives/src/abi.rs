use codec::{Decode, Encode};
use frame_support::ensure;
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::U256;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
use sp_runtime::RuntimeString;
use sp_std::{boxed::Box, vec, vec::Vec};

#[cfg(feature = "std")]
use std::fmt::Debug;

pub type Bytes = sp_core::Bytes;

/// ToDo: Introduce Compact Encoding for u8 + u16 + u32
#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
    Contract,
    Ref(Box<Type>),
    Option(Box<Type>),
    OptionalInsurance,
    OptionalReward,
    StorageRef(Box<Type>),
    /// There is no way to declare value in Solidity (should there be?)
    Value,
    /// DynamicBytes and String are lowered to a vector.
    Slice,
    Hasher(HasherAlgo, u16),
    Crypto(CryptoAlgo),
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum HasherAlgo {
    Blake2,
    Keccak256,
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CryptoAlgo {
    Ed25519,
    Sr25519,
    Ecdsa,
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Describe ABI configuration for a gateway so that it's possible to cast types
/// of inbound and outbound messages to that gateway
pub struct GatewayABIConfig {
    /// block number type in bytes
    pub block_number_type_size: u16,
    /// hash size in bytes
    pub hash_size: u16,
    /// hashing algorithm
    pub hasher: HasherAlgo,
    /// cryptography algorithm
    pub crypto: CryptoAlgo,
    /// address length in bytes
    pub address_length: u16,
    /// value length in bytes
    pub value_type_size: u16,
    /// value length in bytes
    pub decimals: u16,
    /// value length in bytes. ToDo: move as part of metadata.
    pub structs: Vec<StructDecl>,
}

impl Default for GatewayABIConfig {
    fn default() -> GatewayABIConfig {
        GatewayABIConfig {
            block_number_type_size: 32,
            hash_size: 32,
            hasher: HasherAlgo::Blake2,
            crypto: CryptoAlgo::Sr25519,
            address_length: 32, // 32 bytes : 32 * 8 = 256 bits
            value_type_size: 8, // u64 = 8 bytes = 64 bits.
            decimals: 8,
            structs: vec![],
        }
    }
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ContractActionDesc<Hash, TargetId, AccountId> {
    pub action_id: Hash,
    pub target_id: Option<TargetId>,
    pub to: Option<AccountId>,
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Parameter {
    /// The name can empty (e.g. in an event field or unnamed parameter/return); encoded vector
    pub name: Option<Vec<u8>>,
    /// ABI type
    pub ty: Type,
    /// number in order
    pub no: u32,
    /// is indexed - follows the ethereum logs pattern where longer exceeding 32 bytes values are indexed
    pub indexed: Option<bool>,
}

#[derive(PartialEq, Clone, Encode, Decode, Eq, Hash, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StructDecl {
    /// encoded name of the struct
    pub name: Type,
    /// list of fields, each of them will have corresponding no.
    pub fields: Vec<Parameter>,
    /// List of offsets of the fields, last entry is the offset for the struct overall size
    pub offsets: Vec<u16>,
}

impl Type {
    /// Calculate how much memory we expect this type to use when allocated on the
    /// stack or on the heap. Depending on the llvm implementation there might be
    /// padding between elements which is not accounted for.
    pub fn size_of(&self, gen: &GatewayABIConfig) -> Result<usize, &'static str> {
        match self {
            Type::Enum(_) => Ok(1),
            Type::Bool => Ok(1),
            Type::Contract | Type::Address(_) => Ok(gen.address_length as usize),
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
                    .unwrap_or(0);
                Ok(struct_size.into())
            },
            Type::String | Type::DynamicBytes => Ok(4),
            Type::Hasher(_hasher_alg, _hash_size) => Ok(gen.hash_size as usize),
            _ => unimplemented!(),
        }
    }

    pub fn to_string_bytes(&self) -> &[u8] {
        match self {
            Type::Enum(_) => b"enum",
            Type::Bool => b"bool",
            Type::Contract => b"contract",
            Type::Address(_) => b"address",
            Type::Bytes(_) => b"bytes",
            Type::Uint(n) => match n {
                32 => b"uint32",
                64 => b"uint64",
                128 => b"uint128",
                _ => unimplemented!(),
            },
            Type::Int(n) => match n {
                32 => b"int32",
                64 => b"int64",
                128 => b"int128",
                _ => unimplemented!(),
            },
            Type::String => b"string",
            Type::DynamicBytes => b"dynamic_bytes",
            Type::DynamicAddress => b"dynamic_address",
            Type::Hasher(hasher_alg, hash_size) => match hasher_alg {
                HasherAlgo::Blake2 => match hash_size {
                    128 => b"blake2_128",
                    256 => b"blake2_256",
                    _ => unimplemented!(),
                },
                HasherAlgo::Keccak256 => match hash_size {
                    256 => b"blake2_256",
                    _ => unimplemented!(),
                },
            },
            Type::Crypto(crypto_alg) => match crypto_alg {
                CryptoAlgo::Ed25519 => b"ed25519",
                CryptoAlgo::Sr25519 => b"sr25519",
                CryptoAlgo::Ecdsa => b"ecdsa",
            },
            _ => unimplemented!(),
        }
    }

    pub fn to_string(&self) -> RuntimeString {
        match self {
            Type::Enum(_) => RuntimeString::from("enum"),
            Type::Bool => RuntimeString::from("bool"),
            Type::Contract => RuntimeString::from("contract"),
            Type::Address(_) => RuntimeString::from("address"),
            Type::Bytes(_) => RuntimeString::from("bytes"),
            Type::Uint(n) => match n {
                32 => RuntimeString::from("uint32"),
                64 => RuntimeString::from("uint64"),
                128 => RuntimeString::from("uint128"),
                _ => unimplemented!(),
            },
            Type::Int(n) => match n {
                32 => RuntimeString::from("int32"),
                64 => RuntimeString::from("int64"),
                128 => RuntimeString::from("int128"),
                _ => unimplemented!(),
            },
            Type::String => RuntimeString::from("string"),
            Type::DynamicBytes => RuntimeString::from("dynamic_bytes"),
            Type::Hasher(hasher_alg, hash_size) => match hasher_alg {
                HasherAlgo::Blake2 => match hash_size {
                    128 => RuntimeString::from("blake2_128"),
                    256 => RuntimeString::from("blake2_256"),
                    _ => unimplemented!(),
                },
                HasherAlgo::Keccak256 => match hash_size {
                    256 => RuntimeString::from("blake2_256"),
                    _ => unimplemented!(),
                },
            },
            Type::Crypto(crypto_alg) => match crypto_alg {
                CryptoAlgo::Ed25519 => RuntimeString::from("ed25519"),
                CryptoAlgo::Sr25519 => RuntimeString::from("sr25519"),
                CryptoAlgo::Ecdsa => RuntimeString::from("ecdsa"),
            },
            _ => unimplemented!(),
        }
    }

    /// eval assumes encoded_val is bytes Vector encoded with SCALE
    pub fn eval_abi(
        &self,
        encoded_val: Vec<u8>,
        gen: &GatewayABIConfig,
    ) -> Result<Vec<u8>, &'static str> {
        match self {
            Type::Address(size) => match size {
                20 => {
                    let res: [u8; 20] = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                32 => {
                    let res: [u8; 32] = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                _ => Err("Unknown Address size"),
            },
            Type::DynamicAddress => match gen.address_length {
                20 => {
                    let res: [u8; 20] = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                32 => {
                    let res: [u8; 32] = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                _ => unimplemented!(),
            },
            Type::Bool => {
                let res: bool = decode_buf2val(encoded_val)?;
                Ok(res.encode())
            },
            Type::OptionalInsurance => {
                // Assume it's gonna be an array of 2 x u128 values (Circuit's Balance)
                //  First one is the insurance amount
                //  Second one is the reward amount
                // Since it's Optional - value can be an empty vector too.
                match encoded_val.len() {
                    0 => Ok(encoded_val),
                    32 => {
                        let res: [u128; 2] = decode_buf2val(encoded_val)?;
                        Ok(res.encode())
                    },
                    _ => Err("ABI OptionalInsurance eval error - wrong arg size"),
                }
            },
            Type::Int(size) => match size {
                32 => {
                    let res: i32 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                64 => {
                    let res: i64 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                128 => {
                    let res: i128 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                _ => Err("Unknown Uint size"),
            },
            Type::Uint(size) => match size {
                32 => {
                    let res: u32 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                64 => {
                    let res: u64 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                128 => {
                    let res: u128 = decode_buf2val(encoded_val)?;
                    Ok(res.encode())
                },
                _ => Err("Unknown Uint size"),
            },
            Type::Bytes(_) => {
                let res: Bytes = decode_buf2val(encoded_val)?;
                Ok(res.to_vec())
            },
            Type::DynamicBytes => Ok(encoded_val),
            Type::String => {
                let res: RuntimeString = decode_buf2val(encoded_val)?;
                Ok(res.encode())
            },
            Type::Value => {
                match gen.value_type_size {
                    8 => {
                        // 8 bytes = 64 bits
                        let res: u64 = decode_buf2val::<u64>(encoded_val)?;
                        Ok(res.encode())
                    },
                    16 => {
                        // 16 bytes = 128 bits
                        let res: u128 = decode_buf2val::<u128>(encoded_val)?;
                        Ok(res.encode())
                    },
                    32 => {
                        // 32 bytes = 256 bits
                        let res: U256 = decode_buf2val::<U256>(encoded_val)?;
                        Ok(res.encode())
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }

    /// eval assumes encoded_val is bytes Vector encoded with SCALE
    pub fn eval(
        &self,
        encoded_val: Vec<u8>,
        // _gen: &GatewayABIConfig,
    ) -> Result<Box<dyn sp_std::any::Any>, &'static str> {
        match self {
            Type::Address(size) => match size {
                20 => {
                    let res: [u8; 20] = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                32 => {
                    let res: [u8; 32] = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                _ => Err("Unknown Address size"),
            },
            Type::DynamicAddress => {
                let res: Vec<u8> = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            },
            Type::Bool => {
                let res: bool = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            },
            Type::OptionalInsurance => {
                // Assume it's gonna be an array of 2 x u128 values (Circuit's Balance)
                //  First one is the insurance amount
                //  Second one is the reward amount
                // Since it's Optional - value can be an empty vector too.
                match encoded_val.len() {
                    0 => Ok(Box::new(encoded_val)),
                    32 => {
                        let res: [u128; 2] = decode_buf2val(encoded_val)?;
                        Ok(Box::new(res))
                    },
                    _ => Err("ABI OptionalInsurance eval error - wrong arg size"),
                }
            },
            Type::Int(size) => match size {
                32 => {
                    let res: i32 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                64 => {
                    let res: i64 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                128 => {
                    let res: i128 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                _ => Err("Unknown Uint size"),
            },
            Type::Uint(size) => match size {
                32 => {
                    let res: u32 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                64 => {
                    let res: u64 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                128 => {
                    let res: u128 = decode_buf2val(encoded_val)?;
                    Ok(Box::new(res))
                },
                _ => Err("Unknown Uint size"),
            },
            Type::Bytes(_) => {
                let res: Bytes = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            },
            Type::DynamicBytes => {
                let res: Vec<u8> = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            },
            Type::String => {
                let res: RuntimeString = decode_buf2val(encoded_val)?;
                Ok(Box::new(res))
            },
            Type::Hasher(hasher_alg, hash_size) => match hasher_alg {
                HasherAlgo::Blake2 => match hash_size {
                    128 => Ok(Box::new(sp_io::hashing::blake2_128)),
                    256 => Ok(Box::new(sp_io::hashing::blake2_256)),
                    _ => unimplemented!(),
                },
                HasherAlgo::Keccak256 => match hash_size {
                    256 => Ok(Box::new(sp_io::hashing::keccak_256)),
                    _ => unimplemented!(),
                },
            },
            _ => unimplemented!(),
        }
    }
}

pub fn eval_to_encoded(t: Type, raw_val: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    match t {
        Type::Address(size) => match size {
            20 => {
                let res: [u8; 20] = decode_buf2val(raw_val)?;
                Ok(res.to_vec())
            },
            32 => {
                let res: [u8; 32] = decode_buf2val(raw_val)?;
                Ok(res.to_vec())
            },
            _ => Err("Unknown Address size"),
        },
        Type::Uint(size) => match size {
            32 => {
                let res: u32 = decode_buf2val(raw_val)?;
                Ok(res.encode())
            },
            64 => {
                let res: u64 = decode_buf2val(raw_val)?;
                Ok(res.encode())
            },
            128 => {
                let res: u128 = decode_buf2val(raw_val)?;
                Ok(res.encode())
            },
            _ => Err("Unknown Uint size"),
        },
        Type::OptionalInsurance => {
            // Assume it's gonna be an array of 2 x u128 values (Circuit's Balance)
            //  First one is the insurance amount
            //  Second one is the reward amount
            // Since it's Optional - value can be an empty vector too.
            match raw_val.len() {
                0 => Ok(raw_val),
                32 => {
                    let res: [u128; 2] = decode_buf2val(raw_val)?;
                    Ok(res.encode())
                },
                _ => Err("ABI OptionalInsurance eval error - wrong arg size"),
            }
        },
        _ => Ok(vec![]),
    }
}

pub fn eval_trait_dynamically(t: Type) -> Result<Box<dyn sp_std::any::Any>, &'static str> {
    match t {
        Type::Hasher(hasher_alg, hash_size) => match hasher_alg {
            HasherAlgo::Blake2 => match hash_size {
                256 => Ok(Box::new(sp_runtime::traits::BlakeTwo256)),
                _ => unimplemented!(),
            },
            HasherAlgo::Keccak256 => match hash_size {
                256 => Ok(Box::new(sp_runtime::traits::Keccak256)),
                _ => unimplemented!(),
            },
        },
        _ => unimplemented!(),
    }
}

pub fn from_bytes_string(bytes_string: &[u8]) -> Type {
    match bytes_string {
        b"bool" => Type::Bool,
        b"contract" => Type::Contract,
        b"address" => Type::Address(20),
        b"dynamic_address" => Type::DynamicAddress,
        b"bytes" => Type::DynamicBytes,
        b"dynamic_bytes" => Type::DynamicBytes,
        b"uint32" => Type::Uint(32),
        b"uint64" => Type::Uint(64),
        b"uint128" => Type::Uint(128),
        b"int32" => Type::Uint(32),
        b"int64" => Type::Uint(64),
        b"int128" => Type::Uint(128),
        b"string" => Type::String,
        b"blake2_256" => Type::Hasher(HasherAlgo::Blake2, 256),
        b"blake2_128" => Type::Hasher(HasherAlgo::Blake2, 128),
        b"keccak256" => Type::Hasher(HasherAlgo::Keccak256, 128),
        b"sr25519" => Type::Crypto(CryptoAlgo::Sr25519),
        b"ed25519" => Type::Crypto(CryptoAlgo::Ed25519),
        b"ecdsa" => Type::Crypto(CryptoAlgo::Ecdsa),
        _ => unimplemented!(),
    }
}

pub fn create_signature(
    name_encoded: Vec<u8>,
    args_abi: Vec<Type>,
) -> Result<Vec<u8>, &'static str> {
    const BEGIN_ARGS_CHAR: u8 = b'(';
    const END_ARGS_CHAR: u8 = b')';
    const COMMA_SEPARATOR: u8 = b',';

    let name_bytes: &[u8] = name_encoded.as_slice();

    let middle_args = args_abi
        .iter()
        .map(|t| t.to_string_bytes())
        .collect::<Vec<&[u8]>>()
        .as_slice()
        .join(&COMMA_SEPARATOR);

    let r = [
        name_bytes,
        &[BEGIN_ARGS_CHAR],
        middle_args.as_slice(),
        &[END_ARGS_CHAR],
    ]
    .concat();

    Ok(r)
}

pub fn from_signature_to_abi(signature: Vec<u8>) -> Result<(Vec<u8>, Vec<Type>), &'static str> {
    const BEGIN_ARGS_CHAR: u8 = b'(';
    const END_ARGS_CHAR: u8 = b')';
    const COMMA_SEPARATOR: u8 = b',';

    let mut signature_iter = signature
        .as_slice()
        .split(|x| x.eq(&BEGIN_ARGS_CHAR) || x.eq(&COMMA_SEPARATOR) || x.eq(&END_ARGS_CHAR))
        .filter(|&x| !x.is_empty());

    let maybe_name = signature_iter.next().unwrap_or(&[]);

    ensure!(
        !maybe_name.is_empty(),
        "Can't find a name while reading event's ABI"
    );

    let types = signature_iter.map(from_bytes_string).collect::<Vec<Type>>();

    Ok((maybe_name.to_vec(), types))
}

pub fn extract_property_names_from_signature_as_bytes(
    signature: Vec<u8>,
) -> Result<(Vec<u8>, Vec<Vec<u8>>), &'static str> {
    const BEGIN_ARGS_CHAR: u8 = b'(';
    const END_ARGS_CHAR: u8 = b')';
    const COMMA_SEPARATOR: u8 = b',';

    let mut signature_iter = signature
        .as_slice()
        .split(|x| x.eq(&BEGIN_ARGS_CHAR) || x.eq(&COMMA_SEPARATOR) || x.eq(&END_ARGS_CHAR))
        .filter(|&x| !x.is_empty());

    let maybe_name = signature_iter.next().unwrap_or(&[]);

    ensure!(
        !maybe_name.is_empty(),
        "Can't find a name while reading event's ABI"
    );

    let property_names = signature_iter
        .map(|arg_candidate| arg_candidate.to_vec())
        .collect::<Vec<Vec<u8>>>();

    Ok((maybe_name.to_vec(), property_names))
}

pub fn decode_buf2val<D: Decode>(buf: Vec<u8>) -> Result<D, &'static str> {
    D::decode(&mut &buf[..]).map_err(|_| "Decoding error: decode_buf2val")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    #[test]
    fn successfully_creates_signature() {
        let test_types_vec: Vec<Type> = vec![Type::Bytes(64), Type::Address(20), Type::Uint(64)];
        let test_name = b"testName".to_vec();
        let signature_bytes = create_signature(test_name, test_types_vec).unwrap();
        let signature_string = String::from_utf8(signature_bytes).unwrap();

        assert_eq!(signature_string, "testName(bytes,address,uint64)");
    }

    #[test]
    fn successfully_interprets_signature_into_abi_types() {
        let test_signature_bytes = b"testName(bytes,address,uint32)".to_vec();

        let res = from_signature_to_abi(test_signature_bytes).unwrap();
        assert_eq!(
            (
                b"testName".to_vec(),
                vec![Type::DynamicBytes, Type::Address(20), Type::Uint(32),],
            ),
            res
        );
    }

    #[test]
    fn successfully_abi_evals_empty_insurance_and_reward_optional_insurance() {
        let empty_bytes: Vec<u8> = vec![];
        let res = eval_to_encoded(Type::OptionalInsurance, empty_bytes.clone());
        assert_eq!(res, Ok(empty_bytes));
    }

    #[test]
    fn successfully_abi_evals_valid_insurance_and_reward_optional_insurance() {
        let valid_insurance_args_arr: [u128; 2] = [1, 2];
        let res = eval_to_encoded(Type::OptionalInsurance, valid_insurance_args_arr.encode());
        assert_eq!(
            Ok(vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]),
            res
        );
    }
}
