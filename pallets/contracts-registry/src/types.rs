use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
pub use t3rn_primitives::contracts_registry::{RegistryContract, RegistryContractId};
/// The possible errors that can happen querying the storage of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// The specified contract is a tombstone and thus cannot have any storage.
    IsTombstone,
}

impl From<ContractAccessError> for i64 {
    fn from(e: ContractAccessError) -> i64 {
        match e {
            ContractAccessError::DoesntExist => 1,
            ContractAccessError::IsTombstone => 2,
        }
    }
}

#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ContractsRegistryResult<T> {
    pub gas_consumed: u64,
    pub result: T,
    pub flags: u32,
}

pub type FetchContractsResult = ContractsRegistryResult<Result<Vec<u8>, ContractAccessError>>;
