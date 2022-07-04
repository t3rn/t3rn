use crate::{
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
pub enum PrecompileArgs<T: frame_system::Config> {
    GetState(T::Origin, GetState<T>),
    SubmitSideEffects(T::Origin, SideEffects<T::AccountId, u128, T::Hash>),
    Signal(T::Origin, ExecutionSignal<T::Hash>),
}

/// The happy return type of an invocation
pub enum PrecompileInvocation<T: frame_system::Config> {
    GetState(LocalStateExecutionView<T>),
    Submit,
    Signal,
}

impl<T: frame_system::Config> PrecompileInvocation<T> {
    pub fn get_state(&self) -> Option<&LocalStateExecutionView<T>> {
        match self {
            PrecompileInvocation::GetState(state) => Some(state),
            _ => None,
        }
    }
}

pub trait Precompile<T>
where
    T: frame_system::Config,
{
    /// Looks up a precompile function pointer
    fn lookup(dest: &T::Hash) -> Option<u8>;

    /// Invoke a precompile, providing raw bytes and a pointer
    fn invoke_raw(precompile: &u8, args: &[u8], output: &mut Vec<u8>);

    /// Invoke a precompile
    fn invoke(args: PrecompileArgs<T>) -> Result<PrecompileInvocation<T>, DispatchError>;
}

pub trait LocalStateAccess<T>
where
    T: frame_system::Config,
{
    fn load_local_state(
        origin: &T::Origin,
        xtx_id: Option<&T::Hash>,
    ) -> Result<LocalStateExecutionView<T>, DispatchError>;
}

pub trait Remuneration<T: frame_system::Config, Balance> {
    /// Try to remunerate the fees from the given module
    fn try_remunerate<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        module: &Module,
    ) -> DispatchResult;

    /// Try to remunerate the fees from the given module with a custom balance
    fn try_remunerate_exact<Module: ModuleOperations<T, Balance>>(
        payee: &T::AccountId,
        amount: Balance,
        module: &Module,
    ) -> DispatchResult;
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

#[derive(Encode, Decode, Debug, PartialEq)]
pub enum SignalOpcode {
    Initiated,
    Bounced,
}

pub trait ThreeVm<T, Balance>:
    Precompile<T>
    + Signaller<T::Hash, Result = Result<SignalOpcode, DispatchError>>
    + Remuneration<T, Balance>
where
    T: frame_system::Config,
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

    fn try_persist_author<Module: ModuleOperations<T, Balance>>(
        contract: &T::AccountId,
        module: &Module,
    ) -> Result<(), DispatchError>;
}

pub trait ModuleOperations<T: frame_system::Config, Balance> {
    fn get_bytecode(&self) -> &Vec<u8>;
    fn get_author(&self) -> Option<&AuthorInfo<T::AccountId, Balance>>;
    fn set_author(&mut self, author: AuthorInfo<T::AccountId, Balance>);
    fn get_type(&self) -> &ContractType;
    fn set_type(&mut self, kind: ContractType);
}
