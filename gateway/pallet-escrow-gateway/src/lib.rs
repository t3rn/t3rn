#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::vec::Vec;
use codec::{Decode, Encode};
use frame_support::{debug, decl_error, decl_event, decl_module, ensure, decl_storage, dispatch, traits::{ExistenceRequirement, Currency, Time}, storage::{child, child::ChildInfo}};

use frame_system::{self as system, ensure_signed, ensure_none, Phase};
use sp_runtime::{traits::{Hash, Saturating}, DispatchError};

use contracts::{BalanceOf, Gas, ContractAddressFor, ContractInfo, ContractInfoOf, GasMeter, TrieIdGenerator, NegativeImbalanceOf};

// The hard copy that exposed hidden by default features of contracts
use contracts::{
    wasm::{WasmVm, WasmLoader, prepare::prepare_contract, PrefabWasmModule,WasmExecutable, runtime::{Env, ReturnCode}},
    exec::{ExecutionContext, Vm, ReturnFlags, ExecReturnValue, ExecFeeToken, Loader, CallContext, TransferCause, TransactorKind, TransferFeeKind, ExecError, ErrorOrigin, ExecResult},
    escrow_exec::{EscrowCallContext, TransferEntry, just_transfer, escrow_transfer},
    rent,
    Config
};

#[macro_use]
mod escrow;
use crate::escrow::{ContractsEscrowEngine, EscrowExecuteResult};
use node_runtime::AccountId;

pub type CodeHash<T> = <T as frame_system::Trait>::Hash;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub fn cleanup_failed_execution<T: Trait>(
    escrow_account: T::AccountId,
    requester: T::AccountId,
    transfers: &mut Vec<TransferEntry>,
) {
    // Give the money back to the requester from the transfers that succeeded.
    for transfer in transfers.iter() {
        just_transfer::<T>(&escrow_account, &requester, BalanceOf::<T>::from(transfer.value));
    }
    transfers.clear();
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[codec(compact)]
pub struct ExecutionProofs<T: Trait> {
    result: T::Hash,
    storage: Vec<u8>,
    deferred_transfers: Vec<TransferEntry>,
}

pub fn instantiate_temp_execution_contract<'a, T: Trait>(
    origin: T::Origin,
    code: Vec<u8>,
    input_data: &Vec<u8>,
    endowment: BalanceOf<T>,
    gas_limit: Gas,
) -> dispatch::DispatchResult {
    let code_hash_res = <contracts::Module<T>>::put_code(origin.clone(), code.clone());
    println!("DEBUG multistep_call -- contracts::put_code {:?}", code_hash_res);
    code_hash_res.map_err(|_e| <Error<T>>::PutCodeFailure)?;
    let code_hash = T::Hashing::hash(&code.clone());
    // ToDo: Instantiate works - but charging accounts in unit tests doesn't (due to GenesisConfig not present in Balance err)
    // Step 2: contracts::instantiate
    // ToDo: Smart way of calculating endowment that would be enough for initialization + one call.
    let init_res = <contracts::Module<T>>::instantiate(origin.clone(), endowment, gas_limit, code_hash, input_data.clone());
    init_res.map_err(|_e| <Error<T>>::InitializationFailure)?;
    // If not instantiate just transfer endowment directly.
    // if endowment > BalanceOf::<T>::from(0 as u32) {
    //     just_transfer::<T>(&escrow_account, &dest, endowment);
    // }
    println!("DEBUG multistepcall -- contracts::instantiate_temp_execution_contract init_res {:?}", init_res);
    Ok(())
}

