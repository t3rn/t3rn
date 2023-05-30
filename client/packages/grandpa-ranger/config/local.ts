export default {
	circuit: {
		rpc1: {
			ws: "ws://localhost:9944",
			http: "http://localhost:9933"
		},
		rpc2: {
			ws: "ws://localhost:9944",
			http: "http://localhost:9933"
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
	rangeInterval: 10, // time between range submissions in seconds
	targetGatewayId: "roco",
	circuitSigner: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561"
}