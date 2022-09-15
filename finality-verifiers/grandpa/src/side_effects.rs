use crate::{Config, Error};
use codec::{Decode, Encode};
use sp_runtime::DispatchError;
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

pub(crate) fn decode_event<T: Config<I>, I: 'static>(
    id: &[u8; 4],
    mut encoded_event: Vec<u8>,
    value_abi_unsigned_type: &[u8],
) -> Result<(Vec<Vec<u8>>, Vec<u8>), DispatchError> {
    // the first byte is the pallet index, which we don't need
    let _ = encoded_event.remove(0);
    match &id {
        &b"tran" => {
            // Assume that the different Pallet ID Circuit vs Target wouldn't matter for decoding on Circuit.
            match value_abi_unsigned_type {
                b"uint32" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u32>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                b"uint64" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u64>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                b"uint128" => match Decode::decode(&mut &encoded_event[..]) {
                    Ok(TransferEventStub::<T, u128>::Transfer { from, to, amount }) =>
                        Ok((vec![from.encode(), to.encode(), amount.encode()], vec![])),
                    _ => Err(Error::<T, I>::EventDecodingFailed.into()),
                },
                &_ => Err(Error::<T, I>::EventDecodingFailed.into()),
            }
        },
        &b"swap" | &b"aliq" => match value_abi_unsigned_type {
            b"uint32" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u32, CurrencyId>::Transfer {
                       currency_id,
                       from,
                       to,
                       amount,
                   }) => Ok((vec![
                    from.encode(),
                    to.encode(),
                    currency_id.encode(),
                    amount.encode(),
                ], vec![])),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            b"uint64" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u64, CurrencyId>::Transfer {
                       currency_id,
                       from,
                       to,
                       amount,
                   }) => Ok((vec![
                    from.encode(),
                    to.encode(),
                    currency_id.encode(),
                    amount.encode(),
                ], vec![])),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            b"uint128" => match Decode::decode(&mut &encoded_event[..]) {
                Ok(MultiTransferEventStub::<T, u128, CurrencyId>::Transfer {
                       currency_id,
                       from,
                       to,
                       amount,
                   }) => Ok((vec![
                    from.encode(),
                    to.encode(),
                    currency_id.encode(),
                    amount.encode(),
                ], vec![])),
                _ => Err(Error::<T, I>::EventDecodingFailed.into()),
            },
            &_ => Err(Error::<T, I>::EventDecodingFailed.into()),
        },
        &_ => Err(Error::<T, I>::UnkownSideEffect.into()),
    }
}

#[cfg(all(feature = "testing", test))]
pub mod tests {
    use crate::bridges::runtime::Chain;
    use codec::Encode;
    use frame_support::parameter_types;
    use sp_std::convert::{TryFrom, TryInto};
    // use crate::TestRuntime;

    use hex_literal::hex;
    use sp_runtime::{
        testing::{Header, H256},
        traits::{BlakeTwo256, IdentityLookup},
        AccountId32,
    };

    use crate::{decode_event, side_effects::*};

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
    type Block = frame_system::mocking::MockBlock<TestRuntime>;
    type Balance = u64;

    frame_support::construct_runtime!(
        pub enum TestRuntime where
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

    parameter_types! {
        pub const HeadersToStore: u32 = 5;
        pub const SessionLength: u64 = 5;
        pub const NumValidators: u32 = 5;
    }

    impl Config for TestRuntime {
        type BridgedChain = TestCircuitLikeChain;
        type HeadersToStore = HeadersToStore;
        type WeightInfo = ();
    }

    #[derive(Debug)]
    pub struct TestCircuitLikeChain;

    impl Chain for TestCircuitLikeChain {
        type BlockNumber = <TestRuntime as frame_system::Config>::BlockNumber;
        type Hash = <TestRuntime as frame_system::Config>::Hash;
        type Hasher = <TestRuntime as frame_system::Config>::Hashing;
        type Header = <TestRuntime as frame_system::Config>::Header;
    }

    impl frame_system::Config for TestRuntime {
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
    impl pallet_balances::Config for TestRuntime {
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
        let encoded_balance_transfer_event = pallet_balances::Event::<TestRuntime>::Transfer {
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
        let mut encoded_balance_transfer_event = pallet_balances::Event::<TestRuntime>::Transfer {
            from: hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            to: hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            amount: 1,
        }
            .encode();

        let encoded_transfer_stub = TransferEventStub::<TestRuntime, u64>::Transfer {
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

        let res = decode_event::<TestRuntime, ()>(b"tran", encoded_event, b"uint64").unwrap();

        assert_eq!(
            res,
            (vec![
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9,
                ],
                vec![
                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                    6, 6, 6, 6, 6, 6,
                ],
                vec![1, 0, 0, 0, 0, 0, 0, 0],
            ], vec![])
        );
    }
}
