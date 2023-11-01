import { ApiPromise } from "@t3rn/sdk"

export class Order {
  dest: string
  asset: number
  targetAccount: string
  amount: number
  maxReward: number
  rewardAsset: number
  insurance: number
  remote_origin_nonce: number

  constructor(
    dest: string,
    asset: number,
    targetAccount: string,
    amount: number,
    maxReward: number,
    rewardAsset: number,
    insurance: number,
    remote_origin_nonce: number,
  ) {
    this.dest = dest
    this.asset = asset
    this.targetAccount = targetAccount
    this.amount = amount
    this.maxReward = maxReward
    this.rewardAsset = rewardAsset
    this.insurance = insurance
    this.remote_origin_nonce = remote_origin_nonce
  }
}

export function build_tx_vacuum_multi_order(
  circuit: ApiPromise,
  batchOrders: Order[],
  speedMode: number,
) {
  return circuit.tx.vacuum.order(
    batchOrders.map((order: Order) => {
      return {
        sfx_action: {
          Transfer: [
            order.dest,
            order.asset,
            order.targetAccount,
            order.amount,
          ],
        },
        max_reward: order.maxReward,
        reward_asset: order.rewardAsset,
        insurance: order.insurance,
        remote_origin_nonce: order.remote_origin_nonce,
      }
    }),
    speedMode,
  )
}

export function build_tx_vacuum_single_order(
  circuit: ApiPromise,
  order: Order,
  speedMode: number,
) {
  return circuit.tx.vacuum.singleOrder(
    order.dest,
    order.asset,
    order.amount,
    order.rewardAsset,
    order.maxReward,
    order.insurance,
    order.targetAccount,
    speedMode,
  )
}

export function build_tx_batch_single_order(
  circuit: ApiPromise,
  batchOrders: Order[],
  speedMode: number,
) {
  return circuit.tx.createBatch(
    batchOrders.map((order: Order) => {
      circuit.tx.vacuum.singleOrder(
        order.dest,
        order.asset,
        order.amount,
        order.rewardAsset,
        order.maxReward,
        order.insurance,
        order.targetAccount,
        speedMode,
      )
    }),
  )
}
