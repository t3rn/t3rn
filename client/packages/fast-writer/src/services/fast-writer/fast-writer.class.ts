import { sleep } from '../../utils/helpers'
import { logger } from '../../utils/logger'
import { CircuitConnection } from '../circuit/connection.class'
import { CircuitClient } from '../circuit/client'
import { Prometheus } from '../../prometheus'
import { Config } from '../../config/config'
import { Order } from './send-tx'
import { ApiPromise } from '@t3rn/sdk/.'
import { TxType } from './enums'

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
    this.prometheus.interval.set(this.config.intervalSeconds)
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
    // eslint-disable-next-line no-constant-condition
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
          sideEffect.amount,
          sideEffect.maxReward,
          sideEffect.rewardAsset,
          sideEffect.insurance,
          sideEffect.remote_origin_nonce,
          sideEffect.count,
          sideEffect.txType,
        )
        logger.info({ order }, 'Submitting SFX to Circuit')
        await this.execute(order, nonce)
      }

      await sleep(
        this.config.intervalSeconds,
        'Waiting between SFX submissions to Circuit',
      )
    }
  }

  execute(order: Order, nonce) {
    if (order.txType == TxType.Single) {
      this.submitSingleOrder(order, nonce)
    } else if (order.txType == TxType.Batch) {
      this.submitBatchOrder(order, nonce)
    } else {
      logger.error('Invalid txType', order.txType)
    }
  }

  async submitBatchOrder(
    order: Order,
    nonce: number,
    speedMode: number = 1,
  ) {
    const transactions = []
    const sdk = this.circuitClient.sdk

    for (let i = 0; i < order.count; i++) {
      // amount is increased by 1 for each order to avoid SetupFailedDuplicatedXtx
      const transaction = this.circuitClient.sdk.client.tx.vacuum.singleOrder(
        order.target,
        order.asset,
        order.amount + i,
        order.rewardAsset,
        order.maxReward,
        order.insurance,
        order.targetAccount,
        speedMode,
      )
      transactions.push(transaction as never)
    }

    async function customSignAndSend() {
      try {
        const tx = sdk.circuit.tx.createBatch(transactions)
        const res = await sdk.circuit.tx.signAndSend(tx, { nonce })
        logger.info({ res }, `Transaction included in block ${res}`)
      } catch (e) {
        logger.error(`signAndSend failed with error: ${e}`)
      }
    }

    customSignAndSend()
      .then((block) => {
        // Handle success here if necessary
        logger.info('Transaction sent successfully', block)
      })
      .catch((err) => {
        // Handle uncaught errors here if necessary
        logger.error('Unhandled error:', err)
      })
  }

  async submitSingleOrder(
    order: Order,
    nonce: number,
    speedMode: number = 1,
  ) {
    const sdk = this.circuitClient.sdk
    let txSize = 0

    logger.info(`Submitting ${order.count} transaction with nonce ${nonce}`)
    async function customSignAndSend(client) {
      try {
        const tx = client.sdk.client.tx.vacuum.singleOrder(
          order.target,
          order.asset,
          order.amount,
          order.rewardAsset,
          order.maxReward,
          order.insurance,
          order.targetAccount,
          speedMode,
        )
        txSize += tx.encodedLength
        const res = await sdk.circuit.tx.signAndSend(tx, { nonce })
        logger.info(`Transaction included in block ${res}`)
        return res.status.inBlock
      } catch (e) {
        logger.error(`signAndSend failed with error: ${e}`)
      }
    }

    this.prometheus.txSize.set({target: order.target}, txSize)

    for (let i = 0; i < order.count; i++) {
      customSignAndSend(this.circuitClient)
        .then((blockHash) => {
          // Handle success here if necessary
          logger.info(`Transaction sent successfully in ${blockHash}`)
          this.prometheus.submissions.inc({
            target: order.target,
            status: 'success',
            type: order.txType,
          })
        })
        .catch((err) => {
          // Handle uncaught errors here if necessary
          logger.error('Unhandled error:', err)
          this.prometheus.submissions.inc({
            target: order.target,
            status: 'failure',
            type: order.txType,
          })
        })
      nonce += 1
    }
  }
}
