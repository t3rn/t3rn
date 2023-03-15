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
            transferData: {
                receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                gatewayConfig: {
                    blockNumberTypeSize: 32,
                    hashSize: 32,
                    hasher: "Blake2",
                    crypto: "sr25519",
                    addressLength: 32,
                    valueTypeSize: 16,
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
            }
        },
        {
            name: "Rockmine",
            id: "mine",
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
                    tokenSymbol: "ROC",
                    tokenDecimals: 12,
                    ss58Format: 42
                },
                allowedSideEffects: ["tran"],
            }
        },
        {
            name: "Basilisk",
            id: "bslk",
            rpc: "wss://rococo-basilisk-rpc.hydration.dev",
            subscan: "",
            transferData: {
                receiver: "bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX",
                fee: 0,
            },
            registrationData: {
                owner: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                parachain: {
                    relayChainId: "roco",
                    id: 2090
                },
                gatewayConfig: {
                    blockNumberTypeSize: 32,
                    hashSize: 32,
                    hasher: "Blake2",
                    crypto: "sr25519",
                    addressLength: 32,
                    valueTypeSize: 16,
                    decimals: 12,
                    structs: []
                },
                gatewayVendor: "Rococo",
                gatewayType: { ProgrammableExternal: 1 },
                gatewaySysProps: {
                    tokenSymbol: "BSX",
                    tokenDecimals: 12,
                        ss58Format: 10041
                },
                allowedSideEffects: ["tran"],
            }
        }
    ]
}