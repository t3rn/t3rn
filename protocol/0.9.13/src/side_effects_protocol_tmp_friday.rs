#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::{
    abi::{GatewayABIConfig, Type},
    GatewayInboundProtocol,
};

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;

pub struct SideEffectsProtocol {
    gateway_abi: GatewayABIConfig,
}

pub trait SideEffectsConfirmationProtocol {
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
// impl SideEffectsConfirmationProtocol for EthereumSideEffectsProtocol {}

pub struct SubstrateSideEffectsProtocol {
    gateway_abi: GatewayABIConfig,
}

pub trait SideEffectType {
    fn dirty();
    fn escrowed();
    fn reversible();
}

// Side Effect could look like:
// moonbeam:transfer:dirty[myargs]
// moonbeam:transfer:dirty
// moonbeam:transfer:escrow
// moonbeam::xcm:transfer:reversible

// register_gateway(moonbeam, vendor::substrate, allowed_standard_methods["transfer:dirty"],
//      custom_methods: [
//          reversible:
//              "call_contract:reversible",
//                  ValidateWASM, ConfirmReversibleWASM, ConfirmReversibleSuccess, ConfirmReversibleSuccess
//
//
//          ]
//
//
// ]
// )

// 3VM::custom_side_effect_handle(
//      custom_args
// )

// implement reversible swap
//      validate_args(args)
// why would i care if relayer does the swap or not?
//      handle(args) -> confirm_reversible(..args) -> Event::Swapped(X, Y, Relayer)
//      handle(args) -> confirm_success(..args) -> Event::Transfer(Y, Target)
//      handle(args) -> confirm_error(..args) -> Event::Swapped(Y, Z, Relayer)
//
/**

What is step?
Step is an array of side effect available for execution that refers to the same Xtx state.
The can all be done parallel (1-round of N parallel steps) or with dependencies (M sequential rounds of N parallel steps)

Suggest the following split at the type of side effect:

1. Dirty
    Those are irreversible and not insured. Can only be placed at the end of each step (multiple dirty ones too).

    register_gateway(moonbeam, gateway_id: MOON#PURE_STAKE, vendor::substrate, allowed_methods[
            "transfer:dirty": "std" // take the standard implementation of dirty transfer for moonbeam gateway]
            "call_contract:dirty": "std" // take the standard implementation of dirty transfer for moonbeam gateway]
            "instantiate_contract:dirty": DirtySideEffect { // provide the custom implementation of instantiate_contract available only on ?gateway_id: MOON#PURE_STAKE? but not on std gateway?
                state: ["from->arg_0::address,to->arg_1::address,value->arg_2::u64,arg_3::rent->u32,arg_4::fees->u32"],
                validate_args: SideEffectArgumentsValidation([
                      input: "address,address,address,u64,u32,u32",
                      // or instead of state keep additional confirmation field: "from->address,to->address,value->u64,rent->u32,fees->u32",
                ]),
                confirm.rs: SideEffectEventsConfirmation("Event::instantiated(address,address,u64,u32,u32)"),
                // or confirm.rs: SideEffectEventsConfirmation("Event::instantiated(from,to,value,rent,fees)"),
            }
        )


2. Escrowed
    Those are insured and reversible, therefore can be places at any point of the execution step.
    Only need a single phase to execute.
    Trustless - rely on escrow accounts - smart contracts / special pieces of trustless logic deployed on the target chains.

    "instantiate_contract:escrow": EscrowSideEffect { // provide the custom implementation of instantiate_contract available only on ?gateway_id: MOON#PURE_STAKE? but not on std gateway?
        state: [ "escrow_address: 0x9238328392 ",
                 "calls: "instantiate(address,address,u64,u32,u32)"
                 "from->arg_0::address,to->arg_1::address,value->arg_2::u64,arg_3::rent->u32,arg_4::fees->u32"
        ],
        validate_args: SideEffectArgumentsValidation([
              input: "address,address,address,u64,u32,u32",
        ]),
        confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
        commit_confirm: ?not needed since we trust the escrow? otherwise expect the actual event here "Event::instantiated(from,to,u64,u32,u32)"
        revert_confirm: ?not needed since we trust the escrow?
        report_confirm_error: ?perhaps the escrow can emit the event if for some reason neither commit not revert happen? or how to proof the the action hasn't happen on the target chain?
    }


3. Reversible
    Those are insured and reversible but the degree of trust falls into the relayer's responsibility of correct behaviour.
    Reversible logic can be deemed always custom and optionally come with 2 phases - (as a feature can come in 2 steps - exec + revert / commit

    Those are insured and reversible, therefore can be places at any point of the execution step.
    Only need a single phase to execute.
    Trustless? - rely on escrow accounts - smart contracts / special pieces of trustless logic deployed on the target chains.

    "instantiate_contract:escrow": ReversibleSideEffect { // provide the custom implementation of instantiate_contract available only on ?gateway_id: MOON#PURE_STAKE? but not on std gateway?
        state: ["from->arg_0::address,to->arg_1::address,value->arg_2::u64,arg_3::rent->u32,arg_4::fees->u32"],
        validate_args: SideEffectArgumentsValidation([
              input: "address,address,address,u64,u32,u32",
              confirmation: "from->address,to->address,value->u64,rent->u32,fees->u32",
        ]),
        confirm.rs: SideEffectEventsConfirmation("Event::instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
        commit_confirm: SideEffectEventsCommitConfirmation("Event::?contract_transfer_ownership?(from,to,u64,u32,u32)")
        revert_confirm: SideEffectEventsCommitConfirmation("Event::?destruct?(from,to,u64,u32,u32)")
        report_confirm_error: ?perhaps the escrow can emit the event if for some reason neither commit not revert happen? or how to proof the the action hasn't happen on the target chain?
    }

**/

//          // this assumes escrow account can be any account that is active executioner?
// if that's the actual swap perhaps we could also check if the actual swap has been made - i mean i can just make a transfer to myself
//      handle(args) -> confirm_reversible(..args) -> Event::Deposit(Target, Y, ActiveExecutioner)
//      handle(args) -> confirm_success(..args) -> Event::Transfer(ActiveExecutioner, Y, Target)
//      handle(args) -> confirm_error(..args) -> { ... ? User not on Target Chain ? - so the ActiveExecutioner is in risk of loss here during the exchange }
//      handle(args) -> prove_misbehaviour(..args) -> { ... ? Happens on the protocol level of I got error during confirmation
//
//          couple of methods 3VM should then expose in std
//
//          - report_misbehaviour - when errors at reversible confirmation logic detected
//          - escrow_account(target_id)? -> if exists and brings the features of automated funds return if error or timeout, otherwise transfers to target?
//
//             -> it already starts looking like a new VM for writing composable smart contracts?
//              conclusion:
//                  custom:reversible implementation are composable contracts!
// }




// TransferSideEffectProtocol
// - substrate
// - ethereum
// - xcm
//

// confirm.rs (event_encoded)

// 1. gateway_X::parse_side_effect(event_encoded)

// Defining Events -> Parsing of events and confirmation
//  eth_abi::parser
//  substrate_parser for transfer event -> IN: encoded_event, OUT: transferSideEffect
//  substrate_parser for transfer event -> IN: encoded_event, OUT: transferSideEffect
///
/// let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
///
/// - SmartContractExecutionId
    // ExecutionState: Map<
    //   SmartContractExecutionId,
    //   ExecutionState
    // >;
    // enum ExecutionState {
    //   PendingStep(State),
    //   PendingSideEffects(Vec<SideEffect>),
    //   Finished,
    // }
    // SideEffectStates: Map<
    //   ExecutionId,
    //   Vec<(SideEffectNumber, SideEffectState)>
    // >
    // use sp_std::collections::HashMap;
    // enum SideEffectState {
    //   Populated(HashMap<A, B>),
    // }
///                         if !self.type_sizes.contains_key(&primitive) {
///        self.type_sizes.insert(name.to_string(), size);

pub struct TransferSideEffectProtocol {

