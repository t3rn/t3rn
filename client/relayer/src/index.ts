import { ApiPromise, WsProvider } from '@polkadot/api';
import { types } from '@t3rn/types';
import { monitorCircuitEvents } from './event_fetch/circuit';
import { Emitter } from './utils/types';
import { executionRouter } from './utils/sideEffectRouter';

if (process.env.NODE_ENV !== 'production') {
  require('dotenv').config();
}

const eventEmitter = new Emitter();
eventEmitter.on('NewSideEffect', async (payload, circuitApi, gatewayApi) => {
  await executionRouter(payload, circuitApi, gatewayApi);
});

const westendProvider = new WsProvider(process.env.WESTEND_WS_URL);
const circuitProvider = new WsProvider(process.env.CIRCUIT_WS_URL);

main().catch((error) => {
  console.error(error);
  process.exit(1);
});

async function main() {
  const gatewayApi = await ApiPromise.create({ provider: westendProvider });
  console.log('Gateway connected');

  const circuitApi = await ApiPromise.create({ provider: circuitProvider, types });
  console.log('Circuit connected');

  monitorCircuitEvents(circuitApi, gatewayApi, eventEmitter);
}
