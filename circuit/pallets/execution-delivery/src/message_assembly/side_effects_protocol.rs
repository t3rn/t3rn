#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::{GatewayABIConfig,GatewayInboundProtocol, Type};
type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;

trait InboundSideEffectsProtocol {
    fn get_storage(
        &self,
        args: Arguments,
        gac: GatewayABIConfig,
    ) -> Result<(), &'static str>;

    fn transfer(
        &self,
        bytes: Arguments,
        gac: GatewayABIConfig,
    ) -> Result<(), &'static str>;
}

// // ToDo: implement for Eth & Substrate!
// struct EthereumSideEffectsProtocol {}
// impl InboundSideEffectsProtocol for EthereumSideEffectsProtocol {}
struct SubstrateSideEffectsProtocol {}

impl InboundSideEffectsProtocol for SubstrateSideEffectsProtocol {
    fn get_storage(
        &self,
        outbound_side_effect: OutboundSideEffect,
        encoded_effect: Bytes,
        gac: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        // ToDo: Decode encoded_effect into signature / InboundSideEffect here
        // let inbound_side_effect_args = self.effect_to_args(encoded_effect, gac);
        // ToDo: Compare now!
        // inbound_side_effect_args.iter().enumerate().map(|i, arg| { arg != outbound_side_effect.args[i])
        Ok(())
    }

    fn transfer(
        &self,
        encoded_effect: Bytes,
        gac: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        // ToDo: Decode encoded_effect into signature / InboundSideEffect here
        // let inbound_side_effect_args = self.effect_to_args(encoded_effect, gac);
        // ToDo: Compare now!
        // inbound_side_effect_args.iter().enumerate().map(|i, arg| { arg != outbound_side_effect.args[i])
        Ok(())
    }
}

impl SideEffectsProtocol {
    fn get_storage(
        &self,
        args: Arguments,
        gac: GatewayABIConfig,
    ) -> Result<(), &'static str> {
    // Perhaps could also return specifically defined arguments already?
        // Result<GenericValue, &'static str> {
        // ToDo: Change arguments to const, like below
        // const GET_STORAGE_ARGUMENTS_ABI: Vec<Type> = vec![Type::DynamicBytes];
        // const TRANSFER_ARGUMENTS_ABI: Vec<Type> = vec![Type::Address, Type::Address, Type::Value];
        let GET_STORAGE_ARGUMENTS_ABI = vec![Type::Uint(gac.value_type_size)];

        // Args number must match with the args number in the protocol
        assert!(GET_STORAGE_ARGUMENTS_ABI.len() == args.len());

        // ToDo: Extract
        for (i, arg) in args.iter().enumerate()  {
            let type_n = &GET_STORAGE_ARGUMENTS_ABI[i];
            type_n.eval(arg.clone())?;
        }

        // ToDo: Maybe return a signature assuming it isn't created by a user?
        Ok(().into())
    }

    fn transfer(
        &self,
        _args: Vec<Bytes>,
        _gateway_abi_config: GatewayABIConfig,
    ) -> Result<Arguments, &'static str> {
    // Perhaps could also return specifically defined arguments already?
        //  Result<GenericAddress, GenericAddress, GenericValue, &'static str>
        // ToDo: Change arguments to const, like below
        // const GET_STORAGE_ARGUMENTS_ABI: Vec<Type> = vec![Type::DynamicBytes];
        // const TRANSFER_ARGUMENTS_ABI: Vec<Type> = vec![Type::Address, Type::Address, Type::Value];

        // Args number must match with the args number in the protocol
        assert!(GET_STORAGE_ARGUMENTS_ABI.len() == args.len());

        // ToDo: Extract
        for (i, arg) in args.iter().enumerate()  {
            let type_n = &GET_STORAGE_ARGUMENTS_ABI[i];
            type_n.eval(arg.clone())?;
        }

        Ok(().into())
    }

    pub fn validate_input_args(
        &self,
        action: Bytes,
        args: Vec<Bytes>,
        gateway_abi_config: GatewayABIConfig,
    ) -> Result<Arguments, &'static str> {
        // Need to parse the action first
        let _GET_STORAGE: Vec<u8> = b"get_storage".to_vec();
        let _TRANSFER: Vec<u8> = b"transfer".to_vec();

        match action {
            _GET_STORAGE => {
                self.get_storage(args, gateway_abi_config)
            },
            _TRANSFER => {
                self.transfer(args, gateway_abi_config)
            },
            _ => Err("Not an ethereum address"),
        }
    }
}
