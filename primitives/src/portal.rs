pub use crate::light_client::{HeaderResult, HeightResult, InclusionReceipt};
use crate::{
    gateway::GatewayABIConfig, ChainId, ExecutionVendor, GatewayGenesisConfig, GatewayType,
    GatewayVendor, SpeedMode, TokenInfo,
};
use codec::{Decode, Encode};

use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::{convert::TryFrom, vec::Vec};
use t3rn_abi::{recode::Codec, types::Bytes, Abi};
pub use t3rn_light_client_commons::traits::{HeaderResult, HeightResult};
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

    fn get_latest_finalized_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_latest_updated_height(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn get_current_epoch(
        gateway_id: ChainId,
    ) -> Result<HeightResult<T::BlockNumber>, DispatchError>;

    fn read_fast_confirmation_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn read_rational_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError>;

    fn read_finalized_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError>;

    fn read_epoch_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError>;

    fn header_speed_mode_satisfied(
        gateway_id: [u8; 4],
        header: Bytes,
        speed_mode: SpeedMode,
    ) -> Result<bool, DispatchError>;

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<InclusionReceipt<T::BlockNumber>, DispatchError>;

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
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
            PortalExecution::BlockNumber(x) => x.encode(),
            PortalExecution::Data(x) => x.encode(),
            PortalExecution::Switched(x) => x.encode(),
            PortalExecution::Noop => sp_std::vec![],
        }
    }
}

#[derive(Clone, Eq, Decode, Encode, PartialEq, Debug, TypeInfo)]
pub enum PortalPrecompileInterfaceEnum {
    GetLatestFinalizedHeader(ChainId),
    GetLatestFinalizedHeight(ChainId),
    GetLatestUpdatedHeight(ChainId),
    GetCurrentEpoch(ChainId),
    ReadFastConfirmationOffset(ChainId),
    ReadRationalConfirmationOffset(ChainId),
    ReadEpochOffset(ChainId),
    VerifyEventInclusion([u8; 4], Bytes),
    VerifyStateInclusion([u8; 4], Bytes),
    VerifyTxInclusion([u8; 4], Bytes),
}

pub fn get_portal_interface_abi() -> Abi {
    Abi::try_from(PORTAL_INTERFACE_ABI_DESCRIPTOR.to_vec())
        .expect("Expect parsing PORTAL_INTERFACE_ABI_DESCRIPTOR to succeed.")
}

pub const PORTAL_INTERFACE_ABI_DESCRIPTOR: &[u8] = b"PortalPrecompileInterface:Enum(\
        GetLatestFinalizedHeader:Bytes4,\
        GetLatestFinalizedHeight:Bytes4,\
        GetLatestUpdatedHeight:Bytes4,\
        GetCurrentEpoch:Bytes4,\
        ReadFastConfirmationOffset:Bytes4,\
        ReadRationalConfirmationOffset:Bytes4,\
        ReadEpochOffset:Bytes4,\
        VerifyEventInclusion:Tuple(Bytes4,Bytes),\
        VerifyStateInclusion:Tuple(Bytes4,Bytes),\
        VerifyTxInclusion:Tuple(Bytes4,Bytes),\
    )";

#[cfg(test)]
pub mod portal_precompile_decode_test {
    use super::*;
    use t3rn_abi::{Abi, Codec, FilledAbi};

    #[test]
    fn portal_interface_abi_descriptor_parses() {
        let portal_interface_abi = get_portal_interface_abi();
        assert_eq!(
            portal_interface_abi,
            Abi::Enum(
                Some(b"PortalPrecompileInterface".to_vec()),
                vec![
                    Box::new(Abi::Bytes4(Some(b"GetLatestFinalizedHeader".to_vec()))),
                    Box::new(Abi::Bytes4(Some(b"GetLatestFinalizedHeight".to_vec()))),
                    Box::new(Abi::Bytes4(Some(b"GetLatestUpdatedHeight".to_vec()))),
                    Box::new(Abi::Bytes4(Some(b"GetCurrentEpoch".to_vec()))),
                    Box::new(Abi::Bytes4(Some(b"ReadFastConfirmationOffset".to_vec()))),
                    Box::new(Abi::Bytes4(Some(
                        b"ReadRationalConfirmationOffset".to_vec()
                    ))),
                    Box::new(Abi::Bytes4(Some(b"ReadEpochOffset".to_vec()))),
                    Box::new(Abi::Tuple(
                        Some(b"VerifyEventInclusion".to_vec()),
                        (Box::new(Abi::Bytes4(None)), Box::new(Abi::Bytes(None))),
                    )),
                    Box::new(Abi::Tuple(
                        Some(b"VerifyStateInclusion".to_vec()),
                        (Box::new(Abi::Bytes4(None)), Box::new(Abi::Bytes(None))),
                    )),
                    Box::new(Abi::Tuple(
                        Some(b"VerifyTxInclusion".to_vec()),
                        (Box::new(Abi::Bytes4(None)), Box::new(Abi::Bytes(None))),
                    )),
                ]
            )
        );
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_latest_finalized_header() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::GetLatestFinalizedHeader([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetLatestFinalizedHeader".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_latest_finalized_height() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::GetLatestFinalizedHeight([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetLatestFinalizedHeight".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_latest_updated_height() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::GetLatestUpdatedHeight([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetLatestUpdatedHeight".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_get_current_epoch() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::GetCurrentEpoch([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"GetCurrentEpoch".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_read_fast_confirmation_offset() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::ReadFastConfirmationOffset([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"ReadFastConfirmationOffset".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_read_rational_confirmation_offset() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::ReadRationalConfirmationOffset([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(
                Some(b"ReadRationalConfirmationOffset".to_vec()),
                vec![1u8; 4]
            )
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_read_epoch_offset() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::ReadEpochOffset([1u8; 4]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Bytes4(Some(b"ReadEpochOffset".to_vec()), vec![1u8; 4])
        )
    }

    #[test]
    fn portal_precompile_selects_enum_for_verify_event_inclusion() {
        let portal_precompile_interface = get_portal_interface_abi();

        let filled_abi = FilledAbi::try_fill_abi(
            portal_precompile_interface,
            PortalPrecompileInterfaceEnum::VerifyEventInclusion([1u8; 4], vec![4u8; 32]).encode(),
            Codec::Scale,
        )
        .unwrap();

        assert_eq!(
            filled_abi,
            FilledAbi::Tuple(
                Some(b"VerifyEventInclusion".to_vec()),
                (
                    Box::new(FilledAbi::Bytes4(None, vec![1u8; 4])),
                    Box::new(FilledAbi::Bytes(None, vec![4u8; 32].encode())),
                ),
            )
        )
    }
}
