export default {
 circuit: {
  rpc1: {
   ws: "wss://rpc.t2rn.io",
   http: "https://rpc.t2rn.io"
  },
  rpc2: {
   ws: "wss://rpc.t2rn.io",
   http: "https://rpc.t2rn.io"
  },
 },
 target: { // we dont need to specify the http endpoint for the target
  rpc1: {
   ws: "wss://rpc.polkadot.io",
  },
  rpc2: {
   ws: "wss://polkadot-rpc.dwellir.com"
  },
 },
 rangeInterval: 5, // time between range submissions in seconds
 quickSyncLimit: 0, // for more than 200 blocks behind, use quick sync
 targetGatewayId: "pdot",
 bridgeName: "polkadotBridge",
 batches_max: 1,
 batching: true,
}
