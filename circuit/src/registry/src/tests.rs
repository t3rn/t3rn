use crate::{mock::*, ContractRegistry, Error, Event as RegistryEvent};
use frame_support::{assert_err, assert_noop, assert_ok, StorageDoubleMap};
use frame_system::{EventRecord, Phase};
use sp_runtime::traits::BadOrigin;

// NOTE
// Using `run_to_block(2)` cos block#1 never includes events.

#[test]
fn it_stores_a_contract_in_the_registry() {
    new_test_ext().execute_with(|| {
        run_to_block(2);

        assert_ok!(Registry::store(
            Origin::root(),
            REQUESTER,
            contract_name(),
            contract(),
        ));

        assert_eq!(
            Registry::get(REQUESTER, contract_name_hash()),
            Some(contract())
        );

        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: Event::pallet_registry(
                    RegistryEvent::<Test>::ContractStored(
                        REQUESTER,
                        contract_name(),
                    )
                ),
                topics: vec![],
            }]
        );
    });
}

#[test]
fn it_fails_to_store_a_contract_if_its_key_already_exists() {
    new_test_ext().execute_with(|| {
        run_to_block(2);

        assert_ok!(Registry::store(
            Origin::root(),
            REQUESTER,
            contract_name(),
            contract(),
        ));

        assert_noop!(
            Registry::store(
                Origin::root(),
                REQUESTER,
                contract_name(),
                contract(),
            ),
            Error::<Test>::KeyAlreadyExists
        );

        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: Event::pallet_registry(
                    RegistryEvent::<Test>::ContractStored(
                        REQUESTER,
                        contract_name(),
                    )
                ),
                topics: vec![],
            }]
        );
    });
}

#[test]
fn it_purges_a_contract_from_the_registry() {
    new_test_ext().execute_with(|| {
        run_to_block(2);

        assert_ok!(Registry::store(
            Origin::root(),
            REQUESTER,
            contract_name(),
            contract(),
        ));

        assert_ok!(Registry::purge(
            Origin::root(),
            REQUESTER,
            contract_name()
        ));

        assert_eq!(Registry::get(REQUESTER, contract_name_hash()), None);

        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::pallet_registry(
                        RegistryEvent::<Test>::ContractStored(
                            REQUESTER,
                            contract_name(),
                        )
                    ),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::pallet_registry(
                        RegistryEvent::<Test>::ContractPurged(
                            REQUESTER,
                            contract_name(),
                        )
                    ),
                    topics: vec![],
                },
            ]
        );
    });
}

#[test]
fn it_fails_to_purge_a_contract_if_its_key_does_not_exist() {
    new_test_ext().execute_with(|| {
        run_to_block(2);

        assert_ok!(Registry::store(
            Origin::root(),
            REQUESTER,
            contract_name(),
            contract(),
        ));

        assert_ok!(Registry::purge(
            Origin::root(),
            REQUESTER,
            contract_name()
        ));

        assert_noop!(
            Registry::purge(
                Origin::root(),
                REQUESTER,
                contract_name(),
            ),
            Error::<Test>::KeyDoesNotExist
        );

        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::pallet_registry(
                        RegistryEvent::<Test>::ContractStored(
                            REQUESTER,
                            contract_name(),
                        )
                    ),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: Event::pallet_registry(
                        RegistryEvent::<Test>::ContractPurged(
                            REQUESTER,
                            contract_name(),
                        )
                    ),
                    topics: vec![],
                },
            ]
        );
    });
}

#[test]
fn it_stores_contracts_separately_per_requester() {
    assert_ne!(
        <ContractRegistry<Test>>::hashed_key_for(
            REQUESTER,
            contract_name_hash()
        ),
        <ContractRegistry<Test>>::hashed_key_for(
            ANOTHER_REQUESTER,
            contract_name_hash()
        )
    );
}

#[test]
fn it_gets_none_for_non_existing_contracts() {
    new_test_ext().execute_with(|| {
        assert_eq!(Registry::get(REQUESTER, contract_name_hash()), None);
    });
}

#[test]
fn it_fails_for_non_root_origins() {
    new_test_ext().execute_with(|| {
        run_to_block(2);

        assert_err!(
            Registry::store(
                Origin::signed(419),
                REQUESTER,
                contract_name(),
                contract(),
            ),
            BadOrigin
        );

        assert_err!(
            Registry::store(
                Origin::none(),
                REQUESTER,
                contract_name(),
                contract(),
            ),
            BadOrigin
        );

        assert_err!(
            Registry::purge(
                Origin::signed(419),
                REQUESTER,
                contract_name(),
            ),
            BadOrigin
        );

        assert_err!(
            Registry::purge(
                Origin::none(),
                REQUESTER,
                contract_name(),
            ),
            BadOrigin
        );

        assert_eq!(System::events(), vec![]);
    });
}
