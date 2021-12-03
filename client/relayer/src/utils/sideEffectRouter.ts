import { Bytes } from '@polkadot/types';
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

import { getEventProofs, getStorage, submit_transfer } from '../chain_interactions/rococo';
import { parseTransferArguments, parseStorageArguments } from './argumentParse';
import { NewSideEffectsAvailableEvent } from './types';

export async function executionRouter(payload: NewSideEffectsAvailableEvent, api: ApiPromise) {
    // we have xtx_id and account outside which are common.
    for (let index = 0; index < payload.sideEffects.length; index++) {
        let sideEffect = payload.sideEffects[index];
        switch (sideEffect.encoded_action.toHuman()) {
            case "transfer":
                console.log("Execution Router : Transfer");
                let transfer_parameters = parseTransferArguments(api, sideEffect.encoded_args);
                await submit_transfer(api, transfer_parameters).then(
                    async result => {
                        if (result.status) {
                            let inclusion_proofs = await getEventProofs(api, result.blockHash);
                            let encoded_effect: Bytes = api.createType('Bytes', 'test');
                            let { status } = await send_tx_confirm_side_effect(
                                api,
                                payload.requester,
                                payload.xtx_id,
                                sideEffect,
                                inclusion_proofs.proof[0],
                                encoded_effect);
                        }
                        else {
                            console.log("Transfer failed");
                        }
                    }
                );
                break;
            case "getStorage":
                console.log("Execution Router : getStorage");
                let getStorage_parameters = parseStorageArguments(api, sideEffect.encoded_args);
                let storageData = await getStorage(api, getStorage_parameters);
                console.log(storageData);

                let inclusion_proofs = api.createType('Bytes', '');
                let encoded_effect: Bytes = api.createType('Bytes', storageData.value);
                let { status } = await send_tx_confirm_side_effect(
                    api,
                    payload.requester,
                    payload.xtx_id,
                    sideEffect,
                    inclusion_proofs,
                    encoded_effect);
                break;
            default:
                console.log("Default");
        }
    }
    console.log("Execution end");

}