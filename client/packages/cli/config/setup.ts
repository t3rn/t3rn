export default {
    circuit: {
        rpc: "ws://127.0.0.1:9944",
        decimals: 12,
        valueTypeSize: 16 //bytes
    },
    gateways: [
        {
            name: "Rococo",
            id: "roco",
            rpc: "wss://rococo-rpc.polkadot.io",
            subscan: "https://rococo.api.subscan.io",
            tokenId: "roco",
            transferData: {
                receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                allowedSideEffects: [["tran", 4]],
                gatewayVendor: "Rococo",
                gatewaySysProps: {
                    tokenSymbol: "ROC",
                    tokenDecimals: 12,
                    ss58Format: 42
                },
            }
        },
        {
            name: "Rockmine",
            id: "mine",
            token_id: "mine",
            rpc: "wss://rococo-rockmine-rpc.polkadot.io",
            transferData: {
                receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                fee: 0,
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                parachain: {
                    relayChainId: "roco",
                    id: 1000
                },
                gatewayVendor: "Rococo",
                gatewaySysProps: {
                    tokenSymbol: "ROC",
                    tokenDecimals: 12,
                    ss58Format: 42
                },
                allowedSideEffects: [["tran", 4]],
            }
        }
    ]
}