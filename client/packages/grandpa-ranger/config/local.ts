export interface Config {
	circuit: {
		rpc1: {
			ws: string,
			http: string,
		},
		rpc2: {
			ws: string,
			http: string,
		},
		signer: string,
	},
	target: {
		rpc1: { ws: string, }
		rpc2: { ws: string, }
	},
	rangeBreak: number,
	targetGatewayId: string
	circuitSigner: string,
}

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
		signer: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561",
	},
	target: { // we dont need to specify the http endpoint for the target
		rpc1: {
			ws: "wss://rococo-rpc.polkadot.io",
		},
		rpc2: {
			ws: "wss://rococo-community-rpc.laminar.codes/ws"
		},
	},
	rangeBreak: 10, // time between range submissions in seconds
	targetGatewayId: "roco",
	circuitSigner: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561"
}