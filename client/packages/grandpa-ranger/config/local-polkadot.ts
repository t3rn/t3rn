export default {
 circuit: {
  rpc1: {
   ws: "ws://localhost:9944",
   http: "http://localhost:9944"
  },
  rpc2: {
   ws: "ws://localhost:9944",
   http: "http://localhost:9944"
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
 rangeInterval: 0, // time between range submissions in seconds
 quickSyncLimit: 200, // for more than 200 blocks behind, use quick sync
 targetGatewayId: "pdot",
 bridgeName: "polkadotBridge",
 batches_max: 1,
 batching: true,
}
