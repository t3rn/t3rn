#![cfg_attr(not(feature = "std"), no_std)]
use crate::types::{Bytes, Codec, LightClientId, ChainId};

use sp_runtime::DispatchError;
use frame_system::pallet_prelude::OriginFor;

pub trait LightClientPortal<T: frame_system::Config> {

    // Verify transaction is included in selected light client by ID and its blockchain and finalized.
    // Returns true if transaction is included and finalized, false otherwise.
    // Also, allows to recode the message with the selected (optional) output codec and returns the re-coded message.
    // This is useful for the case when the message is encoded with SCALE codec
    //  but the user's VM expects and can handle only a single codec, e.g. Solidity contracts codec only accept RLP-encoded messages.
    fn verify_tx_and_recode(
        light_client_id: LightClientId,
        chain_id: ChainId,
        message: Bytes,
        output_codec: Option<Codec>
    ) -> Result<(bool, Bytes), DispatchError>;

    // Verify event is included in selected light client by ID and its blockchain and finalized.
    fn verify_event_and_recode(
        light_client_id: LightClientId,
        chain_id: ChainId,
        message: Bytes,
        output_codec: Option<Codec>
    ) -> Result<(bool, Bytes), DispatchError>;

    // Verify state is included in selected light client by ID and its blockchain and finalized.
    fn verify_state_and_recode(
        light_client_id: LightClientId,
        chain_id: ChainId,
        message: Bytes,
        output_codec: Option<Codec>
    ) -> Result<(bool, Bytes), DispatchError>;
}

pub trait LightClient<T: frame_system::Config> {
    fn get_latest_finalized_header() -> Result<Bytes, DispatchError>;

    fn get_latest_exec_header() -> Result<Bytes, DispatchError>;

    fn initialize(
        origin: T::Origin,
        chain_id: ChainId,
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn set_operational(
        origin: T::Origin,
        chain_id: ChainId,
        operational: bool,
    ) -> Result<(), DispatchError>;

    fn submit_headers(
        origin: OriginFor<T>,
        chain_id: ChainId,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError>;

    fn submit_finality_header(
        origin: OriginFor<T>,
        chain_id: ChainId,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError>;

    fn verify_state_included(
        chain_id: ChainId,
        message: Bytes,
    ) -> Result<bool, DispatchError>;

    fn verify_event_included(
        chain_id: ChainId,
        message: Bytes,
    ) -> Result<bool, DispatchError>;

    fn verify_tx_included(
        chain_id: ChainId,
        message: Bytes,
    ) -> Result<bool, DispatchError>;
}
