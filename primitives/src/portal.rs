pub use crate::light_client::{HeaderResult, HeightResult, InclusionReceipt};
use crate::{
    gateway::GatewayABIConfig, ChainId, ExecutionSource, ExecutionVendor, GatewayGenesisConfig,
    GatewayType, GatewayVendor, SpeedMode, TokenInfo,
};
use codec::{Decode, Encode};

use crate::light_client::LightClientHeartbeat;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::{convert::TryFrom, vec::Vec};
use t3rn_abi::{recode::Codec, types::Bytes, Abi, FilledAbi};
use t3rn_types::sfx::Sfx4bId;

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub struct RegistrationData {
    pub url: Bytes,
    pub gateway_id: ChainId,
    pub gateway_abi: GatewayABIConfig,
    pub gateway_vendor: GatewayVendor,
    pub execution_vendor: ExecutionVendor,
    pub gateway_type: GatewayType,
    pub gateway_genesis: GatewayGenesisConfig,
    pub gateway_sys_props: TokenInfo,
    pub allowed_side_effects: Vec<Sfx4bId>,
    pub encoded_registration_data: Bytes,
}

// This could be split into readable parts here, or even more specific traits in the future, if needed.
// Something like `.. Portal: ReadHeaders + Submit { ..`
pub trait Portal<T: frame_system::Config> {
    fn get_latest_heartbeat(gateway_id: &ChainId)
        -> Result<LightClientHeartbeat<T>, DispatchError>;

    fn get_latest_heartbeat_by_vendor(vendor: GatewayVendor) -> LightClientHeartbeat<T>;

    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<HeaderResult, DispatchError>;

    fn get_finalized_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_rational_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_fast_height(gateway_id: ChainId) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_finalized_header_precompile(gateway_id: ChainId) -> Bytes;

    fn get_finalized_height_precompile(gateway_id: ChainId) -> T::BlockNumber;

    fn get_rational_height_precompile(gateway_id: ChainId) -> T::BlockNumber;

