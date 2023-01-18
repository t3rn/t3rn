use sp_runtime::RuntimeDebug as Debug;

use codec::{Decode, Encode};
use frame_support::storage::child::ChildInfo;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type CodeHash<T> = <T as frame_system::Config>::Hash;
pub type TrieId = Vec<u8>;

// TODO: this needs to be tied in with 3VM and how it utilises storage. At the moment it just makes its
// own copy of the contract, but since we hold it in the registry, this need to be smarter.
/// Information for managing an account and its sub trie abstraction.
/// This is the required info to cache for an account.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default, TypeInfo)]
pub struct RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Unique ID for the subtree encoded as a bytes vector.
    pub trie_id: TrieId,
    /// The total number of bytes used by this contract.
    ///
    /// It is a sum of each key-value pair stored by this contract.
    pub storage_size: u32,
    /// The total number of key-value pairs in storage of this contract.
    pub pair_count: u32,
    /// The code associated with a given account.
    pub code_hash: CodeHash,
    /// Pay rent at most up to this value.
    pub rent_allowance: Balance,
    /// The amount of rent that was paid by the contract over its whole lifetime.
    ///
    /// A restored contract starts with a value of zero just like a new contract.
    pub rent_paid: Balance,
    /// Last block rent has been paid.
    pub deduct_block: BlockNumber,
    /// Last block child storage has been written.
    pub last_write: Option<BlockNumber>,
    /// This field is reserved for future evolution of format.
    pub _reserved: Option<()>,
}

impl<CodeHash, Balance, BlockNumber> RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Associated child trie unique id is built from the hash part of the trie id.
    pub fn child_trie_info(&self) -> ChildInfo {
        child_trie_info(&self.trie_id[..])
    }
}

/// Associated child trie unique id is built from the hash part of the trie id.
fn child_trie_info(trie_id: &[u8]) -> ChildInfo {
    ChildInfo::new_default(trie_id)
}
