// The evm-mapping pallet is inspired by evm mapping designed by AcalaNetwork

// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(test)]
use super::*;

use frame_support::assert_ok;

use crate::Event;
use circuit_mock_runtime::{
    alice, bob, eth, sig, AccountMapping, Balances, ExtBuilder, Runtime, RuntimeEvent,
    RuntimeOrigin, System, ALICE, BOB,
};

use hex_literal::hex;

#[test]
fn claim_account_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Balances::force_set_balance(
            RuntimeOrigin::root(),
            sp_runtime::MultiAddress::Id(ALICE),
            100000
        ));

        let signature_of_evm_address_as_message = sig(&alice(), &hex!("8097c3C354652CB1EEed3E5B65fBa2576470678A").encode(), &[][..]);

        // Log signature as hex string
        println!("Address Bytes: {:?}", &hex!("8097c3C354652CB1EEed3E5B65fBa2576470678A").encode());
        println!("Message: {:?}", hex::encode( &hex!("8097c3C354652CB1EEed3E5B65fBa2576470678A").encode()));
        println!("Signature: {:?}", hex::encode(signature_of_evm_address_as_message.encode()));

        assert_ok!(AccountMapping::claim_eth_account(
			RuntimeOrigin::signed(ALICE),
			hex!("8097c3C354652CB1EEed3E5B65fBa2576470678A").into(),
            signature_of_evm_address_as_message
		));

        let system_event = System::events();
        let last_system_event = system_event.last();
        assert_eq!(last_system_event.is_some(), true);
        assert_eq!(
            last_system_event.unwrap().event,
            RuntimeEvent::AccountMapping(
                circuit_runtime_pallets::pallet_3vm_account_mapping::Event::<Runtime>::ClaimAccount {
                    account_id: ALICE,
                    evm_address: eth(&alice()),
                }
            )
        );
        assert_eq!(AccountMapping::accounts(eth(&alice())).is_some(), true);
        assert_eq!(AccountMapping::evm_addresses(ALICE).is_some(), true);
        assert_eq!(AccountMapping::evm_addresses(ALICE), Some(hex!("8097c3C354652CB1EEed3E5B65fBa2576470678A").into()));
    });
}
#[test]
fn default_evm_account_dereviation_is_compatible_with_eth_standards() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Balances::force_set_balance(
            RuntimeOrigin::root(),
            sp_runtime::MultiAddress::Id(ALICE),
            100000
        ));
        assert_ok!(AccountMapping::claim_default_account(
			RuntimeOrigin::signed(ALICE),
		));
        let system_event = System::events();
        let last_system_event = system_event.last();
        assert_eq!(last_system_event.is_some(), true);
        assert_eq!(
            last_system_event.unwrap().event,
            RuntimeEvent::AccountMapping(
                circuit_runtime_pallets::pallet_3vm_account_mapping::Event::<Runtime>::ClaimAccount {
                    account_id: ALICE,
                    evm_address: hex!("0101010101010101010101010101010101010101").into(),
                }
            )
        );
        assert_eq!(AccountMapping::evm_addresses(ALICE), Some(hex!("0101010101010101010101010101010101010101").into()));
    });
}
