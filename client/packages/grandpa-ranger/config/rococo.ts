export default {
 circuit: {
  rpc1: {
   ws: "wss://rpc.t0rn.io",
   http: "https://rpc.t0rn.io"
  },
  rpc2: {
   ws: "wss://rpc.t0rn.io",
   http: "https://rpc.t0rn.io"
  },
 },
 target: { // we dont need to specify the http endpoint for the target
  rpc1: {
   ws: "wss://rococo-rpc.polkadot.io",
  },
  rpc2: {
   ws: "wss://rococo-community-rpc.laminar.codes/ws"
  },
 },
 rangeInterval: 0, // time between range submissions in seconds
 targetGatewayId: "roco",
 quickSyncLimit: 200, // for more than 200 blocks behind, use quick sync
 batches_max: 10,
 batching: true,
}
