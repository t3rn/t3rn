import { Emitter } from './../utils/types';
import { ApiPromise } from '@polkadot/api';
import { NewSideEffectsAvailableEvent } from '../utils/types';

export async function monitorCircuitEvents(circuitApi: ApiPromise, gatewayApi: ApiPromise, emitter: Emitter) {
  // Subscribe to system events
  circuitApi.query.system.events((events) => {
    // NewSideEffectsAvailable: AugmentedEvent<ApiType, [AccountId, XtxId, Vec<SideEffect>]>;

    events.forEach((record) => {
      // Extract the phase, event and the event types
      const { event } = record;
      const types = event.typeDef;
      if (event.method === 'NewSideEffectsAvailable') {
        let parsed = <NewSideEffectsAvailableEvent>{};
        // parse out this particular event
        for (let index = 0; index < event.data.length; index++) {
          switch (types[index].type) {
            case 'AccountId':
              parsed.requester = circuitApi.createType('AccountId', event.data[index]);
              break;
            case 'XtxId':
              parsed.xtx_id = circuitApi.createType('XtxId', event.data[index]);
              break;
            case 'Vec<SideEffect>':
              parsed.sideEffects = circuitApi.createType('Vec<SideEffect>', event.data[index]);
              break;
          }
        }

        // raise event for the parsed payload
        emitter.emitSideEffect(parsed, circuitApi, gatewayApi);
      }
    });
  });
}
