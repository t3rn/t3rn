use crate::{
    tests::{test_utils::set_balance, ContractsRegistry, Test},
    AccountIdOf, CodeHash, ContractInfoOf, CurrencyOf, Nonce, Storage,
};
use codec::Encode;
use frame_support::{assert_ok, traits::Currency};
use sp_core::crypto::AccountId32;
use sp_runtime::traits::Hash;
use t3rn_primitives::{
    contract_metadata::ContractMetadata,
    contracts_registry::{AuthorInfo, RegistryContract},
};

/// Extracts a contract id from an event, if $addr is some, it takes the event, otherwise it takes
/// the last event from the system.
#[cfg(test)]
#[macro_export]
macro_rules! take_created_contract_id_from_event {
    ($addr:expr) => {{
        use pallet_contracts_registry::Event as ContractsRegistryEvent;
        use $crate::System;

        let last_event = match $addr {
            Some(addr) => addr,
            None => {
                let events = System::<Test>::events();
                events.last().unwrap().clone()
            },
        };

        if let <Test as frame_system::Config>::Event::ContractsRegistry(
            ContractsRegistryEvent::ContractStored(_, addr),
        ) = last_event.event
        {
            addr.clone()
        } else {
            panic!("Unexpected event: {:?}", last_event);
        }
    }};
}

/// Since this generates a contract in the registry, needs to return the new hash
pub fn place_registry_contract(contract_address: &AccountIdOf<Test>, code_hash: CodeHash<Test>) {
    let seed = <Nonce<Test>>::mutate(|counter| {
        *counter = if let Some(v) = counter.checked_add(1) {
            v
        } else {
            *counter
        };
        *counter
    });
    let trie_id = Storage::<Test>::generate_trie_id(contract_address, seed);
    set_balance(contract_address, CurrencyOf::<Test>::minimum_balance() * 10);

    let reg_contract = create_test_registry_contract::<Test>(
        vec![],
        &code_hash,
        contract_address.to_owned(),
        None,
        None,
    );

    assert_ok!(ContractsRegistry::add_new_contract(
        crate::tests::Origin::root(),
        contract_address.clone(),
        reg_contract
    ));
    let addr = crate::take_created_contract_id_from_event!(None);

    let contract = Storage::<Test>::new_contract(contract_address, trie_id, addr).unwrap();

    <ContractInfoOf<Test>>::insert(contract_address, contract);
}

pub(crate) fn create_test_registry_contract<T: frame_system::Config>(
    wasm: Vec<u8>,
    code_hash: &<T::Hashing as Hash>::Output,
    author: AccountId32,
    author_fees: Option<u64>,
    meta: Option<ContractMetadata>,
) -> RegistryContract<CodeHash<T>, AccountId32, u64, T::BlockNumber> {
    RegistryContract::new(
        code_hash.clone().encode(),
        wasm,
        AuthorInfo::new(author, author_fees),
        None,
        vec![],
        None,
        meta.unwrap_or_default(),
    )
}

pub(crate) fn create_test_system_registry_contract<T: frame_system::Config>(
    wasm: Vec<u8>,
    code_hash: &<T::Hashing as Hash>::Output,
    author: AccountId32,
    author_fees: Option<u64>,
) -> RegistryContract<CodeHash<T>, AccountId32, u64, T::BlockNumber> {
    RegistryContract::new(
        code_hash.clone().encode(),
        wasm,
        AuthorInfo::new(author, author_fees),
        None,
        vec![],
        None,
        ContractMetadata::system_contract(),
    )
}
