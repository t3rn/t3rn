import{ ApiPromise, Keyring }from'@polkadot/api';

export class CircuitRelayer {
    circuit: ApiPromise;
    signer: any;

    constructor(circuit: ApiPromise, signer: any) {
        this.circuit = circuit;
        this.signer = signer;
    }

    sudoSignAndSend(transaction: any) {
        const keyring = new Keyring({ type: "sr25519" })
        const signer: any = keyring.addFromUri("//Alice");
        return new Promise((res, rej) => {
            return this.circuit.tx.sudo.sudo(transaction).signAndSend(signer, async result => {
                if (result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                    rej(result.toHuman().dispatchError)
                } else if (result.isInBlock) {
                    const blockNumber = await this.getBlockNumber(result.status.toHuman().InBlock)
                    res(blockNumber)
                }
            })
        })
    }

    onExtrinsicTrigger(args: any) {
        return new Promise((res: any, rej: any) => {
            // console.log(args.sideEffects.toHuman())
            return this.circuit.tx.circuit
                .onExtrinsicTrigger(args.sideEffects.toHuman(), args.fee, args.sequential)
                .signAndSend(this.signer, async result => {
                    // @ts-ignore
                    if (result && result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                        // @ts-ignore
                        rej(result.toHuman().dispatchError)
                    } else if (result.isInBlock) {
                        const blockNumber = await this.getBlockNumber(result.status.toHuman().InBlock)
                        res(blockNumber)
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
                .signAndSend(this.signer, { nonce }, async result => {
                    // @ts-ignore
                    if (result && result.toHuman().dispatchError !== undefined) { // The pallet doesn't return a proper error
                        // @ts-ignore
                        rej(result.toHuman().dispatchError)
                    } else if (result.isInBlock) {
                        // res(true)
                        // @ts-ignore
                        const blockNumber = await this.getBlockNumber(result.status.toHuman().InBlock)
                        res(blockNumber)
                    }
                })
        })
    }

    async getBlockNumber(hash: string) {
        let res = await this.circuit.rpc.chain.getBlock(hash)
        return res.block.header.number.toNumber()
    }

    async fetchNonce (
        address: string
    ) {
        return this.circuit.rpc.system.accountNextIndex(address)
    }
}