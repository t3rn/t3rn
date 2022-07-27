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

    async submitHeaders(gatewayId: string, args: any[]) {
        const nonce = await this.fetchNonce(this.signer.address)
        return new Promise(async (res, rej) => {
            await this.circuit.tx.utility
                .batch(
                    args.map((arg: any) =>
                        this.circuit.tx.portal.submitHeaders(
                            gatewayId,
                            arg.toHex() // we submit in encoded form to portal
                        )
                    )
                )
                .signAndSend(this.signer, { nonce }, result => {
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

    async fetchNonce (
        address: string
    ) {
        return this.circuit.rpc.system.accountNextIndex(address)
    }
}