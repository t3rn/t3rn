// use crate::{
//     mock::{
//         Balances, ContractsRegistry, RuntimeEvent as Event, RuntimeOrigin as Origin, System, Test,
//         EVM,
//     },
//     take_created_contract_id_from_event,
//     tests::{
//         create_2_contract_address,
//         threevm::utils::{
//             compile_module, create_test_registry_contract, initialize_block, CHARLIE, ESCROW,
//             GAS_LIMIT,
//         },
//         AccountId32, Error, Externalities, ALICE_H160,
//     },
//     AddressMapping, Config, CurrencyOf, REG_OPCODE_PREFIX,
// };
// use frame_support::{assert_err_ignore_postinfo, assert_ok, traits::Currency};
// use frame_system::{EventRecord, Phase};
// use hex_literal::hex;
// use primitive_types::{H160, H256, U256};
// use sha3::{Digest, Keccak256};
// use std::str::FromStr;
// use t3rn_primitives::contract_metadata::{ContractMetadata, ContractType};
//
// #[test]
// fn system_contract_does_not_instantiate() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let alice_balance = 1_000_000_u64;
//     let salt = H256::default();
//
//     Externalities::new()
//         .with_balance(None, 12345)
//         .build()
//         .execute_with(|| {
//             initialize_block(1);
//             let contract = create_test_registry_contract::<Test>(
//                 blob,
//                 &code_hash,
//                 ESCROW,
//                 alice_balance.checked_div(10),
//                 Some(ContractMetadata::default().with_type(ContractType::System)),
//             );
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 ESCROW,
//                 contract
//             ));
//
//             let addr = take_created_contract_id_from_event!(None);
//             //             origin: OriginFor<T>,
//             //             source: H160,
//             //             init: Vec<u8>,
//             //             salt: H256,
//             //             value: U256,
//             //             gas_limit: u64,
//             //             max_fee_per_gas: U256,
//             //             max_priority_fee_per_gas: Option<U256>,
//             //             nonce: Option<U256>,
//             //             access_list: Vec<(H160, Vec<H256>)>,
//             assert_err_ignore_postinfo!(
//                 EVM::create2(
//                     Origin::signed(get_alice()),
//                     REG_OPCODE_PREFIX
//                         .iter()
//                         .chain(addr.as_bytes())
//                         .cloned()
//                         .collect::<Vec<u8>>(),
//                     vec![], // init
//                     salt,
//                     U256::from(100)
//                         .checked_mul(U256::from(CurrencyOf::<Test>::minimum_balance()))
//                         .unwrap(),
//                     GAS_LIMIT,
//                     U256::from(1),
//                     None,
//                     None,
//                     vec![],
//                 ),
//                 pallet_3vm::Error::<Test>::CannotInstantiateContract
//             );
//
//             assert_eq!(Balances::free_balance(ESCROW), 0);
//         });
// }
//
// #[test]
// fn rent_fees_are_deducted_on_init() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let salt = H256::default();
//     let alice_balance = 1_000_000_000;
//
//     let author_fees = 100_000;
//
//     Externalities::new()
//         .with_balance(None, 12345)
//         .build()
//         .execute_with(|| {
//             let _ = Balances::deposit_creating(&get_alice(), alice_balance);
//             let _ = Balances::deposit_creating(&CHARLIE, 10);
//
//             initialize_block(1);
//
//             let contract = create_test_registry_contract::<Test>(
//                 blob,
//                 &code_hash,
//                 CHARLIE,
//                 Some(author_fees),
//                 Some(ContractMetadata::default().with_type(ContractType::VolatileEvm)),
//             );
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 CHARLIE,
//                 contract
//             ));
//
//             let addr = take_created_contract_id_from_event!(None);
//
//             initialize_block(2);
//
//             assert_ok!(EVM::create2(
//                 Origin::signed(get_alice()),
//                 REG_OPCODE_PREFIX
//                     .iter()
//                     .chain(addr.as_bytes())
//                     .cloned()
//                     .collect::<Vec<u8>>(),
//                 vec![],
//                 salt,
//                 U256::zero(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             let events = System::events();
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::Balances(pallet_balances::Event::Withdraw {
//                     who: get_alice(),
//                     amount: author_fees,
//                 }),
//                 topics: vec![],
//             }));
//
//             println!("EVM System Events {events:?} ");
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
//                     charge_id: H256::repeat_byte(0),
//                     payee: get_alice(),
//                     recipient: Some(CHARLIE),
//                     amount: author_fees,
//                 }),
//                 topics: vec![],
//             }));
//         });
// }
//
// #[test]
// fn rent_fees_are_deducted_on_init_and_call() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let alice_balance = 1_000_000_000;
//     let salt = H256::default();
//     let author_fees = 100_000;
//
//     Externalities::new()
//         .with_balance(None, 12345)
//         .build()
//         .execute_with(|| {
//             let _ = Balances::deposit_creating(&get_alice(), alice_balance);
//             let _ = Balances::deposit_creating(&CHARLIE, 10);
//             let contract = create_test_registry_contract::<Test>(
//                 blob.clone(),
//                 &code_hash,
//                 CHARLIE,
//                 Some(author_fees),
//                 Some(ContractMetadata::default().with_type(ContractType::VolatileEvm)),
//             );
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 CHARLIE,
//                 contract
//             ));
//
//             let registry_addr = take_created_contract_id_from_event!(None);
//
//             initialize_block(2);
//
//             assert_ok!(EVM::create2(
//                 Origin::signed(get_alice()),
//                 REG_OPCODE_PREFIX
//                     .iter()
//                     .chain(registry_addr.as_bytes())
//                     .cloned()
//                     .collect::<Vec<u8>>(),
//                 vec![],
//                 salt,
//                 U256::zero(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             assert!(System::events().contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
//                     charge_id: H256::repeat_byte(0),
//                     payee: get_alice(),
//                     recipient: Some(CHARLIE),
//                     amount: author_fees,
//                 }),
//                 topics: vec![],
//             }));
//
//             initialize_block(3);
//
//             let slice = H256::from_slice(Keccak256::digest(&blob).as_slice());
//             let addr =
//                 create_2_contract_address(&H160::from_str(ALICE_H160).unwrap(), &slice, &salt);
//
//             assert_ok!(EVM::call(
//                 Origin::signed(get_alice()),
//                 addr,
//                 hex::decode("2e64cec1").unwrap(),
//                 U256::zero(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             let events = System::events();
//
//             println!("EVM MMMMM {events:?}");
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
//                     charge_id: H256(hex!(
//                         "0100000000000000000000000000000000000000000000000000000000000000"
//                     )),
//                     payee: get_alice(),
//                     recipient: Some(CHARLIE),
//                     amount: author_fees,
//                 }),
//                 topics: vec![],
//             }));
//
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::EVM(crate::Event::Executed(H160::from([
//                     106, 99, 104, 109, 58, 132, 29, 119, 137, 16, 106, 116, 184, 32, 168, 37, 243,
//                     241, 86, 250,
//                 ]))),
//                 topics: vec![],
//             }));
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
//                     charge_id: sp_core::H256(hex_literal::hex!(
//                         "0100000000000000000000000000000000000000000000000000000000000000"
//                     )),
//                     payee: get_alice(),
//                     recipient: Some(CHARLIE),
//                     amount: author_fees
//                 }),
//                 topics: vec![],
//             }));
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::Balances(pallet_balances::Event::Deposit {
//                     who: CHARLIE,
//                     amount: 23479, // gas
//                 }),
//                 topics: vec![],
//             }));
//         });
// }
//
// #[test]
// fn rent_fees_cant_be_deducted_if_not_enough_funds() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let salt = H256::default();
//     let author_fees: u64 = u64::max_value();
//
//     Externalities::new()
//         .with_balance(None, 12345)
//         .build()
//         .execute_with(|| {
//             initialize_block(1);
//
//             let contract = create_test_registry_contract::<Test>(
//                 blob,
//                 &code_hash,
//                 ESCROW,
//                 Some(author_fees),
//                 Some(ContractMetadata::default().with_type(ContractType::VolatileEvm)),
//             );
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 ESCROW,
//                 contract
//             ));
//
//             let addr = take_created_contract_id_from_event!(None);
//
//             initialize_block(2);
//
//             assert_err_ignore_postinfo!(
//                 EVM::create2(
//                     Origin::signed(get_alice()),
//                     REG_OPCODE_PREFIX
//                         .iter()
//                         .chain(addr.as_bytes())
//                         .cloned()
//                         .collect::<Vec<u8>>(),
//                     vec![],
//                     salt,
//                     U256::zero(),
//                     GAS_LIMIT,
//                     U256::from(1),
//                     None,
//                     None,
//                     vec![],
//                 ),
//                 Error::<Test>::BalanceLow
//             );
//
//             // Drop previous events
//             initialize_block(2);
//
//             assert_eq!(Balances::free_balance(ESCROW), 0);
//         });
// }
//
// #[test]
// fn ensure_registry_contract_can_be_instantiated() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let alice_balance = 1_000_000_u64;
//     let salt = H256::default();
//     let author_fees: u64 = alice_balance.checked_div(10).unwrap();
//
//     Externalities::new()
//         .with_balance(Some(H160::from_str(ALICE_H160).unwrap()), 20000000000000000)
//         .build()
//         .execute_with(|| {
//             initialize_block(1);
//             let _ = Balances::deposit_creating(&get_alice(), alice_balance);
//             let contract = create_test_registry_contract::<Test>(
//                 blob,
//                 &code_hash,
//                 ESCROW,
//                 Some(author_fees),
//                 Some(ContractMetadata::default().with_type(ContractType::VolatileEvm)),
//             );
//
//             let min_balance = CurrencyOf::<Test>::minimum_balance();
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 ESCROW,
//                 contract
//             ));
//
//             let addr = take_created_contract_id_from_event!(None);
//
//             initialize_block(2);
//
//             assert_ok!(EVM::create2(
//                 Origin::signed(get_alice()),
//                 REG_OPCODE_PREFIX
//                     .iter()
//                     .chain(addr.as_bytes())
//                     .cloned()
//                     .collect::<Vec<u8>>(),
//                 vec![],
//                 salt,
//                 U256::from(100)
//                     .checked_mul(U256::from(min_balance))
//                     .unwrap(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             let events = System::events();
//
//             events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::EVM(crate::Event::Created(H160::from([
//                     106, 99, 104, 109, 58, 132, 29, 119, 137, 16, 106, 116, 184, 32, 168, 37, 243,
//                     241, 86, 250,
//                 ]))),
//                 topics: vec![],
//             })
//         });
// }
//
// #[test]
// fn gas_fees_are_deducted_on_init_and_call() {
//     let (blob, code_hash) = compile_module::<Test>("Storage");
//     let alice_balance = 1_000_000_000_u64;
//     let salt = H256::default();
//     let author_fees: u64 = alice_balance.checked_div(100).unwrap();
//
//     Externalities::new()
//         .with_balance(None, 12345)
//         .build()
//         .execute_with(|| {
//             initialize_block(1);
//
//             let _ = Balances::deposit_creating(&get_alice(), alice_balance);
//             let _ = Balances::deposit_creating(&CHARLIE, 10);
//             let contract = create_test_registry_contract::<Test>(
//                 blob.clone(),
//                 &code_hash,
//                 CHARLIE,
//                 Some(author_fees),
//                 Some(ContractMetadata::default().with_type(ContractType::VolatileEvm)),
//             );
//
//             assert_ok!(ContractsRegistry::add_new_contract(
//                 Origin::root(),
//                 CHARLIE,
//                 contract
//             ));
//
//             let registry_addr = take_created_contract_id_from_event!(None);
//
//             initialize_block(2);
//
//             assert_ok!(EVM::create2(
//                 Origin::signed(get_alice()),
//                 REG_OPCODE_PREFIX
//                     .iter()
//                     .chain(registry_addr.as_bytes())
//                     .cloned()
//                     .collect::<Vec<u8>>(),
//                 vec![],
//                 salt,
//                 U256::zero(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             initialize_block(3);
//
//             let slice = H256::from_slice(Keccak256::digest(&blob).as_slice());
//             let addr =
//                 create_2_contract_address(&H160::from_str(ALICE_H160).unwrap(), &slice, &salt);
//
//             assert_ok!(EVM::call(
//                 Origin::signed(get_alice()),
//                 addr,
//                 hex::decode("2e64cec1").unwrap(),
//                 U256::zero(),
//                 GAS_LIMIT,
//                 U256::from(1),
//                 None,
//                 None,
//                 vec![],
//             ));
//
//             let events = System::events();
//
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
//                     charge_id: H256(hex!(
//                         "0100000000000000000000000000000000000000000000000000000000000000"
//                     )),
//                     payee: get_alice(),
//                     recipient: Some(CHARLIE),
//                     amount: author_fees,
//                 }),
//                 topics: vec![],
//             }));
//
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::EVM(crate::Event::Executed(H160::from([
//                     106, 99, 104, 109, 58, 132, 29, 119, 137, 16, 106, 116, 184, 32, 168, 37, 243,
//                     241, 86, 250,
//                 ]))),
//                 topics: vec![],
//             }));
//             assert!(events.contains(&EventRecord {
//                 phase: Phase::Initialization,
//                 event: Event::Balances(pallet_balances::Event::Deposit {
//                     who: CHARLIE,
//                     amount: 23479, // gas
//                 }),
//                 topics: vec![],
//             }));
//         });
// }
//
// fn get_alice() -> AccountId32 {
//     <Test as Config>::AddressMapping::get_or_into_account_id(&H160::from_str(ALICE_H160).unwrap())
// }
//
// #[test]
// fn test_hex() {
//     println!("{}", hex::encode(REG_OPCODE_PREFIX));
// }
