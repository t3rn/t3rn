#![cfg_attr(not(feature = "std"), no_std)]
extern crate t3rn_primitives;

use codec::{Decode, Encode};
use pallet_evm::LinearCostPrecompile;

use fp_evm::{
    ExitError, ExitRevert, ExitSucceed, Precompile as EvmPrecompile, PrecompileFailure,
    PrecompileHandle, PrecompileOutput, PrecompileResult,
};

use sp_std::vec::Vec;
use std::marker::PhantomData;
use t3rn_abi::{types::Bytes, Abi, Codec, FilledAbi};
use t3rn_primitives::portal::{
    get_portal_interface_abi, Portal, PortalPrecompileInterfaceEnum,
    PortalPrecompileInterfaceEnum::{
        GetCurrentEpoch, GetLatestFinalizedHeader, GetLatestFinalizedHeight,
        GetLatestUpdatedHeight, ReadEpochOffset, ReadFastConfirmationOffset,
        ReadRationalConfirmationOffset, VerifyEventInclusion,
    },
};

#[cfg(test)]
mod mock;

pub struct PortalPrecompile<T, BlockNumber>(PhantomData<(T, BlockNumber)>);

impl<T: Portal<T> + frame_system::Config, BlockNumber> PortalPrecompile<T, BlockNumber> {
    pub fn call_portal(input: &[u8]) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
        let from_rlp_decoded_portal_call_filled_abi =
            FilledAbi::try_fill_abi(get_portal_interface_abi(), input.to_vec(), Codec::Rlp)
                .map_err(|_| ExitError::Other("Failed to decode portal Enum".into()))?;

        let recoded_call =
            from_rlp_decoded_portal_call_filled_abi.recode_as(&Codec::Rlp, &Codec::Scale)?;

        let recoded_call_as_enum: PortalPrecompileInterfaceEnum =
            Decode::decode(&mut &recoded_call[..])
                .map_err(|_| ExitError::Other("Failed to decode portal Enum".into()))?;

        match recoded_call_as_enum {
            GetLatestFinalizedHeader(chain_id) => {
                let res = T::get_latest_finalized_header(chain_id).map_err(|_| {
                    ExitError::Other("Failed to get latest finalized header".into())
                })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetLatestFinalizedHeight(chain_id) => {
                let res = T::get_latest_finalized_height(chain_id).map_err(|_| {
                    ExitError::Other("Failed to get latest finalized height".into())
                })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetLatestUpdatedHeight(chain_id) => {
                let res = T::get_latest_updated_height(chain_id)
                    .map_err(|_| ExitError::Other("Failed to get latest updated height".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            GetCurrentEpoch(chain_id) => {
                let res = T::get_current_epoch(chain_id)
                    .map_err(|_| ExitError::Other("Failed to get current epoch".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadEpochOffset(chain_id) => {
                let res = T::read_epoch_offset(chain_id)
                    .map_err(|_| ExitError::Other("Failed to read epoch offset".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadFastConfirmationOffset(chain_id) => {
                let res = T::read_fast_confirmation_offset(chain_id).map_err(|_| {
                    ExitError::Other("Failed to read fast confirmation offset".into())
                })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            ReadRationalConfirmationOffset(chain_id) => {
                let res = T::read_rational_confirmation_offset(chain_id).map_err(|_| {
                    ExitError::Other("Failed to read rational confirmation offset".into())
                })?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
            VerifyEventInclusion(chain_id, event) => {
                let res = T::verify_event_inclusion(chain_id, event)
                    .map_err(|_| ExitError::Other("Failed to verify event inclusion".into()))?;

                Ok((ExitSucceed::Returned, res.encode()))
            },
        }
    }
}

impl<T: Portal<T> + frame_system::Config, BlockNumber> LinearCostPrecompile
    for PortalPrecompile<T, BlockNumber>
{
    const BASE: u64 = 3000;
    const WORD: u64 = 200;

    fn execute(
        input: &[u8],
        _max_linear_cost_based_on_input_size: u64,
    ) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
        PortalPrecompile::<T, BlockNumber>::call_portal(input)
    }
}

#[cfg(test)]
pub mod test_portal_precompile {
    use super::*;
    use frame_support::assert_ok;
    use rlp::Encodable;
    use t3rn_primitives::portal::PortalPrecompileInterfaceEnum;

    #[test]
    fn test_get_latest_finalized_header() {
        Externalities::new()
            .with_balance(None, 12345)
            .build()
            .execute_with(|| {
                let chain_id = [0u8; 4];
                let portal_call_option_prefix = 0u8;
                let portal_call = portal_call_option_prefix.extend(&chain_id);
                let portal_call_encoded = portal_call.encode();
                let portal_call_encoded_rlp = portal_call_encoded.rlp_bytes();
                let portal_call_encoded_rlp_vec = portal_call_encoded_rlp.to_vec();

                let res = PortalPrecompile::<Test, u32>::call_portal(&portal_call_encoded_rlp_vec);
                assert_ok!(res);
            });
    }
}
