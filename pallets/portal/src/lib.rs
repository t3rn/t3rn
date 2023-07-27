#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{sp_runtime::DispatchError, traits::Get};
use frame_system::{ensure_root, pallet_prelude::OriginFor};
pub use pallet::*;
use sp_std::{boxed::Box, prelude::*};
use t3rn_abi::recode::{recode_bytes_with_descriptor, Codec};

#[cfg(test)]
mod tests;

use t3rn_abi::types::Bytes;
use t3rn_primitives::{
    self, execution_source_to_option,
    light_client::LightClient,
    portal::{HeaderResult, HeightResult, Portal},
    reexport_currency_types,
    xdns::Xdns,
    ChainId, ExecutionSource, GatewayVendor, SpeedMode, TokenInfo,
};

pub mod weights;

pub trait SelectLightClient<T: frame_system::Config> {
    fn select(vendor: GatewayVendor) -> Result<Box<dyn LightClient<T>>, Error<T>>;
}
use frame_support::transactional;
use sp_runtime::traits::Zero;
use t3rn_primitives::{light_client::LightClientHeartbeat, portal::InclusionReceipt};

reexport_currency_types!();

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use core::convert::TryInto;
    use frame_support::{pallet_prelude::*, traits::Currency};

    use sp_std::vec::Vec;
    use t3rn_primitives::{xdns::Xdns, ChainId, ExecutionVendor, GatewayVendor};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
<<<<<<< HEAD
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Currency access
        type Currency: Currency<Self::AccountId>;
=======
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
>>>>>>> origin/chore/update-flow
        /// Access to XDNS pallet
        type Xdns: Xdns<Self, BalanceOf<Self>>;
        /// Type representing the weight of this pallet
        type WeightInfo: crate::weights::WeightInfo;

        type SelectLightClient: SelectLightClient<Self>;
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
        /// The light client could not be found
        LightClientNotFoundByVendor,
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
        #[pallet::weight(T::DbWeight::get().writes(1))]
        #[transactional]
        pub fn register_gateway(
            origin: OriginFor<T>,
            gateway_id: [u8; 4],
            token_id: u32,
            verification_vendor: GatewayVendor,
            execution_vendor: ExecutionVendor,
            codec: t3rn_abi::Codec,
            registrant: Option<T::AccountId>,
            escrow_account: Option<T::AccountId>,
            allowed_side_effects: Vec<([u8; 4], Option<u8>)>,
            token_props: TokenInfo,
            encoded_registration_data: Bytes,
        ) -> DispatchResult {
            ensure_root(origin.clone())?;
            <T as Config>::Xdns::add_new_gateway(
                gateway_id,
                verification_vendor,
                execution_vendor,
                codec,
                registrant,
                escrow_account,
                allowed_side_effects,
            )?;
            <T as Config>::Xdns::register_new_token(&origin, token_id, token_props.clone())?;
            <T as Config>::Xdns::link_token_to_gateway(token_id, gateway_id, token_props)?;
            <Pallet<T> as Portal<T>>::initialize(origin, gateway_id, encoded_registration_data)
        }
    }
}

// ToDo: this should come from XDNS
pub fn match_vendor_with_codec(vendor: GatewayVendor) -> Codec {
    match vendor {
        GatewayVendor::Rococo => Codec::Scale,
        GatewayVendor::Kusama => Codec::Scale,
        GatewayVendor::Polkadot => Codec::Scale,
        GatewayVendor::Ethereum => Codec::Rlp,
        GatewayVendor::Sepolia => Codec::Rlp,
        GatewayVendor::XBI => Codec::Scale,
    }
}

pub fn match_light_client_by_gateway_id<T: Config>(
    gateway_id: ChainId,
) -> Result<Box<dyn LightClient<T>>, Error<T>> {
    let vendor = <T as Config>::Xdns::get_verification_vendor(&gateway_id)
        .map_err(|_| Error::<T>::GatewayVendorNotFound)?;
    T::SelectLightClient::select(vendor)
}

impl<T: Config> Portal<T> for Pallet<T> {
    fn get_latest_heartbeat(
        gateway_id: &ChainId,
    ) -> Result<LightClientHeartbeat<T>, DispatchError> {
        match_light_client_by_gateway_id::<T>(*gateway_id)?.get_latest_heartbeat()
    }

    fn get_latest_heartbeat_by_vendor(vendor: GatewayVendor) -> LightClientHeartbeat<T> {
        match T::SelectLightClient::select(vendor) {
            Ok(light_client) => light_client.get_latest_heartbeat().unwrap_or_default(),
            Err(_) => LightClientHeartbeat::default(),
        }
    }

    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<HeaderResult, DispatchError> {
        log::debug!(target: "portal", "Getting latest finalized header for gateway id {:?}", gateway_id);
        Ok(match_light_client_by_gateway_id::<T>(gateway_id)?.get_latest_finalized_header())
    }

