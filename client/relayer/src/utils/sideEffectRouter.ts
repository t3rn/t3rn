import { ApiPromise } from '@polkadot/api';
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

import { getProofs, submit_transfer } from '../chain_interactions/rococo';
import { parseTransferArguments } from './argumentParse';
import { NewSideEffectsAvailableEvent } from './types';
export async function executionRouter(payload: NewSideEffectsAvailableEvent, api: ApiPromise) {
    // we have xtx_id and account outside which are common.
    console.log("I am inside execution router");
    for (let index = 0; index < payload.sideEffects.length; index++) {
        // here we parse what the side effect is and what we have to do.
        let item = payload.sideEffects[index];
        console.log(item);
        switch (item.encoded_action.toString()) {
            case "transfer":
                console.log("Execution Router : I am inside transfer");
                let parameters = parseTransferArguments(item.encoded_args);
                await submit_transfer(api, parameters).then(
                    async result => {
                        if (result.status) {
                            console.log("Transfer success");
                            let proofs_in_vec_of_bytes = await getProofs(api, result.blockHash);
                            // let { blockHash, status } = await send_tx_confirm_side_effect(circuitApi, proofs_in_vec_of_bytes);
                            // console.log(proofs_in_vec_of_bytes);
                        }
                        else {
                            console.log("Transfer failed");
                        }
                    }
                );
                break;
            case "getStorage":
                break;
        }
    }
    console.log("Execution end");

}