    // best if state would be a map of length-uniformed keys, like blake2_8b("target_receiver"): "0x923993202332...")
    // pub var_name_to_state_index: Vec<(Bytes, usize)>
    // pub state: Vec<Vec<Bytes>>, //  Map
    pub state: codec::alloc::collections::HashMap<u64, Vec<u8>>,  //  Map

    pub gateway_abi: GatewayABIConfig,

    // let (addr_size, val_size) = (
    // self.gateway_abi.address_length,
    // self.gateway_abi.value_type_size,
    // );
}

pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;
pub type StringBox = Box<String>;

use sp_std::boxed::Box;

pub trait VendorSideEffectsParser {

    fn parse_event(
        name: &'static str,
        event_encoded: Vec<u8>,
        signature: &'static str,
    ) -> Result<Arguments, &'static str>;
}

pub struct SubstrateSideEffectsParser { }

impl VendorSideEffectsParser for SubstrateSideEffectsParser {

    fn parse_event(
        name: &'static str,
        _event_encoded: Vec<u8>,
        // If we go with decoding events based on the pallet-inherited Event encoder we won't need the signature to decode from Substrate
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {

        let output_args = vec![];

        match name {
            "transfer:dirty" => {
                // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
                // let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
            },
            &_ => {}
        }

        Ok(output_args)
    }
}

struct EthereumSideEffectsParser { }

impl VendorSideEffectsParser for EthereumSideEffectsParser {

