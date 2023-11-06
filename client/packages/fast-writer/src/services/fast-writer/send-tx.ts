import { TxType } from "./enums"

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
    this.asset = asset === undefined ? 0 : (asset as number) // Use a type assertion
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

}
