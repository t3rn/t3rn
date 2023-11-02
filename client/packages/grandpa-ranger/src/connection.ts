import { Sdk, ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'
import { Prometheus } from './prometheus'
import { logger } from './logging'

export class Connection {
  client: ApiPromise
  provider: WsProvider
  rpc1: any
  usingPrimaryRpc: boolean = true
  rpc2: any
  isCircuit: boolean
  isActive: boolean = false
  sdk: Sdk | undefined
  signer: any
  prometheus: Prometheus
  target: string
  endpoint: string

  constructor(
    rpc1: any,
    rpc2: any,
    isCircuit: boolean,
    prometheus: Prometheus,
    target: string,
  ) {
    this.rpc1 = rpc1
    this.rpc2 = rpc2
    this.usingPrimaryRpc = true
    this.endpoint = this.rpc1.ws

    this.isCircuit = isCircuit
    this.prometheus = prometheus
    this.target = target
    const keyring = new Keyring({ type: 'sr25519' })
    if (isCircuit) {
      this.signer =
        process.env.CIRCUIT_SIGNER_KEY === undefined
          ? keyring.addFromUri('//Alice')
          : keyring.addFromMnemonic(process.env.CIRCUIT_SIGNER_KEY)
      logger.info(`Using signer ${this.signer.address} for ${this.rpc1.ws}`)
    }
  }

  async connect() {
    this.provider = this.createProvider()
    this.setListeners()

    const keepAlive = async () => {
      try {
        await this.client.rpc.system.health()
        logger.debug({ endpoint: this.endpoint }, 'Connection Alive')
      } catch (err) {
        logger.error({ error: err.message }, 'Connection error')
        this.usingPrimaryRpc = !this.usingPrimaryRpc
        this.prometheus.disconnects.inc({
          target: this.target,
          endpoint: this.endpoint,
        })
      }

      setTimeout(keepAlive, 5000)
    }

    // Start the keep-alive mechanism
    keepAlive()
  }

  async setListeners() {
    const connect = async () => {
      try {
        this.isActive = true
        logger.info(`Connected to ${this.currentProvider().ws}`)

        const sdk = new Sdk(this.provider, this.signer)
        this.sdk = sdk
        if (this.isCircuit) {
          this.client = await sdk.init()
        } else {
          this.client = await ApiPromise.create({ provider: this.provider })
          // We can only subscribe to new blocks on the target
          this.client.derive.chain.subscribeNewHeads(header => {
            this.prometheus.height.set(
              { target: this.target },
              header.number.toNumber(),
            )
          })
        }
      } catch (error) {
        // Handle connection error
        this.isActive = false
        logger.warn(`Error from ${this.currentProvider().ws}: ${error}`)

        // Add a delay before attempting to reconnect (adjust as needed)
        await new Promise(resolve => setTimeout(resolve, 5000))

        // Attempt reconnection
        connect()
      }
    }

    return new Promise((resolve, reject) => {
      this.provider.on('connected', connect)

      this.provider.on('disconnected', () => {
        this.isActive = false
        logger.warn(`Disconnected from provider ${this.currentProvider().ws}`)

        // Add a delay before attempting to reconnect (adjust as needed)
        setTimeout(() => {
          connect()
        }, 5000)
      })

      this.provider.on('error', () => {
        this.isActive = false
        logger.warn(`Error from provider ${this.currentProvider().ws}`)

        // Add a delay before attempting to reconnect (adjust as needed)
        setTimeout(() => {
          connect()
        }, 5000)
      })
    })
  }

  currentProvider(): any {
    // logger.debug(this.usingPrimaryRpc ? this.rpc1 : this.rpc2)
    // logger.debug(this.usingPrimaryRpc)
    // logger.debug(this.rpc2)
    return this.usingPrimaryRpc ? this.rpc1 : this.rpc2
  }

  createProvider() {
    logger.debug(
      `Current provider ${this.usingPrimaryRpc ? this.rpc1.ws : this.rpc2.ws}`,
    )
    return new WsProvider(this.usingPrimaryRpc ? this.rpc1.ws : this.rpc2.ws)
  }
}
