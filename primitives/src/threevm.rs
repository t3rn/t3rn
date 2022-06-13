use std::fmt::Debug;
use codec::{Decode, Encode};
use sp_std::vec::Vec;
use t3rn_sdk_primitives::signal::Signaller;

// TODO: genesis
// TODO: storage
trait Precompile<Hash> {
    /// Looks up a precompile function pointer
    fn lookup(dest: &Hash) -> Option<u8>;

    /// Invoke a precompile
    fn invoke(precompile: &u8);
}

trait LocalStateAccess {}

trait Remuneration<Hash, AccountId, Balance> {
    fn remunerate(xtx_id: &Hash, author: &AccountId, amount: Balance);
}

enum Characteristic {
    Storage,
    Instantiate,
    Remuneration,
    Volatile,
}

/// Passthrough to validator
trait CharacteristicValidator {
    fn validate(characteristic: &Characteristic) -> Result<(), ()>; // TODO: handle error
}

trait ThreeVm<Hash, AccountId, Balance, R>:
Precompile<Hash> +
Signaller<Hash, (), R> +
Remuneration<Hash, AccountId, Balance>
    where
        Hash: Encode + Decode + Debug + Clone + Eq + PartialEq,
        R: Encode + Decode + Debug + Clone + Eq + PartialEq
{ //TODO: R from 3vm current impl

    /// Allows creating a `Module` from a binary blob from the contracts registry
    fn from_registry_blob<T>(blob: Vec<u8>) -> T;

    /// Lookup the author for a contract
    fn lookup_author(contract: &Hash);
}