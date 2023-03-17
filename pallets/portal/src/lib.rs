#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::DispatchError;
pub use pallet::*;

use t3rn_abi::recode::{recode_bytes_with_descriptor, Codec};

#[cfg(test)]
mod tests;

use sp_std::vec::Vec;
use t3rn_primitives::{portal::Portal, xdns::Xdns, ChainId, GatewayVendor};

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

pub mod weights;

// use weights::WeightInfo;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use core::convert::TryInto;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{vec, vec::Vec};
    use t3rn_primitives::{
        gateway::GatewayABIConfig, xdns::Xdns, ChainId, GatewayGenesisConfig, GatewayType,
        GatewayVendor, TokenSysProps,
    };
    use t3rn_types::sfx::Sfx4bId;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_grandpa_finality_verifier::Config<RococoLightClient>
        + pallet_grandpa_finality_verifier::Config<KusamaLightClient>
        + pallet_grandpa_finality_verifier::Config<PolkadotLightClient>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Xdns: Xdns<Self>;
        /// Type representing the weight of this pallet
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

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
        HeaderSubmitted(GatewayVendor, Vec<u8>),
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
        SideEffectConfirmationFailed,
        /// Recoding failed
        SFXRecodeError,
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
            gateway_sys_props: TokenSysProps,
            allowed_side_effects: Vec<Sfx4bId>,
            encoded_registration_data: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin.clone())?;

            // write XDNS record if all else passed
            <T as Config>::Xdns::add_new_xdns_record(
                origin.clone(),
                url,
                gateway_id,
                None,
                gateway_abi,
                gateway_vendor.clone(),
                gateway_type,
                gateway_genesis,
                gateway_sys_props,
                vec![],
                allowed_side_effects,
            )?;

            let res = match gateway_vendor {
                GatewayVendor::Rococo =>
                    pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::initialize(
                        origin,
                        gateway_id,
                        encoded_registration_data,
                    ),
                _ => return Err(Error::<T>::UnimplementedGatewayVendor.into()),
            };

            match res {
                Ok(_) => {
                    Self::deposit_event(Event::GatewayRegistered(gateway_id));
                    Ok(().into())
                },
                Err(msg) => {
                    log::info!("{:?}", msg);
                    Err(Error::<T>::RegistrationError.into())
                },
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_owner(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_new_owner: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)?;

            let res = match vendor {
                GatewayVendor::Rococo => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    RococoLightClient,
                >::set_owner(
                    origin, gateway_id, encoded_new_owner.clone()
                ),
                GatewayVendor::Kusama => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    KusamaLightClient,
                >::set_owner(
                    origin, gateway_id, encoded_new_owner.clone()
                ),
                GatewayVendor::Polkadot => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    PolkadotLightClient,
                >::set_owner(
                    origin, gateway_id, encoded_new_owner.clone()
                ),
                _ => unimplemented!(),
            };

            match res {
                Ok(_) => {
                    Self::deposit_event(Event::SetOwner(gateway_id, encoded_new_owner));
                    Ok(().into())
                },
                Err(_msg) => Err(Error::<T>::SetOwnerError.into()),
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_operational(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            operational: bool,
        ) -> DispatchResultWithPostInfo {
            let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

            let res = match vendor {
                GatewayVendor::Rococo => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    RococoLightClient,
                >::set_operational(
                    origin, operational, gateway_id
                ),
                GatewayVendor::Kusama => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    KusamaLightClient,
                >::set_operational(
                    origin, operational, gateway_id
                ),
                GatewayVendor::Polkadot => pallet_grandpa_finality_verifier::Pallet::<
                    T,
                    PolkadotLightClient,
                >::set_operational(
                    origin, operational, gateway_id
                ),
                _ => unimplemented!(),
            };

            match res {
                Ok(_) => {
                    Self::deposit_event(Event::SetOperational(gateway_id, operational));
                    Ok(().into())
                },
                Err(msg) => {
                    log::info!("{:?}", msg);
                    Err(Error::<T>::SetOperationalError.into())
                },
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_headers(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_header_data: Vec<u8>,
        ) -> DispatchResult {
            let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)?;

            let res = match vendor {
                GatewayVendor::Rococo =>
                    pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::submit_headers(
                        origin,
                        encoded_header_data,
                    ),
                _ => unimplemented!(), // ToDo remove once we remove the old vendors
            };

            match res {
                Ok(height) => {
                    Self::deposit_event(Event::HeaderSubmitted(vendor, height));
                    Ok(())
                },
                Err(msg) => {
                    log::error!("{:?}", msg);
                    Err(Error::<T>::SubmitHeaderError.into())
                },
            }
        }
    }
}

pub fn match_vendor_with_codec(vendor: GatewayVendor) -> Codec {
    match vendor {
        GatewayVendor::Rococo => Codec::Scale,
        GatewayVendor::Kusama => Codec::Scale,
        GatewayVendor::Polkadot => Codec::Scale,
        GatewayVendor::Ethereum => Codec::Rlp,
    }
}

impl<T: Config> Portal<T> for Pallet<T> {
    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<Option<Vec<u8>>, DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        match vendor {
            GatewayVendor::Rococo => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                RococoLightClient,
            >::get_latest_finalized_header(gateway_id)),
            GatewayVendor::Kusama => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                KusamaLightClient,
            >::get_latest_finalized_header(gateway_id)),
            GatewayVendor::Polkadot => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                PolkadotLightClient,
            >::get_latest_finalized_header(gateway_id)),
            _ => Err(Error::<T>::GatewayVendorNotFound.into()),
        }
    }

    fn get_latest_finalized_height(gateway_id: ChainId) -> Result<Option<Vec<u8>>, DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        match vendor {
            GatewayVendor::Rococo => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                RococoLightClient,
            >::get_latest_finalized_height(gateway_id)),
            GatewayVendor::Kusama => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                KusamaLightClient,
            >::get_latest_finalized_height(gateway_id)),
            GatewayVendor::Polkadot => Ok(pallet_grandpa_finality_verifier::Pallet::<
                T,
                PolkadotLightClient,
            >::get_latest_finalized_height(gateway_id)),
            _ => Err(Error::<T>::GatewayVendorNotFound.into()),
        }
    }

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_proof: Vec<u8>,
        side_effect_id: [u8; 4],
    ) -> Result<(Vec<Vec<u8>>, Vec<u8>), DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        match vendor {
            GatewayVendor::Rococo => pallet_grandpa_finality_verifier::Pallet::<T, RococoLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_proof,
                submission_target_height,
            ),
            GatewayVendor::Kusama => pallet_grandpa_finality_verifier::Pallet::<T, KusamaLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            GatewayVendor::Polkadot => pallet_grandpa_finality_verifier::Pallet::<T, PolkadotLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            _ => Err(Error::<T>::GatewayVendorNotFound.into()),
        }
    }

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        match vendor {
            GatewayVendor::Rococo => pallet_grandpa_finality_verifier::Pallet::<T, RococoLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            GatewayVendor::Kusama => pallet_grandpa_finality_verifier::Pallet::<T, KusamaLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            GatewayVendor::Polkadot => pallet_grandpa_finality_verifier::Pallet::<T, PolkadotLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            _ => Err(Error::<T>::GatewayVendorNotFound.into()),
        }
    }

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        submission_target_height: Vec<u8>,
        encoded_inclusion_data: Vec<u8>,
    ) -> Result<Vec<u8>, DispatchError> {
        let vendor = <T as Config>::Xdns::get_gateway_vendor(&gateway_id)
            .map_err(|_| Error::<T>::GatewayVendorNotFound)?;

        match vendor {
            GatewayVendor::Rococo => pallet_grandpa_finality_verifier::Pallet::<T, RococoLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            GatewayVendor::Kusama => pallet_grandpa_finality_verifier::Pallet::<T, KusamaLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            GatewayVendor::Polkadot => pallet_grandpa_finality_verifier::Pallet::<T, PolkadotLightClient>::confirm_event_inclusion(
                gateway_id,
                encoded_inclusion_data,
                submission_target_height,
            ),
            _ => Err(Error::<T>::GatewayVendorNotFound.into()),
        }
    }
}
