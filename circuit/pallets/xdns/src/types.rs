// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_system::Config;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::prelude::*;
use sp_std::vec::Vec;
use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::{ChainId, GatewayGenesisConfig, GatewayType, GatewayVendor};

/// A hash based on encoding the complete XdnsRecord
pub type XdnsRecordId<T> = <T as frame_system::Config>::Hash;

/// A hash based on encoding the Gateway ID
pub type XdnsGatewayId<T> = <T as frame_system::Config>::Hash;

pub type AllowedSideEffect = Vec<u8>;

/// A preliminary representation of a xdns_record in the onchain registry.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
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

    pub registrant: Option<AccountId>,

    pub last_finalized: Option<u64>,

    /// Methods enabled to be called on the remote target
    pub allowed_side_effects: Vec<AllowedSideEffect>,
}

impl<AccountId: Encode> XdnsRecord<AccountId> {
    pub fn new_from_primitives(
        url: Vec<u8>,
        gateway_abi: GatewayABIConfig,
        modules_encoded: Option<Vec<u8>>,
        signed_extension: Option<Vec<u8>>,
        runtime_version: sp_version::RuntimeVersion,
        extrinsics_version: u8,
        genesis_hash: Vec<u8>,
        gateway_id: ChainId,
        gateway_vendor: GatewayVendor,
        gateway_type: GatewayType,
        registrant: Option<AccountId>,
        last_finalized: Option<u64>,
        allowed_side_effects: Vec<AllowedSideEffect>,
    ) -> Self {
        let gateway_genesis = GatewayGenesisConfig {
            modules_encoded,
            signed_extension,
            runtime_version,
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
        allowed_side_effects: Vec<AllowedSideEffect>,
    ) -> Self {
        XdnsRecord {
            url,
            gateway_id,
            gateway_abi,
            gateway_vendor,
            gateway_type,
            gateway_genesis,
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
        T::Hashing::hash(Encode::encode(&self.gateway_id).as_ref())
    }

    pub fn set_last_finalized(&mut self, last_finalized: u64) {
        self.last_finalized = Some(last_finalized)
    }
}

/// The object with XdnsRecords as returned by the RPC endpoint
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FetchXdnsRecordsResponse<AccountId> {
    pub xdns_records: Vec<XdnsRecord<AccountId>>,
}
