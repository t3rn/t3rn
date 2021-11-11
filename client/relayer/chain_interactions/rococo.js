// Import
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

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
                    // Loop through Vec<EventRecord> to display all events
                    // Here we will get to know if we succeeded or failed.
                    // should we return something from here?

                    result.events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method === "ExtrinsicSuccess") {
                            resolve({
                                "blockHash" : result.status.asFinalized,
                                "status" : true
                            });
                        }
                        else if(method === "ExtrinsicFailed") {
                            resolve({
                                "blockHash" : result.status.asFinalized,
                                "status" : false
                            });
                        }
                    });
                    unsub();
                }

            });
    });
}

export async function getReadProof(keys, blockHash)
{

}