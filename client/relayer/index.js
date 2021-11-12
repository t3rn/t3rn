import { fetchRococoEvents } from './event_fetch/rococo.js';
import { fetchCircuitEvents } from './event_fetch/circuit.js';
import { submit_transfer_to_rococo_and_track, getProofs } from './chain_interactions/rococo.js';
import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
    const rococoProvider = new WsProvider('wss://rococo-rpc.polkadot.io');
    const rococoApi = await ApiPromise.create({ provider: rococoProvider });

    const circuitProvider = new WsProvider('ws://localhost:9944');
    const circuitApi = await ApiPromise.create({ provider: circuitProvider });

    fetchRococoEvents(rococoApi);
    fetchCircuitEvents(circuitApi);

    // sending tx to circuit for testing since sending to rococo needs money

    let { blockHash, status } = await submit_transfer_to_rococo_and_track(circuitApi);
    if (status) {
        // the transfer was successful. Hence we now want to get the proofs and stuff
        let proofs_in_vec_of_bytes = await getProofs(circuitApi, blockHash);
        // console.log(proofs_in_vec_of_bytes);
    }
    else {
        throw "Transfer was unsuccessful. Cant move forward";
    }
}

main().catch((error) => {
    console.error(error);
    process.exit(-1);
});