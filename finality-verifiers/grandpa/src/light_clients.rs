#![cfg_attr(not(feature = "std"), no_std)]

use crate::{to_local_block_number, BridgedBlockNumber, Config, Pallet};
use codec::{Decode, Encode};
use frame_support::traits::{Get, Instance};
use frame_system::pallet_prelude::OriginFor;
use num_traits::AsPrimitive;
use sp_runtime::{
    traits::{CheckedConversion, Header},
    DispatchError,
};
use std::{convert::TryInto, marker::PhantomData};
use t3rn_abi::types::Bytes;
use t3rn_primitives::light_client::{LightClient, LightClientHeartbeat};

impl<T: Config<I>, I: Instance> LightClient<T> for Pallet<T, I> {
    fn get_latest_finalized_header(&self) -> Result<Option<Bytes>, DispatchError> {
        let header = Pallet::<T, I>::best_finalized_map();

        Ok(Some(header.encode()))
    }

    fn get_latest_finalized_height(&self) -> Result<Option<T::BlockNumber>, DispatchError> {
        let header = Pallet::<T, I>::best_finalized_map();
        Ok(Some(to_local_block_number::<T, I>(*header.number())?))
    }

    fn get_latest_updated_height(&self) -> Result<Option<T::BlockNumber>, DispatchError> {
        // todo: rework to use best_updated_map
        let header = Pallet::<T, I>::best_finalized_map();
        Ok(Some(to_local_block_number::<T, I>(*header.number())?))
    }

    fn get_latest_heartbeat(&self) -> Result<LightClientHeartbeat<T>, DispatchError> {
        let header = Pallet::<T, I>::best_finalized_map();
        let last_finalized_height = to_local_block_number::<T, I>(*header.number())?;

        Ok(LightClientHeartbeat {
            last_heartbeat: frame_system::Pallet::<T>::block_number(),
            last_finalized_height,
            last_updated_height: last_finalized_height,
            is_halted: false,
        })
    }

    fn read_fast_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError> {
        Ok(T::FastConfirmationOffset::get())
    }

    fn read_rational_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError> {
        Ok(T::RationalConfirmationOffset::get())
    }

    fn read_finalized_confirmation_offset(&self) -> Result<T::BlockNumber, DispatchError> {
        Ok(T::FinalizedConfirmationOffset::get())
    }

    fn get_current_epoch(&self) -> Result<Option<u32>, DispatchError> {
        Ok(None)
    }

    fn read_epoch_offset(&self) -> Result<T::BlockNumber, DispatchError> {
        Ok(T::EpochOffset::get())
    }

    fn submit_headers(&self, origin: T::Origin, headers: Bytes) -> Result<bool, DispatchError> {
        Pallet::<T, I>::submit_headers(origin, headers)?;
        Ok(true)
    }

    fn submit_finality_header(
        &self,
        origin: OriginFor<T>,
        encoded_header_data: Bytes,
    ) -> Result<bool, DispatchError> {
        self.submit_headers(origin, encoded_header_data)
    }

    fn verify_event_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        Pallet::<T, I>::confirm_event_inclusion(gateway_id, message, submission_target_height)
    }

    fn verify_state_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        unimplemented!("GrandpaFV::verify_storage_inclusion not implemented yet")
    }

    fn verify_tx_inclusion(
        &self,
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        unimplemented!("GrandpaFV::verify_tx_inclusion not implemented yet")
    }

    fn initialize(
        &self,
        origin: T::Origin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError> {
        Pallet::<T, I>::initialize(origin, gateway_id, encoded_registration_data)
            .map_err(|str_err| str_err.into())
    }

    fn turn_on(&self, origin: T::Origin) -> Result<bool, DispatchError> {
        Pallet::<T, I>::set_operational(origin, true)?;
        Ok(true)
    }

    fn turn_off(&self, origin: T::Origin) -> Result<bool, DispatchError> {
        Pallet::<T, I>::set_operational(origin, false)?;
        Ok(true)
    }
}
