#![cfg_attr(not(feature = "std"), no_std)]

// pub use circuit_runtime_pallets::pallet_3vm_evm::{
//     executor::stack::{PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet},
//     Context, ExitError, ExitRevert, ExitSucceed, Transfer,
// };
// use sp_std::vec::Vec;
// use t3rn_abi::{Abi, Codec, FilledAbi};
// use t3rn_primitives::portal::get_portal_interface_abi;
//
// pub type PrecompileResult = Result<PrecompileOutput, PrecompileFailure>;
//
// pub struct PortalPrecompile;
//
// impl LinearCostPrecompile for PortalPrecompile {
//     const BASE: u64 = 3000;
//     const WORD: u64 = 0;
//
//     fn execute<T: PortalPrecompileInterface<BlockNumber>, BlockNumber>(
//         input: &[u8],
//         _: u64,
//     ) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
//         let from_rlp_decoded_portal_call_filled_abi =
//             FilledAbi::try_fill_abi(get_portal_interface_abi(), input.clone(), Codec::Rlp)
//                 .map_err(|_| ExitError::Other("Failed to decode portal Enum".into()))?;
//
//         let enum_option_name = from_rlp_decoded_portal_call_filled_abi
//             .get_name()
//             .map_err(|_| ExitError::Other("Failed to get name from portal Enum".into()))?;
//
//         let enum_option_name_str = sp_std::str::from_utf8(enum_option_name).map_err(|_e| {
//             "CrossCodec::failed to stringify current_arg_name_str, it's useful for debug message"
//         })?;
//
//         match enum_option_name_str {
//             "GetLatestFinalizedHeader" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::get_latest_finalized_header(chain_id)
//                     .map_err(|_| ExitError::Other("Failed to get latest finalized header".into()))?
//             },
//             "GetLatestFinalizedHeight" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::get_latest_finalized_height(chain_id)
//                     .map_err(|_| ExitError::Other("Failed to get latest finalized height".into()))?
//             },
//             "GetLatestUpdatedHeight" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::get_latest_updated_height(chain_id)
//                     .map_err(|_| ExitError::Other("Failed to get latest updated height".into()))?
//             },
//             "GetCurrentEpoch" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::get_current_epoch(chain_id)
//                     .map_err(|_| ExitError::Other("Failed to get current epoch".into()))?
//             },
//             "ReadFastConfirmationOffset" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::read_fast_confirmation_offset(chain_id).map_err(|_| {
//                     ExitError::Other("Failed to read fast confirmation offset".into())
//                 })?
//             },
//             "ReadRationalConfirmationOffset" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::read_rational_confirmation_offset(chain_id).map_err(|_| {
//                     ExitError::Other("Failed to read rational confirmation offset".into())
//                 })?
//             },
//             "ReadEpochOffset" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 T::Portal::read_epoch_offset(chain_id)
//                     .map_err(|_| ExitError::Other("Failed to read epoch offset".into()))?
//             },
//             "VerifyEventInclusion" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 let event_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let event: Bytes = Decode::decode(&mut &event_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode event".into()))?;
//                 let block_number: Option<u32> = Decode::decode(&mut &event_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode block_number".into()))?;
//                 T::Portal::verify_event_inclusion(chain_id, event, block_number)
//                     .map_err(|_| ExitError::Other("Failed to verify event inclusion".into()))?
//             },
//             "VerifyStateInclusion" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 let state_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let state: Bytes = Decode::decode(&mut &state_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode state".into()))?;
//                 let block_number: Option<u32> = Decode::decode(&mut &state_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode block_number".into()))?;
//                 T::Portal::verify_state_inclusion(chain_id, state, block_number)
//                     .map_err(|_| ExitError::Other("Failed to verify state inclusion".into()))?
//             },
//             "VerifyTxInclusion" => {
//                 let chain_id_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let chain_id: [u8; 4] = Decode::decode(&mut &chain_id_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode chain_id".into()))?;
//                 let state_bytes = from_rlp_decoded_portal_call_filled_abi.get_data();
//                 let state: Bytes = Decode::decode(&mut &state_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode state".into()))?;
//                 let block_number: Option<u32> = Decode::decode(&mut &state_bytes[..])
//                     .map_err(|_| ExitError::Other("Failed to decode block_number".into()))?;
//                 T::Portal::verify_state_inclusion(chain_id, state, block_number)
//                     .map_err(|_| ExitError::Other("Failed to verify state inclusion".into()))?
//             },
//         }
//
//         Ok((ExitSucceed::Returned, pubkey.to_vec()))
//     }
// }
//
// impl<BlockNumber> PortalPrecompileInterface<BlockNumber> for PortalPrecompile {}
