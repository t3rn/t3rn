import{ ApiPromise }from'@polkadot/api';

export class CircuitRelayer {
    circuit: ApiPromise;
    signer: any;

    constructor(circuit: ApiPromise, signer: any) {
        this.circuit = circuit;
        this.signer = signer;
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

    async submitHeaders(args: any[]) {
        const nonce = await this.fetchNonce(this.signer.address)
        return new Promise(async (res, rej) => {
            await this.circuit.tx.utility
                .batch(
                    args.map((arg: any) => {
                        return this.circuit.tx.portal.submitHeaders(
                            arg.gatewayId,
                            arg.data.toHex() // we submit in encoded form to portal
                        )
                    })
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