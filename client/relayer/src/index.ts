import { ApiPromise, WsProvider } from '@polkadot/api';
import { types } from '@t3rn/types';
import { monitorCircuitEvents } from './event_fetch/circuit';
import { Emitter } from './utils/types';
import { executionRouter } from './utils/sideEffectRouter';

const eventEmitter = new Emitter();
eventEmitter.on('NewSideEffect', async (payload, rococoApi) => {
  await executionRouter(payload, rococoApi);
});

const circuitProvider = new WsProvider(process.env.CIRCUIT_WS_URL);
ApiPromise.create({
  provider: circuitProvider,
  types,
})
  .then((api) => {
    main(api).catch((error) => {
      console.error(error);
      process.exit(1);
    });
  })
  .catch((error) => {
    console.log('Failed to connect to the circuit');
    process.exit(1);
  });

async function main(api: ApiPromise) {
  monitorCircuitEvents(api, eventEmitter);
}
