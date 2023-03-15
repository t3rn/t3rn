use codec::{Decode, Encode};
use scale_info::{
    prelude::{boxed::Box, fmt::Debug, vec, vec::Vec},
    TypeInfo,
};

#[cfg(feature = "runtime")]
use scale_info::prelude::any::Any;

#[cfg(feature = "runtime")]
use primitive_types::U256;

#[cfg(feature = "runtime")]
use crate::types::Bytes;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::DispatchError;

#[cfg(feature = "runtime")]
use sp_runtime::RuntimeString;

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
pub struct GatewayABISpecs {
    /// block number type in bytes
    pub block_number_size: u8,
    /// hash size in bytes
    pub hash_size: u8,
    /// hashing algorithm
    pub hasher: u8,
    /// cryptography algorithm
    pub crypto: u8,
    /// address length in bytes
    pub address_size: u8,
    /// value length in bytes
    pub value_type_size: u8,
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
}

impl Default for GatewayABIConfig {
    fn default() -> GatewayABIConfig {
        GatewayABIConfig {
            block_number_type_size: 32,
            hash_size: 32,
            hasher: HasherAlgo::Blake2,
            crypto: CryptoAlgo::Sr25519,
            address_length: 32,  // 32 bytes : 32 * 8 = 256 bits
            value_type_size: 16, // u128 = 16 bytes = 128 bits.
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
