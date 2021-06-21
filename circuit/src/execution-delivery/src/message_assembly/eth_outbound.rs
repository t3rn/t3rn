#![cfg_attr(not(feature = "std"), no_std)]

use t3rn_primitives::GatewayPointer;

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::{H160, H256, U256};
use sp_std::vec::*;

use ethabi_decode::{encode as eth_abi_encode, Event as EthAbiEvent};

use sp_runtime::RuntimeString;

use crate::message_assembly::abi::GatewayGenesis;
use crate::message_assembly::abi::{create_signature, Bytes, Type};
use crate::message_assembly::circuit_inbound::Proof;
use ethabi_decode::{Param, ParamKind};

use crate::message_assembly::gateway_outbound_protocol::{
    AsGatewayOutboundEvent, GatewayOutboundEvent, GatewayOutboundEventId,
};

#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;
#[cfg(feature = "std")]
use std::fmt::Debug;

// That's the EthLogEntry localised to the block that comes via RPC
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct EthLogEntry {
    pub log: EthLog,
    /// Block Hash
    pub block_hash: Option<H256>,
    /// Block Number
    pub block_number: Option<U256>,
    /// Transaction Hash
    pub transaction_hash: Option<H256>,
    /// Transaction Index
    pub transaction_index: Option<U256>,
    /// Log Index in Block
    pub log_index: Option<U256>,
    /// Log Index in Transaction
    pub transaction_log_index: Option<U256>,
    /// Log Type
    pub log_type: RuntimeString,
    /// Whether Log Type is Removed (Geth Compatibility Field)
    pub removed: bool,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize)]
/// Log on eth being a subject to RLP and blockchain state
/// More at https://github.com/ethereum/go-ethereum/blob/a2ea537a6fb4c69543ed8045337516eb61c7afe8/core/types/log.go#L65
pub struct EthLog {
    /// Address.
    pub address: H160,
    /// Topics.
    pub topics: Vec<H256>,
    /// Data.
    pub data: Bytes,
}

impl AsGatewayOutboundEvent for EthLog {
    fn parse_data_to_gateway_outbound_event(
        &self,
        gateway_pointer: GatewayPointer,
        _gateway_genesis: GatewayGenesis,
        id: GatewayOutboundEventId,
        proof: Option<Proof>,
        args_abi: Vec<Type>,
    ) -> Result<GatewayOutboundEvent, &'static str> {
        // translate address into namespace
        let namespace = RuntimeString::Owned(self.address.to_string());

        // decode first topic's first argument to discover event name
        let name = RuntimeString::Owned(self.topics[0].to_string());

        let expected_arg_types_eth = from_eth_abi(args_abi.clone())?;

        let event = EthAbiEvent {
            signature: &self.topics[0].to_string(),
            inputs: expected_arg_types_eth.as_slice(),
            anonymous: false,
        };

        let args_decoded = event
            .decode(self.topics.clone(), self.data.to_vec())
            .map_err(|_| "Error decoding native eth event using ethabi-decoder")?;

        let args_encoded = args_decoded
            .iter()
            .map(|_a| Bytes::from(eth_abi_encode(&args_decoded)))
            .collect::<Vec<Bytes>>();

        Ok(GatewayOutboundEvent {
            id,
            signature: Some(create_signature(name.encode(), args_abi.clone())?),
            namespace,
            name,
            data: self.data.clone(),
            proof,
            args_abi,
            args_encoded,
            gateway_pointer,
        })
    }
}

pub fn from_eth_abi(from_gateway_abi_type: Vec<Type>) -> Result<Vec<Param>, &'static str> {
    let r = from_gateway_abi_type
        .iter()
        .map(|t: &Type| match t {
            Type::Enum(_) => unimplemented!(),
            Type::Bool => Param {
                kind: ParamKind::Bool,
                indexed: false,
            },
            Type::Contract | Type::Address(_) => Param {
                kind: ParamKind::Address,
                indexed: false,
            },
            Type::Bytes(n) => Param {
                kind: ParamKind::FixedBytes(*n as usize),
                indexed: false,
            },
            Type::Uint(n) => Param {
                kind: ParamKind::Uint(*n as usize),
                indexed: false,
            },
            Type::Int(n) => Param {
                kind: ParamKind::Int(*n as usize),
                indexed: false,
            },
            Type::Struct(_n) => unimplemented!(),
            Type::String => Param {
                kind: ParamKind::String,
                indexed: false,
            },
            Type::DynamicBytes => Param {
                kind: ParamKind::Bytes,
                indexed: false,
            },
            _ => unimplemented!(),
        })
        .collect::<Vec<Param>>();

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use sp_core::Bytes;

    use std::str::FromStr;

    #[test]
    fn eth_log_deserialization_works_for_json() {
        let s = r#"{
			"address" : "0xede84640d1a1d3e06902048e67aa7db8d52c2ce1",
			"data" : "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
			"topics" : [
				"0x0000000000000000000000000000000000000000000000000000000000000000"
			]
		}"#;
        let deserialized_eth_log: EthLog = serde_json::from_str(s).unwrap();

        println!("_deserialized: EthLog {:?}", deserialized_eth_log);

        assert_eq!(
            EthLog {
                address: H160::from_str("ede84640d1a1d3e06902048e67aa7db8d52c2ce1").unwrap(),
                topics: vec![
                    H256::from_str(
                        "0000000000000000000000000000000000000000000000000000000000000000"
                    )
                    .unwrap()
                ],
                data: Bytes(vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
                ])
            },
            deserialized_eth_log
        );
    }

    #[test]
    fn eth_log_translates_to_gateway_outbound_event() {
        let s = r#"{
			"address" : "0xede84640d1a1d3e06902048e67aa7db8d52c2ce1",
			"data" : "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
			"topics" : [
				"0x0000000000000000000000000000000000000000000000000000000000000000"
			]
		}"#;
        let deserialized_eth_log: EthLog = serde_json::from_str(s).unwrap();

        println!("_deserialized: EthLog {:?}", deserialized_eth_log);

        assert_eq!(
            EthLog {
                address: H160::from_str("ede84640d1a1d3e06902048e67aa7db8d52c2ce1").unwrap(),
                topics: vec![
                    H256::from_str(
                        "0000000000000000000000000000000000000000000000000000000000000000"
                    )
                    .unwrap()
                ],
                data: Bytes(vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
                ])
            },
            deserialized_eth_log
        );
    }
}
