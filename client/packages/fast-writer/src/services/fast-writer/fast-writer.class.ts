import { sleep } from '../../utils/helpers'
import { logger } from '../../utils/logger'
import { CircuitConnection } from '../circuit/connection.class'
import { CircuitClient } from '../circuit/client'
import { Prometheus } from '../../prometheus'
import { Config } from '../../config/config'
import { Order } from './build-tx'
import { ApiPromise } from '@t3rn/sdk/.'

export class FastWriter {
  private readonly config: Config
  private readonly circuitConnection: CircuitConnection
  private readonly prometheus: Prometheus
  private circuitClient: CircuitClient

  constructor(config: Config) {
    this.config = config
    this.prometheus = new Prometheus(this.config)
    this.circuitConnection = new CircuitConnection(
      this.config.circuit.rpc1,
      this.config.circuit.rpc2,
      this.config.circuit.signer,
      this.prometheus,
    )
  }

  async start() {
    await this.connectClients()
    await sleep(5, 'Wait for the clients to connect')

    // We need to initialize this after the Circuit connection has been established
    // because otherwise the client connection SDK is not available
    this.circuitClient = new CircuitClient(
      this.circuitConnection,
      this.prometheus,
      this.config,
    )

    this.scheduleSubmissionsToCircuit()
  }

  private async connectClients() {
    this.circuitConnection.connect()
  }

  private async fetchNonce(api: ApiPromise, address: string) {
    return await api.rpc.system.accountNextIndex(address).then((nextIndex) => {
      // @ts-ignore - property does not exist on type
      return parseInt(nextIndex.toNumber())
    })
  }

  private async scheduleSubmissionsToCircuit() {
    while (true) {
      logger.info('Starting new submission loop to Circuit')
      const nonce = await this.fetchNonce(
        this.circuitClient.sdk.client,
        this.circuitClient.sdk.signer.address,
      )

      // TODO submit SFXs
      for (const sideEffect of this.config.sideEffects) {
        const order = new Order(
          sideEffect.target,
          sideEffect.asset,
          sideEffect.targetAccount,
          sideEffect.amount * 10 ** 12,
          sideEffect.maxReward,
          sideEffect.rewardAsset,
          sideEffect.insurance,
          sideEffect.remote_origin_nonce,
          sideEffect.count,
          sideEffect.txType,
        )
        logger.info({ order }, 'Submitting SFX to Circuit')
        order.execute(this.circuitClient, order, nonce)
      }

      logger.info({}, `SFXs submission`)

      await sleep(
        this.config.intervalSeconds,
        'Waiting between SFX submissions to Circuit',
      )
    }
  }
}
