#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use bitflags::bitflags;
use frame_support::{
    ensure,
    storage::child,
    storage::child::ChildInfo,
    traits::{Currency, Time},
};
use frame_system::ensure_signed;

use sp_runtime::{
    traits::{Hash, Saturating},
    DispatchResult,
};
use sp_std::convert::TryInto;
use sp_std::vec;
use sp_std::vec::Vec;
use versatile_wasm::{
    ext::DefaultRuntimeEnv,
    gas::{Gas, GasMeter},
    prepare::prepare_contract,
    runtime::{
        get_child_storage_for_current_execution, raw_escrow_call, CallStamp, DeferredStorageWrite,
        Env,
    },
    ExecResult, WasmExecutable,
};

use t3rn_primitives::{
    transfers::{just_transfer, BalanceOf, TransferEntry},
    EscrowTrait,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

// Bits of transfer flags associated with types on-gateway of transfer.
bitflags! {

    /// Flags used by a contract to customize transfers.
    #[derive(Encode, Decode)]
    pub struct TransferFlags: u8 {
        const DIRTY = 0b00000001;
        const ESCROWED_EXECUTE = 0b00000010;
        const ESCROWED_COMMIT = 0b00000100;
        const ESCROWED_REVERT = 0b00001000;
    }
}

// Bits of storage flags associated with types on-gateway of transfer.
bitflags! {

    /// Flags used by a contract to customize transfers.
    #[derive(Encode, Decode)]
    pub struct StorageFlags: u8 {
        const CHILD = 0b00000001;
        const GLOBAL = 0b00000010;
        const ESCROWED_EXECUTE = 0b00000100;
        const ESCROWED_COMMIT = 0b00001000;
        const ESCROWED_REVERT = 0b00010000;
    }
}

// Bits of storage flags associated with types on-gateway of transfer.
bitflags! {

    /// Flags used by a contract to customize calls.
    #[derive(Encode, Decode)]
    pub struct CallFlags: u8 {
        const TRANSFER_ONLY = 0b00000001;
        const ESCROWED_EXECUTE = 0b00000100;
        const ESCROWED_COMMIT = 0b00001000;
        const ESCROWED_REVERT = 0b00010000;
    }
}

pub fn cleanup_failed_execution<T: Config>(
    escrow_account: T::AccountId,
    requester: T::AccountId,
    transfers: &mut Vec<TransferEntry>,
) -> DispatchResult {
    // Give the money back to the requester from the transfers that succeeded.
    for transfer in transfers.iter() {
        just_transfer::<T>(
            &escrow_account,
            &requester,
            BalanceOf::<T>::from(transfer.value),
        )
        .map_err(|e| e)?;
    }
    transfers.clear();
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Default, Clone)]
pub struct ExecutionProofs {
    pub result: Option<Vec<u8>>,
    pub storage: Option<Vec<u8>>,
    pub deferred_transfers: Vec<TransferEntry>,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Default, Clone)]
pub struct ExecutionStamp {
    pub timestamp: u64,
    pub phase: u8,
    pub proofs: Option<ExecutionProofs>,
    pub call_stamps: Vec<CallStamp>,
    pub failure: Option<u8>, // Error Code
}

pub fn execute_code_in_escrow_sandbox<'a, T: Config>(
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    target_dest: &T::AccountId,
    value: BalanceOf<T>,
    code: Vec<u8>,
    input_data: Vec<u8>,
    gas_meter: &'a mut GasMeter<T>,
    transfers: &mut Vec<TransferEntry>,
    deferred_storage_writes: &mut Vec<DeferredStorageWrite>,
    call_stamps: &mut Vec<CallStamp>,
) -> ExecResult {
    // That only works for code that is received by the call and will be executed and cleaned up after.
    let prefab_module = prepare_contract::<Env>(&code).map_err(|e| e)?;

    let executable = WasmExecutable {
        entrypoint_name: "call",
        prefab_module,
    };

    match raw_escrow_call::<T, DefaultRuntimeEnv<T>>(
        &escrow_account.clone(),
        &requester.clone(),
        &target_dest.clone(),
        value,
        gas_meter,
        input_data.clone(),
        transfers,
        deferred_storage_writes,
        call_stamps,
        &executable,
        T::Hashing::hash(&code.clone()),
    ) {
        Ok(exec_ret_val) => Ok(exec_ret_val),
        Err(exec_err) => {
            // Revert the execution effects on the spot.
            cleanup_failed_execution::<T>(escrow_account.clone(), requester.clone(), transfers)
                .map_err(|_e| <Error<T>>::CleanupFailedAfterUnsuccessfulExecution)?;
            Err(exec_err)?
        }
    }
}

