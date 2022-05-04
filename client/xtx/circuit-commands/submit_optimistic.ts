import { ApiPromise } from "@polkadot/api";
import { createTestPairs } from "@polkadot/keyring/testingPairs";
const BN = require('bn.js');

export const submitOptimisticTransfer = async (api: ApiPromise, target: any[]) => {

    const keyring = createTestPairs({ type: 'sr25519' });
    console.log("From:", keyring.alice.address)
    console.log("To:", keyring.charlie.address)
    const amount = new BN(1000000000000);
    const insurance = new BN(3000000000000);
    const reward = new BN(2000000000000);
    console.log("Amount:", amount.toString());
    console.log("Amount arr:", amount.toArray("le", 16))
    console.log(target)
    return api.tx.circuit
        .onExtrinsicTrigger(
            [
                {
                    target: target, // [97, 98, 99, 100] -> registered for testing, "abcd" in bytes
                    prize: 0,
                    ordered_at: 0,
                    encoded_action: [116, 114, 97, 110], //tran
                    encoded_args: ["0xfc68ae55f42dcfd8060f1f67ec3c68a7dc3bce702f1ddb3d3551baf4e52f1a7d", "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22", amount.toArray("le", 16), insurance.toArray("le", 16).concat(reward.toArray("le", 16))],
                    signature: null,
                    enforce_executioner: null,
                }
            ],
            0, // fee must be set to 0
            false
        ).signAndSend(keyring.alice)
        .catch(err => console.error(err));
}