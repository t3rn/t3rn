#![cfg_attr(not(feature = "std"), no_std)]

use t3rn_primitives::GatewayPointer;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_runtime::RuntimeString;

use sp_std::vec;
use sp_std::vec::*;

use crate::circuit_inbound::Proof;
use t3rn_primitives::abi::GatewayABIConfig;
use t3rn_primitives::abi::Type;

use crate::gateway_outbound_protocol::{
    AsGatewayOutboundEvent, GatewayOutboundEvent, GatewayOutboundEventId,
};

use sp_core::Bytes;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub type SubstrateEventEntry<E, T> = frame_system::EventRecord<E, T>;

// whereas on Substrate there is an  event:
/// But placing it in a context of other events is missing
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
struct SubstrateRawEvent {
    /// The name of the module from whence the Event originated
    pub module: RuntimeString,
    /// The name of the Event
    pub variant: RuntimeString,
    /// The raw Event data
    pub data: Bytes,
}

// Returns new offset
pub fn seek_n(entries: Vec<u8>, n: usize, offset: usize) -> Result<usize, &'static str> {
    let new_position = offset + n;
    if new_position < entries.len() {
        Ok(new_position)
    } else {
        Err("Error::InvalidData")
    }
}

impl AsGatewayOutboundEvent for SubstrateRawEvent {
    fn parse_data_to_gateway_outbound_event(
        &self,
        gateway_pointer: GatewayPointer,
        gateway_genesis: GatewayABIConfig,
        id: GatewayOutboundEventId,
        proof: Option<Proof>,
        args_abi: Vec<Type>,
    ) -> Result<GatewayOutboundEvent, &'static str> {
        let _full_data_length = self.data.len();
        let data_as_vec = self.data.to_vec();
        let mut current_offset: usize = 0;

        let args_encoded = args_abi
            .iter()
            .map(|t| {
                let type_size = t.size_of(&gateway_genesis)?;
                let mut argn_bytes = vec![0; type_size];
                let new_offset = seek_n(
                    data_as_vec.clone(),
                    type_size.clone(),
                    current_offset.clone(),
                )?;
                argn_bytes
                    .copy_from_slice(&data_as_vec[current_offset.clone()..new_offset.clone()]);
                current_offset = new_offset;
                // To make sure we read argument right decode the type
                t.eval(argn_bytes.to_vec())?;
                Ok(Bytes::from(argn_bytes))
            })
            .collect::<Result<Vec<Bytes>, &'static str>>()?;

        Ok(GatewayOutboundEvent {
            id,
            /// translate address into namespace
            signature: None,
            namespace: self.module.clone().encode(),
            /// translate variant into name
            name: self.variant.clone().encode(),
            data: self.data.clone(),
            proof,
            args_abi,
            args_encoded,
            gateway_pointer,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use sp_core::Bytes;
    use sp_runtime::create_runtime_str;

    #[test]
    fn substrate_event_deserialization_works_for_json() {
        let s = r#"{
			"module" : "Balances",
			"variant" : "Transfer",
			"data" : "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
		}"#;

        let deserialized_substrate_event: SubstrateRawEvent = serde_json::from_str(s).unwrap();

        println!(
            "_deserialized: SubstrateRawEvent {:?}",
            deserialized_substrate_event
        );

        assert_eq!(
            SubstrateRawEvent {
                module: create_runtime_str!("Balances"),
                variant: create_runtime_str!("Transfer"),
                data: Bytes(vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
                ])
            },
            deserialized_substrate_event
        );
    }
}
