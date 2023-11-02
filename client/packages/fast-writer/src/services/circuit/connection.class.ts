import { ApiPromise, Keyring, Sdk, WsProvider } from '@t3rn/sdk'
import { logger } from '../../utils/logger'
import { Prometheus } from '../../prometheus'

export class CircuitConnection {
  client: ApiPromise
  provider: WsProvider
  rpc1: any
  usingPrimaryRpc: boolean = true
  rpc2: any
  isActive: boolean = false
  sdk: Sdk
  signer: any
  prometheus: Prometheus

  constructor(
    rpc1: any,
    rpc2: any,
    signer: string,
    prometheus: Prometheus,
  ) {
    this.rpc1 = rpc1
    this.rpc2 = rpc2
    this.usingPrimaryRpc = true
    const keyring = new Keyring({ type: 'sr25519' })
    this.signer = keyring.addFromMnemonic(signer)
    this.prometheus = prometheus
  }

  async connect() {
    while (true) {
      try {
        this.provider = this.createProvider()
        await this.setListeners()
        break
      } catch (e) {
        this.usingPrimaryRpc = !this.usingPrimaryRpc // toggle connection
        const sleepSecs = 2
        logger.info(
          `Retrying in ${sleepSecs} second with ${this.currentProvider().ws}`,
        )

        await new Promise((resolve, _reject) => setTimeout(resolve, sleepSecs))
      }
    }
    return new Promise((resolve, _reject) => resolve)
  }

  async setListeners() {
    return new Promise((resolve, reject) => {
      this.provider.on('connected', async () => {
        this.isActive = true
        logger.info(`Connected to ${this.currentProvider().ws}`)

        const sdk = new Sdk(this.provider, this.signer)
        this.sdk = sdk
        this.client = await sdk.init()
        // TODO: change this to event emission to notify of successful connection.
        //  This way we will avoid ambiguous errors of connection not yet established
        //  before we start submitting epochs. For reference: start() and connectClients()
        //  in EthereumRanger class.
      })

      this.provider.on('disconnected', () => {
        this.prometheus.disconnects.inc({
          endpoint: this.currentProvider().ws,
        })

        this.isActive = false
        logger.warn(`Disconnected from ${this.currentProvider().ws}`)

        this.provider.disconnect()
        if (this.client) {
          this.client.disconnect()
        }
        reject()
      })

      this.provider.on('error', (err) => {
        this.isActive = false
        logger.error(
          { error: err.message },
          `Error from ${this.currentProvider().ws}`,
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
    return new WsProvider(this.usingPrimaryRpc ? this.rpc1.ws : this.rpc2.ws)
  }
}
