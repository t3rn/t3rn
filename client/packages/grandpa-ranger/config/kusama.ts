export default {
 circuit: {
  rpc1: {
   ws: "wss://rpc.t0rn.io",
   http: "https://rpc.t0rn.io"
  },
 },
 target: { // we dont need to specify the http endpoint for the target
  rpc1: {
   ws: "wss://kusama-rpc.polkadot.io",
  },
  rpc2: {
   ws: "wss://kusama-rpc.dwellir.com"
  },
 },
 rangeInterval: 120, // time between range submissions in seconds
 targetGatewayId: "kusm",
}
