import { ApiPromise, WsProvider } from '@polkadot/api';
import { types } from '@t3rn/types';
import { monitorCircuitEvents } from './event_fetch/circuit';
import { Emitter } from './utils/types';
import { executionRouter } from './utils/sideEffectRouter';

if (process.env.NODE_ENV !== 'production') {
  require('dotenv').config();
}

const eventEmitter = new Emitter();
eventEmitter.on('NewSideEffect', async (payload, circuitApi, rococoApi) => {
  await executionRouter(payload, circuitApi, rococoApi);
});

// this process.env is not loading values
const rococoProvider = new WsProvider(process.env.ROCOCO_WS_URL);
// it works because it connects to localhost as a fallback
const circuitProvider = new WsProvider(process.env.CIRCUIT_WS_URL);

main().catch((error) => {
  console.error(error);
  process.exit(1);
});

async function main() {
  const rococoApi = await ApiPromise.create({ provider: rococoProvider });
  console.log('Rococo connected');

  const circuitApi = await ApiPromise.create({ provider: circuitProvider, types });
  console.log('Circuit connected');

  monitorCircuitEvents(circuitApi, rococoApi, eventEmitter);
}
