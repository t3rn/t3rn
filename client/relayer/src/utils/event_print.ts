import { EventRecord } from '@polkadot/types/interfaces/system';

export function print_events(events : EventRecord[])
{
    events.forEach((record: { event: any; phase: any; }) => {
        // Extract the phase, event and the event types
        const { event, phase } = record;
        const types = event.typeDef;

        console.log(`\t${event.section}:${event.method}`);
        event.data.forEach((data: { toString: () => any; }, index: string | number) => {
          console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
        });
      });
}