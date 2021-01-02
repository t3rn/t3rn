#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use escrow_contracts_wrapper::{
    escrow_exec::{CallStamp, DeferredStorageWrite},
    exec::{ExecError, ExecResult, ExecutionContext},
    storage::write_contract_storage,
    wasm::{
        code_cache::load as load_code, // ToDo: Solve the types err while calling loader.load_main directly
        prepare::prepare_contract,
        runtime::Env,
        WasmExecutable,
        WasmLoader,
        WasmVm,
    },
    BalanceOf as ContractsBalanceOf, CodeHash, Config, ContractAddressFor, ContractInfo,
    ContractInfoOf, Gas, GasMeter,
};
use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    storage::child,
    traits::{Currency, Get, Time},
};
use frame_system::{self as system, ensure_signed};
use gateway_escrow_engine::transfers::{
    commit_deferred_transfers, escrow_transfer, just_transfer, BalanceOf, TransferEntry,
};
pub use gateway_escrow_engine::EscrowTrait;
use reduce::Reduce;
use sp_runtime::traits::{Hash, Saturating};
use sp_runtime::DispatchResult;
use sp_std::convert::TryInto;
use sp_std::vec;
use sp_std::vec::Vec;
use sudo;

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

pub fn instantiate_temp_execution_contract<'a, T: Trait>(
    origin: T::Origin,
    code: Vec<u8>,
    input_data: &Vec<u8>,
    endowment: ContractsBalanceOf<T>,
    gas_limit: Gas,
) -> dispatch::DispatchResult {
    let code_hash_res =
        <escrow_contracts_wrapper::Module<T>>::put_code(origin.clone(), code.clone());
    debug::info!(
        "DEBUG gateway_contract_exec -- escrow_contracts_wrapper::put_code {:?}",
        code_hash_res
    );
    code_hash_res.map_err(|_e| <Error<T>>::PutCodeFailure)?;
    let code_hash = T::Hashing::hash(&code.clone());
    // ToDo: Instantiate works - but charging accounts in unit tests doesn't (due to GenesisConfig not present in Balance err)
    // Step 2: escrow_contracts_wrapper::instantiate
    // ToDo: Smart way of calculating endowment that would be enough for initialization + one call.
    let init_res = <escrow_contracts_wrapper::Module<T>>::instantiate(
        origin.clone(),
        endowment,
        gas_limit,
        code_hash,
        input_data.clone(),
    );
    init_res.map_err(|_e| <Error<T>>::InitializationFailure)?;
    // If not instantiate just transfer endowment directly.
    // if endowment > BalanceOf::<T>::from(0 as u32) {
    //     just_transfer::<T>(&escrow_account, &dest, endowment);
    // }
    debug::info!(
        "DEBUG multistepcall -- escrow_contracts_wrapper::instantiate_temp_execution_contract init_res {:?}",
        init_res
    );
    Ok(())
}
pub fn execute_attached_code<'a, T: Trait>(
    origin: T::Origin,
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    target_dest: &T::AccountId,
    value: BalanceOf<T>,
    code: Vec<u8>,
    input_data: Vec<u8>,
    endowment: ContractsBalanceOf<T>,
    mut gas_meter: &mut GasMeter<T>,
    cfg: &Config<T>,
    transfers: &mut Vec<TransferEntry>,
    deferred_storage_writes: &mut Vec<DeferredStorageWrite>,
    call_stamps: &mut Vec<CallStamp>,
) -> ExecResult {
    // Step 1: Temporarily instantiate the contract for the purpose following execution, so it's possible to set_storage etc.
    instantiate_temp_execution_contract::<T>(
        origin,
        code.clone(),
        &input_data.clone(),
        endowment.clone(),
        gas_meter.gas_left(),
    )
    .map_err(|e| ExecError::from(e))?;

    // Step 2. Prepare attached code to be fed for execution.
    let temp_contract_address = T::DetermineContractAddress::contract_address_for(
        &T::Hashing::hash(&code.clone()),
        &input_data.clone(),
        &escrow_account.clone(),
    );
    // That only works for code that is received by the call and will be executed and cleaned up after.
    let prefab_module = prepare_contract::<Env>(&code, &cfg.schedule).unwrap();
    let executable = WasmExecutable {
        entrypoint_name: "call",
        prefab_module,
    };

    // Step 3: Execute attached code as it's any regular contract on that parachain.
    let vm = WasmVm::new(&cfg.schedule);
    let loader = WasmLoader::new(&cfg.schedule);
    let mut ctx = ExecutionContext::top_level(escrow_account.clone(), &cfg, &vm, &loader);

    let value_contracts_compatible =
        ContractsBalanceOf::<T>::from(TryInto::<u32>::try_into(value).ok().unwrap());

    match ctx.escrow_call(
        &escrow_account.clone(),
        &requester.clone(),
        &temp_contract_address.clone(),
        &target_dest.clone(),
        value_contracts_compatible,
        &mut gas_meter,
        input_data.clone(),
        transfers,
        deferred_storage_writes,
        call_stamps,
        &executable,
    ) {
        Ok(exec_ret_val) => Ok(exec_ret_val),
        Err(exec_err) => {
            use escrow_contracts_wrapper::exec::Ext;
            // Revert the execution effects on the spot.
            cleanup_failed_execution::<T>(escrow_account.clone(), requester.clone(), transfers)
                .map_err(|_e| <Error<T>>::CleanupFailedAfterUnsuccessfulExecution)?;
            let mut call_context =
                ctx.new_call_context(escrow_account.clone(), value_contracts_compatible);
            call_context
                .terminate(&temp_contract_address.clone(), &mut gas_meter)
                .map_err(|_e| <Error<T>>::TerminateFailure)?;
            Err(exec_err)?
        }
    }
}

