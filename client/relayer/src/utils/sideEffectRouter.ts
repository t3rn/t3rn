import { Bytes } from '@polkadot/types';
import { ApiPromise } from '@polkadot/api';
import { send_tx_confirm_side_effect } from '../chain_interactions/circuit';

import { getEventProofs, getStorage, submit_transfer } from '../chain_interactions/gateway';
import { parseTransferArguments, parseStorageArguments } from './argumentParse';
import { NewSideEffectsAvailableEvent } from './types';

export async function executionRouter(
  payload: NewSideEffectsAvailableEvent,
  circuitApi: ApiPromise,
  gatewayApi: ApiPromise
) {
  console.log(`Execution start for xtx_id : ${payload.xtx_id}`);
  for (let index = 0; index < payload.sideEffects.length; index++) {
    let sideEffect = payload.sideEffects[index];
    switch (sideEffect.encoded_action.toHuman()) {
      case 'transfer':
        console.log('Execution Router : Transfer');
        let transfer_parameters = parseTransferArguments(gatewayApi, sideEffect.encoded_args);
        await submit_transfer(gatewayApi, transfer_parameters).then(async (result) => {
          if (result.status) {
            let inclusion_proofs = await getEventProofs(gatewayApi, result.blockHash);
            // only one event coming in. Access first element.
            let encoded_effect: Bytes = gatewayApi.createType('Bytes', result.events[0].event.toHex());
            let { status } = await send_tx_confirm_side_effect(
              circuitApi,
              payload.requester,
              payload.xtx_id,
              sideEffect,
              inclusion_proofs.proof[0],
              encoded_effect
            );
          } else {
            console.log('Transfer failed');
          }
        });
        break;
      case 'getStorage':
        console.log('Execution Router : getStorage');
        let getStorage_parameters = parseStorageArguments(gatewayApi, sideEffect.encoded_args);
        let storageData = await getStorage(gatewayApi, getStorage_parameters);
        console.log(storageData);

        let inclusion_proofs = gatewayApi.createType('Bytes', '');
        let encoded_effect: Bytes = gatewayApi.createType('Bytes', storageData.value);
        let { status } = await send_tx_confirm_side_effect(
          circuitApi,
          payload.requester,
          payload.xtx_id,
          sideEffect,
          inclusion_proofs,
          encoded_effect
        );
        break;
      default:
        console.log(`encoded_action : ${sideEffect.encoded_action.toHuman()} not recognized`);
    }
  }
  console.log(`Execution end for xtx_id : ${payload.xtx_id}`);
}
