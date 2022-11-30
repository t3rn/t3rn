export default {
  circuit: {
    rpc: "ws://127.0.0.1::9944",
    ticker: "TRN",
    decimals: 12,
  },
  gateways: [
    {
      name: "Rococo",
      id: "roco",
      rpc: "wss://rococo-rpc.polkadot.io",
      type: "Substrate",
      supportedAssets: ["ROC"],
    },
    {
      name: "Basilisk",
      id: "bslk",
      rpc: "wss://rpc-01.basilisk-rococo.hydradx.io",
      type: "Substrate",
      supportedAssets: ["BSX"],
    },
  ],
  assets: {
    BSX: {
      priceSource: "coingecko",
      id: "basilisk",
    },
    ROC: {
      priceSource: "coingecko",
      id: "polkadot", // this is the id of the coin on the source
    },
    TRN: {
      priceSource: "coingecko",
      id: "tether",
    },
  },
  priceSource: {
    coingecko: {
      endpoint: "https://api.coingecko.com/api/v3/coins/",
    },
  },
  strategies: {
    roco: {
      sfx: {
        minProfitUsd: 3,
        minYield: 0.05,
      },
      xtx: {
        minInsuranceAmountUsd: 1,
        minInsuranceShare: 0.1, // insurance / maxReward
      }
    },
  },
}
