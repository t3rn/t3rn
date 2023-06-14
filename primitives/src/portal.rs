pub use crate::light_client::{HeaderResult, HeightResult, InclusionReceipt};
use crate::{
    gateway::GatewayABIConfig, ChainId, ExecutionVendor, GatewayGenesisConfig, GatewayType,
    GatewayVendor, SpeedMode, TokenInfo,
};
use codec::{Decode, Encode};

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
        message: Bytes,
        speed_mode: SpeedMode,
        source: Option<Bytes>,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_precompile(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        source: Option<Bytes>,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_precompile(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_tx_inclusion_precompile(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        speed_mode: SpeedMode,
        source: Option<Bytes>,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn initialize(
        origin: T::Origin,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError>;

    fn submit_encoded_headers(
        gateway_id: ChainId,
        encoded_header_data: Vec<u8>,
    ) -> Result<(), DispatchError>;

    fn turn_on(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;

    fn turn_off(origin: T::Origin, gateway_id: [u8; 4]) -> Result<bool, DispatchError>;
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

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub enum PrecompileArgs {
    GetLatestFinalizedHeader(ChainId),
    GetFinalizedHeight(ChainId),
    GetRationalHeight(ChainId),
    GetFastHeight(ChainId),
    VerifyEventInclusion([u8; 4], Bytes, SpeedMode, Bytes),
    VerifyStateInclusion([u8; 4], Bytes, SpeedMode),
    VerifyTxInclusion([u8; 4], Bytes, SpeedMode),
}

impl PrecompileArgs {
    pub fn descriptor() -> Vec<u8> {
        b"PrecompileArgs:Enum(\
                GetLatestFinalizedHeader:Bytes4,\
                GetFinalizedHeight:Bytes4,\
                GetRationalHeight:Bytes4,\
                GetFastHeight:Bytes4,\
                VerifyEventInclusion:Triple(Bytes4,Bytes,Byte,Bytes),\
                VerifyStateInclusion:Tuple(Bytes4,Byte,Bytes),\
                VerifyTxInclusion:Tuple(Bytes4,Bytes,Byte),\
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
        assert_eq!(
            portal_interface_abi,
            Abi::Enum(
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
                            Box::new(Abi::Bytes(None)),
                            Box::new(Abi::Byte(None)),
                            Box::new(Abi::Bytes(None))
                        ),
                    )),
                    Box::new(Abi::Triple(
                        Some(b"VerifyStateInclusion".to_vec()),
                        (
                            Box::new(Abi::Bytes4(None)),
                            Box::new(Abi::Bytes(None)),
                            Box::new(Abi::Byte(None))
                        ),
                    )),
                    Box::new(Abi::Triple(
                        Some(b"VerifyTxInclusion".to_vec()),
                        (
                            Box::new(Abi::Bytes4(None)),
                            Box::new(Abi::Bytes(None)),
                            Box::new(Abi::Byte(None))
                        ),
                    )),
                ]
            )
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

    #[test]
    fn portal_precompile_selects_enum_for_verify_event_inclusion() {
        let portal_precompile_interface = PrecompileArgs::interface_abi().unwrap();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PrecompileArgs::VerifyEventInclusion([1u8; 4], vec![4u8; 32], vec![5u8]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Triple(
                Some(b"VerifyEventInclusion".to_vec()),
                (
                    Box::new(FilledAbi::Bytes4(None, vec![1u8; 4])),
                    Box::new(FilledAbi::Bytes(None, vec![4u8; 32].encode())),
                    Box::new(FilledAbi::Bytes(None, vec![5u8].encode()))
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
        let portal_call = PrecompileArgs::VerifyEventInclusion(chain_id, event.clone(), vec![4u8]);
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyEventInclusion(chain_id, event, vec![4u8])
        );
    }

    #[test]
    fn test_verify_state_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let portal_call = PrecompileArgs::VerifyStateInclusion(chain_id, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyStateInclusion(chain_id, event)
        );
    }

    #[test]
    fn test_verify_tx_inclusion_recodes_correctly_to_scale() {
        let chain_id: [u8; 4] = [9, 9, 9, 9];
        let event = vec![1, 2, 3, 4];
        let portal_call = PrecompileArgs::VerifyTxInclusion(chain_id, event.clone());
        let encoded_portal_call = portal_call.encode();
        let recoded_portal_call =
            PrecompileArgs::recode_to_scale_and_decode(&t3rn_abi::Codec::Rlp, &encoded_portal_call)
                .unwrap();

        assert_eq!(
            recoded_portal_call,
            PrecompileArgs::VerifyTxInclusion(chain_id, event)
        );
    }
}
