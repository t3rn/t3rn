import{ ApiPromise, Keyring }from'@polkadot/api';

export class Sudo {
    circuit: ApiPromise;
    signer: any;

    constructor(circuit: ApiPromise) {
        this.circuit = circuit;
        const keyring = new Keyring({ type: "sr25519" })
        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
    }

    sudoSignAndSend(transaction: any) {
        return new Promise((res, rej) => {
            return this.circuit.tx.sudo.sudo(transaction).signAndSend(this.signer, async result => {
                if (result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                    rej(result.toHuman().dispatchError)
                } else if (result.isInBlock) {
                    res(true)
                }
            })
        })
    }
}