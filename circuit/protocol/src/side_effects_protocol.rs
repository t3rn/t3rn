#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec;
use sp_std::vec::*;
use t3rn_primitives::abi::{GatewayABIConfig, Type};

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

pub trait VendorSideEffectsParser {
    fn parse_event(
        name: &'static str,
        event_encoded: Vec<u8>,
        signature: &'static str,
    ) -> Result<Arguments, &'static str>;
}

pub struct SideEffectsProtocol {
    gateway_abi: GatewayABIConfig,
}

pub struct TransferSideEffectProtocol {
    // Best if state would be a map of length-uniformed keys, like blake2_8b("target_receiver"): "0x923993202332...")
    // ToDo: Elevate higher into Xtx Local Store Context
    // pub state: codec::alloc::collections::HashMap<u64, Vec<u8>>,
    pub gateway_abi: GatewayABIConfig,
}

/// The main idea would be to give a possibility of define the side effects dynamically
/// We'd have the "standard" side effects in the codebase, but for the sake of
/// define the new ones dynamically we could have the following generic structs via API, sth like:
///     "instantiate_contract:escrow": ReversibleSideEffect { // provide the custom implementation of instantiate_contract available only on ?gateway_id: MOON#PURE_STAKE? but not on std gateway?
///         state: ["from->arg_0::address,to->arg_1::address,value->arg_2::u64,arg_3::rent->u32,arg_4::fees->u32"],
///         confirm: SideEffectEventsConfirmation("Event::instantiated(from,to,value,rent,fees)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
///         commit_confirm: SideEffectEventsCommitConfirmation("Event::?contract_transfer_ownership?(from,to,value,rent,fees)")
///         revert_confirm: SideEffectEventsCommitConfirmation("Event::?destruct?(from,to,value,rent,fees)")
///         report_confirm_error: ?perhaps the escrow can emit the event if for some reason neither commit not revert happen? or how to proof the the action hasn't happen on the target chain?
///     }
///  Elements like this would have to be defined here:
///       "transfer:dirty",
///       "transfer:escrow",
///       "transfer:reversible",
///       "get_storage",
///       "call:dirty",
///       "call:escrow?",
///       "call:reversible?",
impl<VendorParser: VendorSideEffectsParser> SideEffectDirty<VendorParser, 3, 1>
    for TransferSideEffectProtocol
{
    const NAME: &'static str = "transfer:dirty";
    const ARGS_2_STATE_MAPPER: [&'static str; 3] = ["from", "to", "value"];
    const CONFIRMING_EVENTS: [&'static str; 1] = ["Transfer(from,to,value)"];
    // Arguments ABI will be used to validate the input arguments against expected by the protocol.
    // the information given in input arguments an then populated into state needs to be sufficient to confirm the output side effect from the target chain
    const ARGUMENTS_ABI: [Type; 3] = [
        Type::DynamicAddress, // argument_0: from
        Type::DynamicAddress, // argument_1: to
        Type::Value,          // argument_2: value
    ];
}

// Parser would come to the SideEffects as a parameter that implements the parser of events best suited for each vendor:
// Substrate - probably based on scale let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
// Ethereum - probably based on events decode that uses a signature as a string like Transfer(address,address,value)
pub struct SubstrateSideEffectsParser {}

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
            }
            &_ => {}
        }

        Ok(output_args)
    }
}

struct EthereumSideEffectsParser {}

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

pub trait SideEffectDirty<
    VendorParser: VendorSideEffectsParser,
    const ARGS_LEN: usize,
    const EXPECTED_EVENTS_LEN: usize,
>
{
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
    // confirm: SideEffectEventsConfirmation("Event::escrow_instantiated(from,to,u64,u32,u32)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
    fn confirm(&self, encoded_remote_events: Vec<Vec<u8>>) -> Result<(), &'static str> {
        // 0. Check incoming args with protocol requirements
        assert!(encoded_remote_events.len() == ARGS_LEN);

        // 1. Decode event as relying on Vendor-specific decoding/parsing
        let _decoded_events = encoded_remote_events
            .iter()
            .enumerate()
            .map(|(i, encoded_event)| {
                let expected_event_signature = Self::CONFIRMING_EVENTS[i];
                VendorParser::parse_event(
                    Self::NAME,
                    encoded_event.clone(),
                    expected_event_signature,
                )
            });

        // ToDo: 2. 3. 4. missing
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use codec::Encode;
    use frame_support::{
        parameter_types,
        weights::{constants::WEIGHT_PER_SECOND},
    };
    use hex_literal::hex;
    use sp_runtime::{
        testing::{Header, H256},
        traits::{BlakeTwo256, IdentityLookup},
        AccountId32,
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
            Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub BlockWeights: frame_system::limits::BlockWeights =
            frame_system::limits::BlockWeights::simple_max(2 * WEIGHT_PER_SECOND);
        pub static ExistentialDeposit: u64 = 0;
    }
    impl frame_system::Config for Test {
        type BaseCallFilter = ();
        type BlockWeights = BlockWeights;
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Call = Call;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId32;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u64>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ();
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type Balance = u64;
        type Event = Event;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
    }

    #[test]
    fn successfully_confirms_encoded_balance_transferred_event() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer(
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1,
        );

        assert_eq!(
            encoded_balance_transfer_event.encode(),
            vec![
                2, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                9, 9, 9, 9, 9, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 1, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