    fn parse_event(
        name: &'static str,
        _event_encoded: Vec<u8>,
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {

        let output_args = vec![];

        match name {
            "transfer:dirty" => {
                // Use the similar tricks that are currently used in eth_outbound that use
                //         let signature: &str =
                //             sp_std::str::from_utf8(&name[..]).map_err(|_| "`Can't decode argument to &str")?;
                //
                //         let event = EthAbiEvent {
                //             signature,
                //             inputs: expected_arg_types_eth.as_slice(),
                //             anonymous: false,
                //         };
                //
                //         let args_decoded = event
                //             .decode(self.topics.clone(), self.data.to_vec())
                //             .map_err(|_| "Error decoding native eth event using ethabi-decoder")?;
            }
            &_ => {}
        }

        Ok(output_args)
    }
}

impl <VendorParser: VendorSideEffectsParser>SideEffectDirty<VendorParser, 3, 1> for TransferSideEffectProtocol {
    const NAME: &'static str = "transfer:dirty";
    // // Arguments ABI will be used to validate the input arguments against expected by the protocol.
    // // the information given in input arguments an then populated into state needs to be sufficient to confirm.rs the output side effect from the target chain
    const ARGUMENTS_ABI: [Type; 3] = [
        Type::DynamicAddress, // argument_0: from
        Type::DynamicAddress, // argument_1: to
        Type::Value, // argument_2: value
    ];
    const ARGS_2_STATE_MAPPER: [&'static str; 3] = ["from", "to", "value"];
    const CONFIRMING_EVENTS: [&'static str; 1] = ["Transfer(from,to,value)"];
}

pub trait SideEffectDirty<VendorParser: VendorSideEffectsParser, const ARGS_LEN: usize, const EXPECTED_EVENTS_LEN: usize> {
    const NAME: &'static str;
    const ARGUMENTS_ABI: [Type; ARGS_LEN];
    const ARGS_2_STATE_MAPPER: [&'static str; ARGS_LEN];
    const CONFIRMING_EVENTS: [&'static str; EXPECTED_EVENTS_LEN];

    fn populate_state(&self, _encoded_args: Arguments) -> Result<(), &'static str> {
        // STATE_MAPPER.0 -> "from" = encoded_args.0;
        Ok(())
    }
    // For now just assume that State can only be recreated from args? where arg index (usize) will be translated to the arguments name and therefore could be re-used in created expectations in the signature for confirming Events

    fn validate_args(&self, args: Arguments) -> Result<(), &'static str> {

            // Args number must match with the args number in the protocol
            assert!(Self::ARGUMENTS_ABI.len() == args.len());

            // ToDo: Extract to a separate function
            // Validate that the input arguments set by a user follow the protocol for get_storage side effect
            // Evaluate each input argument against strictly defined type for that gateway.
            // ToDo: Dig now to self.gateway_abi and recover the length of values, addresses to check
            for (i, arg) in args.iter().enumerate() {
                let type_n = &Self::ARGUMENTS_ABI[i];
                type_n.eval(arg.clone())?;
            }

            self.populate_state(args);

            // ToDo: Maybe return a signature assuming it isn't created by a user?
            Ok(())
    }

    // Use CONFIRMING_EVENTS now to
    //  1. Decode each event following it's Vendor decoding implementation (substrate events vs eth events)
    //  2. Use STATE_MAPPER to map each variable name from CONFIRMING_EVENTS into expected value stored in STATE_MAPPER during the "validate_args" step before the SideEffect was emitted for execution
    //  3. Check each argument of decoded "encoded_remote_events" against the values from STATE
    //  4. Return error that will potentially be a subject for a punishment of the executioner - up to the misbehaviour manager
    // confirm.rs: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm(&self, encoded_remote_events: Vec<Vec<u8>>) -> Result<(), &'static str> {

        // 0. Check incoming args with protocol requirements
        assert!(encoded_remote_events.len() == ARGS_LEN);

        // 1. Decode event as relying on Vendor-specific decoding/parsing
        let _decoded_events = encoded_remote_events.iter().enumerate().map(|(i, encoded_event)| {
            let expected_event_signature = Self::CONFIRMING_EVENTS[i];
            VendorParser::parse_event(Self::NAME, encoded_event.clone(), expected_event_signature)
        });

        // ToDo: 2. 3. 4. missing
        Ok(())
    }
}

pub trait SideEffectEscrowed {
    const NAME: Vec<u8>;
    // assume will always have a remote escrow contract to trust?
    const ESCROW_REMOTE_ADDRESS: Vec<u8>;

