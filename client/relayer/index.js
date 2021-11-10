import { fetchRococoEvents } from './event-fetch/rococo.js';
import { fetchCircuitEvents } from './event-fetch/circuit.js';

async function main() {
    fetchRococoEvents();
    fetchCircuitEvents();
}

main().catch((error) => {
    console.error(error);
    process.exit(-1);
});