#![cfg_attr(not(feature = "std"), no_std)]

use codec::Compact;
use ethabi_decode::{self as ethabi, Address, Token};
use sp_core::{H160, U256};
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::transfers::TransferEntry;
use t3rn_primitives::{
    CircuitOutboundMessage, ExtraMessagePayload, GatewayExpectedOutput, GatewayInboundProtocol,
    GatewayType, GatewayVendor, GenericAddress,
};

pub struct EthereumGatewayProtocol {
    escrow_account: H160,
}

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
        // contract_address, requester, data, to, value, gas
        let signature = "call(address,bytes,bytes,uint256,unit256)";
        let contract_address = module_name
            .get(0..20)
            .ok_or("invalid token address")
            .map(|v| Address::from_slice(v))?;
        let value_uint = U256::from_big_endian(value.as_slice());
        let gas_uint = U256::from_big_endian(gas.as_slice());
        let tokens = vec![
            Token::Address(contract_address),
            Token::Bytes(requester.clone()),
            Token::Bytes(data.clone()),
            Token::Uint(value_uint),
            Token::Uint(gas_uint),
        ];

        let expected_outputs = vec![GatewayExpectedOutput::Events {
            // contract_address, sender, data
            signatures: vec!["called(address,bytes)".as_bytes().to_vec()],
        }];

        // call escrow contract which will in turn call the specific contract
        Ok(CircuitOutboundMessage {
            name: b"call".to_vec(),
            module_name: module_name.clone(),
            method_name: fn_name.clone(),
            arguments: vec![module_name.clone(), data, to, value, gas],
            expected_output: expected_outputs,
            sender: Some(requester),
            target: None,
            extra_payload: Some(ExtraMessagePayload {
                signer: vec![],
                module_name: escrow_account,
                method_name: fn_name,
                call_bytes: ethabi::encode_function(signature, &tokens),
                signature: vec![],
                extra: vec![],
                tx_signed: vec![],
                custom_payload: None,
            }),
            gateway_vendor: GatewayVendor::Ethereum,
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
        self.call(
            module_name.as_bytes().to_vec(),
            fn_name.as_bytes().to_vec(),
            data,
            self.escrow_account.as_bytes().to_vec(),
            vec![],
            to,
            value,
            gas,
            gateway_type,
            None,
        )
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
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
    }

    fn transfer(
        &self,
        to: GenericAddress,
        value: Compact<u128>,
        gateway_type: GatewayType,
    ) -> Result<CircuitOutboundMessage, &'static str> {
        let to = match to {
            GenericAddress::Address20(data) => Ok(data.to_vec()),
            _ => Err("Not an ethereum address"),
        }?;

        let value = value.0.to_be_bytes().to_vec();
        self.transfer_escrow(
            self.escrow_account.as_bytes().to_vec(),
            vec![],
            to,
            value,
            &mut vec![],
            gateway_type,
        )
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
        // contract_address, requester, data, to, value
        let signature = "transfer(address,uint256)";
        let to_addr = to
            .get(0..20)
            .ok_or("invalid token address")
            .map(|v| Address::from_slice(v))?;
        let value_uint = U256::from_big_endian(value.as_slice());
        let tokens = vec![Token::Address(to_addr), Token::Uint(value_uint)];

        let expected_outputs = vec![GatewayExpectedOutput::Events {
            // contract_address, sender, data
            signatures: vec!["Transfer(address,address,uint256)".as_bytes().to_vec()],
        }];

        // call escrow contract which will in turn call the specific ERC 20 token
        Ok(CircuitOutboundMessage {
            name: b"transfer".to_vec(),
            module_name: "Erc20".as_bytes().to_vec(),
            method_name: "transfer".as_bytes().to_vec(),
            arguments: vec![to_addr.as_bytes().to_vec(), value],
            expected_output: expected_outputs,
            sender: Some(requester),
            target: None,
            extra_payload: Some(ExtraMessagePayload {
                signer: vec![],
                module_name: escrow_account,
                method_name: "transfer".as_bytes().to_vec(),
                call_bytes: ethabi::encode_function(signature, &tokens),
                signature: vec![],
                extra: vec![],
                tx_signed: vec![],
                custom_payload: None,
            }),
            gateway_vendor: GatewayVendor::Ethereum,
        })
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
