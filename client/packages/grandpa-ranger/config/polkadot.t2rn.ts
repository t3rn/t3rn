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
 prometheusPort: 8084,
 rangeInterval: 6, // time between range submissions in seconds
 targetGatewayId: "pdot",
 batches_max: 3,
 batching: false,
}
