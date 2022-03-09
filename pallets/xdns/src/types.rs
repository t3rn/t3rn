// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_system::Config;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

use sp_runtime::traits::Hash;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

use sp_std::prelude::*;
use sp_std::vec::Vec;
use t3rn_primitives::abi::GatewayABIConfig;
pub use t3rn_primitives::side_effect::{EventSignature, SideEffectId, SideEffectName};
use t3rn_primitives::{
    abi::Type, ChainId, GatewayGenesisConfig, GatewaySysProps, GatewayType, GatewayVendor,
};
/// A hash based on encoding the complete XdnsRecord
pub type XdnsRecordId<T> = <T as frame_system::Config>::Hash;

/// A hash based on encoding the Gateway ID
pub type XdnsGatewayId<T> = <T as frame_system::Config>::Hash;

pub type AllowedSideEffect = [u8; 4];

/// A preliminary representation of a xdns_record in the onchain registry.
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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

    /// Gateway System Properties
    pub gateway_sys_props: GatewaySysProps,

    pub registrant: Option<AccountId>,

    pub last_finalized: Option<u64>,

    /// Methods enabled to be called on the remote target
    pub allowed_side_effects: Vec<AllowedSideEffect>,
}

// ToDo: If I import this from primitives, I get this error. Maybe someone has an idea? Don't understand the conflicting trait impl error
// error[E0119]: conflicting implementations of trait `t3rn_protocol::side_effects::protocol::SideEffectProtocol` for type `t3rn_primitives::side_effect::SideEffectInterface`
//    --> pallets/xdns/src/lib.rs:400:1
//     |
// 400 | impl SideEffectProtocol for SideEffectInterface {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     |
//     = note: conflicting implementation in crate `t3rn_protocol`:
//             - impl t3rn_protocol::side_effects::protocol::SideEffectProtocol for t3rn_primitives::side_effect::SideEffectInterface;
//
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
//    --> pallets/xdns/src/lib.rs:400:1
//     |
// 400 | impl SideEffectProtocol for SideEffectInterface {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------
//     | |                           |
//     | |                           `t3rn_primitives::side_effect::SideEffectInterface` is not defined in the current crate
//     | impl doesn't use only types from inside the current crate
//     |
//     = note: define and implement a trait or new type instead
//
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
//    --> pallets/xdns/src/lib.rs:444:1
//     |
// 444 | impl SideEffectConfirmationProtocol for SideEffectInterface {}
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------
//     | |                                       |
//     | |                                       `t3rn_primitives::side_effect::SideEffectInterface` is not defined in the current crate
//     | impl doesn't use only types from inside the current crate
//     |
//     = note: define and implement a trait or new type instead

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SideEffectInterface {
    pub id: [u8; 4],
    pub name: SideEffectName,
    pub argument_abi: Vec<Type>,
    pub argument_to_state_mapper: Vec<EventSignature>,
    pub confirm_events: Vec<EventSignature>,
    pub escrowed_events: Vec<EventSignature>,
    pub commit_events: Vec<EventSignature>,
    pub revert_events: Vec<EventSignature>,
}

impl SideEffectInterface {
    /// Function that generates an XdnsRecordId hash based on the gateway id
    pub fn generate_id<T: Config>(&self) -> SideEffectId<T> {
        T::Hashing::hash(Encode::encode(&self.id).as_ref())
    }
}

impl<AccountId: Encode> XdnsRecord<AccountId> {
    pub fn new_from_primitives(
        url: Vec<u8>,
        gateway_abi: GatewayABIConfig,
        modules_encoded: Option<Vec<u8>>,
        extrinsics_version: u8,
        genesis_hash: Vec<u8>,
        gateway_id: ChainId,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        gateway_sys_props: GatewaySysProps,
        registrant: Option<AccountId>,
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
            gateway_sys_props,
            registrant,
            last_finalized,
            allowed_side_effects,
        }
    }

    pub fn new(
        url: Vec<u8>,
        gateway_id: ChainId,
        gateway_abi: GatewayABIConfig,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        gateway_genesis: GatewayGenesisConfig,
        gateway_sys_props: GatewaySysProps,
        allowed_side_effects: Vec<AllowedSideEffect>,
    ) -> Self {
        XdnsRecord {
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
            gateway_sys_props,
            registrant: None,
            last_finalized: None,
            allowed_side_effects,
        }
    }

    pub fn assign_registrant(&mut self, registrant: AccountId) {
        self.registrant = Some(registrant)
    }

    /// Function that generates an XdnsRecordId hash based on the gateway id
    pub fn generate_id<T: Config>(&self) -> XdnsRecordId<T> {
        T::Hashing::hash(&self.gateway_id)
    }

    pub fn set_last_finalized(&mut self, last_finalized: u64) {
        self.last_finalized = Some(last_finalized)
    }
}

/// The object with XdnsRecords as returned by the RPC endpoint
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FetchXdnsRecordsResponse<AccountId> {
    pub xdns_records: Vec<XdnsRecord<AccountId>>,
}