    fn get_finalized_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError> {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        Ok(match_light_client_by_gateway_id::<T>(gateway_id)?.get_finalized_height())
    }

    fn get_rational_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError> {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        Ok(match_light_client_by_gateway_id::<T>(gateway_id)?.get_rational_height())
    }

    fn get_fast_height(gateway_id: ChainId) -> Result<HeightResult<T::BlockNumber>, DispatchError> {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        Ok(match_light_client_by_gateway_id::<T>(gateway_id)?.get_fast_height())
    }

    fn get_latest_finalized_header_precompile(gateway_id: ChainId) -> Bytes {
        log::debug!(target: "portal", "Getting latest finalized header for gateway id {:?}", gateway_id);
        if let Ok(light_client) = match_light_client_by_gateway_id::<T>(gateway_id) {
            if let HeaderResult::Header(header) = light_client.get_latest_finalized_header() {
                return header
            }
        }
        vec![]
    }

    fn get_finalized_height_precompile(gateway_id: ChainId) -> T::BlockNumber {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        if let Ok(light_client) = match_light_client_by_gateway_id::<T>(gateway_id) {
            if let HeightResult::Height(height) = light_client.get_finalized_height() {
                return height
            }
        }
        T::BlockNumber::zero()
    }

    fn get_rational_height_precompile(gateway_id: ChainId) -> T::BlockNumber {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        if let Ok(light_client) = match_light_client_by_gateway_id::<T>(gateway_id) {
            if let HeightResult::Height(height) = light_client.get_rational_height() {
                return height
            }
        }
        T::BlockNumber::zero()
    }

    fn get_fast_height_precompile(gateway_id: ChainId) -> T::BlockNumber {
        log::debug!(target: "portal", "Getting latest finalized height for gateway id {:?}", gateway_id);
        if let Ok(light_client) = match_light_client_by_gateway_id::<T>(gateway_id) {
            if let HeightResult::Height(height) = light_client.get_fast_height() {
                return height
            }
        }
        T::BlockNumber::zero()
    }

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: Option<ExecutionSource>,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        // ToDo: we need to verify the event source here
        match_light_client_by_gateway_id::<T>(gateway_id)?
            .verify_event_inclusion(gateway_id, speed_mode, source, message)
    }

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?
            .verify_state_inclusion(gateway_id, speed_mode, message)
    }

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?
            .verify_tx_inclusion(gateway_id, speed_mode, message)
    }

    fn verify_event_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        // ToDo: we need to verify the event source here
        let result = match_light_client_by_gateway_id::<T>(gateway_id)?.verify_event_inclusion(
            gateway_id,
            speed_mode,
            execution_source_to_option(source),
            message,
        )?;
        Ok(result.message)
    }

    fn verify_state_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        let result = match_light_client_by_gateway_id::<T>(gateway_id)?
            .verify_state_inclusion(gateway_id, speed_mode, message)?;

        Ok(result.message)
    }

    fn verify_tx_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError> {
        let result = match_light_client_by_gateway_id::<T>(gateway_id)?
            .verify_tx_inclusion(gateway_id, speed_mode, message)?;
        Ok(result.message)
    }

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        let mut inclusion_check = Self::verify_state_inclusion(gateway_id, speed_mode, message)?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        let recoded_message = recode_bytes_with_descriptor(
            inclusion_check.message,
            abi_descriptor,
            in_codec,
            out_codec,
        )?;
        inclusion_check.message = recoded_message;

        Ok(inclusion_check)
    }

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        let mut inclusion_check = Self::verify_tx_inclusion(gateway_id, speed_mode, message)?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        let recoded_message = recode_bytes_with_descriptor(
            inclusion_check.message,
            abi_descriptor,
            in_codec,
            out_codec,
        )?;
        inclusion_check.message = recoded_message;

        Ok(inclusion_check)
    }

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError> {
        let mut inclusion_check = Self::verify_event_inclusion(
            gateway_id,
            speed_mode,
            execution_source_to_option(source),
            message,
        )?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        let recoded_message = recode_bytes_with_descriptor(
            inclusion_check.message,
            abi_descriptor,
            in_codec,
            out_codec,
        )?;
        inclusion_check.message = recoded_message;

        Ok(inclusion_check)
    }

    fn initialize(
        origin: OriginFor<T>,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.initialize(
            origin,
            gateway_id,
            encoded_registration_data,
        )
    }

    fn submit_encoded_headers(
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>,
    ) -> Result<(), DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?
            .submit_encoded_headers(encoded_header_data)?;
        Ok(())
    }

    fn turn_on(origin: OriginFor<T>, gateway_id: [u8; 4]) -> Result<bool, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.turn_on(origin)
    }

    fn turn_off(origin: OriginFor<T>, gateway_id: [u8; 4]) -> Result<bool, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.turn_off(origin)
    }
}
