#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};

use sp_std::vec;
use sp_std::vec::*;

pub use crate::side_effects::confirm::parser::VendorSideEffectsParser;

type Bytes = Vec<u8>;
type Arguments = Vec<Bytes>;
pub type EventSignature = Vec<u8>;
pub type String = Vec<u8>;

// Parser would come to the SideEffects as a parameter that implements the parser of events best suited for each vendor:
// Substrate - probably based on scale let decoded_event: pallet_balances::Event::transfer = Decode::decode(encoded_event_0)
// Ethereum - probably based on events decode that uses a signature as a string like Transfer(address,address,value)
pub struct SubstrateSideEffectsParser {}

impl VendorSideEffectsParser for SubstrateSideEffectsParser {
    fn parse_event<T: pallet_balances::Config>(
        name: &'static str,
        event_encoded: Vec<u8>,
        // If we go with decoding events based on the pallet-inherited Event encoder we won't need the signature to decode from Substrate
        _signature: &'static str,
    ) -> Result<Arguments, &'static str> {
        match name {
            "transfer:dirty" => {
                // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
                match Decode::decode(&mut &event_encoded[..]) {
                    Ok(pallet_balances::Event::<T>::Transfer(from, to, value)) => Ok(vec![from.encode(), to.encode(), value.encode()]),
                    Ok(pallet_balances::Event::<T>::Endowed(_, _)) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::DustLost(_, _)) => Err("Event decodes to pallet_balances::Event::DustLost, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::Deposit(_, _)) => Err("Event decodes to pallet_balances::Event::Deposit, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::Reserved(_, _)) => Err("Event decodes to pallet_balances::Event::Reserved, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::Unreserved(_, _)) => Err("Event decodes to pallet_balances::Event::Unreserved, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::BalanceSet(_, _, _)) => Err("Event decodes to pallet_balances::Event::BalanceSet, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::__Ignore(_, _)) => Err("Event decodes to pallet_balances::Event::BalanceSet, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::ReserveRepatriated(_, _, _, _)) => Err("Event decodes to pallet_balances::Event::ReserveRepatriated, which is unsupported"),
                    Ok(pallet_balances::Event::<T>::ReserveRepatriated(_, _, _, _)) => Err("Event decodes to pallet_balances::Event::ReserveRepatriated, which is unsupported"),
                    Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                }
            }
            &_ => Err("Event name unrecognized for the Substrate vendor"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{SubstrateSideEffectsParser, VendorSideEffectsParser};
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

    #[test]
    fn successfully_encodes_transferred_event() {
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

    #[test]
    fn successfully_parses_encoded_transferred_event_with_substrate_parser() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer(
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1,
        );

        let res = SubstrateSideEffectsParser::parse_event::<Test>(
            "transfer:dirty",
            encoded_balance_transfer_event.encode(),
            // If we go with decoding events based on the pallet-inherited Event encoder we won't need the signature to decode from Substrate
            "empty signature - not used by Substrate decoder",
        );

        assert_eq!(
            res,
            Ok(vec![
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9
                ],
                vec![
                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                    6, 6, 6, 6, 6, 6
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0]
            ])
        );
    }
}
