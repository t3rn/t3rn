#![cfg_attr(not(feature = "std"), no_std)]

use xcm::latest::Xcm;

pub mod xbi_format;

pub mod primitives;
use primitives::evm::Evm;

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

#[frame_support::pallet]
pub mod pallet {
    use crate::{xbi_format::*, *};

    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_core::Hasher;

    #[pallet::storage]
    #[pallet::getter(fn get_xbi_checkins)]
    pub type XBICheckIns<T> =
        StorageMap<_, Identity, <T as frame_system::Config>::Hash, XBIFormat, OptionQuery>;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Evm: Evm<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AbiInstructionExecuted,
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        XBIInstructionNotAllowedHere,
        XBIAlreadyCheckedIn,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn execute_xcm(origin: OriginFor<T>, _xcm: Xcm<Call<T>>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn execute_xbi(origin: OriginFor<T>, xbi: XBIFormat) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // ToDo: XBI::Step::1 Auth for XBI origin check
            match xbi.instr {
                XBIInstr::Notification {
                    kind: _,
                    instruction_id: _,
                    extra: _,
                } => {
                    Self::check_in_instruction(who, xbi)?;
                },
                XBIInstr::CallNative { ref payload } => {
                    // XBI::Step::2 Is the XBI Instruction Allowed on this Parachain
                    Self::check_xbi_instr_allowed_here(XBIInstr::CallNative {
                        payload: payload.to_vec(),
                    })?;
                    // XBI::Step::3 Check in XBI Instruction entry time
                    Self::check_in_instruction(who, xbi)?;
                    // ToDo: XBI::Step::4 Execute!
                    // let message_call = payload.take_decoded().map_err(|_| Error::FailedToDecode)?;
                    // let actual_weight = match message_call.dispatch(dispatch_origin) {
                    // 	Ok(post_info) => post_info.actual_weight,
                    // 	Err(error_and_info) => {
                    // 		// Not much to do with the result as it is. It's up to the parachain to ensure that the
                    // 		// message makes sense.
                    // 		error_and_info.post_info.actual_weight
                    // 	},
                    // }
                },
                XBIInstr::CallEvm {
                    ref caller,
                    ref dest,
                    ref value,
                    ref input,
                    ref gas_limit,
                    max_fee_per_gas: _,
                    max_priority_fee_per_gas: _,
                    nonce: _,
                    access_list: _,
                } => {
                    // XBI::Step::2 Is the XBI Instruction Allowed on this Parachain
                    Self::check_xbi_instr_allowed_here(XBIInstr::CallEvm {
                        caller: caller.clone(),
                        dest: dest.clone(),
                        value: value.clone(),
                        input: input.clone(),
                        gas_limit: gas_limit.clone(),
                        max_fee_per_gas: None,
                        max_priority_fee_per_gas: None,
                        nonce: None,
                        access_list: None,
                    })?;
                    // XBI::Step::3 Check in XBI Instruction entry time
                    Self::check_in_instruction(who, xbi)?;
                    // ToDo: XBI::Step::4 Execute!
                    // pallet_evm::Pallet::<T>::call(
                    // 	caller,
                    // 	dest,
                    // 	value,
                    // 	input,
                    // 	gas_limit,
                    // 	max_fee_per_gas,
                    // 	max_priority_fee_per_gas,
                    // 	nonce,
                    // 	access_list,
                    // )
                },
                XBIInstr::CallWasm {
                    ref caller,
                    ref dest,
                    ref value,
                    ref input,
                } => {
                    // XBI::Step::2 Is the XBI Instruction Allowed on this Parachain
                    Self::check_xbi_instr_allowed_here(XBIInstr::CallWasm {
                        caller: caller.clone(),
                        dest: dest.clone(),
                        value: value.clone(),
                        input: input.clone(),
                    })?;
                    // XBI::Step::3 Check in XBI Instruction entry time
                    Self::check_in_instruction(who, xbi)?;
                    // ToDo: XBI::Step::4 Execute!
                    // pallet_contracts::Pallet::<T>::call(
                    // 	caller,
                    // 	dest,
                    // 	value,
                    // 	input,
                    // )
                },
                XBIInstr::CallCustom { .. } => {},
                XBIInstr::Transfer {
                    ref dest,
                    ref value,
                } => {
                    // XBI::Step::2 Is the XBI Instruction Allowed on this Parachain
                    Self::check_xbi_instr_allowed_here(XBIInstr::Transfer {
                        dest: dest.clone(),
                        value: value.clone(),
                    })?;
                    // XBI::Step::3 Check in XBI Instruction entry time
                    Self::check_in_instruction(who, xbi)?;
                    // ToDo: XBI::Step::4 Execute!
                    // pallet_balances::Pallet::<T>::transfer(
                    // 	who,
                    // 	dest,
                    // 	value,
                    // )
                },
                XBIInstr::TransferMulti {
                    currency_id: _,
                    ref dest,
                    ref value,
                } => {
                    // XBI::Step::2 Is the XBI Instruction Allowed on this Parachain
                    Self::check_xbi_instr_allowed_here(XBIInstr::TransferMulti {
                        currency_id: Default::default(),
                        dest: dest.clone(),
                        value: value.clone(),
                    })?;
                    // XBI::Step::3 Check in XBI Instruction entry time
                    Self::check_in_instruction(who, xbi)?;
                    // ToDo: XBI::Step::4 Execute!
                    // pallet_orml_tokens::Pallet::<T>::transfer(
                    // 	currency_id,
                    // 	who,
                    // 	dest,
                    // 	value,
                    // )
                },
                XBIInstr::Result { .. } => {
                    // ToDo! Check out the XBI Instruction and send back the results
                },
            }

            Self::deposit_event(Event::<T>::AbiInstructionExecuted);

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn check_xbi_instr_allowed_here(xbi_instr: XBIInstr) -> Result<(), Error<T>> {
            // todo: Expose via pallet_xbi_executor::<T>::Config
            return match xbi_instr {
                XBIInstr::CallNative { .. } => Ok(()),
                XBIInstr::CallEvm { .. } => Err(Error::<T>::XBIInstructionNotAllowedHere),
                XBIInstr::CallWasm { .. } => Err(Error::<T>::XBIInstructionNotAllowedHere),
                XBIInstr::CallCustom { .. } => Err(Error::<T>::XBIInstructionNotAllowedHere),
                XBIInstr::Transfer { .. } => Ok(()),
                XBIInstr::TransferMulti { .. } => Ok(()),
                XBIInstr::Result { .. } => Ok(()),
                XBIInstr::Notification { .. } => Ok(()),
            }
        }

        fn check_in_instruction(_who: T::AccountId, xbi: XBIFormat) -> Result<(), Error<T>> {
            let xbi_id = T::Hashing::hash(&xbi.encode()[..]);

            if <Self as Store>::XBICheckIns::contains_key(xbi_id) {
                return Err(Error::<T>::XBIAlreadyCheckedIn)
            }

            <Self as Store>::XBICheckIns::insert(xbi_id, xbi);

            Ok(())
        }

        fn check_in_notification(_who: T::AccountId, xbi: XBIFormat) -> Result<(), Error<T>> {
            let xbi_id = T::Hashing::hash(&xbi.encode()[..]);

            if <Self as Store>::XBICheckIns::contains_key(xbi_id) {
                return Err(Error::<T>::XBIAlreadyCheckedIn)
            }

            <Self as Store>::XBICheckIns::insert(xbi_id, xbi);

            Ok(())
        }

        fn release_notification(_who: T::AccountId, xbi: XBIFormat) -> Result<(), Error<T>> {
            let xbi_id = T::Hashing::hash(&xbi.encode()[..]);

            if <Self as Store>::XBICheckIns::contains_key(xbi_id) {
                return Err(Error::<T>::XBIAlreadyCheckedIn)
            }

            <Self as Store>::XBICheckIns::insert(xbi_id, xbi);

            Ok(())
        }
    }
}
