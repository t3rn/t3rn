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

use circuit_mock_runtime::{
    alice, bob, AccountMapping, ExtBuilder, Runtime, RuntimeEvent, RuntimeOrigin, System, ALICE, BOB,
    eth, sig
};
use crate::Event;

#[test]
fn claim_account_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(AccountMapping::claim_eth_account(
			RuntimeOrigin::signed(ALICE),
			eth(&alice()),
			sig(&alice(), &eth(&alice()).encode(), &[][..])
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

    });
}