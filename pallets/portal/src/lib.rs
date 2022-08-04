#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::DispatchError;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use t3rn_primitives::{
    portal::{Portal, RococoBridge},
    xdns::Xdns,
    ChainId, GatewayVendor
};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use core::convert::TryInto;
    use sp_std::vec::Vec;
    use t3rn_primitives::{xdns::Xdns};
    use t3rn_primitives::{
        portal::{RococoBridge},
        abi::{GatewayABIConfig},
        ChainId, EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor, GatewayGenesisConfig,
    };
    use t3rn_primitives::portal::RegistrationData;
    use t3rn_primitives::xdns::AllowedSideEffect;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
    frame_system::Config
    + pallet_grandpa_finality_verifier::Config<RococoBridge>
    {
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
        /// Gateway was registered successsfully. [ChainId]
        GatewayRegistered(ChainId),
        /// Gateway owner was set successfully. [ChainId, Vec<u8>]
        SetOwner(ChainId, Vec<u8>),
        /// Gateway was set operational. [ChainId, bool]
        SetOperational(ChainId, bool),
        /// Header was successfully added
        HeaderSubmitted(ChainId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The creation of the XDNS record was not successful
        XdnsRecordCreationFailed,
        ///Specified Vendor is not implemented
        UnimplementedGatewayVendor,
        /// Gateway registration failed
        RegistrationError,
        /// The gateways vendor is not available, which is a result of a missing XDNS record.
        GatewayVendorNotFound,
        /// Finality Verifier owner can't be set.
        SetOwnerError,
        /// Finality Verifiers operational status can't be updated
        SetOperationalError,
        /// The header could not be added
        SubmitHeaderError,
        /// No gateway height could be found
        NoGatewayHeightAvailable,
        /// SideEffect confirmation failed
        SideEffectConfirmationFailed
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
            gateway_vendor: GatewayVendor,
            gateway_type: GatewayType,
            gateway_genesis: GatewayGenesisConfig,
            gateway_sys_props: GatewaySysProps,
            allowed_side_effects: Vec<AllowedSideEffect>,
            encoded_registration_data: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            // ToDo xdns record is written also when the calls after this fail!!!
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

            let res = match gateway_vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::initialize(origin, gateway_id, encoded_registration_data),
                _ => return Err(Error::<T>::UnimplementedGatewayVendor.into())
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::GatewayRegistered(gateway_id));
                     return Ok(().into())
                },
                Err(msg) => {
                    log::info!("{:?}", msg);
                    return Err(Error::<T>::RegistrationError.into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_owner(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_new_owner: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

            let vendor = match vendor_result {
                Ok(vendor) => vendor,
                Err(_msg) => {
                    return Err(Error::<T>::GatewayVendorNotFound.into())
                }
            };

            let res = match vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::set_owner(origin, gateway_id, encoded_new_owner.clone()),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::SetOwner(gateway_id, encoded_new_owner));
                     return Ok(().into())
                },
                Err(_msg) => {
                    return Err(Error::<T>::SetOwnerError.into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_operational(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            operational: bool
        ) -> DispatchResultWithPostInfo {
            // ToDo find more concise way of doing this
            let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

            let vendor = match vendor_result {
                Ok(vendor) => vendor,
                Err(_msg) => {
                    return Err(Error::<T>::GatewayVendorNotFound.into())
                }
            };

            let res = match vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::set_operational(origin, operational, gateway_id),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::SetOperational(gateway_id, operational));
                     return Ok(().into())
                },
                Err(msg) => {
                    log::info!("{:?}", msg);
                    return Err(Error::<T>::SetOperationalError.into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_headers(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_header_data: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

            let vendor = match vendor_result {
                Ok(vendor) => vendor,
                Err(_msg) => {
                    log::info!("GatewayVendorNotFound");
                    return Err(Error::<T>::GatewayVendorNotFound.into())
                }
            };

            let res = match vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::submit_headers(origin, gateway_id, encoded_header_data),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                    Self::deposit_event(Event::HeaderSubmitted(gateway_id));
                    return Ok(().into())
                 },
                Err(msg) => {
                    log::info!("{:?}", msg);
                    return Err(Error::<T>::SubmitHeaderError.into())
                }
            }
        }
    }
}

impl<T: Config> Portal<T> for Pallet<T> {

    fn get_latest_finalized_header(
        gateway_id: ChainId
    ) -> Option<Vec<u8>> {
        let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

        let vendor = match vendor_result {
            Ok(vendor) => vendor,
            Err(_msg) => {
                return None
            }
        };

        match vendor {
            GatewayVendor::Rococo =>  return pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::get_latest_finalized_header(gateway_id),
            _ => unimplemented!()
        };
    }

    fn get_latest_finalized_height(
        gateway_id: ChainId
    ) -> Result<Vec<u8>, DispatchError> {
        let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;


        let res = match vendor {
            GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::get_latest_finalized_height(gateway_id),
            _ => unimplemented!()
        };

        match res {
            Some(height) => Ok(height),
            None => Err(Error::<T>::NoGatewayHeightAvailable.into())
        }
    }

    fn confirm_and_decode_payload_params(
        gateway_id: [u8; 4],
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<Vec<Vec<u8>>>, DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        let res = match vendor {
            GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::confirm_and_decode_payload_params(
                gateway_id,
                encoded_inclusion_data,
                <T as Config>::Xdns::get_gateway_value_unsigned_type_unsafe(&gateway_id).to_string_bytes()
            ),
            _ => unimplemented!()
        };

        match res {
            Err(_msg) => {
                Err(Error::<T>::SideEffectConfirmationFailed.into())
            },
            Ok(parameters) => Ok(parameters),
        }
    }
}