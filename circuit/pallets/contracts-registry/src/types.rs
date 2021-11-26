// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Hash;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use t3rn_primitives::abi::ContractActionDesc;
use t3rn_primitives::contract_metadata::ContractMetadata;
use t3rn_primitives::Compose;
pub use t3rn_primitives::RegistryContractId;
use volatile_vm::storage::RawAliveContractInfo;

type ChainId = [u8; 4];

/// A preliminary representation of a contract in the onchain registry.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct RegistryContract<Hash, AccountId, BalanceOf, BlockNumber> {
    /// Original code text
    pub code_txt: Vec<u8>,
    /// Bytecode
    pub bytes: Vec<u8>,
    /// Original code author
    pub author: AccountId,
    /// Optional remuneration fee for the author
    pub author_fees_per_single_use: Option<BalanceOf>,
    /// Optional ABI
    pub abi: Option<Vec<u8>>,
    /// Action descriptions (calls for now)
    pub action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
    /// Contracts Info after Contracts Pallet
    pub info: Option<RawAliveContractInfo<Hash, BalanceOf, BlockNumber>>,
    /// Contract metadata to be used in queries
    pub meta: ContractMetadata,
}

impl<Hash: Encode, AccountId: Encode, BalanceOf: Encode, BlockNumber: Encode>
    RegistryContract<Hash, AccountId, BalanceOf, BlockNumber>
{
    pub fn new(
        code_txt: Vec<u8>,
        bytes: Vec<u8>,
        author: AccountId,
        author_fees_per_single_use: Option<BalanceOf>,
        abi: Option<Vec<u8>>,
        action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
        info: Option<RawAliveContractInfo<Hash, BalanceOf, BlockNumber>>,
        meta: ContractMetadata,
    ) -> Self {
        RegistryContract {
            code_txt,
            bytes,
            author,
            author_fees_per_single_use,
            abi,
            action_descriptions,
            info,
            meta,
        }
    }

    pub fn generate_id<T: frame_system::Config>(&self) -> RegistryContractId<T> {
        let mut protocol_part_of_contract = self.code_txt.clone();
        protocol_part_of_contract.extend(self.bytes.clone());
        T::Hashing::hash(Encode::encode(&protocol_part_of_contract).as_ref())
    }

    pub fn from_compose(
        compose: Compose<AccountId, BalanceOf>,
        action_descriptions: Vec<ContractActionDesc<Hash, ChainId, AccountId>>,
        author: AccountId,
        author_fees_per_single_use: Option<BalanceOf>,
        abi: Option<Vec<u8>>,
        info: Option<RawAliveContractInfo<Hash, BalanceOf, BlockNumber>>,
        meta: ContractMetadata,
    ) -> RegistryContract<Hash, AccountId, BalanceOf, BlockNumber> {
        RegistryContract::new(
            compose.code_txt,
            compose.bytes,
            author,
            author_fees_per_single_use,
            abi,
            action_descriptions,
            info,
            meta,
        )
    }
}

/// The possible errors that can happen querying the storage of a contract.
#[derive(Eq, PartialEq, Encode, Decode, Debug, Clone)]
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

#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ContractsRegistryResult<T> {
    pub gas_consumed: u64,
    pub result: T,
    pub flags: u32,
}

pub type FetchContractsResult = ContractsRegistryResult<Result<Vec<Vec<u8>>, ContractAccessError>>;
