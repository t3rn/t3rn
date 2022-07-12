use crate::{
    abi::{GatewayABIConfig, Type},
    protocol::SideEffectProtocol,
    ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};
use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
use frame_system::pallet_prelude::OriginFor;
use scale_info::TypeInfo;
use sp_std::{boxed::Box, collections::btree_map::BTreeMap, vec::Vec};

pub type AllowedSideEffect = [u8; 4];

/// A hash based on encoding the complete XdnsRecord
pub type XdnsRecordId = [u8; 4];

/// A hash based on encoding the Gateway ID
pub type XdnsGatewayId<T> = <T as frame_system::Config>::Hash;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Parachain {
    // gateway_id of relaychain
    pub relay_chain_id: ChainId,
    // parachain_id
    pub id: u32,
}

/// The object with XdnsRecords as returned by the RPC endpoint
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct FetchXdnsRecordsResponse<AccountId> {
    pub xdns_records: Vec<XdnsRecord<AccountId>>,
}

/// A preliminary representation of a xdns_record in the onchain registry.
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct XdnsRecord<AccountId> {
    /// SCALE-encoded url string on where given Consensus System can be accessed
    pub url: Vec<u8>,

    pub gateway_abi: GatewayABIConfig,

    pub gateway_genesis: GatewayGenesisConfig,

    /// Gateway Vendor
    pub gateway_vendor: GatewayVendor,

    /// Gateway Type
    pub gateway_type: GatewayType,

    /// Gateway Id
    pub gateway_id: ChainId,

    pub parachain: Option<Parachain>,

    /// Gateway System Properties
    pub gateway_sys_props: GatewaySysProps,

    pub registrant: Option<AccountId>,

    /// Leave empty if there's no escrow capabilities on the remote gateway
    pub security_coordinates: Vec<u8>,

    pub last_finalized: Option<u64>,

    /// Methods enabled to be called on the remote target
    pub allowed_side_effects: Vec<AllowedSideEffect>,
}

impl<AccountId: Encode> XdnsRecord<AccountId> {
    pub fn new_from_primitives(
        url: Vec<u8>,
        gateway_abi: GatewayABIConfig,
        modules_encoded: Option<Vec<u8>>,
        extrinsics_version: u8,
        genesis_hash: Vec<u8>,
        gateway_id: ChainId,
        parachain: Option<Parachain>,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        gateway_sys_props: GatewaySysProps,
        registrant: Option<AccountId>,
        security_coordinates: Vec<u8>,
        last_finalized: Option<u64>,
        allowed_side_effects: Vec<AllowedSideEffect>,
    ) -> Self {
        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded,
            extrinsics_version,
            genesis_hash,
        };

        XdnsRecord {
            url,
            gateway_abi,
            gateway_genesis,
            gateway_vendor,
            gateway_type,
            gateway_id,
            parachain,
            gateway_sys_props,
            registrant,
            security_coordinates,
            last_finalized,
            allowed_side_effects,
        }
    }

    pub fn new(
        url: Vec<u8>,
        gateway_id: ChainId,
        parachain: Option<Parachain>,
        gateway_abi: GatewayABIConfig,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        gateway_genesis: GatewayGenesisConfig,
        gateway_sys_props: GatewaySysProps,
        security_coordinates: Vec<u8>,
        allowed_side_effects: Vec<AllowedSideEffect>,
    ) -> Self {
        XdnsRecord {
            url,
            gateway_id,
            parachain,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            registrant: None,
            security_coordinates,
            last_finalized: None,
            allowed_side_effects,
        }
    }

    pub fn assign_registrant(&mut self, registrant: AccountId) {
        self.registrant = Some(registrant)
    }

    /// Function that generates an XdnsRecordId hash based on the gateway id
    pub fn generate_id<T: frame_system::Config>(&self) -> XdnsRecordId {
        self.gateway_id
    }

    pub fn set_last_finalized(&mut self, last_finalized: u64) {
        self.last_finalized = Some(last_finalized)
    }
}

pub trait Xdns<T: frame_system::Config> {
    /// Locates the best available gateway based on the time they were last finalized.
    /// Priority goes Internal > External > TxOnly, followed by the largest last_finalized value
    fn best_available(gateway_id: ChainId) -> Result<XdnsRecord<T::AccountId>, &'static str>;

    /// Fetches all known XDNS records
    fn fetch_records() -> Vec<XdnsRecord<T::AccountId>>;

    fn add_new_xdns_record(
        origin: OriginFor<T>,
        url: Vec<u8>,
        gateway_id: ChainId,
        parachain: Option<Parachain>,
        gateway_abi: GatewayABIConfig,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        gateway_genesis: GatewayGenesisConfig,
        gateway_sys_props: GatewaySysProps,
        security_coordinates: Vec<u8>,
        allowed_side_effects: Vec<AllowedSideEffect>,
        force: bool,
    ) -> DispatchResult;

    fn allowed_side_effects(gateway_id: &ChainId)
        -> BTreeMap<[u8; 4], Box<dyn SideEffectProtocol>>;

    fn fetch_side_effect_interface(
        id: [u8; 4],
    ) -> Result<Box<dyn SideEffectProtocol>, &'static str>;

    fn update_gateway_ttl(gateway_id: ChainId, last_finalized: u64) -> DispatchResultWithPostInfo;

    fn get_abi(chain_id: ChainId) -> Result<GatewayABIConfig, &'static str>;

    fn get_gateway_value_unsigned_type_unsafe(chain_id: &ChainId) -> Type;

    fn get_gateway_type_unsafe(chain_id: &ChainId) -> GatewayType;

    fn get_gateway_security_coordinates(chain_id: &ChainId) -> Result<Vec<u8>, &'static str>;
}
