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
    while (true) {
      try {
        this.provider = this.createProvider()
        await this.setListeners()
        break
      } catch (e) {
        const endpoint = this.usingPrimaryRpc ? this.rpc1.ws : this.rpc2.ws
        this.isCircuit
          ? this.prometheus.circuitDisconnected.inc({
              endpoint,
              target: this.target,
            })
          : this.prometheus.targetDisconnected.inc({
              endpoint,
              target: this.target,
            })

        this.isCircuit
          ? this.prometheus.circuitDisconnectsTotal.inc(
              { target: this.target },
              1
            )
          : this.prometheus.targetDisconnectsTotal.inc(
              { target: this.target },
              1
            )

        this.usingPrimaryRpc = !this.usingPrimaryRpc // toggle connection
        logger.warn(`Retrying in 2 second with ${this.currentProvider().ws}`)
        await new Promise((resolve, _reject) => setTimeout(resolve, 2000))
      }
    }
  }

  async setListeners() {
    return new Promise((resolve, reject) => {
      this.provider.on("connected", async () => {
        this.isActive = true
        logger.info(`Connected to ${this.currentProvider().ws}`)
        if (this.isCircuit) {
          this.prometheus.circuitActive = true
          const sdk = new Sdk(this.provider, this.signer, true)
          this.sdk = sdk
          this.client = await sdk.init()
        } else {
          this.prometheus.targetActive = true
          this.client = await ApiPromise.create({
            provider: this.provider,
          })

          // update prometheus metrics with incoming blocks
          this.client.derive.chain.subscribeNewHeads(header => {
            this.prometheus.targetHeight.set(header.number.toNumber())
          })
        }
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
