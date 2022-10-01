use crate::{
    account_manager::Outcome,
    circuit::LocalStateExecutionView,
    contract_metadata::ContractType,
    contracts_registry::{AuthorInfo, RegistryContract},
};
use codec::{Decode, Encode};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::{result::Result, vec::Vec};
use t3rn_sdk_primitives::{
    signal::{ExecutionSignal, Signaller},
    state::SideEffects,
};

#[derive(Encode, Decode)]
pub struct GetState<T: frame_system::Config> {
    pub xtx_id: Option<T::Hash>,
}

#[derive(Encode, Decode)]
pub enum PrecompileArgs<T, Balance>
where
    T: frame_system::Config,
    Balance: Encode + Decode,
{
    GetState(T::Origin, GetState<T>),
    SubmitSideEffects(T::Origin, SideEffects<T::AccountId, Balance, T::Hash>),
    Signal(T::Origin, ExecutionSignal<T::Hash>),
}

/// The happy return type of an invocation
pub enum PrecompileInvocation<T: frame_system::Config, Balance> {
    GetState(LocalStateExecutionView<T, Balance>),
    Submit,
    Signal,
}

impl<T: frame_system::Config, Balance> PrecompileInvocation<T, Balance> {
    pub fn get_state(&self) -> Option<&LocalStateExecutionView<T, Balance>> {
        match self {
            PrecompileInvocation::GetState(state) => Some(state),
            _ => None,
        }
    }
}

pub trait Precompile<T, Balance>
where
    T: frame_system::Config,
    Balance: Encode + Decode,
{
    /// Looks up a precompile function pointer
    fn lookup(dest: &T::Hash) -> Option<u8>;

    /// Invoke a precompile, providing raw bytes and a pointer
    fn invoke_raw(precompile: &u8, args: &[u8], output: &mut Vec<u8>);

    /// Invoke a precompile
    fn invoke(
        args: PrecompileArgs<T, Balance>,
    ) -> Result<PrecompileInvocation<T, Balance>, DispatchError>;
}

pub trait LocalStateAccess<T, Balance>
where
    T: frame_system::Config,
{
    fn load_local_state(
        origin: &T::Origin,
        xtx_id: Option<&T::Hash>,
    ) -> Result<LocalStateExecutionView<T, Balance>, DispatchError>;
}

pub struct Remunerated<Hash> {
    pub remuneration_id: Option<Hash>,
}

impl<Hash> Default for Remunerated<Hash> {
    fn default() -> Self {
        Remunerated {
            remuneration_id: None,
        }
    }
}

impl<Hash> Remunerated<Hash> {
    pub fn new(id: Option<Hash>) -> Self {
        Remunerated {
            remuneration_id: id,
        }
    }
}

pub trait Remuneration<T: frame_system::Config, Balance> {
    /// Try to remunerate the fees from the given module
    fn try_remunerate<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError>;

    /// Try to remunerate the fees from the given module with a custom balance
    fn try_remunerate_exact<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        amount: Balance,
        module: &Module,
    ) -> Result<Remunerated<T::Hash>, sp_runtime::DispatchError>;

    /// Try to finalize a ledger item with an reason
    fn try_finalize(ledger_id: T::Hash, outcome: Outcome) -> DispatchResult;
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

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
pub enum SignalOpcode {
    Initiated,
    Bounced,
}

pub trait ThreeVm<T, Balance>:
    Precompile<T, Balance>
    + Signaller<T::Hash, Result = Result<SignalOpcode, DispatchError>>
    + Remuneration<T, Balance>
where
    T: frame_system::Config,
    Balance: Encode + Decode,
{
    fn peek_registry(
        id: &T::Hash,
    ) -> Result<RegistryContract<T::Hash, T::AccountId, Balance, T::BlockNumber>, DispatchError>;

    /// Allows creating a `Module` from a binary blob from the contracts registry
    fn from_registry<Module, ModuleGen>(
        id: &T::Hash,
        module_generator: ModuleGen,
    ) -> Result<Module, DispatchError>
    where
        Module: ModuleOperations<T, Balance>,
        ModuleGen: Fn(Vec<u8>) -> Module;

    fn instantiate_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn storage_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn volatile_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn remunerable_check(kind: &ContractType) -> Result<(), DispatchError>;

    fn try_persist_author(
        contract: &T::AccountId,
        author: Option<&AuthorInfo<T::AccountId, Balance>>,
    ) -> Result<(), DispatchError>;

    fn try_remove_author(contract: &T::AccountId) -> Result<(), DispatchError>;
}

pub trait ModuleOperations<T: frame_system::Config, Balance> {
    fn get_bytecode(&self) -> &Vec<u8>;
    fn get_author(&self) -> Option<&AuthorInfo<T::AccountId, Balance>>;
    fn set_author(&mut self, author: AuthorInfo<T::AccountId, Balance>);
    fn get_type(&self) -> &ContractType;
    fn set_type(&mut self, kind: ContractType);
}