    fn populate_state();

    fn validate_args();

    fn confirm();
}

pub trait SideEffectReversible {
    const NAME: Vec<u8>;

    fn populate_state();

    fn validate_args();

    fn confirm_step_1();

    fn confirm_step_2_commit();

    fn confirm_step_2_revert();
}


pub trait SideEffectActionImpl {
    const NAME: Vec<u8>;

    fn validate_dirty();
    fn validate_escrowed();
    fn validate_reversible();

    fn confirm_dirty();
    fn confirm_escrowed();

    fn confirm_reversible();
    fn confirm_reversible_success();
    fn confirm_reversible_error();
}

impl SideEffectsConfirmationProtocol for SubstrateSideEffectsProtocol {
    // ToDo: Confirm execution! Decode incoming extrinsic.
    fn confirm_get_storage(
        &self,
        _encoded_original_args: Arguments,
        _encoded_effect: Bytes,
        _gateway_abi: GatewayABIConfig,
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
        _encoded_original_args: Arguments,
        _encoded_effect: Bytes,
        _gateway_abi: GatewayABIConfig,
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
    fn get_storage(&self, args: Arguments) -> Result<(), &'static str> {
        // Perhaps could also return specifically defined arguments already?
        // Result<GenericValue, &'static str> {
        let GET_STORAGE_ARGUMENTS_ABI: Vec<Type> =
            vec![Type::Uint(self.gateway_abi.value_type_size)];

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
        Ok(())
    }

    fn transfer(&self, args: Vec<Bytes>) -> Result<(), &'static str> {
        // Perhaps could also return specifically defined arguments already?
        //  Result<GenericAddress, GenericAddress, GenericValue, &'static str>
        let (addr_size, val_size) = (
            self.gateway_abi.address_length,
            self.gateway_abi.value_type_size,
        );
        // ToDo: Change arguments to const, like below
        let TRANSFER_ARGUMENTS_ABI: Vec<Type> = vec![
            Type::Address(addr_size),
            Type::Address(addr_size),
            Type::Uint(val_size),
        ];

        // Args number must match with the args number in the protocol
        assert!(TRANSFER_ARGUMENTS_ABI.len() == args.len());

        // ToDo: Extract
        for (i, arg) in args.iter().enumerate() {
            let type_n = &TRANSFER_ARGUMENTS_ABI[i];
            type_n.eval(arg.clone())?;
        }

        Ok(())
    }

    pub fn validate_input_args(&self, action: Bytes, args: Vec<Bytes>) -> Result<(), &'static str> {
        // Need to parse the action first
        let _GET_STORAGE: Vec<u8> = b"get_storage".to_vec();
        let _TRANSFER: Vec<u8> = b"transfer".to_vec();

        match action {
            _GET_STORAGE => self.get_storage(args),
            _TRANSFER => self.transfer(args),
            _ => Err("Not an ethereum address"),
        }
    }

    pub fn new(gateway_abi: GatewayABIConfig) -> Self {
        SideEffectsProtocol { gateway_abi }
    }
}
