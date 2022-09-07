use codec::{Decode, Encode};
use sp_std::{vec, vec::Vec};

#[derive(Encode, Decode)]
pub enum TransferEventStub<T: frame_system::Config, Balance> {
    Endowed(T::AccountId, Balance),
    DustLost(T::AccountId, Balance),
    Transfer {
        from: T::AccountId,
        to: T::AccountId,
        amount: Balance,
    },
}

type CurrencyId = u32;

#[derive(Encode, Decode)]
pub enum MultiTransferEventStub<T: frame_system::Config, Balance, CurrencyId> {
    Endowed(CurrencyId, T::AccountId, Balance),
    DustLost(CurrencyId, T::AccountId, Balance),
    Transfer {
        currency_id: CurrencyId,
        from: T::AccountId,
        to: T::AccountId,
        amount: Balance,
    },
}

pub(crate) fn decode_event<T: frame_system::Config>(
    id: &[u8; 4],
    mut encoded_event: Vec<u8>,
    value_abi_unsigned_type: &[u8],
) -> Result<Vec<Vec<Vec<u8>>>, &'static str> {
    // the first byte is the pallet index, which we don't need
    let _ = encoded_event.remove(0);
    match &id {
        &b"tran" => {
            // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
            match value_abi_unsigned_type {
                b"uint32" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(TransferEventStub::<T, u32>::Transfer { from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), amount.encode()]]),
                        Ok(TransferEventStub::<T, u32>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(TransferEventStub::<T, u32>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                b"uint64" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(TransferEventStub::<T, u64>::Transfer { from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), amount.encode()]]),
                        Ok(TransferEventStub::<T, u64>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(TransferEventStub::<T, u64>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                b"uint128" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(TransferEventStub::<T, u128>::Transfer { from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), amount.encode()]]),
                        Ok(TransferEventStub::<T, u128>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(TransferEventStub::<T, u128>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                &_ => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported")
            }
        }
        &b"swap" | &b"aliq" => {
            match value_abi_unsigned_type {
                b"uint32" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(MultiTransferEventStub::<T, u32, CurrencyId>::Transfer { currency_id, from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), currency_id.encode(), amount.encode()]]),
                        Ok(MultiTransferEventStub::<T, u32, CurrencyId>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(MultiTransferEventStub::<T, u32, CurrencyId>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                b"uint64" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(MultiTransferEventStub::<T, u64, CurrencyId>::Transfer { currency_id, from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), currency_id.encode(), amount.encode()]]),
                        Ok(MultiTransferEventStub::<T, u64, CurrencyId>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(MultiTransferEventStub::<T, u64, CurrencyId>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                b"uint128" => {
                    match Decode::decode(&mut &encoded_event[..]) {
                        Ok(MultiTransferEventStub::<T, u128, CurrencyId>::Transfer { currency_id, from, to, amount }) => Ok(vec![vec![from.encode(), to.encode(), currency_id.encode(), amount.encode()]]),
                        Ok(MultiTransferEventStub::<T, u128, CurrencyId>::Endowed { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Ok(MultiTransferEventStub::<T, u128, CurrencyId>::DustLost { .. }) => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported"),
                        Err(_) => Err("Decoded event doesn't match expected for substrate form of pallet_balances::Event::Transfer"),
                    }
                }
                &_ => Err("Event decodes to pallet_balances::Event::Endowed, which is unsupported")
            }
        }
        &_ => Err("Event name unrecognized for the Substrate vendor"),
    }
}

#[cfg(all(feature = "testing", test))]
pub mod tests {
    use codec::Encode;
    use frame_support::parameter_types;
    use sp_std::convert::{TryFrom, TryInto};

    use hex_literal::hex;
    use sp_runtime::{
        testing::{Header, H256},
        traits::{BlakeTwo256, IdentityLookup},
        AccountId32,
    };

    use crate::{decode_event, side_effects::*};

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;
    type Balance = u64;

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
            frame_system::limits::BlockWeights::simple_max(1024);
    }
    impl frame_system::Config for Test {
        type AccountData = pallet_balances::AccountData<u64>;
        type AccountId = AccountId32;
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockHashCount = BlockHashCount;
        type BlockLength = ();
        type BlockNumber = u64;
        type BlockWeights = ();
        type Call = Call;
        type DbWeight = ();
        type Event = Event;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Header = Header;
        type Index = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type MaxConsumers = frame_support::traits::ConstU32<16>;
        type OnKilledAccount = ();
        type OnNewAccount = ();
        type OnSetCode = ();
        type Origin = Origin;
        type PalletInfo = PalletInfo;
        type SS58Prefix = ();
        type SystemWeightInfo = ();
        type Version = ();
    }
    parameter_types! {
        pub const ExistentialDeposit: Balance = 1;
    }
    impl pallet_balances::Config for Test {
        type AccountStore = System;
        type Balance = Balance;
        type DustRemoval = ();
        type Event = Event;
        type ExistentialDeposit = ExistentialDeposit;
        type MaxLocks = ();
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type WeightInfo = ();
    }

    #[test]
    fn successfully_encodes_transferred_event() {
        let encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            amount: 1,
        };

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
        let mut encoded_balance_transfer_event = pallet_balances::Event::<Test>::Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            amount: 1,
        }
        .encode();

        let encoded_transfer_stub = TransferEventStub::<Test, u64>::Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            amount: 1,
        }
        .encode();

        assert_eq!(
            encoded_balance_transfer_event.clone(),
            encoded_transfer_stub
        );

        let mut encoded_event = vec![4];
        encoded_event.append(&mut encoded_balance_transfer_event);

        let res = decode_event::<Test>(b"tran", encoded_event, b"uint64").unwrap();

        assert_eq!(
            res,
            vec![vec![
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9,
                ],
                vec![
                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                    6, 6, 6, 6, 6, 6,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0],
            ]]
        );
    }
}
