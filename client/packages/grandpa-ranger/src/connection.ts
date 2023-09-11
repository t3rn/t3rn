import { Sdk, ApiPromise, WsProvider, Keyring } from "@t3rn/sdk"
import { Prometheus } from "./prometheus"
import { logger } from "./logging"

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
    target: string
  ) {
    this.rpc1 = rpc1
    this.rpc2 = rpc2
    this.usingPrimaryRpc = true
    this.endpoint = this.rpc1.ws

    this.isCircuit = isCircuit
    this.prometheus = prometheus
    this.target = target
    const keyring = new Keyring({ type: "sr25519" })
    if (isCircuit) {
      this.signer =
        process.env.CIRCUIT_SIGNER_KEY === undefined
          ? keyring.addFromUri("//Alice")
          : keyring.addFromMnemonic(process.env.CIRCUIT_SIGNER_KEY)
      logger.info(`Using signer ${this.signer.address} for ${this.rpc1.ws}`)
    }
  }

  async connect() {
    this.provider = this.createProvider()
    await this.setListeners()

    logger.error(1)

    const keepAlive = async () => {
      try {
        await this.client.rpc.system.health()
      } catch (error) {
        logger.error({ error }, "Connection error")
        this.usingPrimaryRpc = !this.usingPrimaryRpc
        this.prometheus.disconnects.inc({
          target: this.target,
          endpoint: this.endpoint,
        })
      }

      setTimeout(keepAlive, 2000)
    }

    // Start the keep-alive mechanism
    keepAlive()
  }

  async setListeners() {
    return new Promise((resolve, reject) => {
      this.provider.on("connected", async () => {
        this.isActive = true
        this.isCircuit
          ? (this.prometheus.circuitActive = true)
          : (this.prometheus.targetActive = true)
        logger.info(`Connected to ${this.currentProvider().ws}`)

        const sdk = new Sdk(this.provider, this.signer)
        this.sdk = sdk
        this.isCircuit
          ? (this.client = await sdk.init())
          : (this.client = await ApiPromise.create({ provider: this.provider }))

        this.client.derive.chain.subscribeNewHeads(header => {
          this.prometheus.height.set(header.number.toNumber())
        })
      })

      this.provider.on("disconnected", () => {
        this.isActive = false
        this.isCircuit
          ? (this.prometheus.circuitActive = false)
          : (this.prometheus.targetActive = false)
        logger.warn(`Disconnected from ${this.currentProvider().ws}`)
        this.provider.disconnect()
        if (this.client) {
          this.client.disconnect()
        }
        reject()
      })

      this.provider.on("error", () => {
        this.isActive = false
        this.isCircuit
          ? (this.prometheus.circuitActive = false)
          : (this.prometheus.targetActive = false)
        logger.warn(`Error from ${this.currentProvider().ws}`)
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
