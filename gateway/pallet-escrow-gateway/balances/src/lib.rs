#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    storage::child::kill_storage,
    traits::{Currency, Time},
};
use frame_system::{self as system, ensure_signed};
use reduce::Reduce;
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
    ExecResult, VersatileWasm, WasmExecutable,
};

use sudo;

use gateway_escrow_engine::{
    proofs::EscrowExecuteResult,
    transfers::{
        commit_deferred_transfers, escrow_transfer, just_transfer, BalanceOf, TransferEntry,
    },
    EscrowTrait,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub fn cleanup_failed_execution<T: Trait>(
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
#[codec(compact)]
pub struct ExecutionProofs {
    result: Option<Vec<u8>>,
    storage: Option<Vec<u8>>,
    deferred_transfers: Vec<TransferEntry>,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, Default, Clone)]
pub struct ExecutionStamp {
    timestamp: u64,
    phase: u8,
    proofs: Option<ExecutionProofs>,
    call_stamps: Vec<CallStamp>,
    failure: Option<u8>, // Error Code
}

pub fn execute_code_in_escrow_sandbox<'a, T: Trait>(
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

pub trait Trait: EscrowTrait + VersatileWasm {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ChildStorage {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(fn something): Option<u32>;

        // For each requester address
        //      For each transaction_tx (temporarily dest address)
        //          Store deferred transfers - Vec<TransferEntry>
        DeferredTransfers get(fn deferred_transfers):
            double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::AccountId => Vec<TransferEntry>;

        // ( Requester , CodeHash ) -> [ ExecutionStamp ]
        ExecutionStamps get(fn execution_stamps):
            double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::Hash => ExecutionStamp;

        DeferredResults get(fn deferred_results):
            double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::Hash => Vec<u8>;

        DeferredStorageWrites get(fn deferred_storage_writes):
            double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::Hash => Vec<DeferredStorageWrite>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        SomethingStored(u32, AccountId),

        MultistepExecutionResult(EscrowExecuteResult),

        MultistepCommitResult(Vec<u8>),

        MultistepRevertResult(u32),

        MultistepUnknownPhase(u8),

        RentProjectionCalled(AccountId, AccountId),

        GetStorageResult(Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {

        RequesterNotEnoughBalance,

        BalanceTransferFailed,

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

pub fn stamp_failed_execution<T: Trait>(
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
// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: <T as system::Trait>::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;
        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

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
       #[weight = *gas_limit]
        pub fn multistep_call(
            origin,
            requester: <T as system::Trait>::AccountId,
            target_dest: <T as system::Trait>::AccountId,
            #[compact] phase: u8,
            code: Vec<u8>,
            #[compact] value: BalanceOf<T>,
            #[compact] gas_limit: Gas,
            input_data: Vec<u8>
        ) -> dispatch::DispatchResult {
            let escrow_account = ensure_signed(origin.clone())?;
            ensure!(escrow_account == <sudo::Module<T>>::key(), Error::<T>::UnauthorizedCallAttempt);

            match phase {
                0 => {
                    // Charge Escrow Account from requester first before execution.
                    // Gas charge needs to be worked out. For now assume the multiplier with gas and token = 1.
                    let mut gas_meter = GasMeter::new(gas_limit);
                    let total_precharge = gas_meter.limit_as_fees().map_err(|_| { Error::<T>::FeesOverflow })?;
                    ensure!(
                        <T as EscrowTrait>::Currency::free_balance(&requester).saturating_sub(total_precharge) >=
                            <T as EscrowTrait>::Currency::minimum_balance(),
                        Error::<T>::RequesterNotEnoughBalance,
                    );

                    just_transfer::<T>(&requester, &escrow_account, total_precharge).map_err(|_| {
                        stamp_failed_execution::<T>(ErrCodes::BalanceTransferFailed as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                        Error::<T>::BalanceTransferFailed
                    })?;

                    debug::info!("DEBUG multistep_call -- just_transfer total balance of CONTRACT -- vs REQUESTER {:?} vs ESCROW {:?}", <T as EscrowTrait>::Currency::free_balance(&requester), <T as EscrowTrait>::Currency::free_balance(&escrow_account));

                    let mut transfers = Vec::<TransferEntry>::new();
                    let mut deferred_storage_writes = Vec::<DeferredStorageWrite>::new();
                    let mut call_stamps = Vec::<CallStamp>::new();

                    // Make a distinction on the purpose of the call. Refer to the multistep_call docs.
                    let result_proof: Option<Vec<u8>> = match !code.is_empty() {
                        // Only A.1) - no code, there is no contracts on the balance-only parachains.
                        false => {
                            if value > BalanceOf::<T>::from(0) {
                                escrow_transfer::<T>(
                                    &escrow_account.clone(),
                                    &requester.clone(),
                                    &target_dest.clone(),
                                    value.clone(),
                                    &mut transfers,
                                ).map_err(|e| e)?
                            } else {
                                Err(Error::<T>::NothingToDo)?
                            }
                            None
                        },
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
                                    stamp_failed_execution::<T>(ErrCodes::ExecutionFailure as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                                    Err(err.error)?
                                }
                            };
                            // Store the result in order to reveal during Commit phase or delete during Revert.
                            <DeferredResults<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), result_attached_contract.clone());
                            Some(T::Hashing::hash(&result_attached_contract).encode())
                        },
                    };
                    // Refund difference between gas spend and actual costs to the requester.
                    // ToDo#1: This should also include additional cost of commit phase,
                    //  which can already be predicted here based on the deferred writes and transfers
                    // ToDo#2: On top of the regular fees account additional X% as the service fee.
                    let refund_fees = gas_meter.left_as_fees().map_err(|_| { Error::<T>::FeesOverflow })?;
                    just_transfer::<T>(&escrow_account, &escrow_account, refund_fees).map_err(|_| {
                        stamp_failed_execution::<T>(ErrCodes::BalanceTransferFailed as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                        Error::<T>::BalanceTransferFailed
                    })?;

                    <DeferredTransfers<T>>::insert(&requester, &target_dest.clone(), transfers);

                    let storage_proof = match call_stamps.clone().into_iter().map(|a| a.post_storage).reduce(|a, b| [a, b].concat()) {
                        None => None,
                        Some(merged_post_storage) => Some(T::Hashing::hash(&merged_post_storage).encode()),
                    };

                    let execution_proofs = ExecutionProofs {
                        // Present the execution proof by hashing the results.
                        result: result_proof,
                        storage: storage_proof,
                        deferred_transfers: <DeferredTransfers<T>>::get(&requester, &target_dest.clone()),
                    };
                    debug::info!("DEBUG multistepcall -- Execution Proofs : result {:?} ", execution_proofs.result);
                    debug::info!("DEBUG multistepcall -- Execution storage : storage {:?}", execution_proofs.storage);
                    debug::info!("DEBUG multistepcall -- Execution Proofs : deferred_transfers {:?}", execution_proofs.deferred_transfers);
                    <DeferredStorageWrites<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), deferred_storage_writes);

                    <ExecutionStamps<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), ExecutionStamp {
                        call_stamps,
                        timestamp: TryInto::<u64>::try_into(<T as EscrowTrait>::Time::now()).ok().unwrap(),
                        phase: 0,
                        proofs: Some(execution_proofs),
                        failure: None,
                    });
                }
                // Commit
                1 => {
                    let last_execution_stamp = <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    if ExecutionStamp::default() == last_execution_stamp || last_execution_stamp.phase != 0 || last_execution_stamp.failure != None {
                        Err(Error::<T>::CommitOnlyPossibleAfterSuccessfulExecutionPhase)?
                    }
                    let mut proofs = last_execution_stamp.proofs.unwrap();
                    // Release transfers
                    commit_deferred_transfers::<T>(escrow_account.clone(), &mut proofs.deferred_transfers)
                        .map_err(|_e| <Error<T>>::CommitPhaseFailedToDeliverTransfers)?;
                    // ToDo: Release results -- delegates storing results to circuit?

                    <ExecutionStamps<T>>::mutate(&requester, &T::Hashing::hash(&code.clone()), |stamp| {
                        stamp.phase = 1;
                    });

                    Self::deposit_event(RawEvent::MultistepCommitResult(
                        <DeferredResults<T>>::get(&requester, &T::Hashing::hash(&code.clone()))
                    ));
                },
                // Revert
                2 => {
                   Self::revert(
                        origin,
                        escrow_account.clone(),
                        requester,
                        code.clone(),
                   ).map_err(|e| e)?;
                   kill_storage(
                        &get_child_storage_for_current_execution::<T>(&escrow_account, T::Hashing::hash(&code.clone()))
                   );
                },
                _ => {
                    debug::info!("DEBUG multistep_call -- Unknown Phase {}", phase);
                    Something::put(phase as u32);
                    Self::deposit_event(RawEvent::MultistepUnknownPhase(phase));
                }
            }
            Ok(())
        }

        #[weight = 10_000]
        fn revert(
            origin,
            escrow_account: <T as system::Trait>::AccountId,
            requester: <T as system::Trait>::AccountId,
            code: Vec<u8>,
        ) {
            let last_execution_stamp = <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
            if ExecutionStamp::default() == last_execution_stamp || last_execution_stamp.phase == 2 {
                Err(Error::<T>::CannotRevertMultipleTimes)?
            }
            let mut proofs = last_execution_stamp.proofs.unwrap();
            // Refund transfers
            cleanup_failed_execution::<T>(escrow_account.clone(), requester.clone(), &mut proofs.deferred_transfers)
                .map_err(|_e| <Error<T>>::CleanupFailedDuringRevert)?;

            <ExecutionStamps<T>>::mutate(&requester, &T::Hashing::hash(&code.clone()), |stamp| {
                stamp.phase = 2;
            });

            // Remove the call result from storage.
            <DeferredResults<T>>::take(&requester, &T::Hashing::hash(&code.clone()));
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn rent_projection(
            origin,
            address: <T as system::Trait>::AccountId
        ) -> dispatch::DispatchResult {
            // Ensure that the caller is a regular keypair account
            let caller = ensure_signed(origin)?;
            // Print a test message.
            debug::info!("DEBUG rent_projection by: {:?} for = {:?}", caller, address);
            // For now refer to the contracts rent_projection.
            // In the future rent projection should estimate on % of storage for that address used by escrow account

            // Raise an event for debug purposes
            Self::deposit_event(RawEvent::RentProjectionCalled(address, caller));

            Ok(())
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn get_storage(
            origin,
            address: <T as system::Trait>::AccountId,
            key: [u8; 32],
        ) -> dispatch::DispatchResult {
            // Print a test message.
            Self::deposit_event(RawEvent::GetStorageResult(key.to_vec()));
            Ok(())
        }

        /// Just a dummy entry point.
        /// function that can be called by the external world as an extrinsics call
        /// takes a parameter of the type `AccountId`, stores it, and emits an event
        #[weight = 10_000]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;

            // Code to execute when something calls this.
            // For example: the following line stores the passed in u32 in the storage
            Something::put(something);

            // Here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
    }
}
