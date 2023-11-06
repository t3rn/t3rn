import { CircuitClient } from '../circuit/client'
import { logger } from '../../utils/logger'

enum TxType {
  Single = 'single',
  Batch = 'batch',
}

export class Order {
  target: string
  asset: number | null
  targetAccount: string
  amount: number
  maxReward: number
  rewardAsset: number
  insurance: number
  remote_origin_nonce: number
  count: number
  txType: TxType

  constructor(
    target: string,
    asset: number | undefined,
    targetAccount: string,
    amount: number,
    maxReward: number,
    rewardAsset: number,
    insurance: number,
    remote_origin_nonce: number,
    count: number,
    txType: TxType,
  ) {
    this.target = target
    this.asset = asset === undefined ? null : (asset as number) // Use a type assertion
    this.targetAccount = targetAccount
    this.amount = amount * 10 ** 12
    this.maxReward = maxReward
    this.rewardAsset = rewardAsset
    this.insurance = insurance
    this.remote_origin_nonce = remote_origin_nonce
    this.count = count
    this.txType = txType
  }
  // xdns.token = roco

  execute(circuitClient: CircuitClient, order: Order, nonce) {
    if (this.txType == TxType.Single) {
      this.submitSingleOrder(circuitClient, order, nonce)
    } else if (this.txType == TxType.Batch) {
      this.submitBatchOrder(circuitClient, order, nonce)
    } else {
      logger.error('Invalid txType', this.txType)
    }
  }

  async submitBatchOrder(
    client: CircuitClient,
    order: Order,
    nonce: number,
    speedMode: number = 1,
  ) {
    const transactions = []
    const sdk = client.sdk

    for (let i = 0; i < order.count; i++) {
      // amount is increased by 1 for each order to avoid SetupFailedDuplicatedXtx
      const transaction = client.sdk.client.tx.vacuum.singleOrder(
        order.target,
        order.asset,
        order.amount + i,
        order.rewardAsset,
        order.maxReward,
        order.insurance,
        order.targetAccount,
        speedMode,
      ) as any
      transactions.push(transaction as never)
    }

    async function customSignAndSend() {
      try {
        const tx = sdk.circuit.tx.createBatch(transactions)
        const res = await sdk.circuit.tx.signAndSend(tx, { nonce })
        logger.info(
          `Transaction included in block ${res.status.asFinalized.toString()}`,
        )
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
        console.error('Unhandled error:', err)
      })
  }

  async submitSingleOrder(
    client: CircuitClient,
    order: Order,
    nonce: number,
    speedMode: number = 1,
  ) {
    const transactions = []
    const sdk = client.sdk

    // amount is increased by 1 for each order to avoid SetupFailedDuplicatedXtx
    const tx = client.sdk.client.tx.vacuum.singleOrder(
      order.target,
      order.asset,
      order.amount,
      order.rewardAsset,
      order.maxReward,
      order.insurance,
      order.targetAccount,
      speedMode,
    ) as any

    async function customSignAndSend() {
      try {
        const res = await sdk.circuit.tx.signAndSend(tx, { nonce })
        logger.info(
          `Transaction included in block ${res.status.asFinalized.toString()}`,
        )
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
        console.error('Unhandled error:', err)
      })
  }
}
