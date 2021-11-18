import { Emitter } from './../utils/types';
import { Codec } from '@polkadot/types/types';
import { SideEffect } from './../../../types/src/interfaces/primitives/types';
import { ApiPromise } from '@polkadot/api';
import { NewSideEffectsAvailableEvent } from '../utils/types';

export async function fetchCircuitEvents(api: ApiPromise, emitter: Emitter) {

    // Subscribe to system events via storage
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events from Circuit:`);

        // NewSideEffectsAvailable: AugmentedEvent<ApiType, [AccountId, XtxId, Vec<SideEffect>]>;

        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event } = record;
            const types = event.typeDef;
            if (event.method === "NewSideEffectsAvailable") {

                let parsed = <NewSideEffectsAvailableEvent>{};
                // parse out this particular event
                for (let index = 0; index < event.data.length; index++) {

                    switch (types[index].type) {
                        case "AccountId":
                            parsed.requester = api.createType("AccountId", event.data[index]);
                            break;
                        case "XtxId":
                            parsed.xtx_id = api.createType("XtxId", event.data[index]);
                            break;
                        case "Vec<SideEffect>":
                            parsed.sideEffects = api.createType("Vec<SideEffect>", event.data[index]);
                            break;
                    }
                }

                // raise event for the parsed payload
                console.log("Printing the object created");
                // console.log(parsed);
                emitter.emitSideEffect(parsed, api);
            }
        });
    });
}