pub fn execute_escrow_call_recursively<'a, T: Trait>(
    escrow_account: &T::AccountId,
    requester: &T::AccountId,
    target_dest: &T::AccountId,
    value: BalanceOf<T>,
    input_data: Vec<u8>,
    mut gas_meter: &mut GasMeter<T>,
    cfg: &Config<T>,
    transfers: &mut Vec<TransferEntry>,
    deferred_storage_writes: &mut Vec<DeferredStorageWrite>,
    call_stamps: &mut Vec<CallStamp>,
    code_hash: &CodeHash<T>,
) -> ExecResult {
    let vm = WasmVm::new(&cfg.schedule);
    let loader = WasmLoader::new(&cfg.schedule);
    let mut ctx = ExecutionContext::top_level(escrow_account.clone(), &cfg, &vm, &loader);

    let executable = WasmExecutable {
        entrypoint_name: "call",
        prefab_module: load_code::<T>(code_hash, &cfg.schedule)?,
    };

    match ctx.escrow_call(
        &escrow_account.clone(),
        &requester.clone(),
        &target_dest.clone(),
        &target_dest.clone(),
        ContractsBalanceOf::<T>::from(TryInto::<u32>::try_into(value).ok().unwrap()),
        &mut gas_meter,
        input_data.clone(),
        transfers,
        deferred_storage_writes,
        call_stamps,
        &executable,
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

pub trait Trait: EscrowTrait + escrow_contracts_wrapper::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type WhenStateChangedForceTry: Get<bool>;
}

decl_storage! {
    trait Store for Module<T: Trait> as EscrowGateway {
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

        /// \[timestamp, phase, result, deferred_transfers\]
        ContractsGatewayExecutionSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        /// \[timestamp, phase, result, deferred_transfers\]
        ContractsGatewayCommitSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        /// \[timestamp, phase, result, deferred_transfers\]
        ContractsGatewayRevertSuccess(u64, u8, Vec<u8>, Vec<TransferEntry>),

        MultistepUnknownPhase(u8),

        RentProjectionCalled(AccountId, AccountId),

        GetStorageResult(Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {

        RequesterNotEnoughBalance,

        BalanceTransferFailed,

        PutCodeFailure,

        InitializationFailure,

        ExecutionFailure,

        NothingToDo,

        CallFailure,

        CallFailureNotCallable,

        CallFailureCodeNotFound,

        TerminateFailure,

        CommitPhaseFailedToDeliverTransfers,

        CleanupFailedAfterUnsuccessfulExecution,

        CleanupFailedDuringRevert,

        UnauthorizedCallAttempt,

        CommitOnlyPossibleAfterSuccessfulExecutionPhase,

        CannotRevertMultipleTimes,

        DestinationContractStorageChangedSinceExecution,
    }
}

// ToDo: Encode errors properly before storing making the below enum obsolete.
#[derive(Clone)]
#[repr(u8)]
pub enum ErrCodes {
    RequesterNotEnoughBalance = 0,

    BalanceTransferFailed = 1,

    PutCodeFailure = 2,

    InitializationFailure = 3,

    ExecutionFailure = 4,

    CallFailure = 5,

    TerminateFailure = 6,
}

pub fn get_storage_root_for_code<T: Trait>(
    code: Vec<u8>,
    input_data: Vec<u8>,
    escrow_account: &T::AccountId,
) -> Vec<u8> {
    let temp_contract_address = T::DetermineContractAddress::contract_address_for(
        &T::Hashing::hash(&code.clone()),
        &input_data.clone(),
        &escrow_account.clone(),
    );
    child::root(
        &<ContractInfoOf<T>>::get(temp_contract_address.clone())
            .unwrap()
            .get_alive()
            .unwrap()
            .child_trie_info(),
    )
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
    pub struct Module<T: Trait> for enum Call where origin: <T as frame_system::Trait>::Origin {
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
        /// Based on those effects, gateway_contract_exec can be used in different manners:
        /// * A) For deferring balance transfers:
        ///     * A.1) A single balance transfer to the target_dest can be deferred by calling with empty code and a value
        ///     * A.2) Multiple balance transfers to multiple target destinations by attaching the corresponding contract
        ///     * A.1+2) A single balance transfer can be executed on top of multiple transfers from within the corresponding contract
        /// * B) For attaching and executing "code" within the context of that parachain (and possibly accessing the readonly data of the contracts) and revealing the results only after the Commit phase.
        /// * C) For deferring effects of a call (or recursive calls) to an existing contract(s).
        ///             After successful execution phase no changes are made yet to the target destination contract,
        ///             but the execution is simulated by recording all of the changes to contract,
        ///             retrieving results but as the contract's execution is done but rolling back the changes to a state before the call.
        ///             The hash of contract storage and input data upon which the execution was successful are stored
        ///             in order to be validated against during the final Commit phase at the following call.
        ///     * If the contracts storage hasn't changed since the Execution phase,
        ///             the call applies the changes to the storage of target contracts and returns the results.
        ///     * If the contracts storage has changed since the Execution phase and there are some deferred storage changes,
        ///                 the call relies on the call_requirements configuration.
        ///         * fail_when_state_changed = signal failure and go to Revert phase instead
        ///         * force_try_when_state_changed = try apply the changes to storage of target contract despite their changed state. It can be safe for some contracts (e.g append only changes), whereas deadly dangerous for others (e.g. updates). This may be removed in the near future.
        ///         * re_execute_when_state_changed = repeat the Execution phase and proceed to either Commit or Revert phase immidiately after.
        /// * D) For attaching, instantiating and executing new contracts on that parachain. In that case the newly instantiated contract will be charged with endowment after the Commit phase.
        ///          If the originally temporary contract for execution should stay registered on that parachain set "call_requirements.permanent_exec_contract" flag.
        /// ---
        /// **NOTE:**
        /// As the result is stored, it's accessible from outside of that chain, which for some case
        /// can violate the business logic behind the contracts. This should be fixed by either keeping
        /// the results in memory or elevating responsibility the results management to Gateway Circuit (preferable).
        /// ---
       #[weight = *gas_limit]
        pub fn gateway_contract_exec(
            origin,
            requester: <T as frame_system::Trait>::AccountId,
            target_dest: <T as frame_system::Trait>::AccountId,
            phase: u8,
            code: Vec<u8>,
            #[compact] value: BalanceOf<T>,
            #[compact] gas_limit: Gas,
            input_data: Vec<u8>
        ) -> dispatch::DispatchResult {
            let escrow_account = ensure_signed(origin.clone())?;
            ensure!(escrow_account == <sudo::Module<T>>::key(), Error::<T>::UnauthorizedCallAttempt);
            debug::info!("DEBUG gateway_contract_exec -- contracts args -- {:?}, {:?}, {:?} ,{:?}, {:?}, {:?}, {:?}", requester, target_dest, phase, code, value, gas_limit, input_data);

            match phase {
                0 => {
                    const ENDOWMENT: u32 = 100_000_000;
                    // ToDo: Endowment should be calculated here automatically based on config, applicable fees and expected lifetime of temporary execution contracts
                    let endowment = ContractsBalanceOf::<T>::from(ENDOWMENT as u32);

                    // Charge Escrow Account from requester first before executuion.
                    // Gas charge needs to be worked out. For now assume the multiplier with gas and token = 1.
                    let total_precharge = BalanceOf::<T>::from(gas_limit as u32 + ENDOWMENT);
                    let cfg = Config::<T>::preload();
                    ensure!(
                        <T as EscrowTrait>::Currency::free_balance(&requester).saturating_sub(total_precharge) >=
                            <T as EscrowTrait>::Currency::minimum_balance(),
                        Error::<T>::RequesterNotEnoughBalance,
                    );
                    just_transfer::<T>(&requester, &escrow_account, total_precharge).map_err(|_| {
                        stamp_failed_execution::<T>(ErrCodes::BalanceTransferFailed as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                        Error::<T>::BalanceTransferFailed
                    })?;

                    debug::info!("DEBUG gateway_contract_exec -- just_transfer total balance of CONTRACT -- vs REQUESTER {:?} vs ESCROW {:?}", <T as EscrowTrait>::Currency::free_balance(&requester), <T as EscrowTrait>::Currency::free_balance(&escrow_account));

                    let mut gas_meter = GasMeter::<T>::new(gas_limit);
                    let mut transfers = Vec::<TransferEntry>::new();
                    let mut deferred_storage_writes = Vec::<DeferredStorageWrite>::new();
                    let mut call_stamps = Vec::<CallStamp>::new();

                    // Make a distinction on the purpose of the call. Refer to the gateway_contract_exec docs.
                    let result_proof: Option<Vec<u8>> = match (!code.is_empty(), <ContractInfoOf<T>>::get(&target_dest.clone())) {
                        // Only A.1) - no code, not a contract - just deferrsed transfer.
                        (false, None) => {
                            if value > BalanceOf::<T>::from(0) {
                                escrow_transfer::<T>(
                                    &escrow_account.clone(),
                                    &requester.clone(),
                                    &target_dest.clone(),
                                    value.clone(),
                                    &mut transfers,
                                ).map_err(|e| e)?;
                            } else {
                                Err(Error::<T>::NothingToDo)?
                            }
                            None
                        },
                        // B) + C) OR only B) or only C)
                        // Check for both code attached & contract at dest. Execute both if possible; attached code first.
                        (true, None) | (true, Some(_)) | (false, Some(_)) => {

                            let mut result_attached_contract = vec![];
                            let mut result_called_contract = vec![];

                            if !code.is_empty() {
                                // B) - execute attached code first
                                result_attached_contract = match execute_attached_code(
                                    origin.clone(),
                                    &escrow_account.clone(),
                                    &requester.clone(),
                                    &target_dest.clone(),
                                    value.clone(),
                                    code.clone(),
                                    input_data.clone(),
                                    endowment.clone(),
                                    &mut gas_meter,
                                    &cfg,
                                    &mut transfers,
                                    &mut deferred_storage_writes,
                                    &mut call_stamps,
                                ) {
                                    Ok(exec_res_val) => exec_res_val.data,
                                    Err(err) => {
                                        stamp_failed_execution::<T>(ErrCodes::ExecutionFailure as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                                        Err(err.error)?
                                    }
                                }
                            }
                            // Check again whether it's a contract at dest and ensure it's alive.
                            if let Some(ContractInfo::Alive(info)) = <ContractInfoOf<T>>::get(&target_dest.clone()) {
                                 // C) - execute contract call at target destination.
                                 result_called_contract = match execute_escrow_call_recursively(
                                    &escrow_account.clone(),
                                    &requester.clone(),
                                    &target_dest.clone(),
                                    value.clone(),
                                    input_data.clone(),
                                    &mut gas_meter,
                                    &cfg,
                                    &mut transfers,
                                    &mut deferred_storage_writes,
                                    &mut call_stamps,
                                    &info.code_hash,
                                ) {
                                    Ok(exec_res_val) => exec_res_val.data,
                                    Err(err) => {
                                        stamp_failed_execution::<T>(ErrCodes::ExecutionFailure as u8, &requester.clone(), &T::Hashing::hash(&code.clone()));
                                        Err(err.error)?
                                    }
                                }
                                // println!("IS ALIVE CONTRACT AT TARGET DEST! CALL RES {:?}", result_called_contract.clone());
                                // ()
                            }
                            // Store the result in order to reveal during Commit phase or delete during Revert.
                            // Give priority here to the result of contract at target destination if exist.
                            // The results can be chained -> output from attached code can be redirected into input of the contract call.
                            if !result_called_contract.is_empty() {
                                <DeferredResults<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), result_called_contract.clone());
                                Some(T::Hashing::hash(&result_called_contract).encode())
                            } else {
                                <DeferredResults<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), result_attached_contract.clone());
                                Some(T::Hashing::hash(&result_attached_contract).encode())
                            }
                        },
                    };

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

                    let exec_stamp = ExecutionStamp {
                        call_stamps,
                        timestamp: TryInto::<u64>::try_into(<T as EscrowTrait>::Time::now()).ok().unwrap(),
                        phase: 0,
                        proofs: Some(execution_proofs.clone()),
                        failure: None,
                    };
                    <ExecutionStamps<T>>::insert(&requester, &T::Hashing::hash(&code.clone()), exec_stamp.clone());

                    let exec_stamp = <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    Self::deposit_event(RawEvent::ContractsGatewayExecutionSuccess(
                        exec_stamp.timestamp, 0b00000000u8, T::Hashing::hash(&<DeferredResults<T>>::get(&requester, &T::Hashing::hash(&code.clone()))).encode(), execution_proofs.deferred_transfers
                    ));
                    // ToDo: Return difference between gas spend and actual costs.
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

                    // ToDo: Apply storage changes to target account.
                    let deferred_storage_writes: Vec<DeferredStorageWrite> = <DeferredStorageWrites<T>>::get(&requester, &T::Hashing::hash(&code.clone()));

                    for storage_write in deferred_storage_writes.clone().into_iter() {
                        // Check if dest changed the child root hash
                        let dest = &T::AccountId::decode(&mut &storage_write.dest[..]).unwrap();
                        let current_dest_storage_root = child::root(&<ContractInfoOf<T>>::get(dest.clone()).unwrap().get_alive().unwrap().child_trie_info());
                        let corresponding_call_stamp = last_execution_stamp.call_stamps.clone().into_iter().find(|call_stamp| call_stamp.dest == storage_write.dest).unwrap();
                        if current_dest_storage_root != corresponding_call_stamp.pre_storage && <T as Trait>::WhenStateChangedForceTry::get() == false {
                            Err(Error::<T>::DestinationContractStorageChangedSinceExecution)?
                        }
                    }

                    for storage_write in deferred_storage_writes.into_iter() {
                        write_contract_storage::<T>(
                            &T::AccountId::decode(&mut &storage_write.dest[..]).unwrap(),
                            &storage_write.trie_id,
                            &storage_write.key,
                            storage_write.value,
                        ).map_err(|_e| <Error<T>>::DestinationContractStorageChangedSinceExecution)?;
                    }

                    <ExecutionStamps<T>>::mutate(&requester.clone(), &T::Hashing::hash(&code.clone()), |stamp| {
                        stamp.phase = 1;
                    });

                    let exec_stamp = <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                    Self::deposit_event(RawEvent::ContractsGatewayCommitSuccess(
                        exec_stamp.timestamp, 0b00000001u8, <DeferredResults<T>>::get(&requester, &T::Hashing::hash(&code.clone())).encode(), vec![],
                    ));
                },
                // Revert
                2 => {
                   Self::revert(
                        origin,
                        escrow_account,
                        requester.clone(),
                        code.clone(),
                   ).map_err(|e| e)?;
                   let exec_stamp = <ExecutionStamps<T>>::get(&requester, &T::Hashing::hash(&code.clone()));
                   Self::deposit_event(RawEvent::ContractsGatewayRevertSuccess(
                       exec_stamp.timestamp, 0b00000010u8, vec![], vec![],
                   ));
                },
                _ => {
                    debug::info!("DEBUG gateway_contract_exec -- Unknown Phase {}", phase);
                    Something::put(phase as u32);
                    Self::deposit_event(RawEvent::MultistepUnknownPhase(phase));
                }
            }
            Ok(())
        }

        #[weight = 10_000]
        fn revert(
            origin,
            escrow_account: <T as frame_system::Trait>::AccountId,
            requester: <T as frame_system::Trait>::AccountId,
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

        /// ** Get Storage **
        /// Accesses the storage of underlying contract (appointed by address).
        #[weight = 10_000]
        pub fn get_storage(
            origin,
            address: <T as frame_system::Trait>::AccountId,
            key: [u8; 32],
        ) -> dispatch::DispatchResult {
            // Print a test message.

            // Read the contract's storage
            let val = Some(<escrow_contracts_wrapper::Module<T>>::get_storage(address, key));

            debug::info!("DEBUG get_storage by key: {:?} val = {:?} ", key, val);

            // Raise an event for debug purposes
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
