use crate::{
    take_created_contract_id_from_event,
    tests::{
        compile_module, initialize_block,
        threevm::utils::{create_test_registry_contract, create_test_system_registry_contract},
        BalanceStatus, Balances, Contracts, ContractsRegistry, Error, Event, ExtBuilder, Origin,
        System, Test, ALICE, BOB, ESCROW, GAS_LIMIT,
    },
    ContractInfoOf, CurrencyOf,
};
use codec::Encode;
use frame_support::{assert_err_ignore_postinfo, assert_ok, traits::Currency};
use frame_system::{EventRecord, Phase};
use glob::{glob, Paths};
use t3rn_primitives::contract_metadata::{ContractMetadata, ContractType};

mod contract_metadata {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContractMetadata {
        #[serde(rename = "V3")]
        pub v3: V3,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct V3 {
        pub spec: Spec,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Spec {
        pub constructors: Vec<Constructor>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Constructor {
        pub args: Vec<Value>,
        pub payable: bool,
        pub selector: String,
    }
}

#[test]
fn instantiate_all_smart_contracts() {
    let contracts_path = "fixtures/contracts/*.wasm";
    let contracts: Paths = glob(contracts_path).unwrap();
    assert!(
        glob(contracts_path).unwrap().count() > 0,
        "Contracts not found"
    );
    for entry in contracts {
        let entry = entry.unwrap();
        let path = entry.to_str().unwrap();
        println!("instantiating: {:?}", path);

        let metadata_path = path.clone().replace(".wasm", ".contract");
        let file = std::fs::File::open(metadata_path).unwrap();
        let reader = std::io::BufReader::new(file);
        let metadata: contract_metadata::ContractMetadata =
            serde_json::from_reader(reader).unwrap();

        let wasm = wat::parse_file(path).expect("Failed to parse file");
        let alice_balance = 1_000_000;

        ExtBuilder::default()
            .existential_deposit(200)
            .build()
            .execute_with(|| {
                let _ = Balances::deposit_creating(&ALICE, alice_balance);

                // build the data, take the first constructor with no args, if there are no constructors, then we can't instantiate
                let ctor = metadata
                    .v3
                    .spec
                    .constructors
                    .iter()
                    .find(|m| m.args.is_empty())
                    .expect("Can't instantiate contract without constructor");

                let ctor_selector =
                    hex::decode(ctor.selector.clone().strip_prefix("0x").unwrap_or_default())
                        .unwrap();
                println!("Found ctor: {:?}", ctor);
                assert_ok!(Contracts::instantiate_with_code(
                    Origin::signed(ALICE),
                    if ctor.payable { 30_000 } else { 0 },
                    GAS_LIMIT,
                    None,
                    wasm,
                    ctor_selector,
                    b"00000001".to_vec(),
                ));
            });
    }
}

#[test]
fn system_contract_does_not_instantiate() {
    let (wasm, code_hash) = compile_module::<Test>("multi_store").unwrap();
    ExtBuilder::default()
        .existential_deposit(200)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);

            let contract =
                create_test_system_registry_contract::<Test>(wasm, &code_hash, ALICE, None);

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                ALICE,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);
            let salt = vec![];

            assert_err_ignore_postinfo!(
                Contracts::instantiate(
                    Origin::signed(ALICE),
                    0,
                    GAS_LIMIT,
                    None,
                    addr,
                    vec![],
                    salt,
                ),
                pallet_3vm::Error::<Test>::CannotInstantiateContract
            );
        });
}

// TODO: test needs reworking now author fees are removed
#[test]
fn rent_fees_are_deducted_on_init() {
    let (wasm, code_hash) = compile_module::<Test>("multi_store").unwrap();
    let alice_balance = 1_000_000_u64;
    let salt = vec![];
    let author_fees = if let Some(v) = alice_balance.checked_div(10) {
        v
    } else {
        alice_balance
    };

    ExtBuilder::default()
        .existential_deposit(200)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, alice_balance);
            let contract = create_test_registry_contract::<Test>(
                wasm,
                &code_hash,
                ESCROW,
                Some(author_fees),
                Some(ContractMetadata::default().with_type(ContractType::VolatileWasm)),
            );

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                ESCROW,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);

            initialize_block(2);

            assert_ok!(Contracts::instantiate(
                Origin::signed(ALICE),
                0,
                GAS_LIMIT,
                None,
                addr,
                vec![],
                salt.clone(),
            ));

            let events = System::events();
            assert!(events.contains(&EventRecord {
                phase: Phase::Initialization,
                event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
                    charge_id: Default::default(),
                    payee: ALICE,
                    recipient: Some(ESCROW),
                    amount: author_fees
                }),
                topics: vec![],
            }));
            // Drop previous events
            initialize_block(2);

            assert_eq!(Balances::reserved_balance(ALICE), 694);
        });
}