pub fn execute_escrow_contract_call<'a, T: Trait>(
        escrow_account: T::AccountId,
        dest: T::AccountId,
        target_dest: T::AccountId,
        requester: T::AccountId,
        code: Vec<u8>,
        value: BalanceOf<T>,
        input_data: Vec<u8>,
        cfg: Config::<T>,
        transfers: &mut Vec<TransferEntry>,
        mut gas_meter: GasMeter<T>
    ) -> ExecResult {
    let vm = WasmVm::new(&cfg.schedule);
    let loader = WasmLoader::new(&cfg.schedule);
    let mut ctx = ExecutionContext::top_level(escrow_account.clone(), &cfg, &vm, &loader);
    let trie_id = T::TrieIdGenerator::trie_id(&dest.clone());

    let prefab_module = prepare_contract::<Env>(&code, &cfg.schedule).unwrap();
    let exec = WasmExecutable {
        entrypoint_name: "call",
        prefab_module,
    };

    ctx.with_nested_context(dest.clone(), trie_id.clone(), |nested| {
        // let mut temp_nested = nested;
        use contracts::exec::Ext;

        println!("escrow_call_ctx -- transfers pre {:?}", transfers);
        let ext = EscrowCallContext {
            config: &cfg,
            block_number: <frame_system::Module<T>>::block_number(),
            caller: escrow_account.clone(),
            requester: requester.clone(),
            timestamp: T::Time::now(),
            value_transferred: value.clone(),
            transfers,
            call_context: nested.new_call_context(escrow_account.clone(), value),
        };
        if value > BalanceOf::<T>::from(0 as u32) {
            // ToDo: Make a transfer here:
            // Make an escrow transfer if value is attached to the transaction.
            escrow_transfer(&escrow_account.clone(), &requester.clone(), &target_dest.clone(), value, &mut gas_meter, ext.transfers, &cfg);
        }
        println!("escrow_call_ctx -- pre exec gas_spent {:?}", gas_meter.gas_spent());
        let exec_res = vm.execute(&exec, ext, input_data, &mut gas_meter);
        println!("escrow_call_ctx -- post exec gas_spent {:?}", gas_meter.gas_spent());

        return match exec_res {
            Ok(exec_ret_val) => Ok(exec_ret_val),
            Err(exec_err) => {
                // Revert the execution effects on the spot.
                cleanup_failed_execution::<T>(escrow_account.clone(), requester.clone(), transfers);
                let mut call_context = nested.new_call_context(escrow_account.clone(), value);
                call_context.terminate(&dest.clone(), &mut gas_meter).map_err(|_e| <Error<T>>::TerminateFailure)?;
                return Err(exec_err);
            },
        };
    })
}

pub fn charge_as_contract_call<'a, T: Trait>(dest: T::AccountId) {
    // Assumption: `collect_rent` doesn't collide with overlay because
    // `collect_rent` will be done on first call and destination contract and balance
    // cannot be changed before the first call
    // We do not allow 'calling' plain accounts. For transfering value
    // `seal_transfer` must be used.
    rent::collect_rent::<T>(&dest);

    println!("escrow_call_ctx -- charge_as_contract_call contract info");
}

pub trait Trait: contracts::Trait + system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as EscrowGateway {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(fn something): Option<u32>;

        // Store deffered transctions after each execution phase:
        // For each requester address
        //      For each transaction_tx (temporarily dest address)
        //          Store deferred transfers - Vec<TransferEntry>
        DeferredTransfers get(fn deferred_transfers):
			double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::AccountId => Vec<TransferEntry>;
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

        MultistepCommitResult(u32),

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

        PutCodeFailure,

        InitializationFailure,

        ExecutionFailure,

        CallFailure,

