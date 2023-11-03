import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import { AddressOrPair } from "@polkadot/api/types"

export interface EstimateSubmittableExtrinsicParams {
  tx: SubmittableExtrinsic
  account: AddressOrPair
}

export const estimateSfxGasFee = async ({
  tx,
  account,
}: EstimateSubmittableExtrinsicParams) => {
  const paymentInfo = await tx.paymentInfo(account)
  return paymentInfo.partialFee.toNumber()
}
