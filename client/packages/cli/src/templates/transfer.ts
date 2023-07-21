const SFX_TRAN_TO = process.env.SFX_TRAN_TO || "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ"
const SFX_TRAN_TARGET = process.env.SFX_TRAN_TARGET || "roco"
const SFX_TRAN_AMOUNT = Number(process.env.SFX_TRAN_AMOUNT) || 1
const SFX_TRAN_INSURANCE = process.env.SFX_TRAN_INSURANCE || "0.1"
const SFX_TRAN_MAX_REWARD = process.env.SFX_TRAN_MAX_REWARD || "40"
const SFX_TRAN_SIGNATURE = process.env.SFX_TRAN_SIGNATURE || "0x"
const SFX_TRAN_SPEED_MODE = process.env.SFX_TRAN_SPEED_MODE || "Fast"

export default {
  sideEffects: [
    {
      target: SFX_TRAN_TARGET,
      maxReward: SFX_TRAN_MAX_REWARD,
      insurance: SFX_TRAN_INSURANCE,
      action: "tran",
      encodedArgs: [
        {
          to: SFX_TRAN_TO,
          amount: SFX_TRAN_AMOUNT,
        },
      ],
      signature: SFX_TRAN_SIGNATURE,
      enforceExecutor: null,
      rewardAssetId: null, // defaults to TRN
    },
  ],
  speed_mode: SFX_TRAN_SPEED_MODE,
}
