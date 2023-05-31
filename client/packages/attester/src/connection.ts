import { Sdk, ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { Prometheus } from './prometheus'
import { logger } from './logging'

export class Connection {
    client: ApiPromise
    provider: WsProvider
    rpc1: any
    usingPrimaryRpc = true
    rpc2: any
    isActive = false
    sdk: Sdk | undefined
    signer: any
    prometheus: Prometheus
    target: string

    constructor(
        rpc1: any,
        rpc2: any,
        prometheus: Prometheus,
        target: string,
        substratePrivateKey: string
    ) {
        this.rpc1 = rpc1
        this.rpc2 = rpc2
        this.usingPrimaryRpc = true
        this.prometheus = prometheus
        this.target = target
        const keyring = new Keyring({ type: 'sr25519' })
        this.signer = keyring.addFromMnemonic(substratePrivateKey)
    }

    async connect() {
        // eslint-disable-next-line no-constant-condition
        while (true) {
            try {
                this.provider = this.createProvider()
                await this.setListeners()
                break
            } catch (e) {
                this.prometheus.circuitDisconnectsTotal.inc({
                    target: this.target,
                })
                this.usingPrimaryRpc = !this.usingPrimaryRpc // toggle connection
                logger.warn(
                    { ws: this.currentProvider().ws },
                    `Retrying in 2 second `
                )
                await new Promise((resolve, _reject) =>
                    setTimeout(resolve, 2000)
                )
            }
        }
    }

    async setListeners() {
        return new Promise((resolve, reject) => {
            this.provider.on('connected', async () => {
                this.isActive = true
                logger.info({ ws: this.currentProvider().ws }, `Connected`)
                const sdk = new Sdk(this.provider, this.signer)
                this.sdk = sdk
                this.client = await sdk.init()
            })

            this.provider.on('disconnected', () => {
                this.isActive = false
                logger.info({ ws: this.currentProvider().ws }, `Disconnected`)
                this.provider.disconnect()
                if (this.client) {
                    this.client.disconnect()
                }
                reject()
            })

            this.provider.on('error', () => {
                this.isActive = false
                this.prometheus.circuitActive = false
                logger.error(
                    { ws: this.currentProvider().ws },
                    `Connection error`
                )
                this.provider.disconnect()
                if (this.client) {
                    this.client.disconnect()
                }
                reject()
            })
        })
    }

    currentProvider(): any {
        return this.usingPrimaryRpc ? this.rpc1 : this.rpc2
    }

    createProvider() {
        return new WsProvider(
            this.usingPrimaryRpc ? this.rpc1.ws : this.rpc2.ws
        )
    }
}
