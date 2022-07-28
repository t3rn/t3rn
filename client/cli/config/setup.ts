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
            transferData: {
                receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                fee: 0,
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
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
                gatewayVendor: "Rococo",
                gatewayType: { ProgrammableExternal: 1 },
                gatewaySysProps: {
                    tokenSymbol: "ROC",
                    tokenDecimals: 12,
                    ss58Format: 42
                },
                allowedSideEffects: ["tran"],
                parachain: null
            }
        },
        {
            name: "Pangolin",
            id: "pang",
            rpc: "wss://pangolin-parachain-rpc.darwinia.network",
            subscan: "https://pangolin-parachain.api.subscan.io",
            transferData: {
                receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                fee: 0,
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                parachain: {
                    relayChainId: "roco",
                    id: 2105
                },
                gatewayConfig: {
                    blockNumberTypeSize: 32,
                    hashSize: 32,
                    hasher: "Blake2",
                    crypto: "sr25519",
                    addressLength: 32,
                    valueTypeSize: 8,
                    decimals: 18,
                    structs: []
                },
                gatewayVendor: "Rococo",
                gatewayType: { ProgrammableExternal: 1 },
                gatewaySysProps: {
                    tokenSymbol: "PRING",
                    tokenDecimals: 12,
                    ss58Format: 42
                },
                allowedSideEffects: ["tran"],
            }
        }
    ]
}