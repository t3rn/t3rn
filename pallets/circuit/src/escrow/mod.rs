use crate::{pallet::Error, *};

use codec::Decode;

use sp_std::marker::PhantomData;

pub const FIRST_REQUESTER_NONCE: u32 = 0;

pub struct Escrow<T: Config> {
    _phantom: PhantomData<T>,
}

trait EscrowExec<T: Config> {
    fn exec(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
    fn revert(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
    fn commit(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str>;
}

impl<T: Config> Escrow<T> {
    pub fn exec(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::exec(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::exec(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::exec(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::exec(encoded_args, escrow_account, executioner),
            // b"call" => Call::exec(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::exec(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::exec(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::exec(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }

    pub fn commit(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::commit(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::commit(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::commit(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::commit(encoded_args, escrow_account, executioner),
            // b"call" => Call::commit(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::commit(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::commit(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::commit(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }

    pub fn revert(
        encoded_type: &[u8; 4],
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        match encoded_type {
            b"tran" => Transfer::<T>::revert(encoded_args, escrow_account, executioner),
            // b"mult" => TransferMulti::revert(encoded_args, escrow_account, executioner),
            // b"swap" => Swap::revert(encoded_args, escrow_account, executioner),
            // b"aliq" => AddLiquidity::revert(encoded_args, escrow_account, executioner),
            // b"call" => Call::revert(encoded_args, escrow_account, executioner),
            // b"wasm" => CallWasm::revert(encoded_args, escrow_account, executioner),
            // b"cevm" => CallEvm::revert(encoded_args, escrow_account, executioner),
            // b"comp" => CallComposable::revert(encoded_args, escrow_account, executioner),
            &_ => Err("Can't match escrow exec with any side effect id"),
        }
    }
}

pub struct Transfer<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> EscrowExec<T> for Transfer<T> {
    fn exec(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let _dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        log::debug!(
            "escrow exec transfer from {:?} to {:?} value {:?}",
            executioner,
            escrow_account,
            value
        );
        CurrencyOf::<T>::transfer(&executioner, &escrow_account, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            executioner,
            escrow_account,
            value,
        ));

        Ok(())
    }

    fn revert(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;

        CurrencyOf::<T>::transfer(&escrow_account, &executioner, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail

        log::debug!(
            "escrow revert transfer from {:?} to {:?} value {:?}",
            escrow_account,
            executioner,
            value
        );

        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(
            escrow_account,
            executioner,
            value,
        ));

        Ok(())
    }

    fn commit(
        encoded_args: Vec<Vec<u8>>,
        escrow_account: T::AccountId,
        _executioner: T::AccountId,
    ) -> Result<(), &'static str> {
        let value: BalanceOf<T> =
            Decode::decode(&mut encoded_args[2].as_ref()).map_err(|_e| "Decoding err")?;
        let dest: T::AccountId =
            Decode::decode(&mut encoded_args[1].as_ref()).map_err(|_e| "Decoding err")?;

        log::debug!(
            "escrow commit from {:?} to {:?} value {:?}",
            escrow_account,
            dest,
            value
        );
        CurrencyOf::<T>::transfer(&escrow_account, &dest, value, AllowDeath)
            .map_err(|_| Error::<T>::RewardTransferFailed)?; // should not fail
        <pallet::Pallet<T>>::deposit_event(Event::EscrowTransfer(escrow_account, dest, value));

        Ok(())
    }
}

#[cfg(test)]
pub mod test {

    use frame_support::{assert_ok, traits::Currency};
    use frame_system::{EventRecord, Phase};

    use circuit_mock_runtime::test_utils::*;
    use t3rn_primitives::{abi::Type};

    use crate::tests::brute_seed_block_1;
    use circuit_mock_runtime::*;
    use circuit_runtime_pallets::pallet_circuit;
    use hex_literal::hex;
    use pallet_circuit::escrow::Escrow;

    #[test]
    fn escrow_transfer_execute_and_commit_work() {
        let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

        let mut valid_transfer_side_effect = produce_and_validate_side_effect(
            t3rn_types::standard::get_transfer_interface(),
            vec![
                (Type::Address(32), ArgVariant::A),
                (Type::Address(32), ArgVariant::B),
                (Type::Uint(128), ArgVariant::A),
                (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
            ],
        );

        valid_transfer_side_effect.target = [3, 3, 3, 3];
        let side_effects = vec![valid_transfer_side_effect.clone()];

        let sequential = true;

        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let _ = Balances::deposit_creating(&ALICE, 1 + 2 + 1); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
                System::set_block_number(1);
                brute_seed_block_1([3, 3, 3, 3]);
                // Submit for execution first
                assert_ok!(Circuit::on_extrinsic_trigger(
                    origin,
                    side_effects,
                    sequential,
                ));

                let _xtx_id: sp_core::H256 =
                    hex!("7ac563d872efac72c7a06e78a4489a759669a34becc7eb7900e161d1b7a978a6").into();

                assert_ok!(Escrow::<Runtime>::exec(
                    b"tran",
                    valid_transfer_side_effect.encoded_args.clone(),
                    Circuit::account_id(),
                    ALICE,
                ));

                let mut latest_events = System::events();

                assert_eq!(
                    latest_events.pop().unwrap(),
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(
                            pallet_circuit::pallet::Event::<Runtime>::EscrowTransfer(
                                hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )
                                .into(), // executor account
                                hex!(
                                "3333333333333333333333333333333333333333333333333333333333333333"
                            )
                                .into(), // circuit account
                                1u128, // value
                            )
                        ),
                        topics: vec![]
                    },
                );

                assert_ok!(Escrow::<Runtime>::commit(
                    b"tran",
                    valid_transfer_side_effect.encoded_args,
                    Circuit::account_id(),
                    ALICE,
                ));
            });
    }

    #[test]
    fn escrow_transfer_execute_and_revert_work() {
        let origin = Origin::signed(ALICE); // Only sudo access to register new gateways for now

        let mut valid_transfer_side_effect = produce_and_validate_side_effect(
            t3rn_types::standard::get_transfer_interface(),
            vec![
                (Type::Address(32), ArgVariant::A),
                (Type::Address(32), ArgVariant::B),
                (Type::Uint(128), ArgVariant::A),
                (Type::OptionalInsurance, ArgVariant::A), // empty bytes instead of insurance
            ],
        );

        valid_transfer_side_effect.target = [3, 3, 3, 3];
        let side_effects = vec![valid_transfer_side_effect.clone()];

        let sequential = true;

        ExtBuilder::default()
            .with_standard_side_effects()
            .with_default_xdns_records()
            .build()
            .execute_with(|| {
                let _ = Balances::deposit_creating(&ALICE, 1 + 2 + 1); // Alice should have at least: fee (1) + insurance reward (2)(for VariantA)
                System::set_block_number(1);
                brute_seed_block_1([3, 3, 3, 3]);

                // Submit for execution first
                assert_ok!(Circuit::on_extrinsic_trigger(
                    origin,
                    side_effects,
                    sequential,
                ));

                assert_ok!(Escrow::<Runtime>::exec(
                    b"tran",
                    valid_transfer_side_effect.encoded_args.clone(),
                    Circuit::account_id(),
                    ALICE,
                ));

                let mut latest_events = System::events();

                assert_eq!(
                    latest_events.pop().unwrap(),
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(
                            pallet_circuit::pallet::Event::<Runtime>::EscrowTransfer(
                                hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )
                                .into(), // executor account
                                hex!(
                                "3333333333333333333333333333333333333333333333333333333333333333"
                            )
                                .into(), // circuit account
                                1u128, // value
                            )
                        ),
                        topics: vec![]
                    },
                );

                assert_ok!(Escrow::<Runtime>::revert(
                    b"tran",
                    valid_transfer_side_effect.encoded_args,
                    Circuit::account_id(),
                    ALICE,
                ));

                let mut latest_events = System::events();

                assert_eq!(
                    latest_events.pop().unwrap(),
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Circuit(
                            pallet_circuit::pallet::Event::<Runtime>::EscrowTransfer(
                                hex!(
                                "3333333333333333333333333333333333333333333333333333333333333333"
                            )
                                .into(), // executor account
                                hex!(
                                "0101010101010101010101010101010101010101010101010101010101010101"
                            )
                                .into(), // circuit account
                                1u128, // value
                            )
                        ),
                        topics: vec![]
                    },
                );
            });
    }
}
