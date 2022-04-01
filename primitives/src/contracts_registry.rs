use crate::abi::ContractActionDesc;
use crate::contract_metadata::ContractMetadata;
use crate::storage::RawAliveContractInfo;
use crate::transfers::BalanceOf;
use crate::{ChainId, Compose, EscrowTrait};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::Hash;
use sp_runtime::RuntimeDebug;

use crate::Vec;

pub type RegistryContractId<T> = <T as frame_system::Config>::Hash;

/// In some instances, to enable the escrow trait to be configured once, we may need to have an escrowable
/// injected to the pallet. In this case you might have something like:
///
///    /// The pallet that handles the contracts registry, used to fetch contracts by their id
///    type ContractsRegistry: ContractsRegistry<Self, Self::Escrowed>;
///    /// An escrow provider to the registry
///    type Escrowed: EscrowTrait;
pub trait ContractsRegistry<Hash, AccountId, BlockNumber, Escrowed>
where
    Escrowed: EscrowTrait,
{
    type Error;

    fn fetch_contract_by_id(
        contract_id: Hash,
    ) -> Result<RegistryContract<Hash, AccountId, BalanceOf<Escrowed>, BlockNumber>, Self::Error>;

    fn fetch_contracts(
        author: Option<AccountId>,
        metadata: Option<Vec<u8>>,
    ) -> Result<Vec<RegistryContract<Hash, AccountId, BalanceOf<Escrowed>, BlockNumber>>, Self::Error>;
}

/// A preliminary representation of a contract in the onchain registry.
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
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

impl<Hash: Encode, AccountId: Encode, BalanceOf: Encode, BlockNumber: Encode> MaxEncodedLen
    for RegistryContract<Hash, AccountId, BalanceOf, BlockNumber>
{
    fn max_encoded_len() -> usize {
        4096 as usize
    }
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
