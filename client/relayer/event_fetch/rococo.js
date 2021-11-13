// Import
import { ApiPromise, WsProvider } from '@polkadot/api';

export async function fetchRococoEvents(api) {

    // Subscribe to system events via storage
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events from Rococo:`);

        // // Loop through the Vec<EventRecord>
        // events.forEach((record) => {
        //     // Extract the phase, event and the event types
        //     const { event, phase } = record;
        //     const types = event.typeDef;

        //     // Show what we are busy with
        //     console.log(`\t${event.section}:${event.method}`);

        //     // Loop through each of the parameters, displaying the type and data
        //     event.data.forEach((data, index) => {
        //         console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
        //     });
        // });
    });
}