// ToDo: Encode errors properly before storing making the below enum obsolete.
#[derive(Clone)]
#[repr(u8)]
pub enum ErrCodes {
    RequesterNotEnoughBalance = 0,

    BalanceTransferFailed = 1,

    FeesRefundFailed = 2,

    PutCodeFailure = 3,

    InitializationFailure = 4,

    ExecutionFailure = 5,

    CallFailure = 6,

    TerminateFailure = 7,
}

pub fn stamp_failed_execution<T: Config>(
    cause_code: u8,
    requester: &T::AccountId,
    code_hash: &T::Hash,
) {
    <ExecutionStamps<T>>::insert(
        requester,
        code_hash,
        ExecutionStamp {
            call_stamps: vec![],
            timestamp: TryInto::<u64>::try_into(<T as EscrowTrait>::Time::now())
                .ok()
                .unwrap(),
            phase: 0,
            proofs: None,
            failure: Option::from(cause_code),
        },
    );
}

#[frame_support::pallet]
pub mod pallet {
    use crate::*;

    use frame_support::{log, pallet_prelude::*, storage::child::kill_storage};

    use frame_system::pallet_prelude::*;
    use versatile_wasm::VersatileWasm;

    use t3rn_primitives::{
        transfers::{
            commit_deferred_transfers, escrow_transfer, just_transfer, BalanceOf, TransferEntry,
        },
        EscrowTrait,
    };
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + VersatileWasm + EscrowTrait + pallet_bridge_messages::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn deferred_transfers)]
    pub type DeferredTransfers<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        Vec<TransferEntry>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn execution_stamps)]
    pub type ExecutionStamps<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::Hash,
        ExecutionStamp,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn deferred_results)]
    pub type DeferredResults<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::Hash,
        Vec<u8>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn deferred_storage_writes)]
    pub type DeferredStorageWrites<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::Hash,
        Vec<DeferredStorageWrite>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Just a dummy event.
        SomethingStored(u32, T::AccountId),

        /// \[timestamp, phase, result, deferred_transfers\]
        RuntimeGatewayVersatileExecutionSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        /// \[timestamp, phase, result, deferred_transfers\]
        RuntimeGatewayVersatileCommitSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        /// \[timestamp, phase, result, deferred_transfers\]
        RuntimeGatewayVersatileRevertSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        /// \[from, to, value, escrow_account\]
        XTransfer(
            T::AccountId,
            T::AccountId,
            BalanceOf<T>,
            Option<T::AccountId>,
        ),

        MultistepCommitResult(Vec<u8>),

        MultistepRevertResult(u32),

        XEmitEvent(Vec<u8>),

        MultistepUnknownPhase(u8),

        RentProjectionCalled(T::AccountId, T::AccountId),

        XGetStorage(Vec<u8>, Vec<u8>),

        XSetStorage(Vec<u8>, Vec<u8>),

        GetStorageResult(Vec<u8>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        RequesterNotEnoughBalance,

        BalanceTransferFailed,

        XBalanceTransferFailed,

        UnknownXCallFlag,

        UnknownXStorageFlag,

        FeesOverflow,

        FeesRefundFailed,

        PutCodeFailure,

        InitializationFailure,

        ExecutionFailure,

        CleanupFailedAfterUnsuccessfulExecution,

        NothingToDo,

        CallFailure,

        CallFailureNotCallable,

        CallFailureCodeNotFound,

        TerminateFailure,

        UnauthorizedCallAttempt,

        CommitOnlyPossibleAfterSuccessfulExecutionPhase,

        CommitPhaseFailedToDeliverTransfers,

        CannotRevertMultipleTimes,

        CleanupFailedDuringRevert,

        DestinationContractStorageChangedSinceExecution,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// **Multi-Step Call**
        /// Executes attached code following the protocol rules that distincts 3 execution phases - EXECUTE, COMMIT, REVERT.
        ///
        /// Execution is secured by the escrow account (must be pre-registered for that parachain),
        /// which holds off the effects to target destinations until the "Commit" phase. The escrow account acts as "sudo" in sudo module.
        ///
        /// Execution results in threefold effects:
        /// * deferred transfers - those are promised to be sent out using escrow account funds in the Commit phase or be returned to the requester in Revert phase
        /// * storage - changes to the storage of target destination contracts. That's the most complex effect to implement as it relies relies on already registered contracts on that parachains and their behaviour.
        /// * results - results returned by execution of contract on that parachain. Execution phase sends back the result's hash to allow forming consensus over its correctness. The commit phase returns actual result.
        ///
        /// Based on those effects, multistep_call can be used in different manners:
        /// * A) For deferring balance transfers:
        ///     * A.1) A single balance transfer to the target_dest can be deferred by calling with empty code and a value
        ///     * A.2) Multiple balance transfers to multiple target destinations by attaching the corresponding contract
        ///     * A.1+2) A single balance transfer can be executed on top of multiple transfers from within the corresponding contract
        /// * B) For attaching and executing "code" within the context of that parachain (and possibly accessing the readonly data of the contracts) and revealing the results only after the Commit phase.
        /// ---
        /// **NOTE:**
        /// As the result is stored, it's accessible from outside of that chain, which for some case
        /// can violate the business logic behind the contracts. This should be fixed by either keeping
        /// the results in memory or elevating responsibility the results management to Gateway Circuit (preferable).
        /// ---
        #[pallet::weight(500_000_000 + T::DbWeight::get().reads_writes(3,4))]
        pub fn multistep_call(
            origin: OriginFor<T>,
            requester: T::AccountId,
            target_dest: T::AccountId,
            phase: u8,
            code: Vec<u8>,
            // #[compact] value: BalanceOf<T>,
            // #[compact] gas_limit: Gas,
            value: BalanceOf<T>,
            gas_limit: Gas,
            input_data: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let escrow_account = ensure_signed(origin.clone())?;

            ensure!(
                escrow_account == <pallet_sudo::Pallet<T>>::key(),
                Error::<T>::UnauthorizedCallAttempt
            );

            match phase {
                0 => {
                    // Charge Escrow Account from requester first before execution.
                    // Gas charge needs to be worked out. For now assume the multiplier with gas and token = 1.
                    let mut gas_meter = GasMeter::new(gas_limit);
                    let total_precharge = gas_meter
                        .limit_as_fees()
                        .map_err(|_| Error::<T>::FeesOverflow)?;
                    ensure!(
                        <T as EscrowTrait>::Currency::free_balance(&requester)
                            .saturating_sub(total_precharge)
                            >= <T as EscrowTrait>::Currency::minimum_balance(),
                        Error::<T>::RequesterNotEnoughBalance,
                    );

                    just_transfer::<T>(&requester, &escrow_account, total_precharge).map_err(
                        |_| {
                            stamp_failed_execution::<T>(
                                ErrCodes::BalanceTransferFailed as u8,
                                &requester.clone(),
                                &T::Hashing::hash(&code.clone()),
                            );
                            Error::<T>::BalanceTransferFailed
                        },
                    )?;

                    log::debug!("DEBUG multistep_call -- just_transfer total balance of CONTRACT -- vs REQUESTER {:?} vs ESCROW {:?}", <T as EscrowTrait>::Currency::free_balance(&requester), <T as EscrowTrait>::Currency::free_balance(&escrow_account));

                    let mut transfers = Vec::<TransferEntry>::new();
                    let mut deferred_storage_writes = Vec::<DeferredStorageWrite>::new();
                    let mut call_stamps = Vec::<CallStamp>::new();

                    // Make a distinction on the purpose of the call. Refer to the multistep_call docs.
                    let result_proof: Option<Vec<u8>> = match !code.is_empty() {
                        // Only A.1) - no code, there is no contracts on the balance-only parachains.
                        false => {
                            log::debug!("DEBUG multistep_call -- before check escrow_transfer to target dest -- value {:?}", value.clone());
                            if value > BalanceOf::<T>::from(0 as u32) {
                                log::debug!("DEBUG multistep_call -- before escrow_transfer to target dest {:?}", &target_dest.clone());
                                escrow_transfer::<T>(
                                    &escrow_account.clone(),
                                    &requester.clone(),
                                    &target_dest.clone(),
                                    value.clone(),
                                    &mut transfers,
                                )
                                .map_err(|e| e)?
                            } else {
                                Err(Error::<T>::NothingToDo)?
                            }
                            None
                        }
                        // B) - Execute attached code.
                        true => {
                            let result_attached_contract = match execute_code_in_escrow_sandbox::<T>(
                                &escrow_account.clone(),
                                &requester.clone(),
                                &target_dest.clone(),
                                value.clone(),
                                code.clone(),
                                input_data.clone(),
                                &mut gas_meter,
                                &mut transfers,
                                &mut deferred_storage_writes,
                                &mut call_stamps,
                            ) {
                                Ok(exec_res_val) => exec_res_val.data,
                                Err(err) => {
                                    stamp_failed_execution::<T>(
                                        ErrCodes::ExecutionFailure as u8,
                                        &requester.clone(),
                                        &T::Hashing::hash(&code.clone()),
                                    );
                                    Err(err.error)?
                                }
                            };
                            // Store the result in order to reveal during Commit phase or delete during Revert.
                            <DeferredResults<T>>::insert(
                                &requester,
                                &T::Hashing::hash(&code.clone()),
                                result_attached_contract.clone(),
                            );
                            Some(T::Hashing::hash(&result_attached_contract).encode())
                        }
                    };
                    // Refund difference between gas spend and actual costs to the requester.
                    // ToDo#1: This should also include additional cost of commit phase,
                    //  which can already be predicted here based on the deferred writes and transfers
                    // ToDo#2: On top of the regular fees account additional X% as the service fee.
                    let refund_fees = gas_meter
                        .left_as_fees()
                        .map_err(|_| Error::<T>::FeesOverflow)?;
                    just_transfer::<T>(&escrow_account, &requester, refund_fees).map_err(|_| {
                        stamp_failed_execution::<T>(
                            ErrCodes::BalanceTransferFailed as u8,
                            &requester.clone(),
                            &T::Hashing::hash(&code.clone()),
                        );
                        Error::<T>::BalanceTransferFailed
                    })?;

                    <DeferredTransfers<T>>::insert(&requester, &target_dest.clone(), transfers);

                    let storage_proof = match call_stamps
                        .clone()
                        .into_iter()
                        .map(|a| a.post_storage)
                        .reduce(|a, b| [a, b].concat())
                    {
                        None => None,
                        Some(merged_post_storage) => {
                            Some(T::Hashing::hash(&merged_post_storage).encode())
                        }
                    };

                    let execution_proofs = ExecutionProofs {
                        // Present the execution proof by hashing the results.
                        result: result_proof.clone(),
                        storage: storage_proof,
                        deferred_transfers: <DeferredTransfers<T>>::get(
                            &requester,
                            &target_dest.clone(),
                        ),
                    };
                    log::debug!(
                        "DEBUG multistepcall -- Execution Proofs : result {:?} ",
                        execution_proofs.result
                    );
                    log::debug!(
                        "DEBUG multistepcall -- Execution storage : storage {:?}",
                        execution_proofs.storage
                    );
                    log::debug!(
                        "DEBUG multistepcall -- Execution Proofs : deferred_transfers {:?}",
                        execution_proofs.deferred_transfers
                    );
                    log::debug!(
                        "DEBUG multistepcall -- Execution Proofs : gas_spent {:?} vs left {:?}",
                        gas_meter.gas_spent(),
                        gas_meter.gas_left()
                    );
                    <DeferredStorageWrites<T>>::insert(
                        &requester,
                        &T::Hashing::hash(&code.clone()),
                        deferred_storage_writes,
                    );
                    let exec_stamp = ExecutionStamp {
                        call_stamps,
                        timestamp: TryInto::<u64>::try_into(<T as EscrowTrait>::Time::now())
                            .ok()
                            .unwrap(),
                        phase: 0,
                        proofs: Some(execution_proofs.clone()),
                        failure: None,
                    };
                    <ExecutionStamps<T>>::insert(
                        &requester,
                        &T::Hashing::hash(&code.clone()),
                        exec_stamp.clone(),
                    );
                    Self::deposit_event(Event::RuntimeGatewayVersatileExecutionSuccess(
                        exec_stamp.timestamp,
                        0b00000000u8,
                        T::Hashing::hash(&<DeferredResults<T>>::get(
                            &requester,
                            &T::Hashing::hash(&code.clone()),
                        ))
                        .encode(),
                        execution_proofs.deferred_transfers,
                    ));
                }
                // Commit
                1 => {
                    let last_execution_stamp =
                        <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    if ExecutionStamp::default() == last_execution_stamp
                        || last_execution_stamp.phase != 0
                        || last_execution_stamp.failure != None
                    {
                        Err(Error::<T>::CommitOnlyPossibleAfterSuccessfulExecutionPhase)?
                    }
                    let mut proofs = last_execution_stamp.proofs.unwrap();
                    // Release transfers
                    commit_deferred_transfers::<T>(
                        escrow_account.clone(),
                        &mut proofs.deferred_transfers,
                    )
                    .map_err(|_e| <Error<T>>::CommitPhaseFailedToDeliverTransfers)?;
                    // ToDo: Release results -- delegates storing results to circuit?

                    <ExecutionStamps<T>>::mutate(
                        &requester,
                        &T::Hashing::hash(&code.clone()),
                        |stamp| {
                            stamp.phase = 1;
                        },
                    );
                    let exec_stamp =
                        <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    Self::deposit_event(Event::RuntimeGatewayVersatileCommitSuccess(
                        exec_stamp.timestamp,
                        0b00000001u8,
                        <DeferredResults<T>>::get(&requester, &T::Hashing::hash(&code.clone()))
                            .encode(),
                        vec![],
                    ));
                }
                // Revert
                2 => {
                    Self::revert(
                        origin,
                        escrow_account.clone(),
                        requester.clone(),
                        code.clone(),
                    )
                    .map_err(|e| e)?;
                    kill_storage(
                        &get_child_storage_for_current_execution::<T>(
                            &escrow_account,
                            T::Hashing::hash(&code.clone()),
                        ),
                        None,
                    );
                    let exec_stamp =
                        <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    Self::deposit_event(Event::RuntimeGatewayVersatileRevertSuccess(
                        exec_stamp.timestamp,
                        0b00000010u8,
                        vec![],
                        vec![],
                    ));
                }
                _ => {
                    log::debug!("DEBUG multistep_call -- Unknown Phase {}", phase);
                    Self::deposit_event(Event::MultistepUnknownPhase(phase));
                }
            }
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn get_storage(
            _origin: OriginFor<T>,
            key: [u8; 32],
            child_key: Option<Vec<u8>>,
            storage_flags: Option<StorageFlags>,
        ) -> DispatchResultWithPostInfo {
            let val = match storage_flags {
                None | Some(StorageFlags::GLOBAL) => sp_io::storage::get(&key),
                Some(StorageFlags::CHILD) => {
                    let child = child_key
                        .expect("Expect child key provided alongside with CHILD storage flag");
                    child::get_raw(&ChildInfo::new_default(&child), &key)
                }
                _ => Err(Error::<T>::UnknownXStorageFlag)?,
            };

            Self::deposit_event(Event::XGetStorage(key.to_vec(), val.encode()));
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn set_storage(
            _origin: OriginFor<T>,
            key: [u8; 32],
            val: Option<Vec<u8>>,
            child_key: Option<Vec<u8>>,
            storage_flags: Option<StorageFlags>,
        ) -> DispatchResultWithPostInfo {
            let res_val = match storage_flags {
                None | Some(StorageFlags::GLOBAL) => match val {
                    Some(new_value) => sp_io::storage::set(&key, &new_value[..]),
                    _ => sp_io::storage::clear(&key),
                },
                Some(StorageFlags::CHILD) => {
                    let child = child_key
                        .expect("Expect child key provided alongside with CHILD storage flag");
                    let child_info = &ChildInfo::new_default(&child);

                    match val {
                        Some(new_value) => child::put_raw(&child_info, &key, &new_value[..]),
                        _ => child::kill(&child_info, &key),
                    }
                }
                _ => Err(Error::<T>::UnknownXStorageFlag)?,
            };
            Self::deposit_event(Event::XSetStorage(key.to_vec(), res_val.encode()));
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn transfer(
            _origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            value: BalanceOf<T>,
            maybe_escrow_account: Option<T::AccountId>,
            maybe_transfer_flags: Option<TransferFlags>,
        ) -> DispatchResultWithPostInfo {
            match maybe_transfer_flags {
                None | Some(TransferFlags::DIRTY) => just_transfer::<T>(&from, &to, value)
                    .map_err(|_| Error::<T>::XBalanceTransferFailed)?,
                Some(TransferFlags::ESCROWED_EXECUTE) => {
                    let escrow = maybe_escrow_account
                        .clone()
                        .expect("transfer_escrow requires valid escrow account");
                    let mut transfers = Vec::<TransferEntry>::new();
                    escrow_transfer::<T>(&escrow, &from, &to, value, &mut transfers)
                        .map_err(|_| Error::<T>::XBalanceTransferFailed)?
                }
                Some(TransferFlags::ESCROWED_COMMIT) => {
                    let escrow = maybe_escrow_account
                        .clone()
                        .expect("escrow transfer commit requires valid escrow account");
                    just_transfer::<T>(&escrow, &to, value)?;
                }
                Some(TransferFlags::ESCROWED_REVERT) => {
                    let escrow = maybe_escrow_account
                        .clone()
                        .expect("escrow transfer revert requires valid escrow account");
                    just_transfer::<T>(&escrow, &from, value)?;
                }
                _ => Err(Error::<T>::UnknownXCallFlag)?,
            }

            Self::deposit_event(Event::XTransfer(from, to, value, maybe_escrow_account));
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn emit_event(
            _origin: OriginFor<T>,
            event_encoded: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            // Print a test message.
            Self::deposit_event(Event::XEmitEvent(event_encoded));
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn generic(_origin: OriginFor<T>, key: [u8; 32]) -> DispatchResultWithPostInfo {
            // Print a test message.
            Self::deposit_event(Event::GetStorageResult(key.to_vec()));
            Ok(().into())
        }

        #[pallet::weight(100_000_000 + T::DbWeight::get().reads_writes(1,0))]
        pub fn swap(_origin: OriginFor<T>, key: [u8; 32]) -> DispatchResultWithPostInfo {
            // Print a test message.
            Self::deposit_event(Event::GetStorageResult(key.to_vec()));
            Ok(().into())
        }

        #[pallet::weight(300_000_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn revert(
            _origin: OriginFor<T>,
            escrow_account: T::AccountId,
            requester: T::AccountId,
            code: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let last_execution_stamp =
                <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
            if ExecutionStamp::default() == last_execution_stamp || last_execution_stamp.phase == 2
            {
                Err(Error::<T>::CannotRevertMultipleTimes)?
            }
            let mut proofs = last_execution_stamp.proofs.unwrap();
            // Refund transfers
            cleanup_failed_execution::<T>(
                escrow_account.clone(),
                requester.clone(),
                &mut proofs.deferred_transfers,
            )
            .map_err(|_e| <Error<T>>::CleanupFailedDuringRevert)?;

            <ExecutionStamps<T>>::mutate(&requester, &T::Hashing::hash(&code.clone()), |stamp| {
                stamp.phase = 2;
            });

            // Remove the call result from storage.
            <DeferredResults<T>>::take(&requester, &T::Hashing::hash(&code.clone()));

            Ok(().into())
        }
    }
}
