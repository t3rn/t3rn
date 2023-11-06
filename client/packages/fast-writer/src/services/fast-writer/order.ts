import { logger } from '../../utils/logger'
import { TxType } from './enums'
import * as CryptoJS from 'crypto-js'

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
  hash: string

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
    this.asset = asset === undefined ? 0 : (asset as number) // Use a type assertion
    this.targetAccount = targetAccount
    // TODO: this should be known from xdns.assets...decimals
    this.amount = amount * 10 ** 12
    this.maxReward = maxReward
    this.rewardAsset = rewardAsset
    this.insurance = insurance
    this.remote_origin_nonce = remote_origin_nonce
    this.count = count
    this.txType = txType

    // each order has hash to not pollute logs
    const data = JSON.stringify(this)
    this.hash = CryptoJS.SHA256(data).toString()

    logger.info({ order: this }, '💾 Created new order')
  }
}