        TerminateFailure,

    }
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

        /// As of now call gets through the general dispatchable call and only receives the current phase.
       #[weight = *gas_limit]
        pub fn multistep_call(
            origin,
            requester: <T as frame_system::Trait>::AccountId,
            target_dest: <T as frame_system::Trait>::AccountId,
		    #[compact] phase: u8,
		    code: Vec<u8>,
		    #[compact] value: BalanceOf<T>,
		    #[compact] gas_limit: Gas,
		    input_data: Vec<u8>
        ) -> dispatch::DispatchResult {
            // ToDo: Endowment should be calculated here automatically based on config, applicable fees and expected lifetime of temporary execution contracts
            let endowment = BalanceOf::<T>::from(100_000_000 as u32);

            let escrow_account = ensure_signed(origin.clone())?;
            // ensure!(sender == <sudo::Module<T>>::key(), "Sender must be the Escrow Account owner");

            // Charge Escrow Account from requester first before executuion.
            // Gas charge needs to be worked out. For now assume the multiplier with gas and token = 1.
            let total_precharge =  BalanceOf::<T>::from(gas_limit as u32) + endowment;
            let cfg = Config::<T>::preload();
            ensure!(
                T::Currency::free_balance(&requester).saturating_sub(total_precharge) >=
                    cfg.existential_deposit.saturating_add(cfg.tombstone_deposit),
                Error::<T>::RequesterNotEnoughBalance,
            );
            just_transfer::<T>(&requester, &escrow_account, total_precharge).map_err(|_| Error::<T>::BalanceTransferFailed)?;
            println!("DEBUG multistep_call -- just_transfer total balance of CONTRACT -- vs REQUESTER {:?} vs ESCROW {:?}", T::Currency::free_balance(&requester), T::Currency::free_balance(&escrow_account));

            // Step 1: contracts::put_code
            instantiate_temp_execution_contract::<T>(origin, code.clone(), &input_data.clone(), endowment.clone(), gas_limit).map_err(|e| e)?;
            let mut gas_meter = GasMeter::<T>::new(gas_limit);
            let dest = T::DetermineContractAddress::contract_address_for(&T::Hashing::hash(&code.clone()), &input_data.clone(), &escrow_account.clone());
            let mut transfers = Vec::<TransferEntry>::new();
            println!("DEBUG multistep_call -- instantiate total balance of CONTRACT {:?} vs REQUESTER {:?} vs ESCROW {:?}", T::Currency::free_balance(&dest), T::Currency::free_balance(&requester), T::Currency::free_balance(&escrow_account));

            // ToDo: Sort out charges for temporary and permanent contracts and cover with tests.
            // charge_as_contract_call::<T>(dest.clone());

            // Proceed with execution
            println!("DEBUG multistep_call -- vm.execute PRE total balance of CONTRACT {:?} vs REQUESTER {:?} vs ESCROW {:?} vs GAS_SPENT ", T::Currency::free_balance(&dest.clone()), T::Currency::free_balance(&requester), T::Currency::free_balance(&escrow_account.clone()));
            let exec_res = execute_escrow_contract_call(escrow_account.clone(), dest.clone(), target_dest.clone(), requester.clone(), code.clone(), value, input_data, cfg, &mut transfers, gas_meter);
            println!("DEBUG multistep_call -- vm.execute POST total balance of CONTRACT {:?} vs REQUESTER {:?} vs ESCROW {:?} vs DAVE {:?} ", T::Currency::free_balance(&dest.clone()), T::Currency::free_balance(&requester), T::Currency::free_balance(&escrow_account.clone()), T::Currency::free_balance(&target_dest));
            let exec_res_val = match exec_res {
				Ok(exec_res_val) => exec_res_val,
				_ => Err(Error::<T>::ExecutionFailure)?
			};

            <DeferredTransfers<T>>::insert(&requester, &dest.clone(), transfers);

            let execution_proofs = ExecutionProofs::<T> {
                // Present the execution proof by hashing the results.
                result: T::Hashing::hash(&exec_res_val.data),
                storage: child::root(&<ContractInfoOf<T>>::get(dest.clone()).unwrap().get_alive().unwrap().child_trie_info()),
                deferred_transfers: <DeferredTransfers<T>>::get(&requester, &dest.clone()),
            };
            println!("DEBUG multistepcall -- Execution Proofs : result {:?}", execution_proofs.result);

           Ok(())
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn rent_projection(
            origin,
            address: <T as frame_system::Trait>::AccountId
        ) -> dispatch::DispatchResult {
            // Ensure that the caller is a regular keypair account
            let caller = ensure_signed(origin)?;
            // Print a test message.
            debug::info!("DEBUG rent_projection by: {:?} for = {:?}", caller, address);
            // For now refer to the contracts rent_projection.
            // In the future rent projection should estimate on % of storage for that address used by escrow account
            <contracts::Module<T>>::rent_projection(address.clone());

            // Raise an event for debug purposes
            Self::deposit_event(RawEvent::RentProjectionCalled(address, caller));

            Ok(())
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn get_storage(
            origin,
            address: <T as frame_system::Trait>::AccountId,
		    key: [u8; 32],
        ) -> dispatch::DispatchResult {
            // Print a test message.

            // Read the contract's storage
            let val = Some(<contracts::Module<T>>::get_storage(address, key));

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
