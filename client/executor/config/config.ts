export default {
    circuit: {
        rpc: "ws://127.0.0.1::9944"
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
        }
    },
    priceSource: {
        coingecko: {
            endpoint: "https://api.coingecko.com/api/v3/coins/"
        }
    }
}