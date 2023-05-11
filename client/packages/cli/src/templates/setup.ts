const CIRCUIT_WS_ENDPOINT = process.env.CIRCUIT_WS_ENDPOINT || "ws://127.0.0.1:9944"
const CIRCUIT_RPC_ENDPOINT = process.env.CIRCUIT_RPC_ENDPOINT || "http://127.0.0.1:9933"

export default {
  circuit: {
    ws: CIRCUIT_WS_ENDPOINT,
    http: CIRCUIT_RPC_ENDPOINT,
    decimals: 12,
    valueTypeSize: 16, //bytes
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
        owner:
          "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        allowedSideEffects: [["tran", 4]],
        verificationVendor: "Rococo",
        executionVendor: "Substrate",
        runtimeCodec: "Scale",
        tokenInfo: {
          Substrate: {
            symbol: "ROC",
            decimals: 12,
            id: 42,
          },
        },
      },
    },
    {
      name: "Rockmine",
      id: "mine",
      tokenId: "mine",
      rpc: "wss://rococo-rockmine-rpc.polkadot.io",
      transferData: {
        receiver: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        fee: 0,
      },
      registrationData: {
        owner:
          "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        parachain: {
          relayChainId: "roco",
          id: 1000,
        },
        verificationVendor: "Rococo",
        executionVendor: "Substrate",
        runtimeCodec: "Scale",
        tokenInfo: {
          Substrate: {
            symbol: "ROC",
            decimals: 12,
            id: 42,
          },
        },
        allowedSideEffects: [["tran", 4]],
      },
    },
    {
      name: "Ethereum",
      id: "eth2",
      tokenId: "eth2",
      transferData: {
        receiver: "0x1234567890123456789012345678901234567890",
        fee: 0,
      },
      registrationData: {
        owner:
          "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        verificationVendor: "Ethereum",
        executionVendor: "EVM",
        runtimeCodec: "RLP",
        tokenInfo: {
          Ethereum: {
            symbol: "eth",
            decimals: 18,
          },
        },
        allowedSideEffects: [["tran", 4]],
      },
    },
  ],
}
