export default {
  sideEffects: [
    {
      target: "roco",
      maxReward: "40",
      insurance: "0.1",
      action: "tran",
      encodedArgs: [
        {
          from: "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ",
          to: "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ",
        },
      ],
      signature: "0x",
      enforceExecutor: null,
      rewardAssetId: null, // defaults to TRN
    },
  ],
  speed_mode: "Fast",
}
