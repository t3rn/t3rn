#![cfg_attr(not(feature = "std"), no_std)]
use crate::OutboundSideEffect;
use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::{
    GatewayInboundProtocol,
    abi::{GatewayABIConfig, Type},
};

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;

pub struct SideEffectsProtocol {
    gateway_abi: GatewayABIConfig,
}

pub trait InboundSideEffectsProtocol {
    fn confirm_get_storage(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str>;

    fn confirm_transfer(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str>;
}

// // ToDo: implement for Eth & Substrate!
// pub struct EthereumSideEffectsProtocol {
//     gateway_abi: GatewayABIConfig,
// }
// impl InboundSideEffectsProtocol for EthereumSideEffectsProtocol {}

pub struct SubstrateSideEffectsProtocol {
    gateway_abi: GatewayABIConfig,
}

impl InboundSideEffectsProtocol for SubstrateSideEffectsProtocol {
    // ToDo: Confirm execution! Decode incoming extrinsic.
    fn confirm_get_storage(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        // ToDo: Decode encoded_effect into signature / InboundSideEffect here
        // let inbound_side_effect_args = self.effect_to_args(encoded_effect, gateway_abi);
        // Call::Balances(pallet_balances::Call::transfer {
        //     dest: outbound_side_effect.arguments.0 // dest, like Bob,
        //     value: outbound_side_effect.arguments.1 // value, like 69 * DOLLARS,
        // }),
        // ToDo: Compare now! - From this form I could either Decode the incoming effect or encode the UncheckedExtrinsic and
        //  compare with relayed result - depends if I'm able to go to unsigned bytes on target chain
        // inbound_side_effect_args.iter().enumerate().map(|i, arg| { arg != outbound_side_effect.args[i])
        Ok(())
    }

    fn confirm_transfer(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        // ToDo: Decode encoded_effect into signature / InboundSideEffect here
        // let inbound_side_effect_args = self.effect_to_args(encoded_effect, gateway_abi);
        // Call::Balances(pallet_balances::Call::transfer {
        //     dest: outbound_side_effect.arguments.0 // dest, like Bob,
        //     value: outbound_side_effect.arguments.1 // value, like 69 * DOLLARS,
        // }),
        // ToDo: Compare now! - From this form I could either Decode the incoming effect or encode the UncheckedExtrinsic and
        //  compare with relayed result - depends if I'm able to go to unsigned bytes on target chain
        // inbound_side_effect_args.iter().enumerate().map(|i, arg| { arg != outbound_side_effect.args[i])
        Ok(())
    }
}

impl SideEffectsProtocol {
    fn get_storage(
        &self,
        args: Arguments,
    ) -> Result<(), &'static str> {
        // Perhaps could also return specifically defined arguments already?
        // Result<GenericValue, &'static str> {
        let GET_STORAGE_ARGUMENTS_ABI: Vec<Type> = vec![Type::Uint(self.gateway_abi.value_type_size)];

        // Args number must match with the args number in the protocol
        assert!(GET_STORAGE_ARGUMENTS_ABI.len() == args.len());

        // ToDo: Extract to a separate function
        // Validate that the input arguments set by a user follow the protocol for get_storage side effect
        // Evaluate each input argument against strictly defined type for that gateway.
        for (i, arg) in args.iter().enumerate() {
            let type_n = &GET_STORAGE_ARGUMENTS_ABI[i];
            type_n.eval(arg.clone())?;
        }

        // ToDo: Maybe return a signature assuming it isn't created by a user?
        Ok(().into())
    }

    fn transfer(
        &self,
        args: Vec<Bytes>,
    ) -> Result<Arguments, &'static str> {
        // Perhaps could also return specifically defined arguments already?
        //  Result<GenericAddress, GenericAddress, GenericValue, &'static str>
        let (addr_size, val_size) = (self.gateway_abi.address_length, self.gateway_abi.value_type_size);
        // ToDo: Change arguments to const, like below
        let TRANSFER_ARGUMENTS_ABI: Vec<Type> = vec![Type::Address(addr_size), Type::Address(addr_size), Type::Uint(val_size)];

        // Args number must match with the args number in the protocol
        assert!(TRANSFER_ARGUMENTS_ABI.len() == args.len());

        // ToDo: Extract
        for (i, arg) in args.iter().enumerate() {
            let type_n = &TRANSFER_ARGUMENTS_ABI[i];
            type_n.eval(arg.clone())?;
        }

        Ok(().into())
    }

    pub fn validate_input_args(
        &self,
        action: Bytes,
        args: Vec<Bytes>,
    ) -> Result<Arguments, &'static str> {
        // Need to parse the action first
        let GET_STORAGE: Vec<u8> = b"get_storage".to_vec();
        let TRANSFER: Vec<u8> = b"transfer".to_vec();

        match action {
            GET_STORAGE => self.get_storage(args, self.gateway_abi),
            TRANSFER => self.transfer(args, self.gateway_abi),
            _ => Err("Not an ethereum address"),
        }
    }

    pub fn new<InboundVendor: InboundSideEffectsProtocol>(gateway_abi: GatewayABIConfig) {
        SideEffectsProtocol { gateway_abi }
    }
}

impl InboundSideEffectsProtocol for SideEffectsProtocol {
    fn confirm_get_storage<InboundVendor: InboundSideEffectsProtocol>(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        InboundVendor::confirm_get_storage(
            &self,
            encoded_original_args,
            encoded_effect,
            gateway_abi,
        )
    }

    fn confirm_transfer<InboundVendor: InboundSideEffectsProtocol>(
        &self,
        encoded_original_args: Arguments,
        encoded_effect: Bytes,
        gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        InboundVendor::confirm_get_storage(
            &self,
            encoded_original_args,
            encoded_effect,
            gateway_abi,
        )
    }
}
