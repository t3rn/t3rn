const SFX_XTRAN_BENEFICIARY =
  process.env.SFX_XTRAN_BENEFICIARY ||
  '0xfc68ae55f42dcfd8060f1f67ec3c68a7dc3bce702f1ddb3d3551baf4e52f1a7d'
const SFX_XTRAN_TARGET = process.env.SFX_XTRAN_TARGET || 'roco'
const SFX_XTRAN_ASSET = process.env.SFX_XTRAN_ASSET || 'ROC'
const SFX_XTRAN_TYPE = process.env.SFX_XTRAN_TYPE || 'relay'
const SFX_XTRAN_CHAIN_ID = Number(process.env.SFX_XTRAN_CHAIN_ID) || 3333
const SFX_XTRAN_AMOUNT = Number(process.env.SFX_XTRAN_AMOUNT) || 100_000_000_000
const SFX_XTRAN_INSURANCE = process.env.SFX_XTRAN_INSURANCE || '0.1'
const SFX_XTRAN_MAX_REWARD = process.env.SFX_XTRAN_MAX_REWARD || '40'
const SFX_XTRAN_SIGNATURE = process.env.SFX_XTRAN_SIGNATURE || '0x'
const SFX_XTRAN_SPEED_MODE = process.env.SFX_XTRAN_SPEED_MODE || 'Fast'

export default {
  sideEffects: [
    {
      target: SFX_XTRAN_TARGET,
      maxReward: SFX_XTRAN_MAX_REWARD,
      insurance: SFX_XTRAN_INSURANCE,
      action: 'tass',
      encodedArgs: [
        {
          destChainId: SFX_XTRAN_CHAIN_ID,
          beneficiary: SFX_XTRAN_BENEFICIARY,
          asset: SFX_XTRAN_ASSET,
          amount: SFX_XTRAN_AMOUNT,
          xTransferType: SFX_XTRAN_TYPE,
        },
      ],
      signature: SFX_XTRAN_SIGNATURE,
      enforceExecutor: null,
      rewardAssetId: null, // defaults to TRN
    },
  ],
  speed_mode: SFX_XTRAN_SPEED_MODE,
}
