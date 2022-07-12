#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// mod types;
// mod crypto;

// use crate::types::{Header, ValidatorSet, H256, Proof, Receipt};
// use t3rn_primitives::bsc_finality_verifier::{BinanceFV};
// use codec::Decode;
// use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use core::convert::TryInto;
	use sp_std::vec::Vec;
	use t3rn_primitives::{xdns::Xdns};
	use codec::Decode;
    // use grandpa_finality_verifier::Event;
    use t3rn_primitives::{
        side_effect::interface::SideEffectInterface,
        xdns::{AllowedSideEffect, XdnsRecord},
        abi::{GatewayABIConfig},
        ChainId, EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor, GatewayGenesisConfig,
    };

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Xdns: Xdns<Self>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// #[pallet::storage]
	// #[pallet::getter(fn validators)]
	// pub type Validators<T> = StorageValue<_, ValidatorSet>;
    //
	// #[pallet::storage]
	// #[pallet::getter(fn headers)]
	// pub type Headers<T> = StorageMap<
    //     _,
    //     Identity,
    //     H256,
    //     Header,
    // >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Test([u8; 32]),
		// HeaderRangeSubmitted(u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        /// The creation of the XDNS record was not successful
		XdnsRecordCreationFailed,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
         #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
         pub fn register_gateway(
            origin: OriginFor<T>,
            url: Vec<u8>,
            gateway_id: ChainId,
            gateway_abi: GatewayABIConfig,
            gateway_vendor: GatewayVendor, // Maps to FV
            gateway_type: GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            gateway_sys_props: GatewaySysProps,
            allowed_side_effects: Vec<AllowedSideEffect>,
            registration_data: Vec<u8>
         ) -> DispatchResultWithPostInfo {
             <T as Config>::Xdns::add_new_xdns_record(
                origin.clone(),
                url,
                gateway_id,
                None,
                gateway_abi.clone(),
                gateway_vendor.clone(),
                gateway_type.clone(),
                gateway_genesis,
                gateway_sys_props.clone(),
                allowed_side_effects.clone(),
            )?;

             // match gateway_vendor {
             //     GatewayVendor::Rococo =>
             // }



             Ok(().into())
         }


	}
}

// impl<T: Config> Portal<T> for Pallet<T> {
	// fn init_bridge_instance (
	// 	encoded_header: Vec<u8>
	// ) -> Result<(), &'static str> {
    //
	// 	let header: Header = match Decode::decode(&mut &*encoded_header) {
	// 		Ok(header) => header,
	// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
	// 	};
    //
	// 	if header.number % 200 != 0 {
	// 		return Err(Error::<T>::NonEpochBlock.into())
	// 	}
    //
	// 	// header is invalid. returning error
	// 	if let Err(None) = header.signature_valid() {
	// 		return Err(Error::<T>::InvalidHeader.into())
	// 	}
    //
	// 	let validators = ValidatorSet {
	// 		last_update: header.number,
	// 		validators: header.validators.unwrap()
	// 	};
    //
	// 	<Validators<T>>::put(validators);
	// 	<Headers<T>>::insert(
	// 		header.hash(),
	// 		header,
	// 	);
    //
	// 	Ok(())
	// }

	// fn check_inclusion(
	// 	enc_receipt: Vec<u8>,
	// 	enc_proof: Option<Vec<u8>>,
	// 	enc_block_hash: Vec<u8>
	// ) -> Result<(), &'static str> {
	// 	// ToDo: remove for release
	// 	if let None = enc_proof {
	// 		return Ok(())
	// 	}
	// 	let block_hash: H256 = match Decode::decode(&mut &*enc_block_hash) {
	// 		Ok(res) => res,
	// 		Err(_) => return Err(Error::<T>::InvalidHash.into())
	// 	};
    //
	// 	let receipt_root: H256 = match <Headers<T>>::try_get(block_hash) {
	// 		Ok(res) => res.receipts_root,
	// 		Err(_) => return Err(Error::<T>::HeaderNotFound.into())
	// 	};
    //
	// 	let receipt: Receipt = match Decode::decode(&mut &*enc_receipt) {
	// 		Ok(res) => res,
	// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
	// 	};
    //
	// 	let proof: Proof = match Decode::decode(&mut &*enc_proof.unwrap()) {
	// 		Ok(res) => res,
	// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
	// 	};
    //
	// 	match receipt.in_block(receipt_root.as_fixed_bytes(), proof) {
	// 		Ok(_) => return Ok(()),
	// 		Err(_) => return Err(Error::<T>::InvalidInclusionProof.into())
	// 	}
	// }
// }