#[test]
fn rent_fees_are_deducted_on_init_and_call() {
    let (wasm, code_hash) = compile_module::<Test>("multi_store").unwrap();
    let alice_balance = 1_000_000;
    let salt = vec![];
    let author_fees = alice_balance / 10;

    ExtBuilder::default()
        .existential_deposit(200)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, alice_balance);
            let contract = create_test_registry_contract::<Test>(
                wasm,
                &code_hash,
                ESCROW,
                Some(author_fees),
                Some(ContractMetadata::default().with_type(ContractType::VolatileWasm)),
            );

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                ESCROW,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);

            initialize_block(2);

            assert_ok!(Contracts::instantiate(
                Origin::signed(ALICE),
                0,
                GAS_LIMIT,
                None,
                addr,
                vec![],
                salt.clone(),
            ));

            println!(
                "threeVM -- contracts System::events() {:?}",
                System::events()
            );

            assert!(System::events().contains(&EventRecord {
                phase: Phase::Initialization,
                event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
                    charge_id: Default::default(),
                    payee: ALICE,
                    recipient: Some(ESCROW),
                    amount: author_fees
                }),
                topics: vec![],
            }));

            assert_eq!(Balances::reserved_balance(ALICE), 694);

            // Drop previous events
            initialize_block(2);

            let addr = Contracts::contract_address(&ALICE, &code_hash, &salt);

            System::reset_events();
            // Create storage
            assert_ok!(Contracts::call(
                Origin::signed(ALICE),
                addr,
                42,
                GAS_LIMIT,
                None,
                (1_000u32, 5_000u32).encode(),
            ));

            println!("{:#?}", System::events());
            assert!(System::events().contains(&EventRecord {
                phase: Phase::Initialization,
                event: Event::AccountManager(pallet_account_manager::Event::DepositReceived {
                    charge_id: sp_core::H256(hex_literal::hex!(
                        "0100000000000000000000000000000000000000000000000000000000000000"
                    )),
                    payee: ALICE,
                    recipient: Some(ESCROW),
                    amount: author_fees
                }),
                topics: vec![],
            }));
            assert_eq!(Balances::reserved_balance(ALICE), 694);
        });
}

#[test]
fn rent_fees_cant_be_deducted_if_not_enough_funds() {
    let (wasm, code_hash) = compile_module::<Test>("multi_store").unwrap();
    let alice_balance = 1_000_000_u64;
    let salt = vec![];
    let author_fees: u64 = if let Some(v) = alice_balance.checked_mul(10) {
        v
    } else {
        alice_balance
    };

    ExtBuilder::default()
        .existential_deposit(200)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, alice_balance);
            let contract = create_test_registry_contract::<Test>(
                wasm,
                &code_hash,
                ESCROW,
                Some(author_fees),
                Some(ContractMetadata::default().with_type(ContractType::VolatileWasm)),
            );

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                ESCROW,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);

            initialize_block(2);

            assert_err_ignore_postinfo!(
                Contracts::instantiate(
                    Origin::signed(ALICE),
                    0,
                    GAS_LIMIT,
                    None,
                    addr,
                    vec![],
                    salt.clone(),
                ),
                Error::<Test>::StorageDepositLimitExhausted
            );

            // Drop previous events
            initialize_block(2);

            assert_eq!(Balances::free_balance(ESCROW), 0);
        });
}

