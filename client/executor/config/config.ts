export default {
    circuit: {
        rpc: "ws://127.0.0.1::9944",
        ticker: "TRN",
        decimals: 12
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
            supportedAssets: ["BSX"]
        }
    ],
    assets: {
        "BSX": {
            priceSource: "coingecko",
            id: "basilisk",
        },
        "ROC": {
            priceSource: "coingecko",
            id: "polkadot"
        },
        "TRN": {
            priceSource: "coingecko",
            id: "tether"
        }
    },
    priceSource: {
        coingecko: {
            endpoint: "https://api.coingecko.com/api/v3/coins/"
        }
    },
    strategies: {
        "roco": {
            minProfitUsd: 1,
            minYield: 0.05,
        },
        "bslk": {
            minProfitUsd: 1,
            minYield: 0.03,
            maxTxFeesUsd: 1,
            maxTxFeeShare: 10,
        }
    }
}