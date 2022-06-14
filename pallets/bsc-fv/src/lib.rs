#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
mod crypto;


// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use core::convert::TryInto;

	#[cfg(not(test))]
	use log::{info, warn};

	#[cfg(test)]
	use std::{println as info, println as warn};

	use crate::types::{Header, ValidatorSet, H256, Receipt, Proof};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub type Validators<T> = StorageValue<_, ValidatorSet>;

	#[pallet::storage]
	#[pallet::getter(fn headers)]
	pub type Headers<T> = StorageMap<
        _,
        Identity,
        H256,
        Header,
    >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		HeaderSubmitted([u8; 32]),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The pallet has not been initialized
		NotInitialized,
		/// Non-Epoch block submitted, which is required to this operation
		NonEpochBlock,
		/// Unable to decode the submitted header
		InvalidEncoding,
		/// Invalid header signature detected
		InvalidHeader,
		/// Header Signer not validator
		InvalidSigner,
		/// Hash couldn't be decoded
		InvalidHash,
		/// Header can't be finalized by current ValidatorSet
		ValidatorsUnauthorized,
		/// Header already present in storage
		DuplicateHeader,
		/// Header was not found in storage
		HeaderNotFound,
		/// The submitted inclusion proof does not verify the submitted receipt
		InvalidInclusionProof,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn init_bridge_instance (
			origin: OriginFor<T>,
			encoded_header: Vec<u8>
		) -> DispatchResult {

			let header: Header = match Decode::decode(&mut &*encoded_header) {
				Ok(header) => header,
				Err(_) => return Err(Error::<T>::InvalidEncoding.into())
			};

			if header.number % 200 != 0 {
				return Err(Error::<T>::NonEpochBlock.into())
			}

			// header is invalid. returning error
			if let Err(None) = header.signature_valid() {
				return Err(Error::<T>::InvalidHeader.into())
			}

			let validators = ValidatorSet {
				last_update: header.number,
				validators: header.validators.unwrap()
			};

			<Validators<T>>::put(validators);
			<Headers<T>>::insert(
				header.hash(),
				header,
        	);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_header (
			origin: OriginFor<T>,
			encoded_header: Vec<u8>
		) -> DispatchResult {
			let header: Header = match Decode::decode(&mut &*encoded_header) {
				Ok(header) => header,
				Err(_) => return Err(Error::<T>::InvalidEncoding.into())
			};
			// __________________________________________
			// |	Begin: Header Validity Guards		 |
			// __________________________________________

			// Check: Signature in header is valid
			if let Err(None) = header.signature_valid() {
				return Err(Error::<T>::InvalidHeader.into())
			}

			// Check (Impl): pallet is initialized
			let validators = Validators::<T>::get()
				.ok_or_else(|| <Error<T>>::NotInitialized)?;

			// Check: signer is validator
			if let Err(None) = header.signer_valid(&validators) {
				return Err(Error::<T>::InvalidSigner.into())
			}

			// Check: validators are authorized
			if let Err(None) = header.validator_set_authorized(validators) {
				return Err(Error::<T>::ValidatorsUnauthorized.into())
			}

			// ToDo Check: its the signers turn

			// __________________________________________
			// |	End: Header Validity Guards	         |
			// |   Header is safe to use from here on	 |
			// __________________________________________

			// if header is epoch block, update ValidatorSet
			if header.number % 200 == 0 {
				<Validators<T>>::put(ValidatorSet {
					last_update: header.number,
					validators: header.validators.unwrap() // this is safe, as this is part of the signature
				});
			}

			// write header to storage.
			if let Ok(_) = <Headers<T>>::try_get(&header.hash()) {
				return Err(Error::<T>::DuplicateHeader.into())
			}

			<Headers<T>>::insert(
				&header.hash(), // We use the "wrong" hash here (clique instead of PoSA), as this is used for everything except signature checks
				&header,
			);

			Self::deposit_event(Event::HeaderSubmitted(header.hash().into()));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_header_range(
			origin: OriginFor<T>,
			encoded_headers_reversed: Vec<Vec<u8>>,
			anchor_header_hash: Vec<u8>, // in preparation of LVL3, all parameters should be passed as Vec<u8>. This will allow routing of the FinalityVerifiers through the same pallet
		) -> DispatchResult {

			let anchor_hash: H256 = match Decode::decode(&mut &*anchor_header_hash) {
				Ok(res) => res,
				Err(_) => return Err(Error::<T>::InvalidHash.into())
			};

			let mut anchor = match <Headers<T>>::try_get(anchor_hash) {
				Ok(res) => res,
				Err(_) => return Err(Error::<T>::HeaderNotFound.into())
			};

			// ToDo: this design allows header to be overwritten. While the consensus checks should ensure only valid headers are added, it feels a bit wrong, that there is no range limit in place
			// ToDo - 2: this design ensures only parent headers are added, but it doesn't throw an error if they are incorrect.
			for encoded_header in encoded_headers_reversed {
				let decoded: Header = match Decode::decode(&mut &*encoded_header) {
					Ok(res) => res,
					Err(_) => return Err(Error::<T>::InvalidEncoding.into())
				};

				if decoded.hash() == anchor.parent_hash {
					// We can ignore updating the ValidatorSet here, as we're going backwards
					<Headers<T>>::insert(
						&decoded.hash(),
						&decoded,
					);
				} else {
					break;
				}
				anchor = decoded;
			}

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn check_inclusion(
			origin: OriginFor<T>,
			enc_receipt: Vec<u8>,
			enc_proof: Vec<u8>,
			enc_block_hash: Vec<u8>
		) -> DispatchResult {
			let block_hash: H256 = match Decode::decode(&mut &*enc_block_hash) {
				Ok(res) => res,
				Err(_) => return Err(Error::<T>::InvalidHash.into())
			};

			let receipt_root: H256 = match <Headers<T>>::try_get(block_hash) {
				Ok(res) => res.receipts_root,
				Err(_) => return Err(Error::<T>::HeaderNotFound.into())
			};

			let receipt: Receipt = match Decode::decode(&mut &*enc_receipt) {
				Ok(res) => res,
				Err(_) => return Err(Error::<T>::InvalidEncoding.into())
			};

			let proof: Proof = match Decode::decode(&mut &*enc_proof) {
				Ok(res) => res,
				Err(_) => return Err(Error::<T>::InvalidEncoding.into())
			};

			match receipt.in_block(receipt_root.as_fixed_bytes(), proof) {
				Ok(_) => return Ok(()),
				Err(_) => return Err(Error::<T>::InvalidInclusionProof.into())
			}
		}
	}
}
