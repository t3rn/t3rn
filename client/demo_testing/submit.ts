import { ApiPromise } from "@polkadot/api";
import { createTestPairs } from "@polkadot/keyring/testingPairs";

export const submitTransfer = async (api: ApiPromise, target: any[]) => {

    
    const keyring = createTestPairs({ type: 'sr25519' });
    console.log({
        target: target, // [97, 98, 99, 100] -> registered for testing, "abcd" in bytes
        prize: 0,
        ordered_at: 0,
        encoded_action: [116, 114, 97, 110], //tran
        encoded_args: [keyring.alice.address, keyring.charlie.address, [1, 0, 0, 0, 0, 0, 0, 0]],
        signature: [],
        enforce_executioner: false,
    })
    return api.tx.circuit
        .onExtrinsicTrigger(
            [
                {
                    target: target, // [97, 98, 99, 100] -> registered for testing, "abcd" in bytes
                    prize: 0,
                    ordered_at: 0,
                    encoded_action: [116, 114, 97, 110], //tran
                    encoded_args: [keyring.alice.address, keyring.charlie.address, [1, 0, 0, 0, 0, 0, 0, 0]],
                    signature: [],
                    enforce_executioner: false,
                }
            ],
            0, // fee must be set to 0
            true
        ).signAndSend(keyring.alice)
        .catch(err => console.error(err));

}

