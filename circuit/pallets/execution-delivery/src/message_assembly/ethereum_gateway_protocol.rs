#![cfg_attr(not(feature = "std"), no_std)]

use codec::Compact;
use ethabi_decode::{self as ethabi, Address, Token};
use sp_core::U256;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::{
    CircuitOutboundMessage, ExtraMessagePayload, GatewayExpectedOutput, GatewayInboundProtocol,
    GatewayType, GenericAddress,
};

pub struct EthereumGatewayProtocol {}

impl GatewayInboundProtocol for EthereumGatewayProtocol {
    fn get_storage(
        &self,
        key: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn set_storage(
        &self,
        key: Vec<u8>,
        value: Option<Vec<u8>>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn call(
        &self,
        module_name: Vec<u8>,
        fn_name: Vec<u8>,
        data: Vec<u8>,
        escrow_account: Vec<u8>,
        requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        // erc_token, requester, to, value, gas
        let signature = "unlock(address,bytes,address,uint256,unit256)";
        let token_address = module_name
            .get(0..20)
            .ok_or("invalid token address")
            .map(|v| Address::from_slice(v))?;
        let to_address = to
            .get(0..20)
            .ok_or("invalid to address")
            .map(|v| Address::from_slice(v))?;
        let value_uint = U256::from_big_endian(value.as_slice());
        let gas_uint = U256::from_big_endian(gas.as_slice());
        let tokens = vec![
            Token::Address(token_address),
            Token::Bytes(requester.clone()),
            Token::Address(to_address),
            Token::Uint(value_uint),
            Token::Uint(gas_uint),
        ];

        let expected_outputs = vec![GatewayExpectedOutput::Events {
            // sender, receiver, amount
            signatures: vec!["Unlocked(address,address,uint256)".as_bytes().to_vec()],
        }];

        // call escrow contract which will in turn call the specific ERC 20 token
        Ok(CircuitOutboundMessage {
            name: b"call".to_vec(),
            module_name: escrow_account,
            method_name: fn_name,
            arguments: vec![module_name.clone(), to, value, gas],
            expected_output: expected_outputs,
            sender: Some(requester),
            target: None,
            extra_payload: Some(ExtraMessagePayload {
                signer: vec![],
                module_name,
                method_name: vec![],
                call_bytes: ethabi::encode_function(signature, &tokens),
                signature: vec![],
                extra: vec![],
                tx_signed: vec![],
                custom_payload: None,
            }),
        })
    }

    fn call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn custom_call_static(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn custom_call_dirty(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn custom_call_escrow(
        &self,
        module_name: &str,
        fn_name: &str,
        data: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
        return_value: Option<Vec<u8>>,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn transfer(
        &self,
        to: GenericAddress,
        value: Compact<u128>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn transfer_escrow(
        &self,
        escrow_account: Vec<u8>,
        requester: Vec<u8>,
        to: Vec<u8>,
        value: Vec<u8>,
        transfers: &mut Vec<TransferEntry>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        todo!()
    }

    fn swap_dirty(
        &self,
        to: Vec<u8>,
        value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }

    fn swap_escrow(
        &self,
        from: Vec<u8>,
        x_token: Vec<u8>,
        y_token: Vec<u8>,
        x_value: Vec<u8>,
        y_value: Vec<u8>,
        gas: Vec<u8>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        unimplemented!()
    }
}
