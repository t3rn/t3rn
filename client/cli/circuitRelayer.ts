import{ ApiPromise, Keyring }from'@polkadot/api';

export class CircuitRelayer {
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

    onExtrinsicTrigger(args: any[]) {
        return new Promise((res: any, rej: any) => {
            return this.circuit.tx.circuit
                .onExtrinsicTrigger(...args)
                .signAndSend(this.signer, async result => {
                    // @ts-ignore
                    if (result && result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                        // @ts-ignore
                        rej(result.toHuman().dispatchError)
                    } else if (result.isInBlock) {
                        res(true)
                    }
                })
        })
    }

    submitHeader(args: any[]) {
        return new Promise((res: any, rej: any) => {
            return this.circuit.tx.portal
                .submitHeader(...args)
                .signAndSend(this.signer, async result => {
                    // @ts-ignore
                    if (result && result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                        // @ts-ignore
                        rej(result.toHuman().dispatchError)
                    } else if (result.isInBlock) {
                        res(true)
                    }
                })
        })
    }
}