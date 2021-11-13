// Import
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { xxhashAsHex, xxhashAsU8a } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';
import { TypeRegistry } from '@polkadot/types';

export async function submit_transfer_to_rococo_and_track(api) {
    return new Promise(async function (resolve) {

        // ToDo : Replace with real signer
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');

        console.log(`Submitting transfer`);

        // Make a transfer from Alice to BOB, waiting for inclusion
        const unsub = await api.tx.balances
            .transfer(bob.address, 12345)
            .signAndSend(alice, (result) => {
                console.log(`Current status is ${result.status}`);
                let isSuccess = false;

                if (result.status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);

                    result.events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method === "ExtrinsicSuccess") {
                            resolve({
                                "blockHash": result.status.asFinalized,
                                "status": true
                            });
                        }
                        else if (method === "ExtrinsicFailed") {
                            resolve({
                                "blockHash": result.status.asFinalized,
                                "status": false
                            });
                        }
                    });
                    unsub();

                }

            });
    });
}

function generateSystemEventKey() {
    // lets prepare the storage key for system events.
    let module_hash = xxhashAsU8a("System", 128)
    let storage_value_hash = xxhashAsU8a("Events", 128);

    // Special syntax to concatenate Uint8Array
    let final_key = new Uint8Array([
        ...module_hash,
        ...storage_value_hash
    ]);

    return u8aToHex(final_key);

}

export async function getProofs(api, blockHash) {
    let key = generateSystemEventKey()
    console.log(key);
    let proofs = await api.rpc.state.getReadProof([key], blockHash);
    console.log("PROOF");
    console.log(proofs.proof.toJSON());

    const registry = new TypeRegistry();
    return registry.createType('Vec<Bytes>', proofs.proof.toJSON());
}