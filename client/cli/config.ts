export default {
    circuit: {
        rpc: "ws://127.0.0.1:9944",
    },
    gateways: [
        {
            name: "Rococo",
            id: "roco",
            rpc: "wss://rococo-rpc.polkadot.io",
            subscan: "https://rococo.api.subscan.io",
            registrationData: {
                relaychain: null,
                gatewayConfig: {
                    blockNumberTypeSize: 32,
                    hashSize: 32,
                    hasher: "Blake2",
                    crypto: "sr25519",
                    addressLength: 32,
                    valueTypeSize: 8,
                    decimals: 12,
                    structs: []
                },
                gatewayVendor: "Substrate",
                gatewayType: { ProgrammableExternal: 1 },
                gatewaySysProps: {
                    tokenSymbol: "ROC",
                    tokenDecimals: 12,
                    ss58Format: 60
                },
                allowedSideEffects: ["tran"]
            }
        }
    ]
}