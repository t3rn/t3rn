// import { fetchRococoEvents } from './event_fetch/rococo';
import { fetchCircuitEvents } from './event_fetch/circuit';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { types } from '@t3rn/types';
import { Emitter } from './utils/types';
import { send_tx_confirm_side_effect } from './chain_interactions/circuit';
import { executionRouter } from './utils/sideEffectRouter';
import { TypeRegistry } from '@polkadot/types';

const eventEmitter = new Emitter();
eventEmitter.on('NewSideEffect', async (payload, rococoApi) => {
  await executionRouter(payload, rococoApi);
});

const circuitProvider = new WsProvider('ws://localhost:9944');
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
  // const rococoProvider = new WsProvider('wss://rococo-rpc.polkadot.io');
  // const rococoApi = await ApiPromise.create({ provider: rococoProvider });
  // fetchRococoEvents(rococoApi);

  fetchCircuitEvents(api, eventEmitter);

  // let { blockHash, status } = await send_tx_confirm_side_effect(circuitApi, proofs_in_vec_of_bytes);
}
