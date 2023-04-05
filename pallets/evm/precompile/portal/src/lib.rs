#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use pallet_evm::LinearCostPrecompile;

use fp_evm::{
    ExitError, ExitRevert, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure,
    PrecompileHandle, PrecompileOutput, PrecompileResult,
};

use frame_support::pallet_prelude::DispatchError;
use sp_std::vec::Vec;
use std::marker::PhantomData;
use t3rn_abi::{types::Bytes, Abi, Codec, FilledAbi};
use t3rn_primitives::portal::{
    get_portal_interface_abi, Portal, PortalPrecompileInterfaceEnum,
    PortalPrecompileInterfaceEnum::{
        GetCurrentEpoch, GetLatestFinalizedHeader, GetLatestFinalizedHeight,
        GetLatestUpdatedHeight, ReadEpochOffset, ReadFastConfirmationOffset,
        ReadRationalConfirmationOffset, VerifyEventInclusion, VerifyStateInclusion,
        VerifyTxInclusion,
    },
};

pub struct PortalPrecompile<T>(PhantomData<T>);

pub fn recode_input_as_portal_api_enum(
    input: &[u8],
) -> Result<PortalPrecompileInterfaceEnum, PrecompileFailure> {
    let enum_selector_byte: u8 = *input.get(0).ok_or(ExitError::Other(
        "PortalPrecompile failed to derive PortalInterface enum option for provided input".into(),
    ))?;

    let from_rlp_decoded_portal_call_filled_abi = FilledAbi::try_fill_abi(
        get_portal_interface_abi(),
        input.to_vec(),
        Codec::Rlp,
    )
    .map_err(|e| match e {
        DispatchError::Other(err_str) => ExitError::Other(err_str.into()),
        _ => ExitError::Other(
            "PortalPrecompile failed to derive PortalInterface enum option for provided input"
                .into(),
        ),
    })?;

    let mut recoded_interface_option = from_rlp_decoded_portal_call_filled_abi
        .recode_as(&Codec::Rlp, &Codec::Scale)
        .map_err(|e| match e {
            DispatchError::Other(err_str) => ExitError::Other(err_str.into()),
            _ => ExitError::Other(
                "PortalPrecompile failed to recode portal Enum from Rlp to Scale".into(),
            ),
        })?;

    recoded_interface_option.insert(0, enum_selector_byte);

    let recoded_call_as_enum: PortalPrecompileInterfaceEnum =
        PortalPrecompileInterfaceEnum::decode(&mut &recoded_interface_option[..])
            .map_err(|e| ExitError::Other(e.to_string().into()))?;

    Ok(recoded_call_as_enum)
}

impl<T: Portal<T> + frame_system::Config> PortalPrecompile<T> {
    pub fn call_portal(input: &[u8]) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
        match recode_input_as_portal_api_enum(input)? {
            GetLatestFinalizedHeader(chain_id) => {
                let res =
                    <T as Portal<T>>::get_latest_finalized_header(chain_id).map_err(|_| {
                        ExitError::Other("Failed to get latest finalized header".into())
                    })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetLatestFinalizedHeight(chain_id) => {
                let res =
                    <T as Portal<T>>::get_latest_finalized_height(chain_id).map_err(|_| {
                        ExitError::Other("Failed to get latest finalized height".into())
                    })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetLatestUpdatedHeight(chain_id) => {
                let res = <T as Portal<T>>::get_latest_updated_height(chain_id)
                    .map_err(|_| ExitError::Other("Failed to get latest updated height".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetCurrentEpoch(chain_id) => {
                let res = <T as Portal<T>>::get_current_epoch(chain_id)
                    .map_err(|_| ExitError::Other("Failed to get current epoch".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadEpochOffset(chain_id) => {
                let res = <T as Portal<T>>::read_epoch_offset(chain_id)
                    .map_err(|_| ExitError::Other("Failed to read epoch offset".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadFastConfirmationOffset(chain_id) => {
                let res =
                    <T as Portal<T>>::read_fast_confirmation_offset(chain_id).map_err(|_| {
                        ExitError::Other("Failed to read fast confirmation offset".into())
                    })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadRationalConfirmationOffset(chain_id) => {
                let res = <T as Portal<T>>::read_rational_confirmation_offset(chain_id).map_err(
                    |_| ExitError::Other("Failed to read rational confirmation offset".into()),
                )?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            VerifyEventInclusion(chain_id, event) => {
                let res = <T as Portal<T>>::verify_event_inclusion(chain_id, event, None)
                    .map_err(|_| ExitError::Other("Failed to verify event inclusion".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            VerifyStateInclusion(chain_id, event) => {
                let res = <T as Portal<T>>::verify_state_inclusion(chain_id, event, None)
                    .map_err(|_| ExitError::Other("Failed to verify state inclusion".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            VerifyTxInclusion(chain_id, event) => {
                let res = <T as Portal<T>>::verify_tx_inclusion(chain_id, event, None)
                    .map_err(|_| ExitError::Other("Failed to verify tx inclusion".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
        }
    }
}

impl<T: Portal<T> + frame_system::Config> LinearCostPrecompile for PortalPrecompile<T> {
    const BASE: u64 = 3000;
    const WORD: u64 = 200;

    fn execute(
        input: &[u8],
        _max_linear_cost_based_on_input_size: u64,
    ) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
        PortalPrecompile::<T>::call_portal(input)
    }
}

#[cfg(test)]
pub mod test_portal_precompile {
    use super::*;
    use frame_support::assert_ok;
    use rlp::Encodable;
    use t3rn_mini_mock_runtime::{MiniRuntime, Portal, System};
    use t3rn_primitives::portal::{Portal as PortalT, PortalPrecompileInterfaceEnum};

    #[test]
    fn test_get_latest_finalized_header_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = GetLatestFinalizedHeader(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, GetLatestFinalizedHeader(chain_id));
    }

    #[test]
    fn test_get_latest_finalized_height_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = GetLatestFinalizedHeight(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, GetLatestFinalizedHeight(chain_id));
    }

    #[test]
    fn test_get_latest_updated_height_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = GetLatestUpdatedHeight(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, GetLatestUpdatedHeight(chain_id));
    }

    #[test]
    fn test_get_current_epoch_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = GetCurrentEpoch(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, GetCurrentEpoch(chain_id));
    }

    #[test]
    fn test_read_epoch_offset_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = ReadEpochOffset(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, ReadEpochOffset(chain_id));
    }

    #[test]
    fn test_read_fast_confirmation_offset_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = ReadFastConfirmationOffset(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, ReadFastConfirmationOffset(chain_id));
    }

    #[test]
    fn test_read_rational_confirmation_offset_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = ReadRationalConfirmationOffset(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(
            recoded_portal_call,
            ReadRationalConfirmationOffset(chain_id)
        );
    }

    #[test]
    fn test_verify_event_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let portal_call = VerifyEventInclusion(chain_id, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, VerifyEventInclusion(chain_id, event));
    }

    #[test]
    fn test_verify_state_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let portal_call = VerifyStateInclusion(chain_id, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, VerifyStateInclusion(chain_id, event));
    }

    #[test]
    fn test_verify_tx_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let portal_call = VerifyTxInclusion(chain_id, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call = recode_input_as_portal_api_enum(&encoded_portal_call).unwrap();

        assert_eq!(recoded_portal_call, VerifyTxInclusion(chain_id, event));
    }
}
