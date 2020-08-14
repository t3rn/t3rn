#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::vec::Vec;
use codec::{Decode, Encode};
use frame_support::{debug, decl_error, decl_event, decl_module, ensure, decl_storage, dispatch};
use frame_system::{self as system, ensure_signed, ensure_none};

use sp_std::vec::Vec;
use sp_runtime::{
    traits::{Hash},
};
use contracts::{BalanceOf, Gas};

use codec::{Decode, Encode};

pub type CodeHash<T> = <T as frame_system::Trait>::Hash;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: contracts::Trait + system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as EscrowGateway {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(fn something): Option<u32>;
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

        PutCodeFailure,

        InitializationFailure,

        CallFailure,

        TerminateFailure,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
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
		    #[compact] phase: u8,
		    code: Vec<u8>,
		    #[compact] value: BalanceOf<T>,
		    #[compact] gas_limit: Gas,
		    input_data: Vec<u8>
        ) -> dispatch::DispatchResult {
            let origin_account = origin.clone();
            // ToDo: Configure Sudo module.
            // Check whether the origin comes from the escrow_account owner.
            // Note: Should be similar as sudo-contracts https://github.com/shawntabrizi/sudo-contract/blob/v1.0/src/lib.rs#L34
            let _sender = ensure_signed(origin_account)?;
            // ensure!(sender == <sudo::Module<T>>::key(), "Sender must be the Escrow Account owner");
            // let escrow_engine = ContractsEscrowEngine::new(&<contracts::Module<T>>::current_schedule());
            let escrow_engine = ContractsEscrowEngine::new();

            match phase {
                0 => {
                    debug::info!("DEBUG Execute");
                    // Step 1: contracts::put_code
                    let code_hash_res = <contracts::Module<T>>::put_code(origin.clone(), code.clone());

                    debug::info!("DEBUG multistep_call -- contracts::put_code {:?}", code_hash_res);
                    code_hash_res.map_err(|_e| <Error<T>>::PutCodeFailure)?;

                    let code_hash = T::Hashing::hash(&code);

                    // ToDo: Instantiate works - but charging accounts in unit tests doesn't (due to GenesisConfig not present in Balance err)
                    // Step 2: contracts::instantiate
                    // ToDo: Smart way of calculating endowment that would be enough for initialization + one call.
                    // let temp_endowment = BalanceOf::<T>::from(1_000_000 as u32);
                    //
                    // let init_res = <contracts::Module<T>>::instantiate(origin, temp_endowment, gas_limit, code_hash, input_data);
                    // println!("DEBUG multistepcall -- contracts::instantiate init_res {:?}", init_res);
                    // init_res.map_err(|_e| <Error<T>>::InitializationFailure)?;

                    escrow_engine.feed_escrow_from_contract();

                    // // Step 2.5: contracts::contract_address_for
                    // let dest = <contracts::Module<T>>::contract_address_for(code_hash, origin, input_data);
                    //
                    // // Step 3: contracts::bare_call
                    // let call_res = <contracts::Module<T>>::bare_call(origin, dest, value, gas_limit, input_data);
                    // let (exec_result, gas_used) = call_res.ok_or(<Error<T>>::CallFailure)?;
                    let exec_res = escrow_engine.execute().unwrap();

                    // // Step 4: Cleanup; contracts::ExecutionContext::terminate
                    // let terminate_res = <contracts::Module<T>>::ExecutionContext::terminate(origin, <contracts::Module<T>>:GasMeter);

                    debug::info!("DEBUG multistep_call -- escrow_engine.execute  {:?}", exec_res);
                    Self::deposit_event(RawEvent::MultistepExecutionResult(exec_res));
                },
                1 => {
                    let debug_res = escrow_engine.feed_contract_from_escrow();
                    debug::info!("DEBUG multistep_call -- Commit {}", debug_res);
                    Self::deposit_event(RawEvent::MultistepCommitResult(debug_res));
                    Something::put(debug_res);
                },
                2 => {
                    let debug_res = escrow_engine.revert();
                    debug::info!("DEBUG multistep_call -- Revert {}", debug_res);
                    Self::deposit_event(RawEvent::MultistepRevertResult(debug_res));
                    Something::put(debug_res);
                },
                _ => {
                    debug::info!("DEBUG multistep_call -- Unknown Phase {}", phase);
                    Something::put(phase as u32);
                    Self::deposit_event(RawEvent::MultistepUnknownPhase(phase));
                }
            }

            Ok(())
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn rent_projection(origin, something: u32) -> dispatch::DispatchResult {
            // Ensure that the caller is a regular keypair account
            let caller = ensure_signed(origin)?;
            // Print a test message.
            debug::info!("DEBUG rent_projection by: {:?} val = {}", caller, something);

            Something::put(something);
            // Here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, caller));

            Ok(())
        }

        /// Just a dummy get_storage entry point.
        #[weight = 10_000]
        pub fn get_storage(origin, something: u32) -> dispatch::DispatchResult {
            // Ensure that the caller is a regular keypair account
            let caller = ensure_signed(origin)?;
            // Print a test message.
            debug::info!("DEBUG get_storage by: {:?} val = {}", caller, something);

            Something::put(something);
            // Here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, caller));

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