    fn get_fast_height_precompile(gateway_id: ChainId) -> T::BlockNumber;

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: Option<ExecutionSource>,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion_precompile(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        speed_mode: SpeedMode,
        source: ExecutionSource,
        message: Bytes,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn initialize(
        origin: T::RuntimeOrigin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn submit_encoded_headers(
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>,
    ) -> Result<(), DispatchError>;

    fn turn_on(origin: T::RuntimeOrigin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;

    fn turn_off(origin: T::RuntimeOrigin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub enum PortalExecution<T: frame_system::Config> {
    Header(HeaderResult),
    Height(HeightResult<T::BlockNumber>),
    Inclusion(InclusionReceipt<T::BlockNumber>),
    BlockNumber(T::BlockNumber),
    Data(Bytes),
    Switched(bool),
    Noop,
}

impl<T: frame_system::Config> From<HeaderResult> for PortalExecution<T> {
    fn from(value: HeaderResult) -> Self {
        Self::Header(value)
    }
}
impl<T: frame_system::Config> From<HeightResult<T::BlockNumber>> for PortalExecution<T> {
    fn from(value: HeightResult<T::BlockNumber>) -> Self {
        Self::Height(value)
    }
}
impl<T: frame_system::Config> From<InclusionReceipt<T::BlockNumber>> for PortalExecution<T> {
    fn from(value: InclusionReceipt<T::BlockNumber>) -> Self {
        Self::Inclusion(value)
    }
}
impl<T: frame_system::Config> From<Bytes> for PortalExecution<T> {
    fn from(value: Bytes) -> Self {
        Self::Data(value)
    }
}
impl<T: frame_system::Config> From<bool> for PortalExecution<T> {
    fn from(value: bool) -> Self {
        Self::Switched(value)
    }
}
impl<T: frame_system::Config> From<()> for PortalExecution<T> {
    fn from(_value: ()) -> Self {
        Self::Noop
    }
}

// Justification, don't need from here, would require unneeded implementation too
#[allow(clippy::from_over_into)]
impl<T: frame_system::Config> Into<Bytes> for PortalExecution<T> {
    fn into(self) -> Bytes {
        match self {
            PortalExecution::Header(x) => x.encode(),
            PortalExecution::Height(x) => x.encode(),
            PortalExecution::Inclusion(x) => x.encode(),
            PortalExecution::BlockNumber(x) => x.encode(),
            PortalExecution::Data(x) => x.encode(),
            PortalExecution::Switched(x) => x.encode(),
            PortalExecution::Noop => sp_std::vec![],
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, TypeInfo)]
pub enum PrecompileArgs {
    GetLatestFinalizedHeader(ChainId),
    GetFinalizedHeight(ChainId),
    GetRationalHeight(ChainId),
    GetFastHeight(ChainId),
    VerifyEventInclusion(ChainId, SpeedMode, ExecutionSource, Bytes),
    VerifyStateInclusion(ChainId, SpeedMode, Bytes),
    VerifyTxInclusion(ChainId, SpeedMode, Bytes),
}

impl Encode for PrecompileArgs {
    fn encode_to<W: codec::Output + ?Sized>(&self, dest: &mut W) {
        match self {
            PrecompileArgs::GetLatestFinalizedHeader(chain_id) => {
                0u8.encode_to(dest);
                chain_id.encode_to(dest);
            },
            PrecompileArgs::GetFinalizedHeight(chain_id) => {
                1u8.encode_to(dest);
                chain_id.encode_to(dest);
            },
            PrecompileArgs::GetRationalHeight(chain_id) => {
                2u8.encode_to(dest);
                chain_id.encode_to(dest);
            },
            PrecompileArgs::GetFastHeight(chain_id) => {
                3u8.encode_to(dest);
                chain_id.encode_to(dest);
            },
            PrecompileArgs::VerifyEventInclusion(chain_id, speed_mode, source, bytes) => {
                4u8.encode_to(dest);
                chain_id.encode_to(dest);
                speed_mode.encode_to(dest);
                source.encode_to(dest);
                dest.write(bytes);
            },
            PrecompileArgs::VerifyStateInclusion(chain_id, speed_mode, bytes) => {
                5u8.encode_to(dest);
                chain_id.encode_to(dest);
                speed_mode.encode_to(dest);
                dest.write(bytes);
            },
            PrecompileArgs::VerifyTxInclusion(chain_id, speed_mode, bytes) => {
                6u8.encode_to(dest);
                chain_id.encode_to(dest);
                speed_mode.encode_to(dest);
                dest.write(bytes);
            },
        }
    }
}
impl Decode for PrecompileArgs {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let tag = u8::decode(input)?;
        match tag {
            0 => {
                let chain_id = ChainId::decode(input)?;
                Ok(PrecompileArgs::GetLatestFinalizedHeader(chain_id))
            },
            1 => {
                let chain_id = ChainId::decode(input)?;
                Ok(PrecompileArgs::GetFinalizedHeight(chain_id))
            },
            2 => {
                let chain_id = ChainId::decode(input)?;
                Ok(PrecompileArgs::GetRationalHeight(chain_id))
            },
            3 => {
                let chain_id = ChainId::decode(input)?;
                Ok(PrecompileArgs::GetFastHeight(chain_id))
            },
            4 => {
                let chain_id = ChainId::decode(input)?;
                let speed_mode = SpeedMode::decode(input)?;
                let source = ExecutionSource::decode(input)?;
                let mut bytes = Vec::new();
                while let Ok(byte) = input.read_byte() {
                    bytes.push(byte);
                }
                Ok(PrecompileArgs::VerifyEventInclusion(
                    chain_id, speed_mode, source, bytes,
                ))
            },
            5 => {
                let chain_id = ChainId::decode(input)?;
                let speed_mode = SpeedMode::decode(input)?;
                let mut bytes = Vec::new();
                while let Ok(byte) = input.read_byte() {
                    bytes.push(byte);
                }
                Ok(PrecompileArgs::VerifyStateInclusion(
                    chain_id, speed_mode, bytes,
                ))
            },
            6 => {
                let chain_id = ChainId::decode(input)?;
                let speed_mode = SpeedMode::decode(input)?;
                let mut bytes = Vec::new();
                while let Ok(byte) = input.read_byte() {
                    bytes.push(byte);
                }
                Ok(PrecompileArgs::VerifyTxInclusion(
                    chain_id, speed_mode, bytes,
                ))
            },
            _ => Err("Invalid tag".into()),
        }
    }
}

impl PrecompileArgs {
    pub fn descriptor() -> Vec<u8> {
        b"PrecompileArgs:Enum(\
                GetLatestFinalizedHeader:Bytes4,\
                GetFinalizedHeight:Bytes4,\
                GetRationalHeight:Bytes4,\
                GetFastHeight:Bytes4,\
                VerifyEventInclusion:Quadruple(Bytes4,Byte,H256,Bytes),\
                VerifyStateInclusion:Triple(Bytes4,Byte,Bytes),\
                VerifyTxInclusion:Triple(Bytes4,Byte,Bytes),\
        )"
        .to_vec()
    }

    pub fn interface_abi() -> Result<Abi, DispatchError> {
        Abi::try_from(Self::descriptor())
    }

    pub fn recode_to_scale_and_decode(
        in_codec: &t3rn_abi::Codec,
        input: &[u8],
    ) -> Result<Self, DispatchError> {
        if input.len() < 2 {
            return Err(DispatchError::Other("Not enough arguments to build enum"))
        }
        // First byte is portal selector
        let portal_selector = &input[0];

        match in_codec {
            t3rn_abi::Codec::Rlp => {
                log::debug!(
                    target: "portal::recode",
                    "Rlp encoding bytes for portal selector {}",
                    portal_selector
                );
                log::trace!(
                    target: "portal::recode",
                    "Bytes {:?}",
                    input
                );
                FilledAbi::try_fill_abi(Self::interface_abi()?, input.to_vec(), in_codec.clone())
                    .and_then(|abi| {
                        log::debug!(
                            target: "portal::recode",
                            "ABI was filled, recoding to scale {}",
                            portal_selector
                        );

                        abi.recode_as(&in_codec.clone(), &t3rn_abi::Codec::Scale)
                    })
            },
            t3rn_abi::Codec::Scale => Ok(input.to_vec()),
        }
        .map(|mut recoded| {
            recoded.insert(0, *portal_selector);
            recoded
        })
        .and_then(|recoded| {
            Self::decode(&mut &recoded[..])
                .map_err(|_e| DispatchError::Other("Failed to decode portal interface enum"))
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use t3rn_abi::{Abi, Codec, FilledAbi};

    #[test]
    fn portal_interface_abi_descriptor_parses() {
        let portal_interface_abi = PrecompileArgs::interface_abi().unwrap();

        let _expected_abi = Abi::Enum(
            Some(b"PrecompileArgs".to_vec()),
            vec![
                Box::new(Abi::Bytes4(Some(b"GetLatestFinalizedHeader".to_vec()))),
                Box::new(Abi::Bytes4(Some(b"GetFinalizedHeight".to_vec()))),
                Box::new(Abi::Bytes4(Some(b"GetRationalHeight".to_vec()))),
                Box::new(Abi::Bytes4(Some(b"GetFastHeight".to_vec()))),
                Box::new(Abi::Quadruple(
                    Some(b"VerifyEventInclusion".to_vec()),
                    (
                        Box::new(Abi::Bytes4(None)),
                        Box::new(Abi::Byte(None)),
                        Box::new(Abi::H256(None)),
                        Box::new(Abi::Bytes(None)),
                    ),
                )),
                Box::new(Abi::Triple(
                    Some(b"VerifyStateInclusion".to_vec()),
                    (
                        Box::new(Abi::Bytes4(None)),
                        Box::new(Abi::Byte(None)),
                        Box::new(Abi::Bytes(None)),
                    ),
                )),
                Box::new(Abi::Triple(
                    Some(b"VerifyTxInclusion".to_vec()),
                    (
                        Box::new(Abi::Bytes4(None)),
                        Box::new(Abi::Byte(None)),
                        Box::new(Abi::Bytes(None)),
                    ),
                )),
            ],
        );

        assert_eq!(
            portal_interface_abi.encode(),
            [
                3, 1, 56, 80, 114, 101, 99, 111, 109, 112, 105, 108, 101, 65, 114, 103, 115, 28, 9,
                1, 96, 71, 101, 116, 76, 97, 116, 101, 115, 116, 70, 105, 110, 97, 108, 105, 122,
                101, 100, 72, 101, 97, 100, 101, 114, 9, 1, 72, 71, 101, 116, 70, 105, 110, 97,
                108, 105, 122, 101, 100, 72, 101, 105, 103, 104, 116, 9, 1, 68, 71, 101, 116, 82,
                97, 116, 105, 111, 110, 97, 108, 72, 101, 105, 103, 104, 116, 9, 1, 52, 71, 101,
                116, 70, 97, 115, 116, 72, 101, 105, 103, 104, 116, 21, 1, 80, 86, 101, 114, 105,
                102, 121, 69, 118, 101, 110, 116, 73, 110, 99, 108, 117, 115, 105, 111, 110, 9, 0,
                14, 0, 7, 0, 8, 0, 20, 1, 80, 86, 101, 114, 105, 102, 121, 83, 116, 97, 116, 101,
                73, 110, 99, 108, 117, 115, 105, 111, 110, 9, 0, 14, 0, 8, 0, 20, 1, 68, 86, 101,
                114, 105, 102, 121, 84, 120, 73, 110, 99, 108, 117, 115, 105, 111, 110, 9, 0, 14,
                0, 8, 0
            ]
        );
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_latest_finalized_header() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PrecompileArgs::GetLatestFinalizedHeader([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetLatestFinalizedHeader".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_finalized_height() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PrecompileArgs::GetFinalizedHeight([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetFinalizedHeight".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_rational_height() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PrecompileArgs::GetRationalHeight([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetRationalHeight".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_fast_height() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PrecompileArgs::GetFastHeight([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetFastHeight".to_vec()), vec![1u8; 4])
        )
    }

    use hex_literal::hex;

    #[test]
    fn portal_precompile_selects_enum_for_verify_event_inclusion() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let encoded_verify_event_inclusion_select = PrecompileArgs::VerifyEventInclusion(
            [1u8; 4],
            SpeedMode::Fast,
            hex!("0202020202020202020202020202020202020202020202020202020202020202"),
            hex!("04040404040404040404040404040404040404040404040404040404040404040505050505")
                .to_vec(),
        )
        .encode();

        assert_eq!(
            encoded_verify_event_inclusion_select,
            vec![
                4, 1, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5
            ]
        );

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            encoded_verify_event_inclusion_select,
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Quadruple(
                Some(b"VerifyEventInclusion".to_vec()),
                (
                    Box::new(FilledAbi::Bytes4(None, hex!("01010101").to_vec())),
                    Box::new(FilledAbi::Byte(None, hex!("00").to_vec())),
                    Box::new(FilledAbi::H256(None, hex!("0202020202020202020202020202020202020202020202020202020202020202").into())),
                    Box::new(FilledAbi::Bytes(None, hex!("04040404040404040404040404040404040404040404040404040404040404040505050505").to_vec()))
                ),
            )
        )
    }

    #[test]
    fn test_get_latest_finalized_header_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = PrecompileArgs::GetLatestFinalizedHeader(chain_id);
        let encoded_portal_call = portal_call.encode();
        println!("Call: {encoded_portal_call:?}");
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::GetLatestFinalizedHeader(chain_id)
        );
    }

    #[test]
    fn test_get_finalized_height_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = PrecompileArgs::GetFinalizedHeight(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::GetFinalizedHeight(chain_id)
        );
    }

    #[test]
    fn test_get_rational_height_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = PrecompileArgs::GetRationalHeight(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::GetRationalHeight(chain_id)
        );
    }

    #[test]
    fn test_get_fast_height_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let portal_call = PrecompileArgs::GetFastHeight(chain_id);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(recoded_portal_call, PrecompileArgs::GetFastHeight(chain_id));
    }

    #[test]
    fn test_verify_event_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let source: [u8; 32] = [5; 32];
        let portal_call = PrecompileArgs::VerifyEventInclusion(
            chain_id,
            SpeedMode::Finalized,
            source,
            event.clone(),
        );
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyEventInclusion(chain_id, SpeedMode::Finalized, source, event)
        );
    }

    #[test]
    fn test_verify_state_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let _source: [u8; 32] = [5; 32];

        let portal_call =
            PrecompileArgs::VerifyStateInclusion(chain_id, SpeedMode::Rational, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyStateInclusion(chain_id, SpeedMode::Rational, event)
        );
    }

    #[test]
    fn test_verify_tx_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let _source: [u8; 32] = [5; 32];

        let portal_call =
            PrecompileArgs::VerifyTxInclusion(chain_id, SpeedMode::Rational, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyTxInclusion(chain_id, SpeedMode::Rational, event)
        );
    }
}
