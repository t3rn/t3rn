export default {
 circuit: {
  rpc1: {
   ws: "wss://ws.t0rn.io",
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
 rangeInterval: 120, // time between range submissions in seconds
 targetGatewayId: "roco",
 circuitSigner: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561"
}
