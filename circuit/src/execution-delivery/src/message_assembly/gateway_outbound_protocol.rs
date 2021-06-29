#![cfg_attr(not(feature = "std"), no_std)]

use t3rn_primitives::GatewayPointer;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_std::vec;
use sp_std::vec::*;

use crate::message_assembly::circuit_inbound::Proof;
use t3rn_primitives::abi::{Bytes, GatewayABIConfig, Type};

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub struct InboundEvent {}

pub static mut GATEWAY_INBOUND_EVENTS: Vec<InboundEvent> = vec![];

pub trait AsGatewayOutboundEvent {
    fn parse_data_to_gateway_outbound_event(
        &self,
        gateway_pointer: GatewayPointer,
        gateway_genesis: GatewayABIConfig,
        id: GatewayOutboundEventId,
        proof: Option<Proof>,
        args_abi: Vec<Type>,
    ) -> Result<GatewayOutboundEvent, &'static str>;
}

pub type GatewayOutboundEventId = u64;

/// GatewayOutboundEvent will be reconstructed from raw data received from foreign gateways.
/// After the specific logs / events were translated to the below form it can be used by:
/// - events_parser to check whether the received event format matches the expected format
///     submitted while registering gateway for that foreign consensus system
/// - versatile-vm to go over args_abi + args_encoded and validate execution's validity
/// - pallet-multi-finality-verifier to utilise the proof and check inclusion into foreign chain.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayOutboundEvent {
    /// Id to find that event easier
    pub id: GatewayOutboundEventId,

    /// Signature -> Ethereum-like event description (first topic), utf-8-encoded bytes
    /// e.g. "Transfer(address,address,value)"
    pub signature: Option<Vec<u8>>,

    /// module -> namespace or address for eth
    pub namespace: Bytes,

    /// variant -> name aka event name
    pub name: Bytes,

    /// That's raw data attached to event - not the whole incoming blob
    /// which acn be found under proof.value
    pub data: Bytes,

    /// Inclusion proof to be checked against the block it's pointing to
    pub proof: Option<Proof>,

    /// Eth topics can be derived from those by (sha3(arg.0,arg.1....arg.n))
    /// Next topics depending on "indexing" are either included as next topic or not
    pub args_abi: Vec<Type>,

    /// Values here were already casted to bytes of appropriate for Scale encoding length
    /// Now can be re-used for comparison in post-run
    pub args_encoded: Vec<Bytes>,

    /// Gateway Pointer contains info about originating message foreign consensus system
    pub gateway_pointer: GatewayPointer,
}

impl GatewayOutboundEvent {
    pub fn new(
        id: GatewayOutboundEventId,
        name: Bytes,
        namespace: Bytes,
        data: Bytes,
        proof: Option<Proof>,
        signature: Option<Vec<u8>>,
        args_abi: Vec<Type>,
        args_encoded: Vec<Bytes>,
        gateway_pointer: GatewayPointer,
    ) -> Self {
        GatewayOutboundEvent {
            id,
            namespace,
            name,
            data,
            proof,
            signature,
            args_abi,
            args_encoded,
            gateway_pointer,
        }
    }
}

#[cfg(test)]
mod tests {}
