import { ApiPromise } from '@polkadot/api';
import { send_tx_confirm_side_effect } from '../chain_interactions/circuit';
// this will run specific actions when we get an event.
// transfer OR getStorage
// but that is only possible if we are able to parse out inside the side effect
// but it is a vector : so they can be different things though
// so we need to fan out maybe or do we run sequentially, I think we can do sequentially for now.
// if parallel, that is easy to do that later. Right now, just important things only.

// export interface SideEffect extends Struct {
//     readonly target: ChainId;
//     readonly prize: BalanceOf;
//     readonly ordered_at: BlockNumber;
//     readonly encoded_action: Bytes;
//     readonly encoded_args: Vec<Bytes>;
//     readonly signature: Bytes;
//     readonly enforce_executioner: Option<AccountId>;
//   }

import { getEventProofs, submit_transfer } from '../chain_interactions/rococo';
import { parseTransferArguments } from './argumentParse';
import { NewSideEffectsAvailableEvent } from './types';

export async function executionRouter(payload: NewSideEffectsAvailableEvent, api: ApiPromise) {
    // we have xtx_id and account outside which are common.
    for (let index = 0; index < payload.sideEffects.length; index++) {
        let sideEffect = payload.sideEffects[index];
        switch (sideEffect.encoded_action.toHuman()) {
            case "transfer":
                console.log("Execution Router : Transfer");
                let parameters = parseTransferArguments(api, sideEffect.encoded_args);
                await submit_transfer(api, parameters).then(
                    async result => {
                        if (result.status) {
                            let proofs = await getEventProofs(api, result.blockHash);
                            let { status } = await send_tx_confirm_side_effect(
                                api, 
                                payload.requester,
                                payload.xtx_id,
                                sideEffect, 
                                proofs);
                            console.log(status);
                        }
                        else {
                            console.log("Transfer failed");
                        }
                    }
                );
                break;
            case "getStorage":
                console.log("Execution Router : I am inside getStorage");
                break;
            default:
                console.log("Default");
        }
    }
    console.log("Execution end");

}