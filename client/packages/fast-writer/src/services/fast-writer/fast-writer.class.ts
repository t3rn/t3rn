import { cryptoWaitReady } from '@t3rn/sdk'
import { sleep } from '../../utils/helpers.js'
import { logger } from '../../utils/logger.js'
import { CircuitConnection } from '../circuit/connection.class.js'
import { CircuitClient } from '../circuit/client.js'
import { Prometheus } from '../../prometheus.js'
import { Config } from '../../config/config.js'

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
    const cryptoIsReady = await cryptoWaitReady()
    if (!cryptoIsReady) {
      throw new Error('Crypto WASM lib is not ready')
    }

    this.circuitConnection.connect()
  }

  private async scheduleSubmissionsToCircuit() {
    while (true) {
      logger.info('Starting new submission loop to Circuit')

      // TODO submit SFXs

      logger.info(
        { },
        `Submission for next slot`,
      )

      await sleep(
        this.config.intervalSeconds,
        'Waiting between SFX submissions to Circuit',
      )
    }
  }


}
