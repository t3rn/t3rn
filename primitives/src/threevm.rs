use codec::{Decode, Encode};
use sp_std::vec::Vec;
use std::fmt::Debug;
use t3rn_sdk_primitives::signal::Signaller;

pub trait Precompile<T>
where
    T: frame_system::Config,
{
    /// Looks up a precompile function pointer
    fn lookup(dest: &T::Hash) -> Option<u8>;

    /// Invoke a precompile
    fn invoke(precompile: &u8);
}

pub trait LocalStateAccess<T>
where
    T: frame_system::Config,
{
}

pub trait Remuneration<T: frame_system::Config, Balance> {
    fn remunerate(xtx_id: &T::Hash, author: &T::AccountId, amount: Balance);
}

pub enum Characteristic {
    Storage,
    Instantiate,
    Remuneration,
    Volatile,
}

/// Passthrough to validator
pub trait CharacteristicValidator {
    fn validate(characteristic: &Characteristic) -> Result<(), ()>; // TODO: handle error
}

pub trait ThreeVm<T, Balance, R>:
    Precompile<T> + Signaller<T::Hash, (), R> + Remuneration<T, Balance>
where
    T: frame_system::Config,
    R: Encode + Decode + Debug + Clone + Eq + PartialEq,
{
    //TODO: R from 3vm current impl

    /// Allows creating a `Module` from a binary blob from the contracts registry
    fn from_registry_blob<Module>(blob: Vec<u8>) -> Module;

    /// Lookup the author for a contract
    fn lookup_author(contract: &T::Hash);
}