#[test]
fn ensure_registry_contract_can_be_instantiated() {
    let (wasm, code_hash) = compile_module::<Test>("dummy").unwrap();
    let alice_balance = 1_000_000_u64;
    let salt = vec![];
    let author_fees: u64 = if let Some(v) = alice_balance.checked_div(10) {
        v
    } else {
        alice_balance
    };

    ExtBuilder::default()
        .existential_deposit(100)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, alice_balance);
            let contract = create_test_registry_contract::<Test>(
                wasm,
                &code_hash,
                ESCROW,
                Some(author_fees),
                Some(ContractMetadata::default().with_type(ContractType::VolatileWasm)),
            );

            let min_balance = CurrencyOf::<Test>::minimum_balance();

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                ESCROW,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);

            initialize_block(2);
            assert_ok!(Contracts::instantiate(
                Origin::signed(ALICE),
                min_balance * 100,
                GAS_LIMIT,
                None,
                addr,
                vec![],
                salt.clone(),
            ));

            initialize_block(2);
            assert_eq!(Balances::free_balance(ESCROW), 0);
        });
}

#[test]
fn storage_deposit_works_even_for_registry_accs() {
    let (wasm, code_hash) = compile_module::<Test>("multi_store").unwrap();
    ExtBuilder::default()
        .existential_deposit(200)
        .build()
        .execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, 1_000_000);
            let _ = Balances::deposit_creating(&BOB, 1_000_000);
            let mut deposit = CurrencyOf::<Test>::minimum_balance();

            let contract = create_test_registry_contract::<Test>(
                wasm,
                &code_hash,
                BOB,
                None,
                Some(ContractMetadata::default().with_type(ContractType::VolatileWasm)),
            );

            assert_ok!(ContractsRegistry::add_new_contract(
                Origin::root(),
                BOB,
                contract
            ));

            let addr = take_created_contract_id_from_event!(None);
            let salt = vec![];

            assert_ok!(Contracts::instantiate(
                Origin::signed(ALICE),
                0,
                GAS_LIMIT,
                None,
                addr,
                vec![],
                salt.clone(),
            ));

            let addr = Contracts::contract_address(&ALICE, &code_hash, &salt);

            // Drop previous events
            initialize_block(2);

            // Create storage
            assert_ok!(Contracts::call(
                Origin::signed(ALICE),
                addr.clone(),
                42,
                GAS_LIMIT,
                None,
                (1_000u32, 5_000u32).encode(),
            ));

            // 4 is for creating 2 storage items
            let charged0: u64 = 4 + 1_000 + 5_000;
            deposit = deposit.checked_add(charged0).unwrap();
            assert_eq!(
                <ContractInfoOf<Test>>::get(&addr).unwrap().storage_deposit,
                deposit
            );

            // Add more storage (but also remove some)
            assert_ok!(Contracts::call(
                Origin::signed(ALICE),
                addr.clone(),
                0,
                GAS_LIMIT,
                None,
                (2_000u32, 4_900u32).encode(),
            ));
            let charged1 = 1_000_u64 - 100;
            deposit = if let Some(v) = deposit.checked_add(charged1) {
                v
            } else {
                deposit
            };
            assert_eq!(
                <ContractInfoOf<Test>>::get(&addr).unwrap().storage_deposit,
                deposit
            );

            // Remove more storage (but also add some)
            assert_ok!(Contracts::call(
                Origin::signed(ALICE),
                addr.clone(),
                0,
                GAS_LIMIT,
                None,
                (2_100u32, 900u32).encode(),
            ));
            let refunded0 = 4_000_u64 - 100;
            deposit = if let Some(v) = deposit.checked_sub(refunded0) {
                v
            } else {
                deposit
            };
            assert_eq!(
                <ContractInfoOf<Test>>::get(&addr).unwrap().storage_deposit,
                deposit
            );

            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::Transfer {
                            from: ALICE,
                            to: addr.clone(),
                            amount: 42,
                        }),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::Transfer {
                            from: ALICE,
                            to: addr.clone(),
                            amount: charged0,
                        }),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::Reserved {
                            who: addr.clone(),
                            amount: charged0,
                        }),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::Transfer {
                            from: ALICE,
                            to: addr.clone(),
                            amount: charged1,
                        }),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::Reserved {
                            who: addr.clone(),
                            amount: charged1,
                        }),
                        topics: vec![],
                    },
                    EventRecord {
                        phase: Phase::Initialization,
                        event: Event::Balances(pallet_balances::Event::ReserveRepatriated {
                            from: addr,
                            to: ALICE,
                            amount: refunded0,
                            destination_status: BalanceStatus::Free,
                        }),
                        topics: vec![],
                    },
                ]
            );
        });
}
