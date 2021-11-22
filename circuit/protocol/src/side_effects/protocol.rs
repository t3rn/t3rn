#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_std::vec::*;
use t3rn_primitives::abi::{GatewayABIConfig, Type};

use sp_runtime::RuntimeDebug;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

/// The main idea would be to give a possibility of define the side effects dynamically
/// We'd have the "standard" side effects in the codebase, but for the sake of
/// define the new ones dynamically we could have the following generic structs via API, sth like:
///     "instantiate_contract:escrow": ReversibleSideEffect { // provide the custom implementation of instantiate_contract available only on ?gateway_id: MOON#PURE_STAKE? but not on std gateway?
///         state: ["from->arg_0::address,to->arg_1::address,value->arg_2::u64,arg_3::rent->u32,arg_4::fees->u32"],
///         confirm.rs: SideEffectEventsConfirmation("Event::instantiated(from,to,value,rent,fees)"), // from here on the trust falls back on the target escrow to emit the claim / refund txs
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
///       Usage:
///         ExecDelivery Validates incoming SideEffects and their arguments:
///             Has the target_id and needs to load gateway_abi for it - lazy loading would make
///             sense
///
///       USEProtocol.lazy_validate<XD(side_effect)
///         USEProtocol::lazy_load_gateway(side_effect.target_id)
///         USEProtocol.validate(side_effect) // throwable if gateway wasn't lazy-loaded
///
///       ExecDelivery<XDNS>::confirmation
///         USEProtocol::lazy_load_gateway<XDNS>(side_effect.target_id)
///         USEProtocol::lazy_confirm(side_effect)
///
///     From 3VM - i want to generate a side effect when the OPCODE encountered:
///
///         3VM.use_protocol.generate_side_effect(arguments, target_id)
///
#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct TransferSideEffectProtocol {}

impl SideEffectProtocol for TransferSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "transfer:dirty"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: from
            Type::DynamicAddress, // argument_1: to
            Type::Value,          // argument_2: value
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "to", "value"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["Transfer(from,to,value)"]
    }
}

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct CallSideEffectProtocol {}

impl SideEffectProtocol for CallSideEffectProtocol {
    fn get_name(&self) -> &'static str {
        "call:dirty"
    }
    fn get_arguments_abi(&self) -> Vec<Type> {
        vec![
            Type::DynamicAddress, // argument_0: from
            Type::DynamicAddress, // argument_1: to
            Type::Value,          // argument_2: value
        ]
    }
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str> {
        vec!["from", "to", "value"]
    }
    fn get_confirming_events(&self) -> Vec<&'static str> {
        vec!["Call(from,to,value)"]
    }
}

pub trait SideEffectProtocol {
    fn get_name(&self) -> &'static str;
    fn get_arguments_abi(&self) -> Vec<Type>;
    fn get_arguments_2_state_mapper(&self) -> Vec<&'static str>;
    fn get_confirming_events(&self) -> Vec<&'static str>;
    fn get_escrowed_events(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_exec(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_commit(&self) -> Vec<&'static str> {
        unimplemented!()
    }
    fn get_reversible_revert(&self) -> Vec<&'static str> {
        unimplemented!()
    }

    fn populate_state(&self, _encoded_args: Arguments) -> Result<(), &'static str> {
        // STATE_MAPPER.0 -> "from" = encoded_args.0;
        Ok(())
    }

    // For now just assume that State can only be recreated from args? where arg index (usize) will be translated to the arguments name and therefore could be re-used in created expectations in the signature for confirming Events

    fn validate_args(
        &self,
        args: Arguments,
        _gateway_abi: GatewayABIConfig,
    ) -> Result<(), &'static str> {
        // Args number must match with the args number in the protocol
        assert!(Self::get_arguments_abi(self).len() == args.len());

        // ToDo: Extract to a separate function
        // Validate that the input arguments set by a user follow the protocol for get_storage side effect
        // Evaluate each input argument against strictly defined type for that gateway.
        // ToDo: Dig now to self.gateway_abi and recover the length of values, addresses to check
        for (i, arg) in args.iter().enumerate() {
            let type_n = &Self::get_arguments_abi(self)[i];
            type_n.eval(arg.clone())?;
        }
        self.populate_state(args);

        // ToDo: Maybe return a signature assuming it isn't created by a user?
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use codec::Encode;
    use frame_support::{parameter_types, weights::constants::WEIGHT_PER_SECOND};
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
}
