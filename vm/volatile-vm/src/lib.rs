//! This module provides a means for executing contracts
//! represented in wasm.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    traits::{Currency, Randomness, Time, UnfilteredDispatchable},
    weights::{GetDispatchInfo, Weight},
    RuntimeDebug,
};

use sp_core::Bytes;

use sp_runtime::{
    traits::{Convert, Hash, Saturating},
    Perbill,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::traits::Get;

use sp_sandbox;
use sp_std::prelude::*;
use t3rn_primitives::{transfers::BalanceOf, EscrowTrait};

pub use crate::pallet::*;

use crate::{
    storage::{AliveContractInfo, ContractInfo, DeletedContract},
    // gas::GasMeter,
    // exec::{Stack as ExecStack, Executable},
    // rent::Rent,
    wasm::PrefabWasmModule,
    weights::WeightInfo,
};
#[macro_use]
pub mod wasm;
pub mod chain_extension;
pub mod exec;
pub mod ext;
pub mod fake_storage;
pub mod fees;
pub mod gas;
pub mod schedule;
pub mod simple_gas;
pub mod simple_schedule_v2;
pub mod storage;
pub mod weights;

pub use crate::exec::Frame;
pub use crate::schedule::Schedule;
pub use crate::simple_schedule_v2::Schedule as SimpleSchedule;

// use self::env_def::ConvertibleToWasm;
use system::Config as SystemTrait;

pub type MomentOf<T> = <<T as EscrowTrait>::Time as Time>::Moment;
pub type AccountIdOf<T> = <T as SystemTrait>::AccountId;
pub type SeedOf<T> = <T as SystemTrait>::Hash;
pub type TopicOf<T> = <T as SystemTrait>::Hash;
pub type BlockNumberOf<T> = <T as SystemTrait>::BlockNumber;

pub type CodeHash<T> = <T as SystemTrait>::Hash;
pub type TrieId = Vec<u8>;

pub struct DisabledDispatchRuntimeCall {}

impl<T: VolatileVM> DispatchRuntimeCall<T> for DisabledDispatchRuntimeCall {
    fn dispatch_runtime_call(
        _module_name: &str,
        _fn_name: &str,
        _input: &[u8],
        _escrow_account: &<T as system::Config>::AccountId,
        _requested: &<T as system::Config>::AccountId,
        _callee: &<T as system::Config>::AccountId,
        _value: BalanceOf<T>,
        _gas: &mut crate::gas::GasMeter<T>,
    ) -> DispatchResult {
        unimplemented!()
    }
}

/// Dispatch calls to runtime requested during execution of WASM Binaries.
pub trait DispatchRuntimeCall<T: VolatileVM> {
    fn dispatch_runtime_call(
        module_name: &str,
        fn_name: &str,
        input: &[u8],
        escrow_account: &<T as system::Config>::AccountId,
        requested: &<T as system::Config>::AccountId,
        callee: &<T as system::Config>::AccountId,
        value: BalanceOf<T>,
        gas: &mut crate::gas::GasMeter<T>,
    ) -> DispatchResult;
}

pub use crate::pallet::Config as VolatileVM;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: system::Config + EscrowTrait + transaction_payment::Config {
        type Event: From<Event<Self>>
            + IsType<<Self as system::Config>::Event>
            + Into<<Self as system::Config>::Event>;

        type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type DispatchRuntimeCall: DispatchRuntimeCall<Self>;

        /// The type of the call stack determines the maximum nesting depth of contract calls.
        ///
        /// The allowed depth is `CallStack::size() + 1`.
        /// Therefore a size of `0` means that a contract cannot use call or instantiate.
        /// In other words only the origin called "root contract" is allowed to execute then.
        type CallStack: smallvec::Array<Item = Frame<Self>>;

        type ContractsLazyLoaded: smallvec::Array<Item = PrefabWasmModule<Self>>;

        type WeightPrice: Convert<Weight, BalanceOf<Self>>;

        type WeightInfo: WeightInfo;

        /// The maximum number of tries that can be queued for deletion.
        #[pallet::constant]
        type DeletionQueueDepth: Get<u32>;

        /// The maximum amount of weight that can be consumed per block for lazy trie removal.
        #[pallet::constant]
        type DeletionWeightLimit: Get<Weight>;

        /// Cost schedule and limits.
        #[pallet::constant]
        type Schedule: Get<Schedule<Self>>;

        /// Number of block delay an extrinsic claim surcharge has.
        ///
        /// When claim surcharge is called by an extrinsic the rent is checked
        /// for current_block - delay
        #[pallet::constant]
        type SignedClaimHandicap: Get<Self::BlockNumber>;

        /// The minimum amount required to generate a tombstone.
        #[pallet::constant]
        type TombstoneDeposit: Get<BalanceOf<Self>>;
        /// The balance every contract needs to deposit to stay alive indefinitely.
        ///
        /// This is different from the [`Self::TombstoneDeposit`] because this only needs to be
        /// deposited while the contract is alive. Costs for additional storage are added to
        /// this base cost.
        ///
        /// This is a simple way to ensure that contracts with empty storage eventually get deleted by
        /// making them pay rent. This creates an incentive to remove them early in order to save rent.
        #[pallet::constant]
        type DepositPerContract: Get<BalanceOf<Self>>;

        /// The balance a contract needs to deposit per storage byte to stay alive indefinitely.
        ///
        /// Let's suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,
        /// then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent.
        /// But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,
        /// then it would pay 500 BU/day.
        #[pallet::constant]
        type DepositPerStorageByte: Get<BalanceOf<Self>>;

        /// The balance a contract needs to deposit per storage item to stay alive indefinitely.
        ///
        /// It works the same as [`Self::DepositPerStorageByte`] but for storage items.
        #[pallet::constant]
        type DepositPerStorageItem: Get<BalanceOf<Self>>;

        /// The fraction of the deposit that should be used as rent per block.
        ///
        /// When a contract hasn't enough balance deposited to stay alive indefinitely it needs
        /// to pay per block for the storage it consumes that is not covered by the deposit.
        /// This determines how high this rent payment is per block as a fraction of the deposit.
        #[pallet::constant]
        type RentFraction: Get<Perbill>;

        /// Reward that is received by the party whose touch has led
        /// to removal of a contract.
        #[pallet::constant]
        type SurchargeReward: Get<BalanceOf<Self>>;

        ///Type that allows the runtime authors to add new host functions for a contract to call.
        type ChainExtension: chain_extension::ChainExtension<Self>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::error]
    pub enum Error<T> {
        /// A new schedule must have a greater version than the current one.
        InvalidScheduleVersion,
        /// An origin must be signed or inherent and auxiliary sender only provided on inherent.
        InvalidSurchargeClaim,
        /// Cannot restore from nonexisting or tombstone contract.
        InvalidSourceContract,
        /// Cannot restore to nonexisting or alive contract.
        InvalidDestinationContract,
        /// Tombstones don't match.
        InvalidTombstone,
        /// An origin TrieId written in the current block.
        InvalidContractOrigin,
        /// The executed contract exhausted its gas limit.
        OutOfGas,
        /// The output buffer supplied to a contract API call was too small.
        OutputBufferTooSmall,
        /// Performing the requested transfer would have brought the contract below
        /// the subsistence threshold. No transfer is allowed to do this in order to allow
        /// for a tombstone to be created. Use `seal_terminate` to remove a contract without
        /// leaving a tombstone behind.
        BelowSubsistenceThreshold,
        /// The newly created contract is below the subsistence threshold after executing
        /// its contructor. No contracts are allowed to exist below that threshold.
        NewContractNotFunded,
        /// Performing the requested transfer failed for a reason originating in the
        /// chosen currency implementation of the runtime. Most probably the balance is
        /// too low or locks are placed on it.
        TransferFailed,
        /// Performing a call was denied because the calling depth reached the limit
        /// of what is specified in the schedule.
        MaxCallDepthReached,
        /// No contract was found at the specified address.
        ContractNotFound,
        /// A tombstone exist at the specified address.
        ///
        /// Tombstone cannot be called. Anyone can use `seal_restore_to` in order to revive
        /// the contract, though.
        ContractIsTombstone,
        /// The called contract does not have enough balance to pay for its storage.
        ///
        /// The contract ran out of balance and is therefore eligible for eviction into a
        /// tombstone. Anyone can evict the contract by submitting a `claim_surcharge`
        /// extrinsic. Alternatively, a plain balance transfer can be used in order to
        /// increase the contracts funds so that it can be called again.
        RentNotPaid,
        /// The code supplied to `instantiate_with_code` exceeds the limit specified in the
        /// current schedule.
        CodeTooLarge,
        /// No code could be found at the supplied code hash.
        CodeNotFound,
        /// A buffer outside of sandbox memory was passed to a contract API function.
        OutOfBounds,
        /// Input passed to a contract API function failed to decode as expected type.
        DecodingFailed,
        /// Contract trapped during execution.
        ContractTrapped,
        /// The size defined in `T::MaxValueSize` was exceeded.
        ValueTooLarge,
        /// Termination of a contract is not allowed while the contract is already
        /// on the call stack. Can be triggered by `seal_terminate` or `seal_restore_to.
        TerminatedWhileReentrant,
        /// `seal_call` forwarded this contracts input. It therefore is no longer available.
        InputForwarded,
        /// The subject passed to `seal_random` exceeds the limit.
        RandomSubjectTooLong,

        /// The amount of topics passed to `seal_deposit_events` exceeds the limit.
        TooManyTopics,
        /// The topics passed to `seal_deposit_events` contains at least one duplicate.
        DuplicateTopics,
        /// The chain does not provide a chain extension. Calling the chain extension results
        /// in this error. Note that this usually  shouldn't happen as deploying such contracts
        /// is rejected.
        NoChainExtension,
        /// Removal of a contract failed because the deletion queue is full.
        ///
        /// This can happen when either calling [`Pallet::claim_surcharge`] or `seal_terminate`.
        /// The queue is filled by deleting contracts and emptied by a fixed amount each block.
        /// Trying again during another block is the only way to resolve this issue.
        DeletionQueueFull,
        /// A contract could not be evicted because it has enough balance to pay rent.
        ///
        /// This can be returned from [`Pallet::claim_surcharge`] because the target
        /// contract has enough balance to pay for its rent.
        ContractNotEvictable,
        /// A storage modification exhausted the 32bit type that holds the storage size.
        ///
        /// This can either happen when the accumulated storage in bytes is too large or
        /// when number of storage items is too large.
        StorageExhausted,
        /// A contract with the same AccountId already exists.
        DuplicateContract,
        /// A contract self destructed in its constructor.
        ///
        /// This can be triggered by a call to `seal_terminate` or `seal_restore_to`.
        TerminatedInConstructor,
        /// The debug message specified to `seal_debug_message` does contain invalid UTF-8.
        DebugMessageInvalidUTF8,
        /// A call tried to invoke a contract that is flagged as non-reentrant.
        ReentranceDenied,
        /// Target changes to an external one that causes execution to break and messages grouped in round.
        TargetChangeAndRoundFinished,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> where T::AccountId: AsRef<[u8]> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        /// An event deposited upon execution of a contract from the account.
        /// \[escrow_account, requester_account, data\]
        VolatileVMEmitted(T::AccountId, T::AccountId, Vec<u8>),
        /// A custom event emitted by the contract.
        /// \[contract, data\]
        ///
        /// # Params
        ///
        /// - `contract`: The contract that emitted the event.
        /// - `data`: Data supplied by the contract. Metadata generated during contract
        ///           compilation is needed to decode it.
        ContractEmitted(T::AccountId, Vec<u8>),

        /// Contract deployed by address at the specified address. \[deployer, contract\]
        TempInstantiated(T::AccountId, T::AccountId),

        /// Contract deployed by address at the specified address. \[deployer, contract\]
        Instantiated(T::AccountId, T::AccountId),

        /// Contract has been evicted and is now in tombstone state. \[contract\]
        Evicted(T::AccountId),

        /// Contract has been terminated without leaving a tombstone.
        /// \[contract, beneficiary\]
        ///
        /// # Params
        ///
        /// - `contract`: The contract that was terminated.
        /// - `beneficiary`: The account that received the contracts remaining balance.
        ///
        /// # Note
        ///
        /// The only way for a contract to be removed without a tombstone and emitting
        /// this event is by calling `seal_terminate`.
        Terminated(T::AccountId, T::AccountId),

        /// Restoration of a contract has been successful.
        /// \[restorer, dest, code_hash, rent_allowance\]
        ///
        /// # Params
        ///
        /// - `restorer`: Account ID of the restoring contract.
        /// - `dest`: Account ID of the restored contract.
        /// - `code_hash`: Code hash of the restored contract.
        /// - `rent_allowance`: Rent allowance of the restored contract.
        Restored(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),

        /// Code with the specified hash has been stored. \[code_hash\]
        CodeStored(T::Hash),

        /// Triggered when the current schedule is updated.
        /// \[version\]
        ///
        /// # Params
        ///
        /// - `version`: The version of the newly set schedule.
        ScheduleUpdated(u32),

        /// A code with the specified hash was removed.
        /// \[code_hash\]
        ///
        /// This happens when the last contract that uses this code hash was removed or evicted.
        CodeRemoved(T::Hash),
    }

    /// A mapping from an original code hash to the original code, untouched by instrumentation.
    #[pallet::storage]
    pub(crate) type PristineCode<T: Config> = StorageMap<_, Identity, CodeHash<T>, Vec<u8>>;

    /// The subtrie counter.
    #[pallet::storage]
    pub(crate) type AccountCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// The code associated with a given account.
    ///
    /// TWOX-NOTE: SAFE since `AccountId` is a secure hash.
    #[pallet::storage]
    pub(crate) type ContractInfoOf<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, ContractInfo<T>>;

    /// Evicted contracts that await child trie deletion.
    ///
    /// Child trie deletion is a heavy operation depending on the amount of storage items
    /// stored in said trie. Therefore this operation is performed lazily in `on_initialize`.
    #[pallet::storage]
    pub(crate) type DeletionQueue<T: Config> = StorageValue<_, Vec<DeletedContract>, ValueQuery>;

    /// A mapping between an original code hash and instrumented wasm code, ready for execution.
    #[pallet::storage]
    pub(crate) type DryRunCodeCandidates<T: Config> =
        StorageMap<_, Identity, CodeHash<T>, PrefabWasmModule<T>>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {
    /// Subsistence threshold is the extension of the minimum balance (aka existential deposit)
    /// by the tombstone deposit, required for leaving a tombstone.
    ///
    /// Rent or any contract initiated balance transfer mechanism cannot make the balance lower
    /// than the subsistence threshold in order to guarantee that a tombstone is created.
    ///
    /// The only way to completely kill a contract without a tombstone is calling `seal_terminate`.
    pub fn subsistence_threshold() -> BalanceOf<T> {
        T::Currency::minimum_balance().saturating_add(T::TombstoneDeposit::get())
    }

    /// Dry run and put to code candidates - allows instantiate
    pub fn dry_run_contracts() {}

    /// Pre-run contracts to determine expected outbound messages output.
    /// Can't instantiate
    pub fn pre_run_contracts() {}

    /// Post-run contracts - compare received inbound messages with expected output.
    pub fn post_run_contracts() {}

    pub fn get_contract_code_lazy(
        code_hash: CodeHash<T>,
    ) -> Result<PrefabWasmModule<T>, pallet::Error<T>> {
        // ) -> Result<PrefabWasmModule<T>, Error<T>> {
        // ) -> Result<PrefabWasmModule<T>, &'static str> {

        // if !T::ContractsLazyLoaded.any(|f| &f.account_id == id && !f.allows_reentry) {
        //
        // }

        // if T::ContractsLazyLoaded::contains_key(code_hash) {
        //     return T::LoadedContractsCache::get(code_hash);
        // }

        <DryRunCodeCandidates<T>>::get(code_hash).ok_or_else(|| Error::<T>::CodeNotFound)

        // pub fn take_non_fungible(&mut self, id: &AssetId) -> Assets {
        //     let mut taken = Assets::new();
        //     let non_fungible = mem::replace(&mut self.non_fungible, Default::default());
        //     non_fungible.into_iter().for_each(|(c, instance)| {
        //         if &c == id {
        //             taken.non_fungible.insert((c, instance));
        //         } else {
        //             self.non_fungible.insert((c, instance));
        //         }
        //     });
        //     taken
        // }

        // Ok()
    }

    pub fn update_contract_metadata_lazy(
        code_hash: CodeHash<T>,
        // mutate_fn: FnOnce(Option<PrefabWasmModule<T>>)
        mutate_fn: Box<
            dyn FnOnce(&mut Option<PrefabWasmModule<T>>) -> Result<(), pallet::Error<T>>,
        >,
    ) -> Result<(), pallet::Error<T>> {
        // <DryRunCodeCandidates<T>>::mutate(code_hash, mutate_fn)

        // /// Mutate the item, only if an `Ok` value is returned. Deletes the item if mutated to a `None`.
        // fn try_mutate_exists<KeyArg: EncodeLike<K>, R, E, F: FnOnce(&mut Option<V>) -> Result<R, E>>(
        //     key: KeyArg,
        //     f: F,
        // ) -> Result<R, E>;

        <DryRunCodeCandidates<T>>::try_mutate_exists(code_hash, mutate_fn)

        // pub fn take_non_fungible(&mut self, id: &AssetId) -> Assets {
        //     let mut taken = Assets::new();
        //     let non_fungible = mem::replace(&mut self.non_fungible, Default::default());
        //     non_fungible.into_iter().for_each(|(c, instance)| {
        //         if &c == id {
        //             taken.non_fungible.insert((c, instance));
        //         } else {
        //             self.non_fungible.insert((c, instance));
        //         }
        //     });
        //     taken
        // }
        // ()
    }

    pub fn add_contract_code_lazy(code_hash: CodeHash<T>, contract_module: PrefabWasmModule<T>) {
        <DryRunCodeCandidates<T>>::insert(code_hash, contract_module);
        // pub fn take_non_fungible(&mut self, id: &AssetId) -> Assets {
        //     let mut taken = Assets::new();
        //     let non_fungible = mem::replace(&mut self.non_fungible, Default::default());
        //     non_fungible.into_iter().for_each(|(c, instance)| {
        //         if &c == id {
        //             taken.non_fungible.insert((c, instance));
        //         } else {
        //             self.non_fungible.insert((c, instance));
        //         }
        //     });
        //     taken
        // }
        // <CodeStorage<T>>::insert(prefab_module.code_hash, prefab_module);
    }

    pub fn remove_contract_code_lazy(code_hash: CodeHash<T>) {
        // <CodeStorage<T>>::remove(prefab_module.code_hash);
        <DryRunCodeCandidates<T>>::remove(code_hash);
    }

    /// Determine the address of a contract,
    ///
    /// This is the address generation function used by contract instantiation. Its result
    /// is only dependend on its inputs. It can therefore be used to reliably predict the
    /// address of a contract. This is akin to the formular of eth's CREATE2 opcode. There
    /// is no CREATE equivalent because CREATE2 is strictly more powerful.
    ///
    /// Formula: `hash(deploying_address ++ code_hash ++ salt)`
    pub fn contract_address(
        deploying_address: &T::AccountId,
        code_hash: &CodeHash<T>,
        salt: &[u8],
    ) -> T::AccountId {
        let buf: Vec<_> = deploying_address
            .encode()
            .iter()
            .chain(code_hash.as_ref())
            .chain(salt)
            .cloned()
            .collect();

        T::AccountId::decode(&mut &T::Hashing::hash(&buf).encode()[..])
            .expect("Hash should deserialize tp 32b account")
    }
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
// #[codec(compact)]
pub struct DeferredStorageWrite {
    pub trie_id: Vec<u8>,
    pub key: [u8; 32],
    pub value: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Default, Clone)]
// #[codec(compact)]
pub struct CallStamp {
    pub pre_storage: Vec<u8>,

    pub post_storage: Vec<u8>,

    pub dest: Vec<u8>,
}

/// A prepared wasm module ready for execution.
#[derive(Clone, Encode, Decode)]
pub struct SimplePrefabWasmModule {
    /// Version of the schedule with which the code was instrumented.
    #[codec(compact)]
    schedule_version: u32,
    #[codec(compact)]
    initial: u32,
    #[codec(compact)]
    maximum: u32,
    /// This field is reserved for future evolution of format.
    ///
    /// Basically, for now this field will be serialized as `None`. In the future
    /// we would be able to extend this structure with.
    _reserved: Option<()>,
    /// Code instrumented with the latest schedule.
    code: Vec<u8>,
}

/// Wasm executable loaded by `WasmLoader` and executed by `WasmVm`.
pub struct WasmExecutable {
    pub entrypoint_name: &'static str,
    pub prefab_module: SimplePrefabWasmModule,
}

use bitflags::bitflags;

pub type StorageKey = [u8; 32];

/// Error returned by contract exection.
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct ExecError {
    /// The reason why the execution failed.
    pub error: DispatchError,
    /// Origin of the error.
    pub origin: ErrorOrigin,
}

impl<T: Into<DispatchError>> From<T> for ExecError {
    fn from(error: T) -> Self {
        Self {
            /// The reason why the execution failed.
            error: error.into(),
            /// Origin of the error.
            origin: ErrorOrigin::Caller,
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub enum ErrorOrigin {
    /// The error happened in the current exeuction context rather than in the one
    /// of the contract that is called into.
    Caller,
    /// The error happened during execution of the called contract.
    Callee,
}

bitflags! {
    /// Flags used by a contract to customize exit behaviour.
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase", transparent))]
    pub struct ReturnFlags: u32 {
        /// If this bit is set all changes made by the contract exection are rolled back.
        const REVERT = 0x0000_0001;
        /// If this bit is set all changes made by the contract exection are rolled back.
        const FOREIGN_TARGET = 0x0000_0002;
    }
}
#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ExecReturnValue {
    /// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
    pub flags: ReturnFlags,
    /// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
    pub data: Bytes,
}

impl ExecReturnValue {
    /// We understand the absense of a revert flag as success.
    pub fn is_success(&self) -> bool {
        !self.flags.contains(ReturnFlags::REVERT)
    }
}

pub type ExecResult = Result<ExecReturnValue, ExecError>;

pub type StackTrace = Vec<StackTraceEntry>;

#[derive(RuntimeDebug, PartialEq, Eq, Clone)]
pub struct StackTraceEntry {
    pub host_fn_name: &'static str,
    pub arguments_list: &'static str,
}

pub type ExecResultTrace = Result<(ExecReturnValue, StackTrace), ExecError>;

pub enum TrapReason {
    /// The supervisor trapped the contract because of an error condition occurred during
    /// execution in privileged code.
    SupervisorError(DispatchError),
    /// Signals that trap was generated in response to call `seal_return` host function.
    Return(ReturnData),
    /// Signals that a trap was generated in response to a successful call to the
    /// `seal_terminate` host function.
    Termination,
    /// Signals that a trap was generated because of a successful restoration.
    Restoration,
}

/// Every error that can be returned to a contract when it calls any of the host functions.
#[repr(u32)]
pub enum ReturnCode {
    /// API call successful.
    Success = 0,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped = 1,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted = 2,
    /// The passed key does not exist in storage.
    KeyNotFound = 3,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold = 4,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed = 5,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded = 6,
    /// No code could be found at the supplied code hash.
    CodeNotFound = 7,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable = 8,
}

// impl ConvertibleToWasm for ReturnCode {
//     type NativeType = Self;
//     const VALUE_TYPE: ValueType = ValueType::I32;
//     fn to_typed_value(self) -> sp_sandbox::Value {
//         sp_sandbox::Value::I32(self as i32)
//     }
//     fn from_typed_value(_: sp_sandbox::Value) -> Option<Self> {
//         debug_assert!(
//             false,
//             "We will never receive a ReturnCode but only send it to wasm."
//         );
//         None
//     }
// }

/// The data passed through when a contract uses `seal_return`.
pub struct ReturnData {
    /// The flags as passed through by the contract. They are still unchecked and
    /// will later be parsed into a `ReturnFlags` bitflags struct.
    pub flags: u32,
    /// The output buffer passed by the contract as return data.
    pub data: Vec<u8>,
}
