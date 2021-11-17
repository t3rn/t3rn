// import { fetchRococoEvents } from './event_fetch/rococo';
import { fetchCircuitEvents } from './event_fetch/circuit';
import { submit_transfer_to_rococo_and_track, getProofs } from './chain_interactions/rococo';
import { ApiPromise, WsProvider } from '@polkadot/api';
import * as definitions from '@t3rn/types';
import { Emitter } from './utils/types';
import { send_tx_confirm_side_effect } from './chain_interactions/circuit';

const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});

const eventEmitter = new Emitter();
eventEmitter.on("NewSideEffect", (payload) => {
    console.log("I was triggered. Here is the object");
    console.log(payload.xtx_id);
});

async function main() {
    // const rococoProvider = new WsProvider('wss://rococo-rpc.polkadot.io');
    // const rococoApi = await ApiPromise.create({ provider: rococoProvider });

    const circuitProvider = new WsProvider('ws://localhost:9944');
    const circuitApi : ApiPromise = await ApiPromise.create({ provider: circuitProvider, types });

    // fetchRococoEvents(rococoApi);
    fetchCircuitEvents(circuitApi, eventEmitter);

    // sending tx to circuit for testing since sending to rococo needs money

    // let test = await submit_transfer_to_rococo_and_track(circuitApi).then(
    //     async result => {
    //         if (result.status) {
    //             console.log("Transfer success");
    //             // the transfer was successful. Hence we now want to get the proofs and stuff
    //             let proofs_in_vec_of_bytes = await getProofs(circuitApi, result.blockHash);
    //             let { blockHash, status } = await send_tx_confirm_side_effect(circuitApi, proofs_in_vec_of_bytes);
    //             // console.log(proofs_in_vec_of_bytes);
    //         }
    //         else{
    //             console.log("Transfer failed");
    //         }
    //     }
    // );
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});